use std::sync::Arc;

use crate::ports::ComponentPort;

#[derive(Clone)]
pub(super) struct AppState<C: ComponentPort> {
    pub component_service: Arc<C>
}