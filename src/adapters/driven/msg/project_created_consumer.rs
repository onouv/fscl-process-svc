use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use anyhow::Context;
use async_nats::Client;
use async_nats::jetstream;
use async_nats::jetstream::AckKind;
use async_nats::jetstream::consumer;
use fscl_core::{ProjectRepositoryPort, UnitOfWorkPort};
use fscl_messaging::{AggregateType, EventEnvelope, ProjectCreatedEvent};
use futures_util::StreamExt;

use crate::application::project_created_event_service::{
    ProjectCreatedEventService, ProjectCreatedEventServiceError,
};

pub trait ProjectCreatedEventHandlerPort: Clone + Send + Sync + 'static {
    fn handle_project_created_event(
        &self,
        event: ProjectCreatedEvent,
    ) -> Pin<Box<dyn Future<Output = Result<(), ProjectCreatedEventServiceError>> + Send>>;
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
    ) -> Pin<Box<dyn Future<Output = Result<(), ProjectCreatedEventServiceError>> + Send>> {
        let service = self.clone();
        Box::pin(async move { service.handle(event).await })
    }
}

#[derive(Clone)]
pub struct ProjectCreatedConsumerConfig {
    pub stream_name: String,
    pub durable_name: String,
    pub subject: String,
    pub ack_policy: consumer::AckPolicy,
    pub ack_wait: Duration,
}

#[derive(Clone)]
pub struct ProjectCreatedConsumer<H>
where
    H: ProjectCreatedEventHandlerPort,
{
    nats_client: Client,
    config: ProjectCreatedConsumerConfig,
    handler: H,
}

impl<H> ProjectCreatedConsumer<H>
where
    H: ProjectCreatedEventHandlerPort,
{
    pub fn new(nats_client: Client, config: ProjectCreatedConsumerConfig, handler: H) -> Self {
        Self {
            nats_client,
            config,
            handler,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let jetstream = jetstream::new(self.nats_client.clone());
        let stream = jetstream
            .get_stream(self.config.stream_name.clone())
            .await
            .with_context(|| format!("failed to open stream {}", self.config.stream_name))?;

        let consumer = stream
            .get_or_create_consumer(
                &self.config.durable_name,
                consumer::pull::Config {
                    durable_name: Some(self.config.durable_name.clone()),
                    ack_policy: self.config.ack_policy,
                    ack_wait: self.config.ack_wait,
                    filter_subject: self.config.subject.clone(),
                    ..Default::default()
                },
            )
            .await
            .with_context(|| {
                format!(
                    "failed to get/create durable consumer {} on stream {}",
                    self.config.durable_name, self.config.stream_name
                )
            })?;

        let mut messages = consumer
            .messages()
            .await
            .context("failed to create pull-message stream")?;

        log::info!(
            "ProjectCreated consumer running on stream '{}' durable '{}' subject '{}'",
            self.config.stream_name,
            self.config.durable_name,
            self.config.subject,
        );

        while let Some(next_message) = messages.next().await {
            let message = match next_message {
                Ok(message) => message,
                Err(error) => {
                    log::error!("failed receiving JetStream message: {}", error);
                    continue;
                }
            };

            let event = match parse_project_created_event(&message.payload) {
                Some(event) => event,
                None => {
                    log::warn!("discarding unparseable project-created message payload");
                    if let Err(error) = message.ack().await {
                        log::error!("failed to ack invalid payload: {}", error);
                    }
                    continue;
                }
            };

            match self.handler.handle_project_created_event(event).await {
                Ok(()) => {
                    if let Err(error) = message.ack().await {
                        log::error!("failed to ack handled message: {}", error);
                    }
                }
                Err(ProjectCreatedEventServiceError::Infrastructure(error)) => {
                    log::error!(
                        "project-created infrastructure failure, requesting retry: {}",
                        error
                    );
                    if let Err(ack_error) = message.ack_with(AckKind::Nak(None)).await {
                        log::error!("failed to NAK message: {}", ack_error);
                    }
                }
                Err(
                    ProjectCreatedEventServiceError::InvalidProjectId(error)
                    | ProjectCreatedEventServiceError::InvalidName(error)
                    | ProjectCreatedEventServiceError::InvalidFormat(error),
                ) => {
                    log::warn!(
                        "project-created validation failure, acknowledging message: {}",
                        error
                    );
                    if let Err(ack_error) = message.ack().await {
                        log::error!("failed to ack invalid message: {}", ack_error);
                    }
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
