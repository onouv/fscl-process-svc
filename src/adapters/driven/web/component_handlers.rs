use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse, Responder};

use crate::application::component_service::ComponentService;
use crate::adapters::driving::db::component_repository::ComponentRepository;

#[derive(Serialize, Deserialize)]
pub struct CreateComponentRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImplementFunctionRequest {
    pub function_id: String,
}

pub async fn create_component<T: ComponentRepository>(
    service: web::Data<ComponentService<T>>,
    req: web::Json<CreateComponentRequest>,
) -> impl Responder {
    service
        .get_ref()
        .new_component(req.into_inner())
        .await
        .map(|_| HttpResponse::Created().finish())
        .unwrap_or_else(|e| {
            log::error!("Failed to create component: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        })
}