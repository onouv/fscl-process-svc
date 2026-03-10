use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use crate::ports::{ComponentApplicationError, RequestBuildError};

pub enum ApiError {
    InternalServerError(String),
    CannotProcessItem(String),
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal server error".to_string(),
                    )),
                )
                    .into_response();
            }
            CannotProcessItem(e) => {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ApiResponseBody::new_error(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "entity cannot be processed".to_string(),
                    )),
                )
                    .into_response();
            }
            Conflict(e) => {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponseBody::new_error(
                        StatusCode::CONFLICT,
                        "Entity conflicts with server state".to_string(),
                    )),
                )
                    .into_response();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ApiResponseBody<T> {
    fn new(status: StatusCode, data: T) -> Self {
        Self {
            status_code: status.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<String> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data:  message,
        }
    }
}

pub(super) struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
} 


impl From<RequestBuildError> for ApiError {
    fn from(value: RequestBuildError) -> Self {
        match value {
            RequestBuildError::InvalidItemId(e) => {
                ApiError::CannotProcessItem(format!("invalid item id: {}", e))
            }
            RequestBuildError::InvalidParentId(e) => {
                ApiError::CannotProcessItem(format!("invalid parent id: {}", e))
            }
        }
    }
}

impl From<ComponentApplicationError> for ApiError {
    fn from(value: ComponentApplicationError) -> Self {
        match value {
            ComponentApplicationError::ItemIdDuplicate { id } => {
                ApiError::Conflict(format!("duplicate item with id {}", id.to_string()))
            }
            ComponentApplicationError::NoSuchItemId { id } => {
                ApiError::CannotProcessItem(format!("no such item with id {}", id.to_string()))
            }
            ComponentApplicationError::NoSuchParentId { id } => {
                ApiError::CannotProcessItem(format!("no such item with id {}", id.to_string()))
            }
            ComponentApplicationError::Unknown => {
                ApiError::InternalServerError("Unknown error".to_string())
            }
        }
    }
}
