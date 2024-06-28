mod arg_presentation;
mod binding_presentation;
mod conversion_interface_presentation;
mod destroy_presentation;
mod doc_presentation;
mod drop_interface_presentation;
mod expansion;
mod ffi_object_presentation;
mod from_conversion_presentation;
mod naming;
mod to_conversion_presentation;
mod ffi_full_path;
mod ffi_variable;
// mod trait_vtable_presentation;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::ReturnType;

pub use self::arg_presentation::*;
pub use self::binding_presentation::*;
pub use self::conversion_interface_presentation::*;
pub use self::destroy_presentation::*;
pub use self::doc_presentation::*;
pub use self::drop_interface_presentation::*;
pub use self::expansion::*;
pub use self::ffi_full_path::*;
pub use self::ffi_object_presentation::*;
pub use self::ffi_variable::*;
pub use self::from_conversion_presentation::*;
pub use self::naming::*;
pub use self::to_conversion_presentation::*;
//pub use self::trait_vtable_presentation::*;

pub fn create_struct<T: ToTokens>(ident: &Ident, attrs: TokenStream2, implementation: T) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #attrs
        pub struct #ident #implementation
    }
}

pub fn create_callback(ident: &Ident, attrs: TokenStream2, ffi_args: TokenStream2, result: ReturnType) -> TokenStream2 {
    match result {
        ReturnType::Default => create_struct(ident, attrs, quote! {{
            pub context: *const std::os::raw::c_void,
            caller: fn(#ffi_args),
        }}),
        ReturnType::Type(_, ref ty) => create_struct(ident, attrs, quote! {{
            pub context: *const std::os::raw::c_void,
            caller: fn(#ffi_args) #result,
            destructor: fn(result: #ty),
        }})
    }
}