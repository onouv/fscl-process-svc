pub use fscl_core::{ResourceId, ResourceIdError};

pub trait Resource {
    fn id(&self) -> ResourceId;
}