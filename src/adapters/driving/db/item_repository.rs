use crate::domain::item::ResourceId;

use super::error::RepositoryError;

pub trait ItemRepository : Clone + Send + Sync + 'static {
    fn exist_item(id: ResourceId) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
