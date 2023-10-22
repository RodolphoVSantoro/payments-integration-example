use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct TabsResponse {
    #[serde(rename = "catalogTabs")]
    pub catalog_tabs: Vec<CatalogTab>,
}

#[derive(Serialize)]
pub struct CatalogTab {
    title: String,
    pub alias: String,
    metadata: serde_json::Value,
    #[serde(rename = "channelId")]
    channel_id: String,
}

impl IntoResponse for TabsResponse {
    fn into_response(self) -> axum::response::Response {
        return (axum::http::StatusCode::OK, axum::Json(self)).into_response();
    }
}
