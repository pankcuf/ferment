// #[ferment_macro::export]
// pub fn get_slice_simple<'a>() -> &'a [u8] {
//     &[]
// }

use crate::errors::context::ContextProviderError;

#[ferment_macro::export]
pub fn slice_simple_len<'a>(slice: &[u8]) -> usize {
    slice.len()
}

#[ferment_macro::export]
pub fn slice_complex_len<'a>(slice: &[ContextProviderError]) -> usize {
    slice.len()
}
#[ferment_macro::export]
pub fn slice_of_string_len<'a>(slice: &[String]) -> usize {
    slice.len()
}

#[ferment_macro::export]
pub fn slice_generic_len<'a>(slice: &[Vec<ContextProviderError>]) -> usize {
    slice.len()
}
