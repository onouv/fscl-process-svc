use crate::domain::{/*ResourceId*/ component::Component};
use super::{
    repository::Repository,
    //error::RepositoryError
};

pub trait ComponentRepository: Repository<Component> {}




