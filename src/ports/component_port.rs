use crate::{domain::item::ItemId};

pub enum ComponentError {
    ItemIdPreExisting,
    NoSuchItemId,
    UnknownError,
}

#[derive(Clone)]
pub struct CreateComponentRequest {
    pub id: ItemId,
    pub name: String,
    pub description: Option<String>,
    pub implementers: Option<Vec<ItemId>>,
}

pub trait ComponentPort: Clone + Send + Sync + 'static
{
    fn new_component(
        &self,
        req: CreateComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentError>> + Send;
    fn new_sub_component(
        &self,
        parent: ItemId,
        req: CreateComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentError>> + Send;
}
