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
mod ffi_full_path;
mod ffi_variable;
// mod trait_vtable_presentation;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Attribute, ReturnType};

pub use self::arg_presentation::*;
pub use self::binding_presentation::*;
pub use self::conversion_interface_presentation::*;
pub use self::doc_presentation::*;
pub use self::drop_interface_presentation::*;
pub use self::expansion::*;
pub use self::ffi_full_path::*;
pub use self::ffi_object_presentation::*;
pub use self::ffi_variable::*;
pub use self::naming::*;
//pub use self::trait_vtable_presentation::*;

pub fn create_struct<T: ToTokens>(ident: &Ident, attrs: &Vec<Attribute>, implementation: T) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone)]
        #(#attrs)*
        pub struct #ident #implementation
    }
}

pub fn create_callback(ident: &Ident, attrs: &Vec<Attribute>, ffi_args: TokenStream2, result: ReturnType) -> TokenStream2 {
    //let context = quote! { pub context: *const std::os::raw::c_void, };
    let result_impl = match result {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ref ty) => quote! { #result, destructor: unsafe extern "C" fn(result: #ty) }
        // ReturnType::Type(_, ref ty) => quote! { #result, destructor: unsafe extern "C" fn(o_0: *const ferment_example_thread_safe::entry::FFIContext, result: #ty) }
    };
    create_struct(ident, attrs, quote! {{
            // #context
            caller: unsafe extern "C" fn(#ffi_args) #result_impl,
        }})
}