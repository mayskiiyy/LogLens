use anyhow::{Context, Result};
use loglens_api::create_router;
use loglens_storage::Database;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting LogLens self-hosted server...");

    let host = std::env::var("LOGLENS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("LOGLENS_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .context("Invalid LOGLENS_PORT value")?;

    let db_url = std::env::var("LOGLENS_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///data/loglens.db".to_string());

    tracing::info!("Connecting to database at {}", db_url);
    let db = Database::new_sqlite(&db_url)
        .await
        .context("Failed to initialize database")?;

    let app = create_router(db).fallback_service(ServeDir::new("apps/web/build"));

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .context("Failed to parse socket address")?;

    tracing::info!("LogLens listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shut down gracefully.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
