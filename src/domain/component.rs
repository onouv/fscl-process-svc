use super::item::{Item, ItemId, ItemIdError};

#[derive(Debug)]
pub struct Component {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    subs: Vec<ItemId>,
    implementers: Vec<ItemId>,
}

impl Component {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ItemIdError> {
        let item_id = ItemId::new(String::from(id))?;

        Ok(Component {
            id: item_id,
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implementers: Vec::new(),
        })
    }
}

impl Item for Component {
    fn id(&self) -> ItemId {
        self.id.clone()
    }
}
