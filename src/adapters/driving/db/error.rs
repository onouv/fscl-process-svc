use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RepositoryError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}