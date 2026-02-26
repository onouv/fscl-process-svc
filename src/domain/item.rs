use std::fmt::{self, Display, Formatter};
use thiserror::Error;

pub trait Item {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: String) -> Result<Self, ItemIdError> {
        if id.is_empty() {
            return Err(ItemIdError::ItemIdEmpty);
        }

        Ok(Self(id))
    }
}

#[derive(Debug, Clone, Error)]
pub enum ItemIdError {
    ItemIdEmpty,
    // There will more parsing errors...
}

impl Display for ItemId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ItemIdError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let result = match self {
            ItemIdError::ItemIdEmpty => "Empty id string."
        }; 
        
        write!(f, "{}", result)
    }
}