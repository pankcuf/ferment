use std::fmt::{Debug, Formatter};
use syn::Item;
use crate::composer::ComposerLink;
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ItemExtension;
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

impl ScopeTreeItem {
    pub fn item(scope: ScopeChain, item: Item, scope_context: ComposerLink<ScopeContext>) -> Self {
        Self::Item { item, scope, scope_context }
    }
    pub fn tree(tree: ScopeTree) -> Self {
        Self::Tree { tree }
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
