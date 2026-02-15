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


/// A generic manager for handling sub-items of any type.
/// This allows any struct to manage sub-items without duplicating logic.
#[derive(Debug, Clone)]
pub struct SubManager<T: HasItemId + Clone> {
    subs: Vec<Rc<T>>,
}

/// Trait for types that can be used with SubManager.
/// They must have an identifiable ItemId.
pub trait HasItemId {
    fn item_id(&self) -> &ItemId;
}

impl<T: HasItemId + Clone> SubManager<T> {
    pub fn new() -> Self {
        Self { subs: vec![] }
    }

    pub fn add_sub(&mut self, sub: T) -> Result<(), Error> {
        let previous = self.subs.iter().find(|s| s.item_id() == sub.item_id());

        if previous.is_some() {
            return Err(Error::ItemIdAlreadyRegistered);
        }

        self.subs.push(Rc::new(sub));
        Ok(())
    }

    pub fn remove_sub(&mut self, sub_id: &ItemId) -> Result<T, Error> {
        let sub_pos = self.subs.iter().position(|s| s.item_id() == sub_id);

        if sub_pos.is_none() {
            return Err(Error::ItemIdNotFound);
        }

        let rc_sub = self.subs.remove(sub_pos.unwrap());

        match Rc::try_unwrap(rc_sub) {
            Ok(sub) => Ok(sub),
            Err(rc) => Ok((*rc).clone()),
        }
    }

    pub fn get_sub(&self, sub_id: &ItemId) -> Option<Rc<T>> {
        self.subs.iter().find(|s| s.item_id() == sub_id).cloned()
    }

    pub fn list_subs(&self) -> Vec<Rc<T>> {
        self.subs.clone()
    }
}

impl<T: HasItemId + Clone> Default for SubManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
