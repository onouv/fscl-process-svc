use std::rc::Rc;

use super::item::{Item, ItemId};
use super::sub_manager::HasItemId;
use super::sub::{self, Sub};

#[derive(Debug, Clone)]
pub struct Function {
    pub item: Item<Function>,
}

impl Function {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ()> {
        let item = match Item::new(ItemId::new(id), name, description) {
            Ok(e) => e,
            Err(e) => {
                println!("{e:?}");
                return Err(());
            }
        };

        Ok(Self { item })
    }
}

impl Sub for Function {
    type T = Self;

    fn add_sub(&mut self, sub: Function) -> Result<(), sub::Error> {
        self.item.add_sub(sub)
    }

    fn remove_sub(&mut self, sub_id: ItemId) -> Result<Self::T, sub::Error> {
        self.item.remove_sub(&sub_id)
    }

    fn get_sub(&self, sub_id: ItemId) -> Option<Rc<Self::T>> {
        self.item.get_sub(&sub_id)
    }
}

impl HasItemId for Function {
    fn item_id(&self) -> &ItemId {
        &self.item.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::sub::Sub;

    #[test]
    fn function_should_accept_subs() {
        let res = Function::new("=100", "Protect PAX", "none");
        
        assert!(res.is_ok());
        let mut f100 = res.unwrap();
        
        let res2 =  Function::new("=100.001", "Prevent Shaft Fall", "none");
        assert!(res2.is_ok());
        let f100_001 = res2.unwrap();
        
        
        let add_res = f100.add_sub(f100_001);
        assert!(add_res.is_ok());

        let res3 = f100.get_sub(ItemId::new("=100.001"));
        assert!(res3.is_some());
        let f_ref = res3.unwrap();
        assert!(f_ref.item.id == ItemId::new("=100.001"));
        assert_eq!(f_ref.item.name, "Prevent Shaft Fall");
    }
}
