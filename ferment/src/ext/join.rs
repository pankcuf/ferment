use quote::ToTokens;
use syn::Item;
use crate::context::ScopeChain;

pub trait Join<T: ToTokens> {
    fn joined(&self, other: &T) -> Self;
}

impl Join<Item> for ScopeChain {
    fn joined(&self, item: &Item) -> Self {
        let self_scope = self.self_scope().joined(item);
        match item {
            Item::Const(..) |
            Item::Type(..) |
            Item::Enum(..) |
            Item::Struct(..) => ScopeChain::Object { crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Trait(..) => ScopeChain::Trait { crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Fn(..) => ScopeChain::Fn { crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            Item::Impl(..) => ScopeChain::Impl { crate_ident: self.crate_ident().clone(), self_scope, trait_scopes: vec![], parent_scope_chain: self.clone().into(), },
            Item::Mod(..) => ScopeChain::Mod { crate_ident: self.crate_ident().clone(), self_scope, parent_scope_chain: self.clone().into() },
            _ => self.clone()
        }
    }
}
