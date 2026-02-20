use crate::{adapters::driving::db::error::RepositoryError, domain::{component::Component}};

use super::{
    repository::Repository,
    seaorm_repository::SeaOrmRepository,

};

pub trait ComponentRepository: Repository<Component> {}


impl Repository<Component> for SeaOrmRepository {
    fn save(&self, item: &Component) -> impl Future<Output = Result<(), RepositoryError>> + Send {
        async move {
            todo!()
        }
    }
}

impl ComponentRepository for SeaOrmRepository {}
