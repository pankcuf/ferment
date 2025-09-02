use std::os::raw::c_void;
use std::sync::Arc;
use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct Processor {
    pub _provider: Arc<dyn CoreProvider>,
}

#[ferment_macro::opaque]
pub struct Cache {
    //pub provider: Arc<dyn CoreProvider>,
}

impl Processor {
    #[allow(unused)]
    pub fn new(_provider: Arc<dyn CoreProvider>) -> Self {
        Self { _provider }
    }
}

#[ferment_macro::export]
impl Processor {
    pub fn register_initial_usernames(&mut self, _model: &mut Cache, _context: *const c_void) {

    }

}