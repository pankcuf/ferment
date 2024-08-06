use crate::entry::provider::CoreProvider;

// #[ferment_macro::opaque]
pub struct MasternodeProcessor {
    pub provider: Box<dyn CoreProvider>,
}

#[ferment_macro::opaque]
pub struct FFICoreProvider {
    pub callback1: Box<dyn Fn(u32) -> Option<String>>,
    pub callback2: Box<dyn Fn([u8; 32]) -> u32>,
}

