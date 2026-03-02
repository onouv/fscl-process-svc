use crate::domain::{Component, Item};

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub(crate) struct CreateComponentResponse {
    pub id: Option<String>,
}

impl From<&Component> for CreateComponentResponse {
    fn from(component: &Component) -> Self {
        Self { id: Some(component.id().to_string()) }
    }
} 