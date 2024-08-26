use std::sync::Arc;

pub trait ContextProvider: Send + Sync {
    fn get_quorum_public_key(
        &self,
        quorum_type: u32,
        quorum_hash: String,
    ) -> String;

}


#[derive(Clone)]
#[ferment_macro::opaque]
pub struct FFIContext {
    pub caller: Arc<dyn Fn(*const FFIContext, u32, String) -> String>,
    // pub destructor: Arc<dyn Fn(*const FFIContext, String)>,
    // pub destructor: Arc<dyn Fn(String)>,
}

// #[ferment_macro::export]
// pub type GetQuorumPublicKey = fn(
//     context: *const FFIContext,
//     quorum_type: u32,
//     quorum_hash: String,
// ) -> String;

#[derive(Clone)]
#[ferment_macro::opaque]
pub struct PlatformProvider {
    // pub get_quorum_public_key_callback: Arc<dyn Fn(*const FFIContext, u32, [u8; 32], u32) -> Result<[u8; 48], String>>,
    pub context: Arc<FFIContext>
}
impl PlatformProvider {
    fn context_ptr(&self) -> *const FFIContext {
        Arc::as_ptr(&self.context)
    }
}
#[ferment_macro::export]
impl PlatformProvider {
    pub fn new(context: Arc<FFIContext>) -> Self {
        Self { context }
    }
    // pub fn new<T: Fn(*const FFIContext, u32, String) -> String + 'static>(get_quorum_public_key_callback: T, context: Arc<FFIContext>) -> Self {
    //     Self { context }
    // }
}

impl ContextProvider for PlatformProvider {
    fn get_quorum_public_key(&self, quorum_type: u32, quorum_hash: String) -> String {
        (self.context.caller)(self.context_ptr(), quorum_type, quorum_hash)
    }
}

unsafe impl Send for PlatformProvider {}
unsafe impl Sync for PlatformProvider {}