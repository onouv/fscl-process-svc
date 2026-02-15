#![allow(dead_code, unused)]

use std::{fmt::{self, Display, Formatter}, str::FromStr};
use std::result::Result;
use std::rc::Rc;

use super::sub;
use super::sub_manager::{SubManager, HasItemId};

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


#[derive(Debug)]
pub enum Error {
    ParseError
}

#[derive(Debug, Clone)]
pub struct Item<T: HasItemId + Clone> {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    subs: SubManager<T>,
}

impl<T: HasItemId + Clone> Item<T> {
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
            id,
            name,
            description,
            subs: SubManager::new(),
        })
    }

    pub fn add_sub(&mut self, sub: T) -> Result<(), sub::Error> {
        self.subs.add_sub(sub)
    }

    pub fn remove_sub(&mut self, sub_id: &ItemId) -> Result<T, sub::Error> {
        self.subs.remove_sub(sub_id)
    }

    pub fn get_sub(&self, sub_id: &ItemId) -> Option<Rc<T>> {
        self.subs.get_sub(sub_id)
    }

    pub fn list_subs(&self) -> Vec<Rc<T>> {
        self.subs.list_subs()
    }
}

impl<T: HasItemId + Clone> Display for Item<T> {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.id, self.name, self.description)
    }
}

impl<T: HasItemId + Clone> HasItemId for Item<T> {
    fn item_id(&self) -> &ItemId {
        &self.id
    }
}