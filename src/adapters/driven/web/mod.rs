use actix_web::web;
use serde::{Serialize, Deserialize};

mod component_handlers;
pub use component_handlers::*;

pub mod http_server;

#[derive(Serialize, Deserialize)]
pub struct AddSubRequest {
    pub child_id: String,
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
           .service(
                web::scope("/components")
                    .route("", web::post().to(create_component))
                    /*
                    .route("", web::get().to(list_components))
                    .route("/{id}", web::get().to(get_component))
                    .route("/{id}/subs", web::post().to(add_component_sub))
                    .route("/{id}/implements", web::post().to(implement_function)),
                    */
            ),
    );
}
