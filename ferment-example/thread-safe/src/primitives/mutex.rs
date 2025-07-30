use std::sync::{Arc, Mutex};
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct MutexExample {
    pub simple: Mutex<u32>,
    pub complex: Mutex<String>,
    pub opaque: Mutex<FFIContext>,
}
#[ferment_macro::export]
pub struct ArcMutexExample {
    pub simple: Arc<Mutex<u32>>,
    pub complex: Arc<Mutex<String>>,
    pub opaque: Arc<Mutex<FFIContext>>,
}