use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Item, ItemTrait, Path, Signature, Type};
use syn::__private::TokenStream2;
use crate::ast::PathHolder;
use crate::composable::{CfgAttributes, TraitDecompositionPart1, TraitModel, TypeModel};
use crate::conversion::{TypeModelKind};
use crate::ext::{collect_bounds, ItemExtension, ResolveAttrs, ToPath, ToType};
use crate::formatter::format_token_stream;
use crate::tree::ScopeTreeExportID;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScopeItemKind {
    Item(Item, PathHolder),
    Fn(Signature, PathHolder),
}

impl ToTokens for ScopeItemKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemKind::Item(item, ..) => item.to_tokens(tokens),
            ScopeItemKind::Fn(sig, ..) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemKind::Item(item, scope) =>
                f.write_str(format!("Item({}, {scope})", format_token_stream(item.maybe_ident())).as_str()),
            ScopeItemKind::Fn(sig, scope) =>
                f.write_str(format!("Fn({}, {scope})", format_token_stream(&sig.ident)).as_str()),
        }
    }
}

impl Display for ScopeItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ResolveAttrs for ScopeItemKind {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ScopeItemKind::Item(item, ..) =>
                item.maybe_attrs()
                    .map(|attrs| attrs.cfg_attributes_or_none())
                    .unwrap_or_default(),
            ScopeItemKind::Fn(sig, ..) =>
                sig.maybe_attrs()
                    .map(|attrs| attrs.cfg_attributes_or_none())
                    .unwrap_or_default()
        }
    }
}

impl ItemExtension for ScopeItemKind {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        match self {
            ScopeItemKind::Item(item, ..) => item.scope_tree_export_id(),
            ScopeItemKind::Fn(sig, ..) => sig.scope_tree_export_id()
        }
    }

    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_attrs(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_attrs()
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_ident(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_ident()
        }
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemKind::Item(item, ..) => item.maybe_generics(),
            ScopeItemKind::Fn(sig, ..) => sig.maybe_generics()
        }
    }
}
impl ScopeItemKind {
    pub fn update_with(&self, ty_to_replace: TypeModel) -> Option<TypeModelKind> {
        //println!("ScopeItemKind::update_with: {} --- {}", self, ty_to_replace);
        match self {
            ScopeItemKind::Item(item, ..) => match item {
                Item::Trait(ItemTrait { ident, items, supertraits, .. }) =>
                    Some(TypeModelKind::Trait(TraitModel::new(ty_to_replace.clone(), TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits)))),
                Item::Enum(..) |
                Item::Struct(..) |
                Item::Fn(..) |
                Item::Impl(..) => {
                    Some(TypeModelKind::Object(ty_to_replace))
                },
                Item::Type(ty) => match &*ty.ty {
                    Type::BareFn(..) => Some(TypeModelKind::FnPointer(ty_to_replace)),
                    _ => Some(TypeModelKind::Object(ty_to_replace)),
                },
                _ => None
            }
            ScopeItemKind::Fn(..) => None
        }
    }
    pub fn scope(&self) -> &PathHolder {
        match self {
            ScopeItemKind::Item(.., scope) |
            ScopeItemKind::Fn(.., scope) => scope
        }
    }
    pub fn path(&self) -> &Path {
        &self.scope().0
    }
}

impl ToType for ScopeItemKind {
    fn to_type(&self) -> Type {
        self.scope().to_type()
    }
}

impl ToPath for ScopeItemKind {
    fn to_path(&self) -> Path {
        self.scope().0.clone()
    }
}