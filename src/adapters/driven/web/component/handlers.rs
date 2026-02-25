use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};

use crate::{adapters::driven::web::app_state::AppState, domain::item::ItemId, ports::component_port::ComponentPort};

use super::requests::CreateComponentRequest;
use super::responses::ComponentResponse;

pub async fn create_component<C>(
   State(state): State<AppState<C>> ,
   Json(request): Json<CreateComponentRequest>
) -> impl IntoResponse
where
    C: ComponentPort + Send + Sync + 'static
{

    Json(ComponentResponse { id: None })
}