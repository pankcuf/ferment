// use std::fmt::Formatter;
// use syn::{Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, Path, Signature, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject};
// use quote::ToTokens;
// use syn::__private::TokenStream2;
// use crate::ast::Depunctuated;
// use crate::composer::{ItemComposerWrapper, ParentComposer};
// use crate::context::{ScopeChain, ScopeContext};
// use crate::conversion::Ferment;
// use crate::presentation::Fermentate;
// use crate::tree::ScopeTreeExportID;
//
//
// #[derive(Clone)]
// #[allow(unused)]
// pub enum ItemConversion {
//     Mod(ItemMod, ScopeChain),
//     Struct(ItemStruct, ScopeChain),
//     Enum(ItemEnum, ScopeChain),
//     Type(ItemType, ScopeChain),
//     Fn(ItemFn, ScopeChain),
//     Trait(ItemTrait, ScopeChain),
//     Impl(ItemImpl, ScopeChain)
// }
//
// impl std::fmt::Debug for ItemConversion {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}: {}", self.name(), self.ident()))
//     }
// }
//
// impl std::fmt::Display for ItemConversion {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         std::fmt::Debug::fmt(self, f)
//     }
// }
//
// impl ToTokens for ItemConversion {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             Self::Mod(item, ..) => item.to_tokens(tokens),
//             Self::Struct(item, ..) => item.to_tokens(tokens),
//             Self::Enum(item, ..) => item.to_tokens(tokens),
//             Self::Type(item, ..) => item.to_tokens(tokens),
//             Self::Fn(item, ..) => item.to_tokens(tokens),
//             Self::Trait(item, ..) => item.to_tokens(tokens),
//             Self::Impl(item, ..) => item.to_tokens(tokens),
//         }
//     }
// }
//
// impl<'a> TryFrom<(&'a Item, &'a ScopeChain)> for ItemConversion {
//     type Error = String;
//     fn try_from(value: (&'a Item, &'a ScopeChain)) -> Result<Self, Self::Error> {
//         match value.0 {
//             Item::Mod(item) => Ok(Self::Mod(item.clone(), value.1.clone())),
//             Item::Struct(item) => Ok(Self::Struct(item.clone(), value.1.clone())),
//             Item::Enum(item) => Ok(Self::Enum(item.clone(), value.1.clone())),
//             Item::Type(item) => Ok(Self::Type(item.clone(), value.1.clone())),
//             Item::Fn(item) => Ok(Self::Fn(item.clone(), value.1.clone())),
//             Item::Trait(item) => Ok(Self::Trait(item.clone(), value.1.clone())),
//             Item::Impl(item) => Ok(Self::Impl(item.clone(), value.1.clone())),
//             item => Err(format!("Error: {}", item.to_token_stream()))
//         }
//     }
// }
//
// fn path_ident_ref(path: &Path) -> Option<&Ident> {
//     path.segments.last().map(|last_segment| &last_segment.ident)
// }
// pub fn type_ident_ref(ty: &Type) -> Option<&Ident> {
//     match ty {
//         Type::Path(TypePath { path, .. }) =>
//             path_ident_ref(path),
//         Type::Reference(TypeReference { elem, .. }) |
//         Type::Ptr(TypePtr { elem, .. }) =>
//             type_ident_ref(elem),
//         Type::TraitObject(TypeTraitObject { bounds, .. }) => {
//             bounds.iter().find_map(|b| match b {
//                 TypeParamBound::Trait(TraitBound { path, ..}) => path_ident_ref(path),
//                 _ => None
//             })
//         },
//         Type::Array(TypeArray { elem, .. }) => type_ident_ref(elem),
//         _ => None
//     }
// }
//
// impl ItemConversion {
//     pub const fn name(&self) -> &str {
//         match self {
//             Self::Mod(..) => "mod",
//             Self::Struct(..) => "struct",
//             Self::Enum(..) => "enum",
//             Self::Type(..) => "type",
//             Self::Fn(..) => "fn",
//             Self::Trait(..) => "trait",
//             Self::Impl(..) => "impl",
//         }
//     }
//
//     pub fn ident(&self) -> ScopeTreeExportID {
//         match self {
//             ItemConversion::Mod(ItemMod { ident, .. }, ..) |
//             ItemConversion::Struct(ItemStruct { ident, .. }, ..) |
//             ItemConversion::Enum(ItemEnum { ident, .. }, ..) |
//             ItemConversion::Type(ItemType { ident, .. }, ..) |
//             ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
//             ItemConversion::Trait(ItemTrait { ident, .. }, ..) =>
//                 ScopeTreeExportID::Ident(ident.clone()),
//             ItemConversion::Impl(ItemImpl { self_ty, trait_, .. }, ..) =>
//                 ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path)),
//         }
//     }
// }
//
// impl Ferment for ItemConversion {
//     fn ferment-sys(&self, scope_context: &ParentComposer<ScopeContext>) -> Depunctuated<Fermentate> {
//         match self {
//             ItemConversion::Struct(item, scope) =>
//                 ItemComposerWrapper::r#struct(item, scope, scope_context)
//                     .ferment-sys(),
//             ItemConversion::Enum(item, scope) =>
//                 ItemComposerWrapper::r#enum(item, scope, scope_context)
//                     .ferment-sys(),
//             ItemConversion::Type(item, scope) =>
//                 ItemComposerWrapper::r#type(item, scope, scope_context)
//                     .ferment-sys(),
//             ItemConversion::Fn(item, scope) =>
//                 ItemComposerWrapper::r#fn(item, scope, scope_context)
//                     .ferment-sys(),
//             ItemConversion::Trait(item, scope) =>
//                 ItemComposerWrapper::r#trait(item, scope, scope_context)
//                     .ferment-sys(),
//             ItemConversion::Impl(item, scope) =>
//                 ItemComposerWrapper::r#impl(item, scope, scope_context)
//                     .ferment-sys(),
//             _ => Depunctuated::new()
//         }
//     }
// }
//
