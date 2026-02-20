use crate::domain::item::ItemId;

use super::error::RepositoryError;

pub trait ItemRepository : Send {
    fn exist_item(id: ItemId) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
