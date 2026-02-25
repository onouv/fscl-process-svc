use crate::{
    adapters::driving::db::component_repository::ComponentRepository, 
    domain::item::ItemId, 
    ports::component_port::{ComponentError, ComponentPort}
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
        req: crate::ports::component_port::CreateComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentError>> + Send {
        async move {
            todo!()
        }
    }

    fn new_sub_component(
        &self,
        parent: ItemId,
        req: crate::ports::component_port::CreateComponentRequest,
    ) -> impl Future<Output = Result<(), crate::ports::component_port::ComponentError>> + Send {
        async move {
            todo!()
        }
    }
}
