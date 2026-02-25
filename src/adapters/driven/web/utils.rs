use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(super) struct AddSubRequest {
    pub child_id: String,
}


#[derive(Serialize)]
pub(super) struct ErrorResponse {
    pub error: String,
}