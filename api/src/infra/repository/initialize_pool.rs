use anyhow::Result;
use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialize_pool<S: AsRef<str>>(db_url: S) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_lifetime(Duration::from_secs(60 * 5))
        .max_connections(10)
        .connect(db_url.as_ref())
        .await?;

    Ok(pool)
}
