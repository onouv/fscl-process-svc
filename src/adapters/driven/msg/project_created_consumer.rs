use std::future::Future;
use std::pin::Pin;

use anyhow::Context;
use async_nats::Client;
use fscl_core::{ProjectRepositoryPort, UnitOfWorkPort};
use fscl_messaging::{AggregateType, EventEnvelope, ProjectCreatedEvent};
use futures_util::StreamExt;

use crate::application::project_created_event_service::ProjectCreatedEventService;

pub trait ProjectCreatedEventHandlerPort: Clone + Send + Sync + 'static {
    fn handle_project_created_event(
        &self,
        event: ProjectCreatedEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
}

impl<U, R> ProjectCreatedEventHandlerPort for ProjectCreatedEventService<U, R>
where
    U: UnitOfWorkPort + 'static,
    U::Error: std::fmt::Display,
    R: ProjectRepositoryPort<Error = U::Error>
        + Send
        + Sync
        + 'static
        + for<'tx> ProjectRepositoryPort<Error = U::Error, Tx<'tx> = U::Tx<'tx>>,
{
    fn handle_project_created_event(
        &self,
        event: ProjectCreatedEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let service = self.clone();
        Box::pin(async move {
            service
                .handle(event)
                .await
                .map_err(|error| error.to_string())
        })
    }
}

#[derive(Clone)]
pub struct ProjectCreatedConsumer<H>
where
    H: ProjectCreatedEventHandlerPort,
{
    nats_client: Client,
    subject: String,
    handler: H,
}

impl<H> ProjectCreatedConsumer<H>
where
    H: ProjectCreatedEventHandlerPort,
{
    pub fn new(nats_client: Client, subject: String, handler: H) -> Self {
        Self {
            nats_client,
            subject,
            handler,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let mut subscriber = self
            .nats_client
            .subscribe(self.subject.clone())
            .await
            .with_context(|| format!("failed to subscribe to {}", self.subject))?;

        log::info!(
            "ProjectCreated consumer subscribed to subject '{}'",
            self.subject
        );

        while let Some(message) = subscriber.next().await {
            if let Some(event) = parse_project_created_event(&message.payload) {
                if let Err(error) = self.handler.handle_project_created_event(event).await {
                    log::error!("ProjectCreated handling failed: {}", error);
                }
            }
        }

        Ok(())
    }
}

fn parse_project_created_event(payload: &[u8]) -> Option<ProjectCreatedEvent> {
    if let Ok(envelope) = serde_json::from_slice::<EventEnvelope>(payload) {
        if envelope.aggregate_type != AggregateType::Project || envelope.event_type != "created" {
            return None;
        }

        return serde_json::from_value::<ProjectCreatedEvent>(envelope.payload)
            .map_err(|error| {
                log::warn!("invalid project event envelope payload: {}", error);
                error
            })
            .ok();
    }

    serde_json::from_slice::<ProjectCreatedEvent>(payload)
        .map_err(|error| {
            log::warn!("invalid project-created message payload: {}", error);
            error
        })
        .ok()
}
