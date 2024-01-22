use proc_macro2::{Ident, TokenStream as TokenStream2};
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use crate::context::ScopeContext;
use crate::helper::ffi_constructor_name;
use crate::presentation::context::OwnedItemPresenterContext;
use crate::presentation::ScopeContextPresentable;

pub enum BindingPresentation {
    Constructor {
        ffi_ident: Ident,
        ctor_arguments: Vec<OwnedItemPresenterContext>,
        body_presentation: TokenStream2,
        context: Rc<RefCell<ScopeContext>>
    },
    EnumVariantConstructor {
        ffi_ident: TokenStream2,
        ffi_variant_ident: Ident,
        ffi_variant_path: TokenStream2,
        ctor_arguments: Vec<OwnedItemPresenterContext>,
        body_presentation: TokenStream2,
        context: Rc<RefCell<ScopeContext>>
    },
    Destructor {
        ffi_name: TokenStream2,
        destructor_ident: TokenStream2
    },

}

impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Constructor { ffi_ident, ctor_arguments, body_presentation, context} => {
                let context = context.borrow();
                let ctor_args = ctor_arguments.iter().map(|arg| arg.present(&context));
                let ffi_name = ffi_constructor_name(ffi_ident);
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_ident #body_presentation)
                    }
                }
            },
            Self::EnumVariantConstructor { ffi_ident, ffi_variant_ident, ffi_variant_path, ctor_arguments, body_presentation, context} => {
                let context = context.borrow();
                let ctor_args = ctor_arguments.iter().map(|arg| arg.present(&context));
                let ffi_name = ffi_constructor_name(ffi_variant_ident);
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_variant_path #body_presentation)
                    }
                }
            },
            Self::Destructor { ffi_name, destructor_ident } => {
                quote! {
                    /// # Safety
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn #destructor_ident(ffi: *mut #ffi_name) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
        }.to_tokens(tokens)
     }
}
