use std::collections::BTreeMap;
use std::sync::Arc;
use ferment::boxed;
use crate::entry::{BlockHashByHeight, ModelByHeight, SomeModel};
use crate::entry::processor::Processor;
use crate::entry::provider::{FFIPtrCoreProvider, FFITraitCoreProvider};

#[ferment_macro::opaque]
pub struct DashSharedCore {
    pub _processor: *mut Processor,
    pub _cache: BTreeMap<String, String>,
    pub _context: *const std::os::raw::c_void,
}

#[ferment_macro::export]
impl DashSharedCore {
    pub fn with_pointers(
        block_hash_by_height: BlockHashByHeight,
        model_by_height: ModelByHeight,
        _context: *const std::os::raw::c_void
    ) -> Self {
        Self {
            _processor: boxed(Processor { _provider: Arc::new(FFIPtrCoreProvider { block_hash_by_height, model_by_height }) }),
            _cache: Default::default(),
            _context
        }
    }
    pub fn with_lambdas<BHH: Fn(u32) -> [u8; 32] + 'static, SBH: Fn(u32) -> SomeModel + 'static>(
        block_hash_by_height: BHH,
        model_by_height: SBH,
        _context: *const std::os::raw::c_void
    ) -> Self
        where {
        Self {
            _processor: boxed(Processor {
                _provider: Arc::new(FFITraitCoreProvider {
                    block_hash_by_height: Box::new(block_hash_by_height),
                    model_by_height: Box::new(model_by_height) }) }),
            _cache: Default::default(),
            _context
        }
    }

    pub fn test_by_ref(&self, data: &[u8]) -> Result<u32, u32> {
        Ok(data.len() as u32)
    }
    pub fn test_vec_by_ref(&self, data: &Vec<u8>) -> Result<u32, u32> {
        Ok(data.len() as u32)
    }
}
