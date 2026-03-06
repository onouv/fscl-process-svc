use crate::{
    adapters::driving::db::component_repository::ComponentRepository,
    domain::{component::Component, item::ItemId},
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
        parent: ItemId,
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
        // if parent_id is Some, then we are meant to be a sub-component
        let v = if let Some(parent_id) = req.parent_id {
            self.new_sub_component(parent_id, req)
        } else {
            self.new_top_level_component(req)
        };

        v
    }
}
