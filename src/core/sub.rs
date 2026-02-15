use std::rc::Rc;

use super::ItemId;

pub trait Sub {
    type T;
    fn add_sub(&mut self,sub: Self::T) -> Result<(), Error>; 
    fn remove_sub(&mut self, sub_id: ItemId) -> Result<Self::T, Error>;
    fn get_sub(&self, sub_id: ItemId) -> Option<Rc<Self::T>>;
}

#[derive(Debug)]
pub enum Error {
   ItemIdAlreadyRegistered,
   ItemIdNotFound 
}