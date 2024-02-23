use proc_macro2::{Ident, TokenStream as TokenStream2};
use std::rc::Rc;
use std::cell::RefCell;
use quote::{format_ident, quote, ToTokens};
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext};
use crate::presentation::ScopeContextPresentable;

pub enum BindingPresentation {
    Constructor {
        ffi_ident: Ident,
        ctor_arguments: Vec<OwnedItemPresenterContext>,
        body_presentation: IteratorPresentationContext,
        context: Rc<RefCell<ScopeContext>>
    },
    EnumVariantConstructor {
        ffi_ident: TokenStream2,
        ffi_variant_ident: Ident,
        ffi_variant_path: TokenStream2,
        ctor_arguments: Vec<OwnedItemPresenterContext>,
        body_presentation: IteratorPresentationContext,
        context: Rc<RefCell<ScopeContext>>
    },
    Destructor {
        ffi_name: TokenStream2,
        destructor_ident: Name
    },
    Getter {
        field_name: TokenStream2,
        obj_type: TokenStream2,
        field_type: TokenStream2
    },
    Setter {
        field_name: TokenStream2,
        obj_type: TokenStream2,
        field_type: TokenStream2
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
                let body = body_presentation.present(&context);
                quote! {
                    /// # Safety
                    #[no_mangle]
                    #[inline(never)]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_ident #body)
                    }
                }
            },
            Self::EnumVariantConstructor { ffi_ident, ffi_variant_ident, ffi_variant_path, ctor_arguments, body_presentation, context} => {
                let context = context.borrow();
                let ctor_args = ctor_arguments.iter().map(|arg| arg.present(&context));
                let ffi_name = Name::Costructor(ffi_variant_ident.clone());
                let body = body_presentation.present(&context);
                quote! {
                    /// # Safety
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(#(#ctor_args),*) -> *mut #ffi_ident {
                        ferment_interfaces::boxed(#ffi_variant_path #body)
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
            },
            BindingPresentation::Getter { field_name, obj_type, field_type } => {
                let getter_name = format_ident!("{}_get_{}", obj_type.to_string(), field_name.to_string());
                // simple: (*obj).#field_name
                // complex: ferment_interfaces::FFIConversion::ffi_to((*obj).#field_name)
                quote! {
                    /// # Safety
                    #[no_mangle]
                    pub unsafe extern "C" fn #getter_name(obj: *const #obj_type) -> #field_type {
                        (*obj).#field_name
                    }
                }
            },
            BindingPresentation::Setter { field_name, obj_type, field_type } => {
                let setter_name = format_ident!("{}_set_{}", obj_type.to_string(), field_name.to_string());
                // simple: (*obj).#field_name
                // complex: ferment_interfaces::FFIConversion::ffi_to((*obj).#field_name)
                quote! {
                    /// # Safety
                    #[no_mangle]
                    pub unsafe extern "C" fn #setter_name(obj: *mut #obj_type, value: #field_type) {
                        (*obj).#field_name = value;
                    }
                }
            },

        }.to_tokens(tokens)
     }
}
