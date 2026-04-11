use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{adapters::driven::web::responses::ApiSuccess, domain::{Component, Resource}};

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub(crate) struct CreateComponentResponse {
    pub id: Option<String>, // TODO! should not be an option. Servce should return error if id is not generated. 
}

impl From<&Component> for CreateComponentResponse {
    fn from(component: &Component) -> Self {
        Self { id: Some(component.id().to_string()) }
    }
} 

