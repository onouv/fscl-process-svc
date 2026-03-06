use super::item::{Item, ItemId};

#[derive(Debug)]
pub struct Component {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    subs: Vec<ItemId>,
    implementers: Vec<ItemId>,
}

impl Component {
    pub fn new(id: ItemId, name: &str, description: &str) -> Self {

        Component {
            id,
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implementers: Vec::new(),
        }
    }
}

impl Item for Component {
    fn id(&self) -> ItemId {
        self.id.clone()
    }
}
