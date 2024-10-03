// use std::fmt::Formatter;
// use quote::ToTokens;
// use syn::{Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, Signature};
// use syn::__private::TokenStream2;
// use crate::ast::Depunctuated;
// use crate::composer::{ItemComposerWrapper, ParentComposer};
// use crate::context::{ScopeChain, ScopeContext};
// use crate::conversion::Ferment;
// use crate::presentation::Fermentate;
// use crate::tree::ScopeTreeExportID;
//
// #[derive(Clone)]
// #[allow(unused)]
// pub enum OpaqueConversion {
//     Mod(ItemMod, ScopeChain),
//     Struct(ItemStruct, ScopeChain),
//     Enum(ItemEnum, ScopeChain),
//     Type(ItemType, ScopeChain),
//     Fn(ItemFn, ScopeChain),
//     Trait(ItemTrait, ScopeChain),
//     Impl(ItemImpl, ScopeChain)
// }
// impl std::fmt::Debug for OpaqueConversion {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}: {}", self.name(), self.ident()))
//     }
// }
//
// impl std::fmt::Display for OpaqueConversion {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         std::fmt::Debug::fmt(self, f)
//     }
// }
// impl ToTokens for OpaqueConversion {
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
// impl<'a> TryFrom<(&'a Item, &'a ScopeChain)> for OpaqueConversion {
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
// impl Ferment for OpaqueConversion {
//     fn ferment-sys(&self, scope_context: &ParentComposer<ScopeContext>) -> Depunctuated<Fermentate> {
//         match self {
//             Self::Struct(item, scope) =>
//                 ItemComposerWrapper::opaque_struct(item, scope, scope_context).ferment-sys(),
//             Self::Impl(..) |
//             Self::Mod(..) |
//             Self::Enum(..) |
//             Self::Type(..) |
//             Self::Fn(..) |
//             Self::Trait(..) =>
//                 Depunctuated::new()
//         }
//     }
// }
//
// impl OpaqueConversion {
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
//     pub fn ident(&self) -> ScopeTreeExportID {
//         match self {
//             Self::Mod(ItemMod { ident, .. }, ..) |
//             Self::Struct(ItemStruct { ident, .. }, ..) |
//             Self::Enum(ItemEnum { ident, .. }, ..) |
//             Self::Type(ItemType { ident, .. }, ..) |
//             Self::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
//             Self::Trait(ItemTrait { ident, .. }, ..) =>
//                 ScopeTreeExportID::Ident(ident.clone()),
//             Self::Impl(ItemImpl { self_ty, trait_, .. }, ..) =>
//                 ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path)),
//         }
//     }
// }
//
