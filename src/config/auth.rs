use axum::extract::FromRef;
use oauth2::{EndpointNotSet, EndpointSet};

pub type Client = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct AuthConfig {
    pub oauth_client: Client,
}

impl FromRef<AuthConfig> for Client {
    fn from_ref(config: &AuthConfig) -> Self {
        config.oauth_client.clone()
    }
}
