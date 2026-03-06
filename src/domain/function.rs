use crate::domain::ItemIdError;

use super::item::{Resource, ResourceId};

#[derive(Debug)]
pub struct Function {
    pub id: ResourceId,
    pub name: String,
    pub description: String,
    subs: Vec<ResourceId>,
    implements: Vec<ResourceId>,
}

impl Function {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ItemIdError> {
        let item_id = ResourceId::new(String::from(id))?;
        
        Ok(Function {
            id: item_id, 
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
            implements: Vec::new(),
        })
    }
}

impl Resource for Function {
    fn id(&self) -> ResourceId {
        self.id.clone()
    }
}
