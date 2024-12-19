use axum::{routing::post, Router};
use registry::AppRegistory;

use crate::handler::auth::{login, logout};

pub fn routes() -> Router<AppRegistory> {
    let auth_router = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));
    Router::new().nest("/auth", auth_router)
}
