use std::fmt::{self, Display, Formatter};
use thiserror::Error;

pub trait Resource {
    fn id(&self) -> ResourceId;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    pub fn new(id: String) -> Result<Self, ItemIdError> {
        if id.is_empty() {
            return Err(ItemIdError::ItemIdEmpty);
        }

        Ok(Self(id))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Error)]
pub enum ItemIdError {
    ItemIdEmpty,
    // There will more parsing errors...
}

impl Display for ResourceId {
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