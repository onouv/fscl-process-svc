use sea_orm::{
    prelude::*, ActiveValue, IntoActiveModel, Condition, QueryOrder, PaginatorTrait,
};
use uuid::Uuid;
use chrono::Utc;
use thiserror::Error;

use dotenv::dotenv;
use std::env;

use crate::models::{function, component, component_implements_function};

pub fn get_database_url() -> String {
    dotenv().ok();
    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "postgres".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "process_svc".to_string());

    format!("{}://{}:{}@{}/{}", db_type, db_user, db_password, db_host, db_name)
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<DbErr> for RepositoryError {
    fn from(err: DbErr) -> Self {
        RepositoryError::DbError(err.to_string())
    }
}

#[derive(Clone)]
pub struct Repository {
    db: DbConn,
}

impl Repository {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }

    // ===== Function Operations =====

    pub async fn create_function(
        &self,
        id: &str,
        name: &str,
        description: &str,
        parent_id: Option<&str>,
    ) -> Result<function::Model, RepositoryError> {
        let now = Utc::now();
        let active_model = function::ActiveModel {
            id: ActiveValue::Set(id.to_string()),
            name: ActiveValue::Set(name.to_string()),
            description: ActiveValue::Set(description.to_string()),
            parent_id: ActiveValue::Set(parent_id.map(|s| s.to_string())),
            version: ActiveValue::Set(1),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        function::Entity::insert(active_model)
            .exec(&self.db)
            .await?;

        self.get_function(id).await?.ok_or(RepositoryError::NotFound)
    }

    pub async fn get_function(&self, id: &str) -> Result<Option<function::Model>, RepositoryError> {
        Ok(function::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?)
    }

    pub async fn list_functions(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<function::Model>, u64), RepositoryError> {
        let paginator = function::Entity::find()
            .order_by_asc(function::Column::CreatedAt)
            .paginate(&self.db, limit);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(offset / limit).await?;

        Ok((items, total))
    }

    pub async fn add_function_sub(
        &self,
        parent_id: &str,
        child_id: &str,
    ) -> Result<(), RepositoryError> {
        // Verify both exist
        if self.get_function(parent_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }
        if self.get_function(child_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }

        let mut child = function::Entity::find_by_id(child_id.to_string())
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?
            .into_active_model();

        child.parent_id = ActiveValue::Set(Some(parent_id.to_string()));
        child.updated_at = ActiveValue::Set(Utc::now());
        child.version = ActiveValue::Unchanged(child.version.unwrap());

        child.update(&self.db).await?;
        Ok(())
    }

    pub async fn get_function_children(&self, parent_id: &str) -> Result<Vec<function::Model>, RepositoryError> {
        Ok(function::Entity::find()
            .filter(function::Column::ParentId.eq(Some(parent_id.to_string())))
            .all(&self.db)
            .await?)
    }

    // ===== Component Operations =====

    pub async fn create_component(
        &self,
        id: &str,
        name: &str,
        description: &str,
        parent_id: Option<&str>,
    ) -> Result<component::Model, RepositoryError> {
        let now = Utc::now();
        let active_model = component::ActiveModel {
            id: ActiveValue::Set(id.to_string()),
            name: ActiveValue::Set(name.to_string()),
            description: ActiveValue::Set(description.to_string()),
            parent_id: ActiveValue::Set(parent_id.map(|s| s.to_string())),
            version: ActiveValue::Set(1),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        component::Entity::insert(active_model)
            .exec(&self.db)
            .await?;

        self.get_component(id).await?.ok_or(RepositoryError::NotFound)
    }

    pub async fn get_component(&self, id: &str) -> Result<Option<component::Model>, RepositoryError> {
        Ok(component::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?)
    }

    pub async fn list_components(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<component::Model>, u64), RepositoryError> {
        let paginator = component::Entity::find()
            .order_by_asc(component::Column::CreatedAt)
            .paginate(&self.db, limit);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(offset / limit).await?;

        Ok((items, total))
    }

    pub async fn add_component_sub(
        &self,
        parent_id: &str,
        child_id: &str,
    ) -> Result<(), RepositoryError> {
        if self.get_component(parent_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }
        if self.get_component(child_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }

        let mut child = component::Entity::find_by_id(child_id.to_string())
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?
            .into_active_model();

        child.parent_id = ActiveValue::Set(Some(parent_id.to_string()));
        child.updated_at = ActiveValue::Set(Utc::now());
        child.version = ActiveValue::Unchanged(child.version.unwrap());

        child.update(&self.db).await?;
        Ok(())
    }

    pub async fn get_component_children(&self, parent_id: &str) -> Result<Vec<component::Model>, RepositoryError> {
        Ok(component::Entity::find()
            .filter(component::Column::ParentId.eq(Some(parent_id.to_string())))
            .all(&self.db)
            .await?)
    }

    // ===== Implementation Operations =====

    pub async fn component_implements_function(
        &self,
        component_id: &str,
        function_id: &str,
    ) -> Result<component_implements_function::Model, RepositoryError> {
        // Verify both exist
        if self.get_component(component_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }
        if self.get_function(function_id).await?.is_none() {
            return Err(RepositoryError::NotFound);
        }

        // Check if already implements
        let existing = component_implements_function::Entity::find()
            .filter(
                Condition::all()
                    .add(component_implements_function::Column::ComponentId.eq(component_id.to_string()))
                    .add(component_implements_function::Column::FunctionId.eq(function_id.to_string()))
            )
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(RepositoryError::Conflict("Component already implements this function".to_string()));
        }

        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let active_model = component_implements_function::ActiveModel {
            id: ActiveValue::Set(id),
            component_id: ActiveValue::Set(component_id.to_string()),
            function_id: ActiveValue::Set(function_id.to_string()),
            created_at: ActiveValue::Set(now),
        };

        component_implements_function::Entity::insert(active_model)
            .exec(&self.db)
            .await?;

        self.get_component_implements_function(component_id, function_id)
            .await?
            .ok_or(RepositoryError::NotFound)
    }

    pub async fn get_component_implements_function(
        &self,
        component_id: &str,
        function_id: &str,
    ) -> Result<Option<component_implements_function::Model>, RepositoryError> {
        Ok(component_implements_function::Entity::find()
            .filter(
                Condition::all()
                    .add(component_implements_function::Column::ComponentId.eq(component_id.to_string()))
                    .add(component_implements_function::Column::FunctionId.eq(function_id.to_string()))
            )
            .one(&self.db)
            .await?)
    }

    pub async fn list_component_implementations(
        &self,
        component_id: &str,
    ) -> Result<Vec<component_implements_function::Model>, RepositoryError> {
        Ok(component_implements_function::Entity::find()
            .filter(component_implements_function::Column::ComponentId.eq(component_id.to_string()))
            .all(&self.db)
            .await?)
    }
}
