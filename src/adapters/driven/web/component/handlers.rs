use crate::{
    adapters::driven::web::{
        app_state::AppState,
        responses::{ApiError, ApiSuccess},
    },
    domain::Component,
    ports::{ComponentApplicationError, ComponentPort, NewComponentRequest},
};
use axum::{extract::State, http::StatusCode, response::Json};

use super::requests::CreateComponentHttpRequestBody;
use super::responses::CreateComponentResponse;

pub async fn create_component<C>(
    State(state): State<AppState<C>>,
    Json(request): Json<CreateComponentHttpRequestBody>,
) -> Result<ApiSuccess<CreateComponentResponse>, ApiError>
where
    C: ComponentPort + Send + Sync + 'static,
{
    
        let application_req = NewComponentRequest::new(
            request.id,
            request.name,
            request.description,
            request.parent_id,
        )?;

        let component = state.component_service.new_component(application_req).await;

        match component {
            Ok(c) => { 
                Ok(ApiSuccess::new(StatusCode::CREATED, CreateComponentResponse::from(&c)))
            },
            Err(app_error) => {
                let api_error = match app_error {
                    ComponentApplicationError::ResourceIdDuplicate { id } => {
                        ApiError::Conflict(format!("{} gibts schon", id.to_string()))
                    },
                    _ => {
                        ApiError::InternalServerError("weiss nich".to_string())
                    }
                };

                Err(api_error)
            }
        }
    
}