use quote::ToTokens;
use syn::{ImplItemMethod, Item, Signature, TraitItemMethod};
use syn::punctuated::Punctuated;
use crate::composable::TypeComposition;
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::conversion::{ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::ext::item::ItemExtension;
use crate::ext::ToType;

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
            Item::Struct(..) =>
                ScopeChain::Object { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Trait(..) =>
                ScopeChain::Trait { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Fn(..) =>
                ScopeChain::Fn { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Impl(..) =>
                ScopeChain::Impl { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into(), },
            Item::Mod(..) =>
                ScopeChain::Mod { info: ScopeInfo { attrs, crate_ident: self.crate_ident().clone(), self_scope }, parent_scope_chain: self.clone().into() },
            _ => self.clone()
        }
    }
}
impl Join<ImplItemMethod> for ScopeChain {
    fn joined(&self, item: &ImplItemMethod) -> Self {
        let ImplItemMethod { attrs, sig, .. } = item;
        let Signature { ident, generics, .. } = sig;
        let self_scope = self.self_scope();
        let self_scope_holder = &self_scope.self_scope;
        let fn_self_scope = self_scope_holder.joined(ident);
        let self_type = fn_self_scope.to_type();
        let self_obj = ObjectConversion::new_item(
            TypeCompositionConversion::Fn(
                TypeComposition::new(
                    self_type,
                    Some(generics.clone()),
                    Punctuated::new())),
            ScopeItemConversion::Fn(
                sig.clone(),
                self_scope_holder.clone()));
        ScopeChain::func(
            Scope::new(fn_self_scope, self_obj),
            attrs,
            self.crate_ident(),
            self
        )
    }
}
impl Join<TraitItemMethod> for ScopeChain {
    fn joined(&self, item: &TraitItemMethod) -> Self {
        let TraitItemMethod { attrs, sig, .. } = item;
        let Signature { ident, generics, .. } = sig;
        let self_scope = self.self_scope();
        let self_scope_holder = &self_scope.self_scope;
        let fn_self_scope = self_scope_holder.joined(ident);
        let self_type = fn_self_scope.to_type();
        let self_obj = ObjectConversion::new_item(
            TypeCompositionConversion::Fn(
                TypeComposition::new(
                    self_type,
                    Some(generics.clone()),
                    Punctuated::new())),
            ScopeItemConversion::Fn(
                sig.clone(),
                self_scope_holder.clone()));
        ScopeChain::func(
            Scope::new(fn_self_scope, self_obj),
            attrs,
            self.crate_ident(),
            self
        )
    }
}
