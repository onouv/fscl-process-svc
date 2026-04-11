use fscl_core::ResourceIdError;

use fscl_core::ResourceId;

use crate::domain::Resource;

#[derive(Debug)]
pub struct Function {
    pub id: ResourceId,
    pub name: String,
    pub description: String,
    subs: Vec<ResourceId>,
    implements: Vec<ResourceId>,
}

impl Function {
    pub fn new(id: &str, name: &str, description: &str) -> Result<Self, ResourceIdError> {
        let resource_id = ResourceId::new(String::from(id))?;
        
        Ok(Function {
            id: resource_id, 
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
