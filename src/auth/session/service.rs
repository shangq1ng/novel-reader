use crate::error::session::SessionError;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tower_sessions_redis_store::fred::interfaces::ClientLike;
use tower_sessions_redis_store::fred::prelude::{Config, Pool};

pub async fn start_redis() -> Result<SessionManagerLayer<RedisStore<Pool>>, SessionError> {
    let client = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let config = Config::from_url(&client)?;
    let pool = Pool::new(config, None, None, None, 10)?;
    pool.connect_pool();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::new(pool);
    let session_manager = SessionManagerLayer::new(session_store)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    Ok(session_manager)
}
