use crate::core::Sub;


struct ItemId(String);
struct Item {
    pub id: ItemId,
    pub name: String
}

trait<T> Subs {
    fn add_sub(sub: T) -> Result<(), String>;
    
}

struct Func(Item);

impl Sub for Func {
    type T = Self;

    fn add_sub(&mut self, sub: Function) -> Result<(), sub::Error> {
    }
}