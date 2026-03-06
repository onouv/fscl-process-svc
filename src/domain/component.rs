use super::item::{Resource, ResourceId};

#[derive(Debug)]
pub struct Component {
    pub id: ResourceId,
    pub name: String,
    pub description: String,
    subs: Vec<ResourceId>,
    implementers: Vec<ResourceId>,
}

impl Component {
    pub fn new(id: ResourceId, name: &str, description: &str) -> Self {

        Component {
            id,
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implementers: Vec::new(),
        }
    }
}

impl Resource for Component {
    fn id(&self) -> ResourceId {
        self.id.clone()
    }
}
