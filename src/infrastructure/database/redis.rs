use redis::Client;
use tracing::{error, info};

pub async fn redis_conn(redis_url: String) -> Client {
    let redis_client = match Client::open(redis_url) {
        Ok(client) => {
            info!("✅Connection to the redis is successful!");
            client
        }
        Err(e) => {
            error!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    redis_client
}
