use std::{fmt::{self, Display, Formatter}, str::FromStr};
use std::result::Result;

use super::sub;
use super::sub_manager::HasItemId;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: &str) -> Self {
        Self(String::from_str(id).unwrap())
    }
}

impl Display for ItemId {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)    }
}

pub trait Item<T> {
    fn add_sub(&mut self, sub: T) -> Result<(), sub::Error>;  
}

#[derive(Debug)]
pub enum Error {
    InvalidId,
    ParseError
}

#[derive(Debug, Clone)]
pub struct BaseItem {
    pub id: ItemId,
    pub name: String,
    pub description: String,
}


impl BaseItem {
    pub fn new(id: ItemId, name: &str, description: &str) -> Result<Self, Error> {
        let name = match String::from_str(name){
            Ok(s) => s,
            Err(_) => {
                return Err(Error::ParseError);
            }
        };

        let description = match String::from_str(description) {
            Ok(s) => s,
            Err(_) => {
                return Err(Error::ParseError)
            }
        };

        Ok(Self {
            id, name, description
        })
    }
}

impl Display for BaseItem {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.id, self.name, self.description)
    }
}

impl HasItemId for BaseItem {
    fn item_id(&self) -> &ItemId {
        &self.id
    }
}