use std::{env, sync::Arc};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info};

pub struct DB(pub(crate) Arc<Pool<Postgres>>);

impl DB {
    pub async fn new() -> DB {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(
                &env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!")),
            )
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });

        DB(Arc::new(pool))
    }
}
