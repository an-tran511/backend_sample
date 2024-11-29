use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListUomsError {
    #[error("internal_server_error")]
    InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListUomsError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            ListUomsError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (
            status,
            Json(json!({
              "ok": false,
              "code": code,
            })),
        )
            .into_response()
    }
}
