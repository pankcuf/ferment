use std::os::raw::c_void;
use std::sync::Arc;

pub mod core;
pub mod processor;
pub mod provider;
pub mod rnt;
pub mod coinjoin;

#[ferment_macro::export]
#[derive(Clone, Debug)]
pub struct SomeModel {
    pub hash: [u8; 32],
    pub desc: String,
}

#[derive(Clone, Debug)]
#[ferment_macro::opaque]
pub struct FFIContext {

}


#[ferment_macro::opaque]
pub type BlockHashByHeight = unsafe extern "C" fn(u32) -> [u8; 32];
#[ferment_macro::opaque]
pub type ModelByHeight = unsafe extern "C" fn(u32) -> u64;

#[derive(Clone)]
// #[ferment_macro::opaque]
pub struct PlatformProvider {
    pub get_quorum_public_key: Arc<dyn Fn(*const FFIContext, u32, [u8; 32], u32) -> Result<[u8; 48], String> + Send + Sync>,
    pub get_data_contract: Arc<dyn Fn(*const FFIContext, String) -> Result<Option<Arc<SomeModel>>, String> + Send + Sync>,
    pub maybe_identity: Arc<dyn Fn(*const c_void, [u8; 32], String) -> Option<FFIContext> + Send + Sync>,
    pub maybe_context_less1: Arc<dyn Fn(u32) -> Option<[u8; 32]> + Send + Sync>,
    pub maybe_context_less2: Arc<dyn Fn([u8; 32]) -> Option<[u8; 32]> + Send + Sync>,
    pub maybe_context_less3: Arc<dyn Fn([u8; 32], [u8; 32]) -> Result<u32, String> + Send + Sync>,
    pub context: Arc<FFIContext>
}

#[ferment_macro::export]
impl PlatformProvider {
    pub fn new<
        QPK: Fn(*const FFIContext, u32, [u8; 32], u32) -> Result<[u8; 48], String> + Send + Sync + 'static,
        DC: Fn(*const FFIContext, String) -> Result<Option<Arc<SomeModel>>, String> + Send + Sync + 'static,
        MaybeIdentity: Fn(*const c_void, [u8; 32], String) -> Option<FFIContext> + Send + Sync + 'static,
        MaybeContextLess1: Fn(u32) -> Option<[u8; 32]> + Send + Sync + 'static,
        MaybeContextLess2: Fn([u8; 32]) -> Option<[u8; 32]> + Send + Sync + 'static,
        MaybeContextLess3: Fn([u8; 32], [u8; 32]) -> Result<u32, String> + Send + Sync + 'static,
    >(
        get_quorum_public_key: QPK,
        get_data_contract: DC,
        maybe_identity: MaybeIdentity,
        maybe_context_less1: MaybeContextLess1,
        maybe_context_less2: MaybeContextLess2,
        maybe_context_less3: MaybeContextLess3,
        context: Arc<FFIContext>
    ) -> Self {
        Self {
            get_quorum_public_key: Arc::new(get_quorum_public_key),
            get_data_contract: Arc::new(get_data_contract),
            maybe_identity: Arc::new(maybe_identity),
            maybe_context_less1: Arc::new(maybe_context_less1),
            maybe_context_less2: Arc::new(maybe_context_less2),
            maybe_context_less3: Arc::new(maybe_context_less3),
            context
        }
    }

    pub fn maybe_context(&self) -> Option<FFIContext> {
        None
    }
    pub fn maybe_contexts(&self) -> Vec<FFIContext> {
        vec![]
    }
}
