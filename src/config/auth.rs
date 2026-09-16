use crate::config::configs::Config;
use axum::extract::FromRef;
use oauth2::{EndpointNotSet, EndpointSet};

pub type Client = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;
#[derive(Clone)]
pub struct AuthConfig {
    pub config: Config,
    pub oauth_client: Client,
}

impl FromRef<AuthConfig> for Client {
    fn from_ref(config: &AuthConfig) -> Self {
        config.oauth_client.clone()
    }
}

impl FromRef<AuthConfig> for Config {
    fn from_ref(config: &AuthConfig) -> Self {
        config.config.clone()
    }
}
