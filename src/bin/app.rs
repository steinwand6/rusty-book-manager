use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::Result;
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistory;
use shared::config::AppConfig;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let config = AppConfig::new()?;

    let pool = connect_database_with(&config.database);

    let registry = AppRegistory::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {addr}");

    Ok(axum::serve(listener, app).await?)
}
