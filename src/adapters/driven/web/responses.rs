
use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use crate::ports::RequestBuildError;

pub enum ApiError {
    InternalServerError(String),
    CannotProcessItem(String),
    Conflict(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    fn new(status: StatusCode, data: T) -> Self {
        Self {
            status_code: status.as_u16(),
            data,
        }
    }
}

pub(super) struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<ApiResponseBody<T>> {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}


impl From<RequestBuildError> for ApiError {
    fn from(value: RequestBuildError) -> Self {
        match value {
            RequestBuildError::InvalidItemId(e) => {
                ApiError::CannotProcessItem(format!("invalid item id: {}", e.to_string()))
            }
            RequestBuildError::InvalidParentId(e) => {
                ApiError::CannotProcessItem(format!("invalid parent id: {}", e.to_string()))
            }
        }
    }
}