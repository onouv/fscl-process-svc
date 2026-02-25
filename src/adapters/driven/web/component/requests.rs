use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateComponentRequest {
    pub id: String,
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImplementFunctionRequest {
    pub function_id: String,
}