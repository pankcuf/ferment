use syn::Path;
use quote::quote;
use syn::__private::TokenStream2;
use crate::ext::Terminated;
use crate::naming::DictionaryFieldName;


pub fn create_struct(path: &Path, implementation: TokenStream2) -> TokenStream2 {
    let ident = &path.segments.last().unwrap().ident;
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        pub struct #ident #implementation
    }
}

pub fn package_unbox_any_expression(expr: TokenStream2) -> TokenStream2 {
    let package = DictionaryFieldName::Package;
    quote!(#package::unbox_any(#expr))
}

pub fn package_unbox_any_expression_terminated(expr: TokenStream2) -> TokenStream2 {
    package_unbox_any_expression(expr)
        .terminated()
}

pub fn package_unboxed_root() -> TokenStream2 {
    package_unbox_any_expression(quote!(ffi))
}

// pub fn package_boxed_expression(expr: TokenStream2) -> TokenStream2 {
//     let package = DictionaryFieldName::Package;
//     quote!(#package::boxed(#expr))
// }

pub fn ffi_from_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = DictionaryFieldName::Package;
    let interface = DictionaryFieldName::Interface;
    quote!(#package::#interface::ffi_from(#field_value))
}

pub fn ffi_to_conversion(field_path: TokenStream2) -> TokenStream2 {
    let package = DictionaryFieldName::Package;
    let interface = DictionaryFieldName::Interface;
    quote!(#package::#interface::ffi_to(#field_path))
}
