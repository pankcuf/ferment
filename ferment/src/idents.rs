use quote::{quote, ToTokens};
use syn::{__private::TokenStream2, Path};
use crate::context::ScopeContext;
use crate::conversion::{GenericPathConversion, PathConversion};
use crate::formatter::format_token_stream;
use crate::helper::{ffi_mangled_ident, path_arguments_to_paths};

pub fn is_primitive(id: &str) -> bool {
    match id {
        "i8" | "u8" |
        "i16" | "u16" |
        "i32" | "u32" |
        "i64" | "u64" |
        "i128" | "u128" |
        "isize" | "usize" |
        "bool" =>
            true,
        _ =>
            false
    }
}



pub fn ffi_dictionary_field_type(path: &Path, context: &ScopeContext) -> TokenStream2 {
    println!("ffi_dictionary_field_type: {}", format_token_stream(path));
    match path.segments.last().unwrap().ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
        "isize" | "usize" | "bool" =>
            quote!(#path),
        "OpaqueContext" =>
            quote!(ferment_interfaces::OpaqueContext_FFI),
        "OpaqueContextMut" =>
            quote!(ferment_interfaces::OpaqueContextMut_FFI),
        "Option" =>
            ffi_dictionary_field_type(path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap(), context),
        "Vec" | "BTreeMap" | "HashMap" => {
            let path = context.scope_type_for_path(path)
                .map_or(path.to_token_stream(), |full_type| ffi_mangled_ident(&full_type).to_token_stream());
            quote!(*mut #path)
        },
        "Result" /*if path.segments.len() == 1*/ => {
            let path = context.scope_type_for_path(path)
                .map_or(path.to_token_stream(), |full_type| ffi_mangled_ident(&full_type).to_token_stream());
            quote!(*mut #path)
        },
        _ =>
            quote!(*mut #path),
    }
}

