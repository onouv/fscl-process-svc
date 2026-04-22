use crate::{
    adapters::driven::web::{
        app_state::AppState,
        responses::{ApiError, ApiSuccess},
    },
    ports::{ComponentPort, NewComponentRequest},
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

        let created_id = application_req.id.to_string();

        state
            .component_service
            .new_component(application_req)
            .await
            .map_err(ApiError::from)?;

        Ok(ApiSuccess::new(
            StatusCode::CREATED,
            CreateComponentResponse::from_id(created_id),
        ))
    
}