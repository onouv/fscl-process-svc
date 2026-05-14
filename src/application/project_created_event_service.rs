use std::future::Future;

use fscl_core::{
    HandleProjectCreatedEventError, ProjectCreatedEventHandlerUow, ProjectRepositoryPort,
    UnitOfWorkPort,
};
use fscl_messaging::ProjectCreatedEvent;

#[derive(Debug, thiserror::Error)]
pub enum ProjectCreatedEventServiceError {
    #[error("cannot process project-created event: {0}")]
    CannotProcess(String),
}

#[derive(Clone)]
pub struct ProjectCreatedEventService<U, R>
where
    U: UnitOfWorkPort,
    R: ProjectRepositoryPort<Error = U::Error>,
{
    handler: ProjectCreatedEventHandlerUow<U, R>,
}

impl<U, R> ProjectCreatedEventService<U, R>
where
    U: UnitOfWorkPort,
    U::Error: std::fmt::Display,
    R: ProjectRepositoryPort<Error = U::Error> + 'static,
{
    pub fn new(handler: ProjectCreatedEventHandlerUow<U, R>) -> Self {
        Self { handler }
    }

    pub fn handle(
        &self,
        event: ProjectCreatedEvent,
    ) -> impl Future<Output = Result<(), ProjectCreatedEventServiceError>> + Send
    where
        R: for<'tx> ProjectRepositoryPort<Error = U::Error, Tx<'tx> = U::Tx<'tx>>,
    {
        let handler = self.handler.clone();

        async move {
            handler.handle(event).await.map_err(|error| match error {
                HandleProjectCreatedEventError::InvalidProjectId(e) => {
                    ProjectCreatedEventServiceError::CannotProcess(e.to_string())
                }
                HandleProjectCreatedEventError::InvalidName(e) => {
                    ProjectCreatedEventServiceError::CannotProcess(e.to_string())
                }
                HandleProjectCreatedEventError::InvalidFormat(e) => {
                    ProjectCreatedEventServiceError::CannotProcess(e.to_string())
                }
                HandleProjectCreatedEventError::Infrastructure(e) => {
                    ProjectCreatedEventServiceError::CannotProcess(e.to_string())
                }
            })
        }
    }
}
