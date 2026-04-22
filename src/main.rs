#![allow(unused, clippy::manual_async_fn)]
mod adapters;
mod application;
mod ports;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use adapters::{driven::web::http_server::HttpServer, driving::db::*};
use dotenv::{dotenv, from_filename};
use fscl_core::{
    ComponentLifecycleUow,
    adapters::driving::{
        db::{SqlxPgDatabase, UnitOfWork},
        messaging::{
            ComponentDomainEventMapper,
            DomainEventOutboxPublisher,
            SqlxOutboxWriter,
        },
    },
};
use fscl_messaging::ensure_outbox_schema;
use sqlx::PgPool;

use crate::{
    adapters::driven::web::http_server::HttpServerConfig,
    adapters::driving::db::sqlx_repository::SqlxRepository,
    application::component_service::ComponentService,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    from_filename("../.env.shared").ok();
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = get_database_url();
    let pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    ensure_outbox_schema(&pool).await?;

    let repository = SqlxRepository::new(pool.clone());
    let uow = UnitOfWork::new(SqlxPgDatabase::from_pool(pool));
    let publisher = DomainEventOutboxPublisher::new(
        "process-view",
        ComponentDomainEventMapper,
        SqlxOutboxWriter,
    );
    let lifecycle = ComponentLifecycleUow::new(uow, repository, publisher);
    let component_service = ComponentService::new(lifecycle);

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

    server.run().await
}
