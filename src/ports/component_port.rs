use std::fmt::Display;

use crate::domain::{component::Component, item::{ResourceId, ItemIdError}};
use thiserror::Error;

pub(crate) enum ComponentApplicationError {
    ItemIdDuplicate { id: ResourceId },
    NoSuchItemId { id: ResourceId },
    NoSuchParentId { id: ResourceId },
    Unknown,
}


#[derive(Debug, Clone, Error)]
pub(crate) enum RequestBuildError {
    InvalidItemId(ItemIdError),
    InvalidParentId(ItemIdError),
}

impl Display for RequestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidItemId(error) => {
                return write!(f, "Invalid item id {}", error);
            }
            Self::InvalidParentId(error) => {
                return write!(f, "Invalid parent id {}", error);
            }
        }
    }
}



#[derive(Clone)]
pub struct NewComponentRequest {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<ResourceId>,
}

impl NewComponentRequest {
    pub(crate) fn new(
        item_id: String,
        name: String,
        description: Option<String>,
        parent_id: Option<String>,
    ) -> Result<Self, RequestBuildError> {
        let id = match ResourceId::new(item_id) {
            Ok(id) => id,
            Err(e) => {
                return Err(RequestBuildError::InvalidItemId(e));
            }
        };

        let parent_id = match parent_id {
            Some(p) => match ResourceId::new(p) {
                Ok(id) => Some(id),
                Err(e) => return Err(RequestBuildError::InvalidParentId(e)),
            },
            None => None,
        };

        Ok(Self {
            id,
            name,
            description,
            parent_id,
        })
    }
}

pub trait ComponentPort: Clone + Send + Sync + 'static {
    async fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<Component, ComponentApplicationError>> + Send;
}
