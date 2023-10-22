use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use tracing::error;

pub enum ApiError {
    NotImplemented { message: String },
    NotFound { message: String },
    InternalServerError { message: String },
    BadRequest { message: String },
}

impl ApiError {
    pub fn not_implemented(message: impl Into<String>) -> Self {
        ApiError::NotImplemented {
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        ApiError::NotFound {
            message: message.into(),
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        ApiError::InternalServerError {
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        ApiError::BadRequest {
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotImplemented { message } => {
                error!("Not implemented: {}", message);
                (StatusCode::NOT_IMPLEMENTED, message).into_response()
            }
            ApiError::NotFound { message } => {
                error!("Not found: {}", message);
                (StatusCode::NOT_FOUND, message).into_response()
            }
            ApiError::InternalServerError { message } => {
                error!("Internal server error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            ApiError::BadRequest { message } => {
                error!("Bad request: {}", message);
                (StatusCode::BAD_REQUEST, message).into_response()
            }
        }
    }
}
