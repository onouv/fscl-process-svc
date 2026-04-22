use crate::{
    ports::{ComponentApplicationError, ComponentPort, NewComponentRequest},
};

use fscl_core::{
    ComponentLifecycleUow,
    CreateComponentError,
    CreateComponentRequest,
    ports::{ComponentRepositoryPort, DomainEventPublisherPort, UnitOfWorkPort},
};

#[derive(Clone)]
pub struct ComponentService<U, R, P>
where
    U: UnitOfWorkPort,
    R: ComponentRepositoryPort<Error = U::Error>,
    P: DomainEventPublisherPort<Error = U::Error>,
{
    lifecycle: ComponentLifecycleUow<U, R, P>,
}

impl<U, R, P> ComponentService<U, R, P>
where
    U: UnitOfWorkPort,
    R: ComponentRepositoryPort<Error = U::Error> + 'static,
    P: DomainEventPublisherPort<Error = U::Error> + 'static,
{
    pub fn new(lifecycle: ComponentLifecycleUow<U, R, P>) -> Self {
        Self { lifecycle }
    }
}

impl<U, R, P> ComponentPort for ComponentService<U, R, P>
where
    U: UnitOfWorkPort + 'static,
    U::Error: std::fmt::Display,
    R: ComponentRepositoryPort<Error = U::Error> + Send + Sync + 'static,
    P: DomainEventPublisherPort<Error = U::Error> + Send + Sync + 'static,
    R: for<'tx> ComponentRepositoryPort<Error = U::Error, Tx<'tx> = U::Tx<'tx>>,
    P: for<'tx> DomainEventPublisherPort<Error = U::Error, Tx<'tx> = U::Tx<'tx>>,
{
    fn new_component(
        &self,
        req: NewComponentRequest,
    ) -> impl Future<Output = Result<(), ComponentApplicationError>> + Send {
        let lifecycle = self.lifecycle.clone();
        async move {
            lifecycle
                .create_component(CreateComponentRequest {
                    id: req.id.to_string(),
                    name: req.name,
                    description: req.description,
                    parent: req.parent_id.map(|p| p.to_string()),
                    children: vec![],
                    parameters: Default::default(),
                })
                .await
                .map_err(|e| match e {
                    CreateComponentError::InvalidId(err) => {
                        ComponentApplicationError::InvalidResourceId(err.to_string())
                    }
                    CreateComponentError::Domain(err) => {
                        ComponentApplicationError::CannotProcess(err.to_string())
                    }
                    CreateComponentError::Infrastructure(err) => {
                        ComponentApplicationError::Infrastructure(err.to_string())
                    }
                })
        }
    }
}
