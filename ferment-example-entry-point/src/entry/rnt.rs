use std::collections::BTreeMap;
use tokio::runtime::Runtime;
use ferment_interfaces::boxed;
use crate::entry::{BlockHashByHeight, ModelByHeight, SomeModel};
use crate::entry::processor::MasternodeProcessor;
use crate::entry::provider::{FFIPtrCoreProvider, FFITraitCoreProvider};

#[ferment_macro::opaque]
pub struct DashSharedCoreWithRuntime {
    pub processor: *mut MasternodeProcessor,
    pub runtime: *mut Runtime,
    pub cache: BTreeMap<String, String>,
    pub context: *const std::os::raw::c_void,
}

#[ferment_macro::export]
impl DashSharedCoreWithRuntime {
    pub fn with_pointers(
        block_hash_by_height: BlockHashByHeight,
        model_by_height: ModelByHeight,
        runtime: *mut Runtime,
        context: *const std::os::raw::c_void) -> Self {
        Self {
            processor: boxed(MasternodeProcessor {
                provider: Box::new(FFIPtrCoreProvider { block_hash_by_height, model_by_height }) }),
            cache: Default::default(),
            runtime,
            context
        }
    }
    pub fn with_lambdas<BHH: Fn(u32) -> [u8; 32] + 'static, SBH: Fn(u32) -> SomeModel + 'static>(
        block_hash_by_height: BHH,
        model_by_height: SBH,
        runtime: *mut Runtime,
        context: *const std::os::raw::c_void) -> Self
        where {
        Self {
            processor: boxed(MasternodeProcessor {
                provider: Box::new(FFITraitCoreProvider {
                    block_hash_by_height: Box::new(block_hash_by_height),
                    model_by_height: Box::new(model_by_height) }) }),
            cache: Default::default(),
            runtime,
            context
        }
    }
}
