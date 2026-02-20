use std::{fmt::{self, Display, Formatter}, str::FromStr};

pub trait Item {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: &str) -> Self {
        Self(String::from_str(id).unwrap())
    }
}

impl Display for ItemId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
