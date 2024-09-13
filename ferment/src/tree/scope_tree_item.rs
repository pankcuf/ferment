use std::fmt::{Debug, Formatter};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, Generics, Item};
use crate::composer::{MaybeComposer, ComposerLink};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ItemExtension;
use crate::presentation::RustFermentate;
use crate::tree::ScopeTree;

#[derive(Clone)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: ScopeChain,
        scope_context: ComposerLink<ScopeContext>,
    },
    Tree {
        tree: ScopeTree
    }
}
impl Debug for ScopeTreeItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ScopeTreeItem::Item { item, scope, scope_context: _} =>
                format!("Item({}, {})", item.ident_string(), scope.self_path_holder_ref()),
            ScopeTreeItem::Tree { tree } =>
                format!("Tree({:?})", tree),
        }.as_str())
    }
}

impl ToTokens for ScopeTreeItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Item { item, scope, scope_context } =>
                if let Some(composer) = <Item as MaybeComposer<RustFermentate, Vec<Attribute>, Option<Generics>>>::maybe_composer(item, scope, scope_context) {

                    composer.ferment().to_tokens(tokens)
                    // composer.to_tokens(tokens)
            },
            Self::Tree { tree} =>
                tree.to_tokens(tokens),
        }
    }
}
