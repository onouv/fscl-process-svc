
use crate::domain::{ResourceId, item::Resource};

use super::{ ItemRepository, error::RepositoryError};

pub trait Repository<T: Resource>:  ItemRepository {
    fn load(&self, id: &ResourceId) -> impl Future<Output = Result<Option<T>, RepositoryError>> + Send;
    fn save(&self, item: &T) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}