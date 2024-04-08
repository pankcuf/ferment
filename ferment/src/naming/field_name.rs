use crate::ext::{Mangle, MangleDefault};
use crate::helper::usize_to_tokenstream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use std::fmt::Formatter;
use syn::__private::TokenStream2;
use syn::{Pat, Path, Type};

#[derive(Clone, Debug)]
pub enum DictionaryFieldName {
    Ok,
    Error,
    Keys,
    Values,
    Count,
    Obj,
    Self_,
    // O,
    Package,
    Interface,
}

#[derive(Clone, Debug)]
pub enum DictionaryExpression {
    BoxedExpression(TokenStream2),
    FromPrimitiveVec(TokenStream2, TokenStream2),
    FromComplexVec(TokenStream2, TokenStream2),
}


impl std::fmt::Display for DictionaryFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::fmt::Display for DictionaryExpression {
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
            DictionaryFieldName::Obj => quote!(obj),
            DictionaryFieldName::Package => quote!(ferment_interfaces),
            DictionaryFieldName::Interface => quote!(FFIConversion),
            DictionaryFieldName::Self_ => quote!(self_),
        }
        .to_tokens(tokens)
    }
}
impl ToTokens for DictionaryExpression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            DictionaryExpression::BoxedExpression(expr) => {
                let package = DictionaryFieldName::Package;
                quote!(#package::boxed(#expr))
            }
            DictionaryExpression::FromPrimitiveVec(values, count) => {
                let package = DictionaryFieldName::Package;
                quote!(#package::from_primitive_vec(#values, #count))
            }
            DictionaryExpression::FromComplexVec(values, count) => {
                let package = DictionaryFieldName::Package;
                quote!(#package::from_complex_vec(#values, #count))
            }
        }.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub enum Name {
    UnnamedArg(usize),
    Constructor(Type),
    Destructor(Type),
    Dictionary(DictionaryFieldName),
    Optional(Option<Ident>),
    UnnamedStructFieldsComp(Type, usize),
    TraitObj(Ident),
    TraitImplVtable(Ident, Ident),
    TraitFn(Type, Type),
    TraitDestructor(Type, Type),
    Vtable(Ident),
    ModFn(Path),
    VTableInnerFn(Ident),
    Getter(Path, TokenStream2),
    Setter(Path, TokenStream2),
    Ident(Ident),
    Pat(Pat),
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Name::UnnamedArg(..) => self.mangle_ident_default().to_token_stream(),
            Name::Constructor(ident) => {
                format_ident!("{}_ctor", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Destructor(ident) => {
                format_ident!("{}_destroy", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream(),
            Name::Vtable(trait_name) => format_ident!("{}_VTable", trait_name).to_token_stream(),
            Name::TraitObj(ident) => ident.to_token_stream(),
            Name::ModFn(path) => path.mangle_ident_default().to_token_stream(),
            Name::TraitFn(item_name, trait_name) => {
                format_ident!("{}_as_{}", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_token_stream()
            }
            Name::TraitDestructor(item_name, trait_name) => {
                format_ident!("{}_as_{}_destroy", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_token_stream()
            }
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Ptr(_) => DictionaryFieldName::Obj.to_token_stream(),
                _ => usize_to_tokenstream(* index)
            },
            // Name::UnnamedStructFieldsComp(ty, index) => match ty {
            //     Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
            //         PathConversion::Primitive(..) => usize_to_tokenstream(*index),
            //         _ => usize_to_tokenstream(*index),
            //     },
            //     Type::Array(_type_array) => usize_to_tokenstream(*index),
            //     Type::Ptr(_type_ptr) => DictionaryFieldName::Obj.to_token_stream(),
            //     _ => unimplemented!("from_unnamed_struct: not supported {}", quote!(#ty)),
            // },
            Name::TraitImplVtable(item_name, trait_vtable_ident) => {
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_token_stream()
            }
            Name::VTableInnerFn(ident) => ident.to_token_stream(),

            Name::Getter(obj_type, field_name) => format_ident!(
                "{}_get_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            )
            .to_token_stream(),
            Name::Setter(obj_type, field_name) => format_ident!(
                "{}_set_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            )
            .to_token_stream(),
            Name::Ident(variant) => quote!(#variant),
            Name::Optional(ident) => quote!(#ident),
            Name::Pat(pat) => pat.to_token_stream(),
        }
        .to_tokens(tokens)
    }
}

impl Mangle<MangleDefault> for Name {
    // type Context = MangleDefault;

    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Path(..) => usize_to_tokenstream(*index).to_string(),
                Type::Array(_type_array) => usize_to_tokenstream(*index).to_string(),
                Type::Ptr(_type_ptr) => DictionaryFieldName::Obj.to_string(),
                _ => unimplemented!(
                    "Name::UnnamedStructFieldsComp :: to_mangled_string: unsupported type {}",
                    quote!(#ty)
                ),
            },
            Name::UnnamedArg(index) => format!("o_{}", index),
            Name::Constructor(ident) => format!("{}_ctor", ident.mangle_ident_default()),
            Name::Destructor(ident) => format!("{}_destroy", ident.mangle_ident_default()),
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream().to_string(),
            Name::ModFn(name) => name.mangle_string(context).to_string(),
            Name::TraitObj(ident) => ident.to_string(),
            Name::TraitImplVtable(item_name, trait_vtable_ident) => {
                format!("{}_{}", item_name, trait_vtable_ident)
            }
            Name::TraitFn(item_name, trait_name) => format!("{}_as_{}", item_name.mangle_ident_default(), trait_name.mangle_ident_default()),
            Name::TraitDestructor(item_name, trait_name) => {
                format!("{}_as_{}_destroy", item_name.mangle_ident_default(), trait_name.mangle_ident_default())
            }
            Name::Vtable(trait_name) => format!("{}_VTable", trait_name),
            Name::Getter(obj_type, field_name) => format!(
                "{}_get_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            ),
            Name::Setter(obj_type, field_name) => format!(
                "{}_set_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            ),
            Name::Ident(variant) => variant.to_string(),
            Name::Optional(ident) => quote!(#ident).to_string(),
            Name::Pat(pat) => pat.to_token_stream().to_string(),
            Name::VTableInnerFn(ident) => ident.to_token_stream().to_string(),

        }
    }
}
