use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::repository::Repository;

#[derive(Serialize, Deserialize)]
pub struct CreateFunctionRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub parent_id: Option<String>,
}



/*
// ===== Function Handlers =====

pub async fn create_function(
    repo: web::Data<Repository>,
    req: web::Json<CreateFunctionRequest>,
) -> impl Responder {
    match repo
        .create_function(
            &req.id,
            &req.name,
            &req.description,
            req.parent_id.as_deref(),
        )
        .await
    {
        Ok(func) => HttpResponse::Created().json(func),
        Err(e) => {
            log::error!("Failed to create function: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn get_function(
    repo: web::Data<Repository>,
    id: web::Path<String>,
) -> impl Responder {
    match repo.get_function(&id).await {
        Ok(Some(func)) => HttpResponse::Ok().json(func),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Function not found".to_string(),
        }),
        Err(e) => {
            log::error!("Failed to get function: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn list_functions(
    repo: web::Data<Repository>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(20);
    let offset = query
        .get("offset")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    match repo.list_functions(limit, offset).await {
        Ok((items, total)) => HttpResponse::Ok().json(serde_json::json!({
            "items": items,
            "total": total,
            "limit": limit,
            "offset": offset,
        })),
        Err(e) => {
            log::error!("Failed to list functions: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn add_function_sub(
    repo: web::Data<Repository>,
    parent_id: web::Path<String>,
    req: web::Json<AddSubRequest>,
) -> impl Responder {
    match repo.add_function_sub(&parent_id, &req.child_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success"})),
        Err(e) => {
            log::error!("Failed to add function sub: {}", e);
            let status = match &e {
                crate::repository::RepositoryError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                crate::repository::RepositoryError::Conflict(_) => {
                    actix_web::http::StatusCode::CONFLICT
                }
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            HttpResponse::build(status).json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

// ===== Component Handlers =====

pub async fn create_component(
    repo: web::Data<ComponentService>,
    req: web::Json<CreateComponentRequest>,
) -> impl Responder {
    match repo
        .create_component(
            &req.id,
            &req.name,
            &req.description,
            req.parent_id.as_deref(),
        )
        .await
    {
        Ok(comp) => HttpResponse::Created().json(comp),
        Err(e) => {
            log::error!("Failed to create component: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn get_component(
    repo: web::Data<Repository>,
    id: web::Path<String>,
) -> impl Responder {
    match repo.get_component(&id).await {
        Ok(Some(comp)) => HttpResponse::Ok().json(comp),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Component not found".to_string(),
        }),
        Err(e) => {
            log::error!("Failed to get component: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn list_components(
    repo: web::Data<Repository>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(20);
    let offset = query
        .get("offset")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    match repo.list_components(limit, offset).await {
        Ok((items, total)) => HttpResponse::Ok().json(serde_json::json!({
            "items": items,
            "total": total,
            "limit": limit,
            "offset": offset,
        })),
        Err(e) => {
            log::error!("Failed to list components: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn add_component_sub(
    repo: web::Data<Repository>,
    parent_id: web::Path<String>,
    req: web::Json<AddSubRequest>,
) -> impl Responder {
    match repo.add_component_sub(&parent_id, &req.child_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success"})),
        Err(e) => {
            log::error!("Failed to add component sub: {}", e);
            let status = match &e {
                crate::repository::RepositoryError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                crate::repository::RepositoryError::Conflict(_) => {
                    actix_web::http::StatusCode::CONFLICT
                }
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            HttpResponse::build(status).json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

pub async fn implement_function(
    repo: web::Data<Repository>,
    component_id: web::Path<String>,
    req: web::Json<ImplementFunctionRequest>,
) -> impl Responder {
    match repo
        .component_implements_function(&component_id, &req.function_id)
        .await
    {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"status": "success"})),
        Err(e) => {
            log::error!("Failed to implement function: {}", e);
            let status = match &e {
                crate::repository::RepositoryError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                crate::repository::RepositoryError::Conflict(_) => {
                    actix_web::http::StatusCode::CONFLICT
                }
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            HttpResponse::build(status).json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}
*/
