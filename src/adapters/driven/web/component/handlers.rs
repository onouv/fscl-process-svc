use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

use crate::{
    adapters::driven::web::app_state::AppState,
    ports::component_port::{
        ComponentApplicationError, ComponentPort, NewComponentRequest, RequestBuildError,
    },
};

use super::requests::CreateComponentHttpRequestBody;
use super::responses::ComponentResponse;

pub async fn create_component<C>(
    State(state): State<AppState<C>>,
    Json(request): Json<CreateComponentHttpRequestBody>,
) -> impl IntoResponse
where
    C: ComponentPort + Send + Sync + 'static,
{
    let svc = state.component_service.clone();

    let application_req = NewComponentRequest::new(
        request.id,
        request.name,
        request.description,
        request.parent_id,
    );

    let res = svc.new_component(request).await;
    Json(ComponentResponse { id: None })
}

impl IntoResponse for ComponentApplicationError {
    fn into_response(self) -> axum::response::Response {}
}

impl IntoResponse for RequestBuildError {
    fn into_response(self) -> axum::response::Response {}
}
