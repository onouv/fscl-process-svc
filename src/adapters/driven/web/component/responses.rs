use axum::{http::StatusCode, response::IntoResponse};

use crate::{adapters::driven::web::responses::ApiSuccess, domain::{Component, Item}};

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub(crate) struct CreateComponentResponse {
    pub id: Option<String>,
}

impl From<&Component> for CreateComponentResponse {
    fn from(component: &Component) -> Self {
        Self { id: Some(component.id().to_string()) }
    }
} 

impl IntoResponse for ApiSuccess<CreateComponentResponse> {
    fn into_response(self) -> axum::response::Response {
        self.into_response()
    }
}