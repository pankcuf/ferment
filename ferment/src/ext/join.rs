use quote::ToTokens;
use syn::Item;
use crate::context::ScopeChain;

pub trait Join<T: ToTokens> {
    fn joined(&self, other: &T) -> Self;
}

impl Join<Item> for ScopeChain {
    fn joined(&self, item: &Item) -> Self {
        match item {
            Item::Const(..) |
            Item::Type(..) |
            Item::Enum(..) |
            Item::Struct(..) =>
                joined_obj(self, item),
            Item::Trait(..) =>
                joined_trait(self, item),
            Item::Fn(..) =>
                joined_fn(self, item),
            Item::Impl(..) =>
                joined_impl(self, item),
            Item::Mod(..) =>
                joined_mod(self, item),
            _ => self.clone()
        }
    }
}

fn joined_obj(scope_chain: &ScopeChain, item: &Item) -> ScopeChain {
    let self_scope = scope_chain.self_scope().joined(item);
    ScopeChain::Object { self_scope, parent_scope_chain: Box::new(scope_chain.clone()) }
}

fn joined_fn(scope_chain: &ScopeChain, item: &Item) -> ScopeChain {
    let self_scope = scope_chain.self_scope().joined(item);
    ScopeChain::Fn { self_scope, parent_scope_chain: Box::new(scope_chain.clone()) }
}

fn joined_trait(scope_chain: &ScopeChain, item: &Item) -> ScopeChain {
    let self_scope = scope_chain.self_scope().joined(item);
    ScopeChain::Trait { self_scope, parent_scope_chain: Box::new(scope_chain.clone()) }
}

fn joined_mod(scope_chain: &ScopeChain, item: &Item) -> ScopeChain {
    let self_scope = scope_chain.self_scope().joined(item);
    ScopeChain::Mod { self_scope }
}

fn joined_impl(scope_chain: &ScopeChain, item: &Item) -> ScopeChain {
    let self_scope = scope_chain.self_scope().joined(item);
    ScopeChain::Impl {
        self_scope,
        trait_scopes: vec![],
        parent_scope_chain: Box::new(scope_chain.clone()),
    }
}

