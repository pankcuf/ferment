use std::sync::{Arc, OnceLock};
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct RwLockExample {
    pub simple: OnceLock<u32>,
    pub complex: OnceLock<String>,
    pub opaque: OnceLock<FFIContext>,
}
#[ferment_macro::export]
pub struct ArcRwLockExample {
    pub simple: Arc<OnceLock<u32>>,
    pub complex: Arc<OnceLock<String>>,
    pub opaque: Arc<OnceLock<FFIContext>>,
}