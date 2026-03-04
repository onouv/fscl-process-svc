use crate::{
    adapters::driving::db::component_repository::ComponentRepository, 
    domain::{component::Component, item::ItemId}, 
    ports::{ComponentApplicationError, ComponentPort, NewComponentRequest},
};

#[derive(Debug, Clone)]
pub struct ComponentService<R> where R: ComponentRepository {
    repo: R,
}

impl<R> ComponentService<R> where R: ComponentRepository {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> ComponentPort for ComponentService<R> 
where 
    R: ComponentRepository + Send + Sync + 'static
{
    fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<Component, ComponentApplicationError>> + Send {
        async move {
            todo!()
        }
    }

    fn new_sub_component(
        &self,
        parent: ItemId,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<Component, ComponentApplicationError>> + Send {
        async move {
            todo!()
        }
    }
}
