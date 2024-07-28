use sqlx::SqlitePool;
use thiserror::Error;

mod auth;

/// Errors occurring while initialising the application
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unable to bind socket")]
    SocketError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, AppError>;

pub async fn serve(db: SqlitePool) -> Result<()> {
    Ok(())
}