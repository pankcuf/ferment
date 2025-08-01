use syn::{parse_quote, Type};
use crate::ext::ToType;
use crate::lang::RustSpecification;
use crate::presentation::FFIFullPath;

impl ToType for FFIFullPath<RustSpecification> {
    fn to_type(&self) -> Type {
        match self {
            FFIFullPath::Type { crate_ident, ffi_name } =>
                parse_quote!(crate::fermented::types::#crate_ident::#ffi_name),
            FFIFullPath::Generic { ffi_name } =>
                parse_quote!(crate::fermented::generics::#ffi_name),
            FFIFullPath::External { path } =>
                parse_quote!(#path),
            FFIFullPath::Dictionary { path } =>
                path.to_type(),
        }
    }
}
