use axum::Json;

use crate::api::error::ApiError;

use super::{request::ItemRequest, response::ItemsResponse};

pub async fn get(Json(_): Json<ItemRequest>) -> Result<ItemsResponse, ApiError> {
    todo!();
}
