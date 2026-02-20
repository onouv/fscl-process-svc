use actix_web::{App, HttpServer, middleware, web};

use crate::adapters::driving::db::ComponentRepository;
use crate::application::component_service::ComponentService;

pub struct Server<R: ComponentRepository> {
    component_service: ComponentService<R>,
}

impl<R: ComponentRepository> Server<R> {
    pub fn new(component_service: ComponentService<R>) -> Self {
        Self { component_service }
    }

    pub async fn run(self) -> std::io::Result<()> {
        log::info!("FSCL process service starting on http://0.0.0.0:8080");

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(self.component_service.clone()))
                .wrap(middleware::Logger::default())
                .configure(super::configure)
                .route("/health", web::get().to(|| async { "OK" }))
        });
        server.bind("0.0.0.0:8080")?.run().await
    }
}
