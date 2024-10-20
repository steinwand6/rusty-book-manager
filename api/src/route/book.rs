use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::book::{register_book, show_book, show_book_list};
use registry::AppRegistory;

pub fn build_book_routers() -> Router<AppRegistory> {
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book));

    Router::new().nest("/books", book_routers)
}
