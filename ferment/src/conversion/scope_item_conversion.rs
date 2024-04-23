use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Item, ItemTrait, Path, Signature};
use syn::__private::TokenStream2;
use crate::composition::{ImportComposition, TraitDecompositionPart1, TypeComposition};
use crate::conversion::{ImportConversion, TypeCompositionConversion};
use crate::formatter::format_token_stream;
use crate::helper::{collect_bounds, ItemExtension};
use crate::holder::PathHolder;
use crate::tree::ScopeTreeExportID;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScopeItemConversion {
    Item(Item),
    Fn(Signature),
}

impl ToTokens for ScopeItemConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ScopeItemConversion::Item(item) => item.to_tokens(tokens),
            ScopeItemConversion::Fn(sig) => sig.to_tokens(tokens)
        }
    }
}
impl Debug for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeItemConversion::Item(item) =>
                f.write_str(format!("Item({})", format_token_stream(item.maybe_ident())).as_str()),
            ScopeItemConversion::Fn(sig) =>
                f.write_str(format!("Fn({})", format_token_stream(&sig.ident)).as_str()),
        }
    }
}

impl Display for ScopeItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ItemExtension for ScopeItemConversion {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        match self {
            ScopeItemConversion::Item(item) => item.scope_tree_export_id(),
            ScopeItemConversion::Fn(sig) => sig.scope_tree_export_id()
        }
    }

    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_attrs(),
            ScopeItemConversion::Fn(sig) => sig.maybe_attrs()
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_ident(),
            ScopeItemConversion::Fn(sig) => sig.maybe_ident()
        }
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            ScopeItemConversion::Item(item) => item.maybe_generics(),
            ScopeItemConversion::Fn(sig) => sig.maybe_generics()
        }
    }

    fn classify_imports(&self, imports: &HashMap<PathHolder, Path>) -> HashMap<ImportConversion, HashSet<ImportComposition>> {
        match self {
            ScopeItemConversion::Item(item) => item.classify_imports(imports),
            ScopeItemConversion::Fn(sig) => sig.classify_imports(imports)
        }
    }
}

impl ScopeItemConversion {
    pub fn update_scope_item(&self, ty_to_replace: TypeComposition) -> Option<TypeCompositionConversion> {
        match self {
            ScopeItemConversion::Item(item) => match item {
                Item::Trait(ItemTrait { ident, items, supertraits, .. }) =>
                    Some(TypeCompositionConversion::Trait(
                        ty_to_replace.clone(),
                        TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits))),
                Item::Enum(..) |
                Item::Struct(..) |
                Item::Type(..) |
                Item::Fn(..) |
                Item::Impl(..) =>
                    Some(TypeCompositionConversion::Object(ty_to_replace.clone())),
                _ => None
            }
            ScopeItemConversion::Fn(_) => None
        }
    }

}