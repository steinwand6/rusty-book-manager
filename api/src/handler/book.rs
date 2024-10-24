use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::id::BookId;

use registry::AppRegistory;
use shared::error::{AppError, AppResult};

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    State(registry): State<AppRegistory>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    State(registry): State<AppRegistory>,
) -> AppResult<Json<Vec<BookResponse>>> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
        .map(Json)
        .map_err(AppError::from)
}

pub async fn show_book(
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistory>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound("Not Found".into())),
        })
        .map_err(AppError::from)
}
