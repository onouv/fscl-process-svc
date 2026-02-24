
mod domain;
mod ports;
mod adapters;
mod application;


use adapters::{
    driving::db::*,
    driven::web::http_server::Server,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let repo = seaorm_repository::SeaOrmRepository::new().await;
    let component_service = application::component_service::ComponentService::new(repo);


    // Start web server
    let server = Server::new(component_service);
    match server.run().await {
        Ok(_) => log::info!("FSCL process service stopped gracefully."),
        Err(e) => log::error!("FSCL process service encountered an error: {}", e),
    } 

    Ok(())
}
