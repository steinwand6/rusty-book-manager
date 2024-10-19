use axum::{extract::State, http::StatusCode};
use registry::AppRegistory;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<AppRegistory>) -> StatusCode {
    if registry.health_check_repogitory().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

/*
#[cfg(test)]
mod test {
    use axum::{extract::State, http::StatusCode};
    use tokio;

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
*/
