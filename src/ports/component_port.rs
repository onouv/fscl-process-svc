use crate::domain::item::{ItemId, ItemIdError};
use thiserror::Error;

pub(crate) enum ComponentApplicationError {
    ItemIdDuplicate { id: ItemId },
    NoSuchItemId,
    NoSuchParentId,
    Unknown,
}

#[derive(Debug, Clone, Error)]
pub(crate) enum RequestBuildError {
    InvalidItemId(ItemIdError),
    InvalidParentId(ItemIdError)
}

#[derive(Clone)]
pub struct NewComponentRequest {
    pub id: ItemId,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<ItemId>,
}

impl NewComponentRequest {
    pub(crate) fn new(item_id: String, name: String, description: Option<String>, parent_id: Option<String>) -> Result<Self, RequestBuildError> {

        let id = match ItemId::new(item_id) {
            Ok(id) => id,
            Err(e) => {
                return Err(RequestBuildError::InvalidItemId(e));
            }
        };

        let parent_id = match parent_id {
            Some(p) => {
                match ItemId::new(p) {
                    Ok(id) => Some(id),
                    Err(e) => {
                        return Err(RequestBuildError::InvalidParentId(e))
                    }
                }
            },
            None => None
        };

        Ok(Self {
            id,
            name,
            description,
            parent_id
        })

    }
}

pub trait ComponentPort: Clone + Send + Sync + 'static
{
    fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentApplicationError>> + Send;
    fn new_sub_component(
        &self,
        parent: ItemId,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentApplicationError>> + Send;
}
