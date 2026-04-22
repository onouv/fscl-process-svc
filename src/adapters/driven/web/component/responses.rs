#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub(crate) struct CreateComponentResponse {
    pub id: String,
}

impl CreateComponentResponse {
    pub fn from_id(id: String) -> Self {
        Self { id }
    }
}

