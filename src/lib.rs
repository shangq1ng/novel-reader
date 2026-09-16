pub mod auth;
pub mod config;
pub mod database;
pub mod error;

use crate::auth::google::{google_client, handle_google_callback, start_google_auth};
use crate::auth::session::service::start_redis;
use crate::config::auth::AuthConfig;
use crate::config::configs::Config;
use crate::database::handler::get_user;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn app() -> anyhow::Result<()> {
    dotenv().ok(); // should change to a match block to catch this error early.

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,tower_http=debug,tower_client=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let session = start_redis().await?;

    let google = google_client().await?;

    let config = Config::new().await?;

    let state: AuthConfig = AuthConfig {
        config,
        oauth_client: google.clone(),
    };

    let config = Config::new().await?;

    let router: Router = Router::new()
        .route("/login", get(start_google_auth))
        .route("/login/callback", get(handle_google_callback))
        .route("/profile", get(get_user))
        .layer(TraceLayer::new_for_http())
        .layer(session)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", config.port)).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
