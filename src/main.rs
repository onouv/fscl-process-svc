#![allow(unused, clippy::manual_async_fn)]
mod adapters;
mod application;
mod domain;
mod ports;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use adapters::{driven::web::http_server::HttpServer, driving::db::*};
use dotenv::{dotenv, from_filename};

use crate::{
    adapters::{
        driven::web::http_server::HttpServerConfig,
        driving::db::seaorm_repository::SeaOrmRepository,
    },
    application::component_service::ComponentService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    from_filename("../.env.shared").ok();
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let repo = seaorm_repository::SeaOrmRepository::new().await?;
    let component_service = application::component_service::ComponentService::new(repo);

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

    let server =
        HttpServer::new::<ComponentService<SeaOrmRepository>>(cfg, component_service).await?;

    server.run().await
}
