use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use thiserror::Error;
use crate::ports::{ComponentApplicationError, RequestBuildError};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("internal server error: {0}")]
    InternalServerError(String),

    #[error("cannot process resource: {0}")]
    CannotProcessResource(String),

    #[error("request in conflict with a resource: {0}")]
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            CannotProcessResource(e) => {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ApiResponseBody::new_error(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "entity cannot be processed".to_string(),
                    )),
                )
                    .into_response()
            }
            Conflict(e) => {
                (
                    StatusCode::CONFLICT,
                    Json(ApiResponseBody::new_error(
                        StatusCode::CONFLICT,
                        "Entity conflicts with server state".to_string(),
                    )),
                )
                    .into_response()
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
            RequestBuildError::InvalidId(e) => {
                ApiError::CannotProcessResource(format!("invalid resource id: {}", e))
            }
            RequestBuildError::InvalidParentId(e) => {
                ApiError::CannotProcessResource(format!("invalid parent id: {}", e))
            }
        }
    }
}

impl From<ComponentApplicationError> for ApiError {
    fn from(value: ComponentApplicationError) -> Self {
        match value {
            ComponentApplicationError::InvalidResourceId(e) => ApiError::CannotProcessResource(e),
            ComponentApplicationError::CannotProcess(e) => ApiError::CannotProcessResource(e),
            ComponentApplicationError::Infrastructure(e) => ApiError::InternalServerError(e),
        }
    }
}
