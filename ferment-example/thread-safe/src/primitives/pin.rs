use std::pin::Pin;
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct PinExamples {
    pub opaque: Pin<Box<FFIContext>>,
}