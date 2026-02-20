pub enum UseCaseError {}

pub trait UseCase {
    fn run(&self) -> Result<(), UseCaseError>;
}