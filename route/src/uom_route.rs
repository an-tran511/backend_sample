use axum::{
    routing::{get, post},
    Router,
};
use def::state::AppState;
use service::measurement_service;
use std::sync::Arc;

pub fn new() -> Router<Arc<AppState>> {
    Router::new()
        .route("/uoms/list", get(measurement_service::list_uoms))
        .route("/uoms/create", post(measurement_service::create_uom))
}
