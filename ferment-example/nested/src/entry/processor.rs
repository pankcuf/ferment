use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct Processor {
    pub chain_id: Box<dyn CoreProvider>,
}

// #[ferment_macro::export]
// pub fn get_provider(processor: Processor) -> Result<Box<dyn CoreProvider>, ProtocolError> {
//     Ok(processor.chain_id)
// }