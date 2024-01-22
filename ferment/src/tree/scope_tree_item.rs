use std::cell::RefCell;
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use syn::Item;
use std::collections::HashSet;
use std::rc::Rc;
use crate::composition::GenericConversion;
use crate::context::ScopeContext;
use crate::conversion::ItemConversion;
use crate::holder::PathHolder;
use crate::tree::ScopeTree;

#[derive(Clone)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: PathHolder,
        scope_context: Rc<RefCell<ScopeContext>>,
    },
    Tree {
        tree: ScopeTree
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
