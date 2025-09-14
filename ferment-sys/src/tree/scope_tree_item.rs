use std::fmt::{Debug, Formatter};
use syn::Item;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::MaybeIdent;
use crate::formatter::format_token_stream;
use crate::tree::ScopeTree;

#[derive(Clone)]
pub enum ScopeTreeItem {
    Item {
        item: Item,
        scope: ScopeChain,
        scope_context: ScopeContextLink,
    },
    Tree {
        tree: ScopeTree
    }
}

impl ScopeTreeItem {
    pub fn item(scope: ScopeChain, item: Item, scope_context: ScopeContextLink) -> Self {
        Self::Item { item, scope, scope_context }
    }
    pub fn tree(tree: ScopeTree) -> Self {
        Self::Tree { tree }
    }
}
impl Debug for ScopeTreeItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item { item, scope, scope_context: _} =>
                f.write_fmt(format_args!("Item({}, {})", item.ident_string(), format_token_stream(scope.self_path_ref()))),
            Self::Tree { tree } =>
                f.write_fmt(format_args!("Tree({tree:?})")),
        }
    }
}
