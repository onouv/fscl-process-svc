
use crate::domain::{ResourceId, item::Resource};

use super::error::RepositoryError;

pub trait Repository<T: Resource> : Clone + Send + Sync + 'static {
    fn load(&self, id: &ResourceId) -> impl std::future::Future<Output = Result<Option<T>, RepositoryError>> + Send;
    fn save(&self, item: &T) -> impl std::future::Future<Output = Result<(), RepositoryError>> + Send;
}