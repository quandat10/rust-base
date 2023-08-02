use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info};

pub async fn postgresql_conn(database_url: String) -> Pool<Postgres> {
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    return pool;
}
