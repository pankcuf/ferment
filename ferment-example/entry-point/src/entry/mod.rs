use std::sync::Arc;

pub mod core;
pub mod processor;
pub mod provider;
pub mod rnt;


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
pub type ModelByHeight = unsafe extern "C" fn(u32) -> SomeModel;

#[derive(Clone)]
// #[ferment_macro::opaque]
pub struct PlatformProvider {
    pub get_quorum_public_key: Arc<dyn Fn(*const FFIContext, u32, [u8; 32], u32) -> Result<[u8; 48], String> + Send + Sync>,
    pub get_data_contract: Arc<dyn Fn(*const FFIContext, String) -> Result<Option<Arc<SomeModel>>, String> + Send + Sync>,
    pub context: Arc<FFIContext>
}

#[ferment_macro::export]
impl PlatformProvider {
    pub fn new<
        QPK: Fn(*const FFIContext, u32, [u8; 32], u32) -> Result<[u8; 48], String> + Send + Sync + 'static,
        DC: Fn(*const FFIContext, String) -> Result<Option<Arc<SomeModel>>, String> + Send + Sync + 'static>(
        get_quorum_public_key: QPK,
        get_data_contract: DC,
        context: Arc<FFIContext>
    ) -> Self {
        Self {
            get_quorum_public_key: Arc::new(get_quorum_public_key),
            get_data_contract: Arc::new(get_data_contract),
            context
        }
    }

    pub fn maybe_context(&self) -> Option<FFIContext> {
        None
    }
}
