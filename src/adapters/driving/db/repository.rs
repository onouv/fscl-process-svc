
use crate::domain::item::Item;

use super::{ ItemRepository, error::RepositoryError};

pub trait Repository<T: Item>:  ItemRepository + Send {
    fn save(&self, item: &T) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}