use std::fmt::Display;

use fscl_core::{IdFormat, ProjectId, ProjectIdError, ResourceId, ResourceIdError};
use thiserror::Error;

pub(crate) enum ComponentApplicationError {
    InvalidResourceId(String),
    CannotProcess(String),
    Infrastructure(String),
}

#[derive(Debug, Error)]
pub(crate) enum RequestBuildError {
    InvalidProjectId(ProjectIdError),
    InvalidId(ResourceIdError),
    InvalidParentId(ResourceIdError),
}

impl Display for RequestBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProjectId(error) => {
                write!(f, "Invalid project id {}", error)
            }
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
    pub project_id: ProjectId,
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<ResourceId>,
}

impl NewComponentRequest {
    pub(crate) fn new(
        project_id: String,
        id: String,
        name: String,
        description: Option<String>,
        parent_id: Option<String>,
    ) -> Result<Self, RequestBuildError> {
        let project_id = match ProjectId::new(project_id) {
            Ok(project_id) => project_id,
            Err(e) => return Err(RequestBuildError::InvalidProjectId(e)),
        };

        let format = IdFormat::new(None, None, None).expect("default format is valid");

        let id = match ResourceId::new(project_id.clone(), id, format.clone()) {
            Ok(id) => id,
            Err(e) => {
                return Err(RequestBuildError::InvalidId(e));
            }
        };

        let parent_id = match parent_id {
            Some(p) => match ResourceId::new(project_id.clone(), p, format) {
                Ok(id) => Some(id),
                Err(e) => return Err(RequestBuildError::InvalidParentId(e)),
            },
            None => None,
        };

        Ok(Self {
            project_id,
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
    ) -> impl std::future::Future<Output = Result<(), ComponentApplicationError>> + Send;
}
