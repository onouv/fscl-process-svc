use std::fmt::Display;

use crate::domain::{component::Component, resource::{ResourceId, ResourceIdError}};
use thiserror::Error;

pub(crate) enum ComponentApplicationError {
    ResourceIdDuplicate { id: ResourceId },
    NoSuchResourceId { id: ResourceId },
    NoSuchParentId { id: ResourceId },
    Unknown,
}


#[derive(Debug, Clone, Error)]
pub(crate) enum RequestBuildError {
    InvalidId(ResourceIdError),
    InvalidParentId(ResourceIdError),
}

impl Display for RequestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidId(error) => {
                write!(f, "Invalid id {}", error)
            }
            Self::InvalidParentId(error) => {
                write!(f, "Invalid parent id {}", error)
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
        id: String,
        name: String,
        description: Option<String>,
        parent_id: Option<String>,
    ) -> Result<Self, RequestBuildError> {
        let id = match ResourceId::new(id) {
            Ok(id) => id,
            Err(e) => {
                return Err(RequestBuildError::InvalidId(e));
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
    fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl std::future::Future<Output = Result<Component, ComponentApplicationError>> + Send;
}
