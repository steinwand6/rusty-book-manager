use sqlx::{postgres::PgConnectOptions, PgPool};

use shared::config::DatabaseConfig;

pub fn get_db_connection(cfg: &DatabaseConfig) -> PgPool {
    let options = PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database);
    PgPool::connect_lazy_with(options)
}
