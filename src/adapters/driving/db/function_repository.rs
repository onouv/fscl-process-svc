use crate::domain::function::Function;

use super::{error::RepositoryError, repository::Repository, seaorm_repository::SeaOrmRepository};

impl Repository<Function> for SeaOrmRepository {
    fn save(item: &Function) -> impl Future<Output = Result<(), RepositoryError>> + Send {
        async move { todo!() }
    }
}
