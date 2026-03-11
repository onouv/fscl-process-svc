
#![allow(unused, clippy::manual_async_fn)]
mod domain;
mod ports;
mod adapters;
mod application;


use std::net::Ipv4Addr;

use adapters::{
    driving::db::*,
    driven::web::http_server::HttpServer,
};

use crate::{adapters::{driven::web::http_server::HttpServerConfig, driving::db::seaorm_repository::SeaOrmRepository}, application::component_service::ComponentService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let repo = seaorm_repository::SeaOrmRepository::new().await?;
    let component_service = application::component_service::ComponentService::new(repo);

    // Start HTTP server
    let cfg = HttpServerConfig {
        ip: std::net::SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 3100),
    };

    let server = HttpServer::new::<ComponentService<SeaOrmRepository>>(cfg, component_service).await?;

    server.run().await
} 
