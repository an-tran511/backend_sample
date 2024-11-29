use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use def::error::uom::ListUomsError;
use def::extractor::uom::UomQueryParams;
use def::state::AppState;
use def::util::{ListResourceResponse, PaginationMeta};
use entity::uom::Entity as Uom;
use sea_orm::{EntityTrait, PaginatorTrait};
use std::sync::Arc;

pub async fn list_uoms(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UomQueryParams>,
) -> Result<impl IntoResponse, ListUomsError> {
    let db = &state.db;
    let per_page = params.per_page.unwrap_or(30);
    let page = params.page.unwrap_or(1) - 1;

    let uom_pages = Uom::find().paginate(db, per_page);
    let uoms = uom_pages.fetch_page(page).await?;

    let items_and_pages = uom_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok((
        StatusCode::OK,
        Json(ListResourceResponse {
            data: uoms,
            meta: PaginationMeta {
                page: page + 1,
                total_pages,
                per_page,
                total,
            },
        }),
    ))
}
