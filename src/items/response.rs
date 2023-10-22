use axum::response::IntoResponse;

use crate::model::item::Item;
use crate::model::list::List;

#[derive(serde::Serialize)]
pub struct ItemsResponse {
    pub items: Vec<Item>,
    pub lists: Vec<List>,
}

impl IntoResponse for ItemsResponse {
    fn into_response(self) -> axum::response::Response {
        return (axum::http::StatusCode::OK, axum::Json(self)).into_response();
    }
}
