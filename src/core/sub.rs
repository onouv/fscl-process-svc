use crate::core::ItemId;

#[derive(Debug)]
pub enum Error {
    ItemIdAlreadyRegistered,
    ItemIdNotFound,
}
