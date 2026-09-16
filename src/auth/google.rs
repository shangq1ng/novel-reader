use crate::auth::models::{GoogleUserResponse, ProviderResponse};
use crate::config::{auth::Client, configs::Config};
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

pub async fn google_client() -> Result<Client, AuthError> {
    let client_id = env::var("GOOGLE_CLIENT_ID").context("Missing env var GOOGLE_CLIENT_ID")?;
    let client_secret =
        env::var("GOOGLE_CLIENT_SECRET").context("Missing env var GOOGLE_CLIENT_SECRET")?;
    let auth_url = env::var("AUTH_URL").context("Missing env var AUTH_URL")?;
    let token_url = env::var("TOKEN_URL").context("Missing env var AUTH_TOKEN")?;
    let redirect_url = env::var("REDIRECT_URL").context("Missing env var redirect_URL")?;

    let google_client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(auth_url)?)
        .set_token_uri(TokenUrl::new(token_url)?)
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);

    Ok(google_client)
}

#[axum::debug_handler]
pub async fn start_google_auth(
    State(client): State<Client>,
    session: Session,
) -> Result<impl IntoResponse, AuthError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    session
        .insert("google_pkce_verifier", &pkce_verifier)
        .await?;

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_extra_param("prompt", "select_account consent")
        .url();

    let token = csrf_token.secret();
    session.insert("google_csrf_token", &token).await?;

    Ok(Redirect::to(auth_url.as_ref()))
}

pub async fn validate_csrf_token(
    res: &ProviderResponse,
    session: &Session,
) -> Result<(), AuthError> {
    let csrf_token: String = session
        .get("google_csrf_token")
        .await
        .map_err(|e| AuthError::InternalServerError(e.into()))?
        .ok_or(AuthError::NotFound)?;

    let is_valid: bool = res.state.as_bytes().ct_eq(csrf_token.as_bytes()).into();

    if !is_valid {
        return Err(AuthError::Unauthorized);
    }
    session.remove::<String>("google_csrf_token").await?;

    Ok(())
}

pub async fn handle_google_callback(
    State(client): State<Client>,
    State(state): State<Config>,
    Query(payload): Query<ProviderResponse>,
    session: Session,
) -> Result<impl IntoResponse, AuthError> {
    validate_csrf_token(&payload, &session).await?;

    let pkce_code_verifier: String = session
        .remove("google_pkce_verifier")
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

    // fetch user data
    let data: GoogleUserResponse = http_client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json::<GoogleUserResponse>()
        .await?;

    session.cycle_id().await?;

    session.insert("authenticated_user", session.id()).await?;

    let provider = "google".to_string();

    sqlx::query_as!(
        UserEntity,
        "INSERT INTO users (provider, email, sub, username, display_name, avatar_url)\
        VALUES ($1, $2, $3, $4, $5, $6)",
        provider,
        data.email,
        data.sub,
        data.name,
        data.name,
        data.picture
    )
    .execute(&state.db)
    .await?;

    Ok(Redirect::to("/profile"))
}
