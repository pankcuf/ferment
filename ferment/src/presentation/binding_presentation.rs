use proc_macro2::{Ident, TokenStream as TokenStream2};
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use crate::context::ScopeContext;
use crate::naming::Name;
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
        destructor_ident: Name
    },
    ObjAsTrait {
        name: Name,
        item_type: TokenStream2,
        trait_type: TokenStream2,
        vtable_name: Name,
    },
    ObjAsTraitDestructor {
        name: Name,
        item_type: TokenStream2,
        trait_type: TokenStream2,
    }

}

impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Constructor { ffi_ident, ctor_arguments, body_presentation, context} => {
                let context = context.borrow();
                let ctor_args = ctor_arguments.iter().map(|arg| arg.present(&context));
                let ffi_name = Name::Costructor(ffi_ident.clone());
                quote! {
                    /// # Safety
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_ident #body_presentation)
                    }
                }
            },
            Self::EnumVariantConstructor { ffi_ident, ffi_variant_ident, ffi_variant_path, ctor_arguments, body_presentation, context} => {
                let context = context.borrow();
                let ctor_args = ctor_arguments.iter().map(|arg| arg.present(&context));
                let ffi_name = Name::Costructor(ffi_variant_ident.clone());
                quote! {
                    /// # Safety
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_variant_path #body_presentation)
                    }
                }
            },
            Self::Destructor { ffi_name, destructor_ident } => quote! {
                /// # Safety
                #[no_mangle]
                pub unsafe extern "C" fn #destructor_ident(ffi: *mut #ffi_name) {
                    ferment_interfaces::unbox_any(ffi);
                }
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, .. } => quote! {
                /// # Safety
                #[no_mangle]
                pub extern "C" fn #name(obj: *const #item_type) -> #trait_type {
                    #trait_type {
                        object: obj as *const (),
                        vtable: &#vtable_name,
                    }
                }
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, } => quote! {
                /// # Safety
                #[no_mangle]
                pub unsafe extern "C" fn #name(obj: #trait_type) {
                    ferment_interfaces::unbox_any(obj.object as *mut #item_type);
                }
            }
        }.to_tokens(tokens)
     }
}
