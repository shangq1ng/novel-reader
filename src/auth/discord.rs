use crate::auth::models::{ProviderResponse, ProviderUserResponse};
use crate::config::auth::Client;
use crate::config::configs::Config;
use crate::error::auth::AuthError;
use anyhow::Context;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl, reqwest,
};
use subtle::ConstantTimeEq;
use tower_sessions::Session;

pub async fn discord_client() -> Result<Client, AuthError> {
    let client_id =
        std::env::var("DISCORD_CLIENT_ID").context("Missing env var DISCORD_CLIENT_ID")?;
    let client_secret =
        std::env::var("DISCORD_CLIENT_SECRET").context("Missing env var DISCORD_CLIENT_SECRET")?;
    let auth_url = std::env::var("AUTH_URL").context("Missing env var AUTH_URL")?;
    let token_url = std::env::var("AUTH_TOKEN").context("Missing env var AUTH_TOKEN")?;
    let redirect_url = std::env::var("REDIRECT_URL").context("Missing env var redirect_URL")?;

    let discord_client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(auth_url)?)
        .set_token_uri(TokenUrl::new(token_url)?)
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    Ok(discord_client)
}

pub async fn start_discord_auth(
    State(client): State<Client>,
    session: Session,
) -> Result<impl IntoResponse, AuthError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    session.insert("pkce_verifier", &pkce_verifier).await?;

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    let token = csrf_token.secret();
    session.insert("csrf_token", &token).await?;

    Ok(Redirect::to(auth_url.as_ref()))
}

pub async fn validate_csrf_token(
    res: &ProviderResponse,
    session: &Session,
) -> Result<(), AuthError> {
    let csrf_token: String = session
        .get("csrf_token")
        .await
        .map_err(|e| AuthError::InternalServerError(e.into()))?
        .ok_or(AuthError::NotFound)?;

    let is_valid: bool = res.state.as_bytes().ct_eq(csrf_token.as_bytes()).into();

    if !is_valid {
        return Err(AuthError::Unauthorized);
    }
    session.remove::<String>("csrf_token").await?;

    Ok(())
}

pub async fn handle_discord_callback(
    Query(payload): Query<ProviderResponse>,
    State(client): State<Client>,
    State(state): State<Config>,
    session: Session,
) -> Result<impl IntoResponse, AuthError> {
    validate_csrf_token(&payload, &session).await?;

    let pkce_code_verifier: String = session
        .remove("pkce_verifier")
        .await
        .map_err(|e| AuthError::InternalServerError(e.into()))?
        .ok_or(AuthError::NotFound)?;

    let pkce_verifier = PkceCodeVerifier::new(pkce_code_verifier);

    let http_client = reqwest::Client::new();

    let token = client
        .exchange_code(AuthorizationCode::new(payload.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    let data: ProviderUserResponse = http_client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json::<ProviderUserResponse>()
        .await?;
    session.cycle_id().await?;

    let provider = "discord".to_string();

    sqlx::query_as!(
        UserEntity,
        "INSERT INTO users (provider, username, display_name, avatar_url)\
        VALUES ($1, $2, $3, $4)",
        provider,
        data.username,
        data.global_name,
        data.avatar,
    )
    .execute(&state.db)
    .await?;

    Ok(Redirect::to("/"))
}
