use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let config = shared::config::AppConfig::new()?;

    // TODO: Refactor creation to use API layer functions instead of directly calling adapter layer functions
    let db_pool = adapter::database::get_db_connection(&config.database);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(db_pool);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {addr}");

    Ok(axum::serve(listener, app).await?)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_db(State(db_pool): State<PgPool>) -> StatusCode {
    let status = sqlx::query("select 1").fetch_one(&db_pool).await;
    match status {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod test {
    use axum::{extract::State, http::StatusCode};

    use super::*;

    #[tokio::test]
    async fn health_check_works() {
        let status_code = health_check().await;
        assert_eq!(status_code, StatusCode::OK);
    }

    #[sqlx::test]
    async fn health_check_db_works(db_pool: sqlx::PgPool) {
        let status_code = health_check_db(State(db_pool)).await;
        assert_eq!(status_code, StatusCode::OK);
    }
}
