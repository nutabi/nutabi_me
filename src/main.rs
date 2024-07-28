use std::env;

use anyhow::Context;
use sqlx::SqlitePool;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load debug environment variables
    // this allows for fast iteration in development across multiple machines
    // in prod, all vars will be set prior to deployment
    if cfg!(debug_assertions) {
        dotenv::dotenv().ok();
    }

    // initialise tracing subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter::EnvFilter::from_default_env())
        .init();

    // start database connection pool
    let db_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let db_pool = SqlitePool::connect(&db_url).await.context("The application cannot establish a database connection")?;
    
    // serve the application
    http::serve(db_pool).await?;
    Ok(())
}
