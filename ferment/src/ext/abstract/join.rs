use quote::ToTokens;
use syn::Item;
use crate::context::ScopeChain;
use crate::helper::ItemExtension;

pub trait Join<T: ToTokens> {
    fn joined(&self, other: &T) -> Self;
}

impl Join<Item> for ScopeChain {
    fn joined(&self, item: &Item) -> Self {
        let attrs = item.maybe_attrs().cloned().unwrap_or_default();
        let self_scope = self.self_scope().joined(item);
        match item {
            Item::Const(..) |
            Item::Type(..) |
            Item::Enum(..) |
            Item::Struct(..) => ScopeChain::Object { attrs, crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Trait(..) => ScopeChain::Trait { attrs, crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Fn(..) => ScopeChain::Fn { attrs, crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Impl(..) => ScopeChain::Impl { attrs, crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into(), },
            Item::Mod(..) => ScopeChain::Mod { attrs, crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            _ => self.clone()
        }
    }
}
