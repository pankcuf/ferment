use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Pat, Type, TypePath};
use crate::conversion::PathConversion;
use crate::helper::usize_to_tokenstream;
use crate::interface::obj;

#[derive(Clone)]
pub enum DictionaryFieldName {
    Ok,
    Error,
    Keys,
    Values,
    Count,
    Obj
}
impl std::fmt::Debug for DictionaryFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DictionaryFieldName::Ok => format!("DictionaryFieldName::Ok"),
            DictionaryFieldName::Error => format!("DictionaryFieldName::Error"),
            DictionaryFieldName::Keys => format!("DictionaryFieldName::Keys"),
            DictionaryFieldName::Values => format!("DictionaryFieldName::Values"),
            DictionaryFieldName::Count => format!("DictionaryFieldName::Count"),
            DictionaryFieldName::Obj => format!("DictionaryFieldName::Obj"),
        }.as_str())
    }
}
impl std::fmt::Display for DictionaryFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl ToTokens for DictionaryFieldName {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            DictionaryFieldName::Ok => quote!(ok),
            DictionaryFieldName::Error => quote!(error),
            DictionaryFieldName::Keys => quote!(keys),
            DictionaryFieldName::Values => quote!(values),
            DictionaryFieldName::Count => quote!(count),
            DictionaryFieldName::Obj => quote!(obj)
        }.to_tokens(tokens)
    }
}

#[derive(Clone)]
pub enum Name {
    UnamedArg(usize),
    Costructor(Ident),
    Destructor(Ident),
    Dictionary(DictionaryFieldName),
    Optional(Option<Ident>),
    Pat(Pat),
    UnnamedStructFieldsComp(Type, usize),
    TraitObj(Ident),
    TraitImplVtable(Ident, Ident),
    TraitFn(Ident, Ident),
    TraitDestructor(Ident, Ident),
    Vtable(Ident),
    ModFn(Ident),
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Name::UnamedArg(index) => format!("Name::UnamedArg({})", index),
            Name::Costructor(ident) => format!("Name::Costructor({})", ident),
            Name::Destructor(ident) => format!("Name::Destructor({})", ident),
            Name::Dictionary(dict_field_name) => format!("Name::Dictionary({})", dict_field_name),
            Name::Optional(ident) => format!("Name::Optional({:?})", ident),
            Name::Pat(pat) => format!("Name::Pat({})", pat.to_token_stream()),
            Name::UnnamedStructFieldsComp(ty, index) => format!("Name::UnnamedStructFieldsComp({}, {})", ty.to_token_stream(), index),
            Name::TraitObj(ident) => format!("Name::TraitObj({})", ident),
            Name::TraitFn(item_name, trait_name) => format!("Name::TraitFn({}, {})", item_name, trait_name),
            Name::TraitDestructor(item_name, trait_name) => format!("Name::TraitDestructor({}, {})", item_name, trait_name),
            Name::Vtable(ident) => format!("Name::Vtable({})", ident),
            Name::ModFn(ident) => format!("Name::Fn({})", ident),
            Name::TraitImplVtable(item_name, trait_vtable_ident) => format!("Name::TraitImplVtable({}, {})", item_name.to_token_stream(), trait_vtable_ident.to_token_stream()),
        }.as_str())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Name::UnamedArg(index) => format_ident!("o_{}", index).to_token_stream(),
            Name::Costructor(ident) => format_ident!("{}_ctor", ident).to_token_stream(),
            Name::Destructor(ident) => format_ident!("{}_destroy", ident).to_token_stream(),
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream(),
            Name::Vtable(trait_name) => format_ident!("{}_VTable", trait_name).to_token_stream(),
            Name::TraitObj(ident) => ident.to_token_stream(), // format_ident!("{}_TraitObject", ident)
            Name::Optional(ident) => quote!(#ident),
            Name::Pat(pat) => pat.to_token_stream(),
            Name::ModFn(name) => format_ident!("{}", name).to_token_stream(), // format_ident!("ffi_{}", fn_name)
            Name::TraitFn(item_name, trait_name) => format_ident!("{}_as_{}", item_name, trait_name).to_token_stream(),
            Name::TraitDestructor(item_name, trait_name) => format_ident!("{}_as_{}_destroy", item_name, trait_name).to_token_stream(),
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                    PathConversion::Primitive(..) => usize_to_tokenstream(*index),
                    _ => usize_to_tokenstream(*index),
                },
                Type::Array(_type_array) => usize_to_tokenstream(*index),
                Type::Ptr(_type_ptr) => obj(),
                _ => unimplemented!("from_unnamed_struct: not supported {}", quote!(#ty))
            },
            Name::TraitImplVtable(item_name, trait_vtable_ident) =>
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_token_stream(),

        }.to_tokens(tokens)
    }
}