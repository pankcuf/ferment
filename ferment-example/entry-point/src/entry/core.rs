use std::collections::BTreeMap;
use std::sync::Arc;
use ferment::boxed;
use crate::entry::{BlockHashByHeight, ModelByHeight};
use crate::entry::processor::MasternodeProcessor;
use crate::entry::provider::{FFIPtrCoreProvider, FFITraitCoreProvider};

#[ferment_macro::opaque]
pub struct DashSharedCore {
    pub processor: *mut MasternodeProcessor,
    pub cache: BTreeMap<String, String>,
    pub context: *const std::os::raw::c_void,
}

#[ferment_macro::export]
impl DashSharedCore {
    pub fn with_pointers(
        block_hash_by_height: BlockHashByHeight,
        model_by_height: ModelByHeight,
        context: *const std::os::raw::c_void) -> Self {
        Self {
            processor: boxed(MasternodeProcessor { provider: Arc::new(FFIPtrCoreProvider { block_hash_by_height, model_by_height }) }),
            cache: Default::default(),
            context
        }
    }
    pub fn with_lambdas<BHH: Fn(u32) -> [u8; 32] + 'static, SBH: Fn(u32) -> u64 + 'static>(
        block_hash_by_height: BHH,
        model_by_height: SBH,
        context: *const std::os::raw::c_void) -> Self
        where {
        Self {
            processor: boxed(MasternodeProcessor {
                provider: Arc::new(FFITraitCoreProvider {
                    block_hash_by_height: Box::new(block_hash_by_height),
                    model_by_height: Box::new(model_by_height)
                })
            }),
            cache: Default::default(),
            context
        }
    }

    pub async fn sign_and_publish_state_transition(&self, _private_key: &[u8]) -> u32 {
        0
    }

}
