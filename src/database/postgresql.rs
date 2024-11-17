use std::time::Duration;
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Result<Pool<Postgres>, Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(&database_url)
        .await?;

    println!("✅ Connection to the database is successful!");
    Ok(pool)
}
