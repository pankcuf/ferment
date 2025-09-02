use std::borrow::Cow;
use crate::entry::FFIContext;

#[ferment_macro::export]
pub struct CowExample<'a> {
    pub simple: Cow<'a, u16>,
    pub complex: Cow<'a, String>,
    pub opaque: Cow<'a, FFIContext>,
}
