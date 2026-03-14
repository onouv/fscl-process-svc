use crate::{
    adapters::driving::db::ComponentRepository,
    domain::component::Component,
    ports::{ComponentApplicationError, ComponentPort, NewComponentRequest},
};

use fscl_core::ResourceId;

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
    ) -> Result<Component, ComponentApplicationError> {
        todo!()
    }
}

impl<R> ComponentPort for ComponentService<R>
where
    R: ComponentRepository + Send + Sync + 'static,
{
    fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<Component, ComponentApplicationError>> + Send {
        async move {
            let component = match self.repo.load(&req.id).await {
                Ok(comp) => comp,
                Err(e) => {
                    log::error!("Error loading component with id {}: {:?}", req.id, e);
                    
                    return Err(ComponentApplicationError::Unknown);
                }
            };

            if component.is_some() {
                log::info!("encountered duplicate resource with id {}", req.id);

                return Err(ComponentApplicationError::ResourceIdDuplicate{ id: req.id});
            }

            // TODO: if parent_id is Some, then we are meant to be a sub-component

            log::trace!("creating new component: (id: {}, name: {})", req.id, req.name);

            Ok(Component::new(req.id, req.name.as_str(), req.description.as_deref().unwrap_or("")))
        }
    }
}
