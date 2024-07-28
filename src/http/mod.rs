use axum::Router;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod auth;

/// Errors occurring while initialising the application
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct AppState {
    db: SqlitePool,
}

/// Start the HTTP server
pub async fn serve(db: SqlitePool) -> Result<()> {
    // create application state
    let state = AppState {
        db
    };

    // create application
    let app = make_router()
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    // bind socket
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).await?;

    // start server
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

fn make_router() -> Router<AppState> {
    Router::new()
}