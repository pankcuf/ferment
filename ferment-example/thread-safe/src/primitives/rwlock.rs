use std::sync::{Arc, RwLock};
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct RwLockExample {
    pub simple: RwLock<u32>,
    pub complex: RwLock<String>,
    pub opaque: RwLock<FFIContext>,
}
#[ferment_macro::export]
pub struct ArcRwLockExample {
    pub simple: Arc<RwLock<u32>>,
    pub complex: Arc<RwLock<String>>,
    pub opaque: Arc<RwLock<FFIContext>>,
}