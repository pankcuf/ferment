use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::{ReturnType, Type};
use crate::composer::{ParenWrapped};
use crate::context::ScopeChain;


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

#[derive(Clone)]
pub struct TraitVTableMethodComposition {
    pub fn_name: Ident,
    pub ffi_fn_name: Ident,
    pub item_type: Type,
    pub trait_type: Type,
    pub argument_conversions: ParenWrapped<TokenStream2, TokenStream2>,
    pub name_and_args: TokenStream2,
    pub output_expression: ReturnType,
    pub output_conversions: TokenStream2,
}
