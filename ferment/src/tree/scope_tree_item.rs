use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use syn::Item;
use std::collections::HashSet;
use crate::composer::ParentComposer;
use crate::composition::GenericConversion;
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::ItemConversion;
use crate::tree::ScopeTree;

#[derive(Clone)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: ScopeChain,
        scope_context: ParentComposer<ScopeContext>,
    },
    Tree {
        tree: ScopeTree
    }
}

impl ScopeTreeItem {
    pub fn scope(&self) -> &ScopeChain {
        match self {
            ScopeTreeItem::Item { scope, .. } => scope,
            ScopeTreeItem::Tree { tree } => &tree.scope,
        }
    }
}

impl ToTokens for ScopeTreeItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Item { item, scope, scope_context } =>
                ItemConversion::try_from((item, scope))
                    .map(|conversion| conversion.make_expansion(scope_context).into_token_stream())
                    .unwrap_or_default(),
            Self::Tree { tree} =>
                tree.to_token_stream()
        }.to_tokens(tokens)
    }
}

impl ScopeTreeItem {
    pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
        match self {
            Self::Item { .. } => HashSet::from([]),
            Self::Tree { tree, .. } => tree.generic_conversions()
        }
    }
}
