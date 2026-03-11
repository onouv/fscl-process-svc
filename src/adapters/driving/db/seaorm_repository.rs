use sea_orm::{
   Database, prelude::*
};
use sea_orm_migration::MigratorTrait;

use dotenv::dotenv;
use std::env;
use migration::Migrator;

use crate::{adapters::driving::db::models, domain::{ResourceId, component::Component}};
use super::{
    error::RepositoryError,
    ComponentRepository,
    repository::Repository,
};

fn get_database_url() -> String {
    dotenv().ok();
    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "postgres".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "fscl".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "fscl".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "process_svc".to_string());

    format!("{}://{}:{}@{}/{}", db_type, db_user, db_password, db_host, db_name)
}



impl From<DbErr> for RepositoryError {
    fn from(err: DbErr) -> Self {
        RepositoryError::DbError(err.to_string())
    }
}

#[derive(Clone)]
pub struct SeaOrmRepository {
    db: DbConn,
}

impl SeaOrmRepository {
    pub async fn new() -> Result<Self, RepositoryError> {
        // Database URL from environment or use default
        let database_url = get_database_url();
        log::info!("Connecting to database: {}", database_url);

        // Connect to database
        let db = Database::connect(&database_url).await.map_err(|err| {
            log::error!("✗ Failed to connect to database: {}", err);
            RepositoryError::DbError(err.to_string())
        })?;
        log::info!("✓ Connected to database");

        Migrator::up(&db, None).await.map_err(|err| {
            log::error!("✗ Failed to run migrations: {}", err);
            RepositoryError::DbError(err.to_string())
        })?;
        log::info!("✓ Migrations applied");

        Ok(Self { db })
    }
}

impl Repository<Component> for SeaOrmRepository {
    fn load(&self, id: &ResourceId) -> impl std::future::Future<Output = Result<Option<Component>, RepositoryError>> + Send {
        let db = self.db.clone();
        let id = id.as_str().to_string();

        async move {
            let db_component = models::component::Entity::find_by_id(id)
                .one(&db)
                .await?;

            match db_component {
                Some(model) => {
                    let resource_id = ResourceId::new(model.id)
                        .map_err(|e| RepositoryError::Unknown(e.to_string()))?;
                    Ok(Some(Component::new(resource_id, &model.name, &model.description)))
                }
                None => Ok(None),
            }
        }
    }

    fn save(&self, resource: &Component) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send {
        let _db = self.db.clone();
        let _id = resource.id.as_str().to_string();

        async move {
            todo!();
        }
    }
}


impl ComponentRepository for SeaOrmRepository {}
