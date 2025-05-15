use std::os::raw::c_void;
use std::sync::Arc;
use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct Processor {
    pub provider: Arc<dyn CoreProvider>,
}

#[ferment_macro::opaque]
pub struct Cache {
    //pub provider: Arc<dyn CoreProvider>,
}

impl Processor {
    pub fn new(provider: Arc<dyn CoreProvider>) -> Self {
        Self { provider }
    }
}

#[ferment_macro::export]
impl Processor {
    pub fn register_initial_usernames(&mut self, model: &mut Cache, context: *const c_void) {

    }

}