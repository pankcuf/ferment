use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::{ReturnType, Type};
use crate::context::ScopeChain;


// #[derive(ferment_macro::CompositionContext)]
// pub enum TraitVTableCompositionContext {
//     Static,
// }

#[derive(Clone)]
pub struct TraitVTableComposition {
    pub fn_name: Ident,
    pub ffi_fn_name: Ident,
    pub implementor_ident: Ident,
    pub implementor_scope: ScopeChain,
    pub trait_ident: Ident,
    pub trait_scope: ScopeChain,
    pub method_compositions: Vec<TraitVTableMethodComposition>,
}

// impl TraitVTableComposition {
//     pub fn new(fn_name: Ident, ffi_fn_name: Ident, implementor_ident: Ident, trait_ident: Ident, implementor_scope: ScopeChain, trait_scope: ScopeChain, method_compositions: Vec<TraitVTableMethodComposition>) -> Self {
//         Self {
//             fn_name,
//             ffi_fn_name,
//             implementor_ident,
//             trait_ident,
//             implementor_scope,
//             trait_scope,
//             method_compositions,
//         }
//     }
//
// }

#[derive(Clone)]
pub struct TraitVTableMethodComposition {
    pub fn_name: Ident,
    pub ffi_fn_name: Ident,
    pub item_type: Type,
    pub trait_type: Type,
    pub argument_names: TokenStream2,
    pub name_and_args: TokenStream2,
    pub output_expression: ReturnType,
    pub output_conversions: TokenStream2,
}
