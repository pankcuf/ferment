use syn::{parse_quote, Type};
use crate::ext::ToType;
use crate::lang::RustSpecification;
use crate::presentation::FFIFullDictionaryPath;

impl ToType for FFIFullDictionaryPath<RustSpecification> {
    fn to_type(&self) -> Type {
        match self {
            FFIFullDictionaryPath::Void => parse_quote!(std::os::raw::c_void),
            FFIFullDictionaryPath::CChar => parse_quote!(std::os::raw::c_char),
            FFIFullDictionaryPath::Phantom(_) => panic!("")
        }
    }
}
