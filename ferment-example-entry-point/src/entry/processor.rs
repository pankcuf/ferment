use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct Processor {
    pub chain_id: Box<dyn CoreProvider>,
}
