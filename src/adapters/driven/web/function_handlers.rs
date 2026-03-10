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


