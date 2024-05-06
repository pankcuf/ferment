use std::fmt::{Debug, Formatter};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::Item;
use crate::composer::ParentComposer;
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::{ItemConversion, MacroType};
use crate::helper::ItemExtension;
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
impl Debug for ScopeTreeItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeTreeItem::Item { item, scope, scope_context} =>
                f.write_str(format!("Item({})", item.ident_string()).as_str()),
            ScopeTreeItem::Tree { tree } =>
                f.write_str(format!("Tree({:?})", tree).as_str()),
        }
    }
}


// impl ScopeTreeItem {
//     pub fn scope(&self) -> &ScopeChain {
//         match self {
//             ScopeTreeItem::Item { scope, .. } => scope,
//             ScopeTreeItem::Tree { tree } => &tree.scope,
//         }
//     }
// }

impl ToTokens for ScopeTreeItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Item { item, scope, scope_context } => match MacroType::try_from(item) {
                Ok(MacroType::Export) => ItemConversion::try_from((item, scope))
                    .map(|conversion| conversion.make_expansion(scope_context).into_token_stream())
                    .unwrap_or_default(),
                _ => quote! {}
            }
            Self::Tree { tree} =>
                tree.to_token_stream()
        }.to_tokens(tokens)
    }
}

// impl ScopeTreeItem {
//     pub fn generic_conversions(&self) -> HashSet<GenericConversion> {
//         match self {
//             Self::Item { .. } => HashSet::from([]),
//             Self::Tree { tree, .. } => tree.generic_conversions()
//         }
//     }
// }
