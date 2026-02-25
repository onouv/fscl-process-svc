use std::{net::SocketAddr, sync::Arc};

use crate::{
    adapters::driven::web::app_state::AppState, ports::component_port::ComponentPort
};
use anyhow::Context;
use axum::{self, Router, routing::post};
use tokio::net::TcpListener;

pub(crate) struct HttpServerConfig {
    pub ip: SocketAddr,
}

pub struct HttpServer {
    router: Router,
    listener: TcpListener
}

impl HttpServer {
    pub async fn new(cfg: HttpServerConfig, component_service: impl ComponentPort) -> anyhow::Result<Self> {
        let state= AppState { component_service: Arc::new(component_service) };
        let router = Router::new() 
            .nest("/api/v1", build_routes())
            .with_state(state);
        let listener = TcpListener::bind(cfg.ip).await.unwrap();
        Ok(Self {
            router,
            listener
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        log::info!("FSCL process service starting on http://0.0.0.0:8080");
        
        axum::serve(self.listener, self.router).await.context("unknown server error")?;
        Ok(()) 
    }
}

fn build_routes<C: ComponentPort>() -> Router<AppState<C>>
{
    Router::new().route(
        "/process/components",
        post(super::component::handlers::create_component::<C>)
    )
}
