use quote::ToTokens;
use syn::Item;
use crate::context::{ScopeChain, ScopeInfo};
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
            Item::Struct(..) => ScopeChain::Object { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Trait(..) => ScopeChain::Trait { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Fn(..) => ScopeChain::Fn { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Impl(..) => ScopeChain::Impl { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into(), },
            Item::Mod(..) => ScopeChain::Mod { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            _ => self.clone()
        }
    }
}
