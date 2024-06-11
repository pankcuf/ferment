use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::ReturnType;


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