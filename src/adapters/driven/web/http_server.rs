use std::{net::SocketAddr, sync::Arc};

use crate::{
    adapters::driven::web::{app_state::AppState, component::handlers::create_component},
    ports::ComponentPort,
};
use anyhow::Context;
use axum::{
    self, Json, Router,
    routing::{get, post},
};
use sea_orm::sea_query::func;
use tokio::net::TcpListener;

pub(crate) struct HttpServerConfig {
    pub ip: SocketAddr,
}

pub struct HttpServer {
    router: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn new<C>(
        cfg: HttpServerConfig,
        component_service: C 
    ) -> anyhow::Result<Self> 
    where 
        C: ComponentPort + Send + Sync + 'static,
    {
        let state = AppState {
            component_service: Arc::new(component_service),
        };

        let component_routes = Router::<AppState<C>>::new()
            .route("/", get(get_all_components))
            .route("/{id}", get(get_component))
            .route("/", post(create_component::<C>));

        let function_routes = Router::<AppState<C>>::new()
            .route("/", get(|| async {}))
            .route("/{id}", get(|| async {}))
            .route("/", post(|| async {}));

        let test_routes = Router::<AppState<C>>::new()
            .route("/", get(root))
            .route("/foo", get(get_foo))
            .route("/bar", get(get_bar));

        let api_routes = Router::<AppState<C>>::new()
            .nest("/component", component_routes)
            .nest("/function", function_routes)
            .nest("/test", test_routes);


        let router = Router::<AppState<C>>::new().nest("/api/v2", api_routes).with_state(state);


        /*let app = Router::new()
            .route("/", get(root))
            .route("/foo", get(get_foo).post(post_foo))
            .route("/foo/bar", get(foo_bar));
        */
        let listener = TcpListener::bind(cfg.ip).await.unwrap();
        log::info!("FSCL process service starting on {}", cfg.ip);
        Ok(Self { router, listener })
    }
    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router)
            .await
            .context("unknown server error")?;
        Ok(())
    }
}
// which calls one of these handlers
async fn root() {
    log::trace!("Hello world!");
}

async fn get_foo() {
    log::trace!("Getting foo...");
}

async fn get_bar() {
    log::trace!("Getting bar...");
}

async fn get_all_components() {
    log::trace!("Get all components...");
}

async fn get_component(Json(id): Json<u64>) {
    log::trace!("Getting component for id {}", id);
}
