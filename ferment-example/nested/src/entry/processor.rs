use std::sync::Arc;
use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct Processor {
    pub provider: Arc<dyn CoreProvider>,
}

impl Processor {
    pub fn new(provider: Arc<dyn CoreProvider>) -> Self {
        Self { provider }
    }
}
