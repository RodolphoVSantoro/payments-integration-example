use axum::Json;

use super::{request::TabRequest, response::TabsResponse};
use crate::api::error::ApiError;

pub async fn get(Json(_): Json<TabRequest>) -> Result<TabsResponse, ApiError> {
    todo!();
}
