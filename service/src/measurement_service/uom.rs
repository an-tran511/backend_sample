use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use def::state::AppState;
use query::uom::{ListUomsError, ListUomsQuery};
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
