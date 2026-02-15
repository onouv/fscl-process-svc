
mod models;
mod repository;
mod handlers;

use actix_web::{web, App, HttpServer, middleware};
use sea_orm::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/fscl_svc".to_string());

    log::info!("Connecting to database: {}", database_url);

    // Connect to database
    let db = match Database::connect(&database_url).await {
        Ok(db) => {
            log::info!("✓ Connected to database");
            db
        }
        Err(err) => {
            log::error!("✗ Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    let repo = repository::Repository::new(db);

    log::info!("FSCL process service (DB-first) starting on http://0.0.0.0:8080");

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .wrap(middleware::Logger::default())
            .configure(handlers::configure)
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
