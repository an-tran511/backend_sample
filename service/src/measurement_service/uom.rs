use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use command::uom::{CreateUomCommand, CreateUomError};
use def::state::AppState;
use query::uom::{ListUomsError, ListUomsQuery};
use serde_json::json;
use std::sync::Arc;

pub async fn list_uoms(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListUomsQuery>,
) -> Result<impl IntoResponse, ListUomsError> {
    let query = ListUomsQuery {
        page: params.page,
        per_page: params.per_page,
    };
    let uoms = state.query_bus.dispatch(query).await?;

    Ok((StatusCode::OK, Json(uoms)))
}

pub async fn create_uom(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUomCommand>,
) -> Result<impl IntoResponse, CreateUomError> {
    let command = CreateUomCommand { name: payload.name };
    match state.command_bus.dispatch(command).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(json!({ "ok": true })))),
        Err(error) => Err(error),
    }
}
