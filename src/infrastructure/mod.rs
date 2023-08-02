use crate::infrastructure::{
    config::Config,
    repository::database::{posgresql::postgresql_conn, redis::redis_conn},
};
use dotenv::dotenv;
use redis::Client;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tracing::info;

use self::rest::router::router;
use std::net::SocketAddr;

pub mod config;
pub mod repository;
pub mod rest;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
    redis: Client,
}

pub async fn server() {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let config = Config::init();
    let port: u16 = config.port.to_owned();

    let app_state = Arc::new(AppState {
        db: postgresql_conn(config.database_url.to_owned()).await,
        redis: redis_conn(config.redis_url.to_owned()).await,
        env: config,
    });

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("🚀 Server started successfully, listening on {}", &address);

    axum::Server::bind(&address)
        .serve(router(app_state).into_make_service())
        .await
        .expect("Failed to start server")
}
