use std::sync::{Arc, Mutex, RwLock};
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct ArcExamples {
    pub simple: Arc<u32>,
    pub complex: Arc<String>,
    pub opaque: Arc<FFIContext>,
    pub opaque_mutex: Arc<Mutex<FFIContext>>,
    pub opaque_rwlock: Arc<RwLock<FFIContext>>,
}