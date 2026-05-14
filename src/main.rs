#![allow(unused, clippy::manual_async_fn)]
mod adapters;
mod application;
mod ports;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use adapters::{
    driven::{msg::project_created_consumer::ProjectCreatedConsumer, web::http_server::HttpServer},
    driving::db::*,
};
use fscl_core::{
    ComponentLifecycleUow, ProjectCreatedEventHandlerUow, ProjectLifecycleUow,
    adapters::driving::{
        db::{SqlxPgDatabase, UnitOfWork},
        messaging::{ComponentDomainEventMapper, DomainEventOutboxPublisher, SqlxOutboxWriter},
    },
};
use fscl_messaging::ensure_outbox_schema;
use sqlx::PgPool;

use crate::{
    adapters::driven::web::http_server::HttpServerConfig,
    adapters::driving::db::{
        sqlx_project_repository::SqlxProjectRepository, sqlx_repository::SqlxRepository,
    },
    application::{
        component_service::ComponentService,
        project_created_event_service::ProjectCreatedEventService,
    },
};

fn get_database_url() -> String {
    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "postgres".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "fscl".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "fscl".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "process_svc".to_string());

    format!(
        "{}://{}:{}@{}:{}/{}",
        db_type, db_user, db_password, db_host, db_port, db_name
    )
}

fn get_nats_url() -> String {
    env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string())
}

fn get_project_created_subject() -> String {
    env::var("NATS_PROJECT_CREATED_SUBJECT")
        .or_else(|_| env::var("NATS_SUBJECT"))
        .unwrap_or_else(|_| "events.project.created".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = get_database_url();
    let pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    ensure_outbox_schema(&pool).await?;

    let repository = SqlxRepository::new(pool.clone());
    let component_uow = UnitOfWork::new(SqlxPgDatabase::from_pool(pool.clone()));
    let project_uow = UnitOfWork::new(SqlxPgDatabase::from_pool(pool));
    let publisher = DomainEventOutboxPublisher::new(
        "process-view",
        ComponentDomainEventMapper,
        SqlxOutboxWriter,
    );
    let lifecycle = ComponentLifecycleUow::new(component_uow, repository, publisher);
    let component_service = ComponentService::new(lifecycle);

    let project_repository = SqlxProjectRepository::new();
    let project_lifecycle = ProjectLifecycleUow::new(project_uow, project_repository);
    let project_created_handler = ProjectCreatedEventHandlerUow::new(project_lifecycle);
    let project_created_event_service = ProjectCreatedEventService::new(project_created_handler);

    let nats_client = async_nats::connect(get_nats_url()).await?;
    let project_created_consumer = ProjectCreatedConsumer::new(
        nats_client,
        get_project_created_subject(),
        project_created_event_service,
    );

    let app_host = env::var("APP_HOST")
        .ok()
        .and_then(|value| value.parse::<IpAddr>().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let app_port = env::var("APP_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3100);

    // Start HTTP server
    let cfg = HttpServerConfig {
        ip: SocketAddr::new(app_host, app_port),
    };

    let server = HttpServer::new(cfg, component_service).await?;
    let http_task = tokio::spawn(async move { server.run().await });
    let consumer_task = tokio::spawn(async move { project_created_consumer.run().await });

    tokio::select! {
        result = http_task => {
            match result {
                Ok(inner) => inner,
                Err(error) => Err(anyhow::anyhow!("http task failed: {}", error)),
            }
        }
        result = consumer_task => {
            match result {
                Ok(inner) => inner,
                Err(error) => Err(anyhow::anyhow!("consumer task failed: {}", error)),
            }
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("shutdown signal received");
            Ok(())
        }
    }
}
