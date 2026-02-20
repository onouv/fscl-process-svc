use crate::{domain::item::ItemId};

pub enum ComponentError {
    ItemIdPreExisting,
    NoSuchItemId,
    UnknownError,
}

#[derive(Clone)]
pub struct CreateComponentRequest {
    id: ItemId,
    name: String,
    description: Option<String>,
    implementers: Option<Vec<ItemId>>,
}

pub trait ComponentPort {
    fn new_component(
        req: CreateComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentError>> + Send;
    fn new_sub_component(
        parent: ItemId,
        req: CreateComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentError>> + Send;
}
