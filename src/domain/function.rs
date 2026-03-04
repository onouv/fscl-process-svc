use crate::domain::ItemIdError;

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
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ItemIdError> {
        let item_id = ItemId::new(String::from(id))?;
        
        Ok(Function {
            id: item_id, 
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implements: Vec::new(),
        })
    }
}

impl Item for Function {
    fn id(&self) -> ItemId {
        self.id.clone()
    }
}
