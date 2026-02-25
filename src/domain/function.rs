use super::item::{Item, ItemId};

#[derive(Debug)]
pub struct Function {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    subs: Vec<ItemId>,
    implements: Vec<ItemId>,
}

impl Function {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Function {
            id: ItemId::new(id),
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implements: Vec::new(),
        }
    }
}

impl Item for Function {}
