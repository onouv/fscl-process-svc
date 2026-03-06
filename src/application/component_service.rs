use crate::{
    adapters::driving::db::component_repository::ComponentRepository,
    domain::{component::Component, item::ResourceId},
    ports::{ComponentApplicationError, ComponentPort, NewComponentRequest},
};

#[derive(Debug, Clone)]
pub struct ComponentService<R>
where
    R: ComponentRepository,
{
    repo: R,
}

impl<R> ComponentService<R>
where
    R: ComponentRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    async fn new_top_level_component(
        &self,
        req: NewComponentRequest,
    ) -> Result<Component, ComponentApplicationError> {

        let component = Component::new(
            req.id,
            req.name.as_str(),
            req.description.as_deref().unwrap_or(""),
        );

        Ok(component) 
    }
    

    async fn new_sub_component(
        &self,
        parent: ResourceId,
        req: NewComponentRequest,
    ) -> Result<Component, ComponentApplicationError>{
        todo!() 
    }
}

impl<R> ComponentPort for ComponentService<R>
where
    R: ComponentRepository + Send + Sync + 'static,
{
    async fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<Component, ComponentApplicationError>> + Send {

    async move {
        if self.repo.exist_item(req.id.clone()).await? {
            return Err(ComponentApplicationError::ComponentAlreadyExists(req.id));
        }


    }       

        // if parent_id is Some, then we are meant to be a sub-component
        
    }

}
