use crate::domain::{ResourceId, component::Component};
use super::{
    repository::Repository,
    error::RepositoryError
};

pub trait ComponentRepository: Repository<Component> {
    async fn load(&self, id: &ResourceId) -> Result<Option<Component>, RepositoryError>;
    async fn save(&self, item: &Component) -> Result<(), RepositoryError>;
}




