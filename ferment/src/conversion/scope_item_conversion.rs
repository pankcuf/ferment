use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Item, ItemTrait, Path, Signature, Type};
use syn::__private::TokenStream2;
use crate::ast::PathHolder;
use crate::composable::{CfgAttributes, TraitDecompositionPart1, TypeComposition};
use crate::conversion::{TypeCompositionConversion};
use crate::ext::{collect_bounds, ItemExtension, ResolveAttrs, ToPath, ToType};
use crate::formatter::format_token_stream;
use crate::tree::ScopeTreeExportID;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScopeItemConversion {
    Item(Item, PathHolder),
    Fn(Signature, PathHolder),
}

impl ToTokens for ScopeItemConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemConversion::Item(item, ..) => item.to_tokens(tokens),
            ScopeItemConversion::Fn(sig, ..) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemConversion::Item(item, scope) =>
                f.write_str(format!("Item({}, {scope})", format_token_stream(item.maybe_ident())).as_str()),
            ScopeItemConversion::Fn(sig, scope) =>
                f.write_str(format!("Fn({}, {scope})", format_token_stream(&sig.ident)).as_str()),
        }
    }
}

impl Display for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ResolveAttrs for ScopeItemConversion {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ScopeItemConversion::Item(item, ..) =>
                item.maybe_attrs()
                    .map(|attrs| attrs.cfg_attributes_or_none())
                    .unwrap_or_default(),
            ScopeItemConversion::Fn(sig, ..) =>
                sig.maybe_attrs()
                    .map(|attrs| attrs.cfg_attributes_or_none())
                    .unwrap_or_default()
        }
    }
}

impl ItemExtension for ScopeItemConversion {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        match self {
            ScopeItemConversion::Item(item, ..) => item.scope_tree_export_id(),
            ScopeItemConversion::Fn(sig, ..) => sig.scope_tree_export_id()
        }
    }

    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemConversion::Item(item, ..) => item.maybe_attrs(),
            ScopeItemConversion::Fn(sig, ..) => sig.maybe_attrs()
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemConversion::Item(item, ..) => item.maybe_ident(),
            ScopeItemConversion::Fn(sig, ..) => sig.maybe_ident()
        }
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemConversion::Item(item, ..) => item.maybe_generics(),
            ScopeItemConversion::Fn(sig, ..) => sig.maybe_generics()
        }
    }
}
impl ScopeItemConversion {
    pub fn update_scope_item(&self, ty_to_replace: TypeComposition) -> Option<TypeCompositionConversion> {
        println!("update_scope_item: {} --- {}", self, ty_to_replace);
        match self {
            ScopeItemConversion::Item(item, ..) => match item {
                Item::Trait(ItemTrait { ident, items, supertraits, .. }) =>
                    Some(TypeCompositionConversion::Trait(
                        ty_to_replace.clone(),
                        TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits))),
                Item::Enum(..) |
                Item::Struct(..) |
                Item::Fn(..) |
                Item::Impl(..) => {
                    Some(TypeCompositionConversion::Object(ty_to_replace))
                },
                Item::Type(ty) => match &*ty.ty {
                    Type::BareFn(..) => Some(TypeCompositionConversion::FnPointer(ty_to_replace)),
                    _ => Some(TypeCompositionConversion::Object(ty_to_replace)),
                },
                _ => None
            }
            ScopeItemConversion::Fn(..) => None
        }
    }
    pub fn scope(&self) -> &PathHolder {
        match self {
            ScopeItemConversion::Item(.., scope) |
            ScopeItemConversion::Fn(.., scope) => scope
        }
    }
    pub fn path(&self) -> &Path {
        &self.scope().0
    }
}

impl ToType for ScopeItemConversion {
    fn to_type(&self) -> Type {
        self.scope().to_type()
    }
}

impl ToPath for ScopeItemConversion {
    fn to_path(&self) -> Path {
        self.scope().0.clone()
    }
}