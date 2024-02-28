use std::fmt::Formatter;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Pat, Path, Type, TypePath};
use crate::conversion::PathConversion;
use crate::ext::{Mangle, ManglingRules};
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
    UnnamedArg(usize),
    Constructor(Box<Name>),
    Destructor(Box<Name>),
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
    Getter(Path, TokenStream2),
    Setter(Path, TokenStream2),
    // Enum(Ident),
    Variant(Ident),
    EnumFFIVariant(Ident, Ident),
    EnumVariant(Type, Ident),
    MangledType(Type),
    Path(Path),
    Type(Type),
    Just(TokenStream2)
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Name::UnnamedArg(index) => format!("Name::UnnamedArg({})", index),
            Name::Constructor(ident) => format!("Name::Constructor({})", ident),
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
            Name::Getter(obj_type, field_name) => format!("Name::Getter({}, {})", quote!(#obj_type), field_name),
            Name::Setter(obj_type, field_name) => format!("Name::Setter({}, {})", quote!(#obj_type), field_name),
            // Name::Enum(name) => format!("Name::Enum({})", name),
            Name::Variant(variant) => format!("Name::Variant({})", variant),
            Name::EnumVariant(enum_name, variant) => format!("Name::EnumVariant({}, {})", enum_name.to_token_stream(), variant),
            Name::EnumFFIVariant(enum_ffi_type, variant) => format!("Name::EnumFFIVariant({}, {})", enum_ffi_type, variant),
            Name::MangledType(ty) => format!("Name::MangledType({})", ty.to_token_stream()),
            Name::Path(path) => format!("Name::Path({})", path.to_token_stream()),
            Name::Type(ty) => format!("Name::Type({})", ty.to_token_stream()),
            Name::Just(ts) => format!("Name::Just({})", ts),
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
            Name::UnnamedArg(index) => format_ident!("o_{}", index).to_token_stream(),
            Name::Constructor(ident) => format_ident!("{}_ctor", ident.to_token_stream().to_string()).to_token_stream(),
            Name::Destructor(ident) => format_ident!("{}_destroy", ident.to_token_stream().to_string()).to_token_stream(),
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

            Name::Getter(obj_type, field_name) => format_ident!("{}_get_{}", obj_type.to_mangled_ident_default().to_string(), field_name.to_string()).to_token_stream(),
            Name::Setter(obj_type, field_name) => format_ident!("{}_set_{}", obj_type.to_mangled_ident_default().to_string(), field_name.to_string()).to_token_stream(),
            // Name::Enum(name) => quote!(#name),
            Name::Variant(variant) => quote!(#variant),
            Name::MangledType(ty) => ty.to_mangled_ident_default().to_token_stream(),
            Name::Path(path) => path.to_token_stream(),
            Name::EnumVariant(enum_name, variant) => {
                // println!("EnumVariant::: {} --- {}", enum_name.to_token_stream(), variant);
                quote!(#enum_name::#variant)
                // format_ident!("{}_{}", enum_name.to_mangled_ident_default().to_string(), variant.to_string()).to_token_stream()
            }
            Name::EnumFFIVariant(enum_name, variant) => {
                format_ident!("{}_{}", enum_name, variant).to_token_stream()
            }
            Name::Type(ty) => quote!(#ty),
            Name::Just(ts) => quote!(#ts)
        }.to_tokens(tokens)
    }
}

impl Mangle for Name {
    fn to_mangled_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                match self {
                    Name::UnnamedArg(index) => format!("o_{}", index),
                    Name::Constructor(ident) => format!("{}_ctor", ident.to_mangled_ident_default()),
                    Name::Destructor(ident) => format!("{}_destroy", ident.to_mangled_ident_default()),
                    Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream().to_string(),
                    Name::Optional(ident) => ident.as_ref().map(|i| i.to_string()).unwrap_or_default(),
                    Name::Pat(pat) => pat.to_token_stream().to_string(),
                    Name::UnnamedStructFieldsComp(ty, index) => match ty {
                        Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                            PathConversion::Primitive(..) => usize_to_tokenstream(*index).to_string(),
                            _ => usize_to_tokenstream(*index).to_string(),
                        },
                        Type::Array(_type_array) => usize_to_tokenstream(*index).to_string(),
                        Type::Ptr(_type_ptr) => obj().to_string(),
                        _ => unimplemented!("from_unnamed_struct: not supported {}", quote!(#ty))
                    },
                    Name::TraitObj(ident) => ident.to_token_stream().to_string(), // format_ident!("{}_TraitObject", ident)
                    Name::TraitImplVtable(item_name, trait_vtable_ident) =>
                        format!("{}_{}", item_name, trait_vtable_ident),
                    Name::TraitFn(item_name, trait_name) => format!("{}_as_{}", item_name, trait_name),
                    Name::TraitDestructor(item_name, trait_name) => format!("{}_as_{}_destroy", item_name, trait_name),
                    Name::Vtable(trait_name) => format!("{}_VTable", trait_name),
                    Name::ModFn(name) => name.to_string(), // format_ident!("ffi_{}", fn_name)
                    Name::Getter(obj_type, field_name) => format!("{}_get_{}", obj_type.to_mangled_ident_default(), field_name.to_string()),
                    Name::Setter(obj_type, field_name) => format!("{}_set_{}", obj_type.to_mangled_ident_default(), field_name.to_string()),
                    Name::Variant(variant) => variant.to_string(),
                    Name::EnumFFIVariant(enum_name, variant) => format!("{}_{}", enum_name, variant),
                    Name::EnumVariant(enum_name, variant) => format!("{}_{}", enum_name.to_mangled_ident_default(), variant),
                    Name::MangledType(ty) => ty.to_mangled_ident_default().to_string(),
                    Name::Path(path) => path.to_mangled_ident_default().to_string(),
                    Name::Type(ty) => ty.to_mangled_ident_default().to_string(),
                    Name::Just(ts) => ts.to_string()
                }
            }
        }
    }
}