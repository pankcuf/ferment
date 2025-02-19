use std::sync::Arc;
use crate::entry::provider::CoreProvider;

#[ferment_macro::opaque]
pub struct MasternodeProcessor {
    pub provider: Arc<dyn CoreProvider>,
}

#[ferment_macro::opaque]
pub struct FFICoreProvider {
    pub callback1: Box<dyn Fn(u32) -> Option<String>>,
    pub callback2: Box<dyn Fn([u8; 32]) -> u32>,
    // pub get_quorum_public_key: Arc<dyn Fn(*const SomeModel, u32, [u8; 32], u32) -> Result<[u8; 48], String> + Send + Sync>,

}

impl MasternodeProcessor {
    pub fn new(provider: Arc<dyn CoreProvider>) -> Self {
        Self { provider }
    }
}
