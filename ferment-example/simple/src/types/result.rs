use crate::errors::context::{ContextProviderError, ContextProviderErrorOpaque};

#[ferment_macro::export]
pub struct ResultAll {
    pub primitive_primitive: Result<u32, u32>,
    pub primitive_string: Result<u32, String>,
    pub primitive_complex: Result<u32, ContextProviderError>,
    pub primitive_generic: Result<u32, Vec<ContextProviderError>>,
    pub primitive_opaque: Result<u32, ContextProviderErrorOpaque>,
}

#[ferment_macro::export]
pub fn primitive_primitive(result: Result<u32, u32>) -> Result<u32, u32> {
    result
}
#[ferment_macro::export]
pub fn primitive_string(result: Result<u32, String>) -> Result<u32, String> {
    result
}
#[ferment_macro::export]
pub fn primitive_complex(result: Result<u32, ContextProviderError>) -> Result<u32, ContextProviderError> {
    result
}
#[ferment_macro::export]
pub fn primitive_generic(result: Result<u32, Vec<ContextProviderError>>) -> Result<u32, Vec<ContextProviderError>> {
    result
}
