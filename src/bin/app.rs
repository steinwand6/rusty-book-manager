use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Context, Result};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
use registry::AppRegistory;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let config = AppConfig::new()?;

    let pool = connect_database_with(&config.database);

    let registry = AppRegistory::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {addr}");

    axum::serve(listener, app)
        .await
        .context("Unexpected error in server")
        .inspect_err(
            |e| tracing::error!(error.cause_chain = ?e, error.message = %e, "Unexpected error"),
        )
}

fn init_logger() -> Result<()> {
    let default_env = match which() {
        Environment::Deveropment => "debug",
        Environment::Production => "info",
    };
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| default_env.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}
