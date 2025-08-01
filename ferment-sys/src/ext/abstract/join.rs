use quote::ToTokens;
use syn::{ImplItemFn, Item, Signature, TraitItemFn};
use syn::punctuated::Punctuated;
use crate::composable::TypeModel;
use crate::context::{Scope, ScopeChain, ScopeInfo};
use crate::kind::{ObjectKind, ScopeItemKind, TypeModelKind};
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
                ScopeChain::Object { info: ScopeInfo { attrs, crate_ident: self.crate_ident(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Trait(..) =>
                ScopeChain::Trait { info: ScopeInfo { attrs, crate_ident: self.crate_ident(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Fn(..) =>
                ScopeChain::Fn { info: ScopeInfo { attrs, crate_ident: self.crate_ident(), self_scope }, parent_scope_chain: self.clone().into() },
            Item::Impl(..) =>
                ScopeChain::Impl { info: ScopeInfo { attrs, crate_ident: self.crate_ident(), self_scope }, parent_scope_chain: self.clone().into(), },
            Item::Mod(..) =>
                ScopeChain::Mod { info: ScopeInfo { attrs, crate_ident: self.crate_ident(), self_scope }, parent_scope_chain: self.clone().into() },
            _ => self.clone()
        }
    }
}
impl Join<ImplItemFn> for ScopeChain {
    fn joined(&self, item: &ImplItemFn) -> Self {
        let ImplItemFn { attrs, sig, .. } = item;
        let Signature { ident, generics, .. } = sig;
        let self_scope = self.self_scope();
        let self_scope_holder = &self_scope.self_scope;
        let fn_self_scope = self_scope_holder.joined(ident);
        let self_type = fn_self_scope.to_type();
        let self_obj = ObjectKind::new_item(
            TypeModelKind::Fn(TypeModel::new_non_gen(self_type, Some(generics.clone()))),
            ScopeItemKind::Fn(sig.clone(), self_scope_holder.clone()));
        ScopeChain::func(
            Scope::new(fn_self_scope, self_obj),
            attrs,
            self.crate_ident_ref(),
            self
        )
    }
}
impl Join<TraitItemFn> for ScopeChain {
    fn joined(&self, item: &TraitItemFn) -> Self {
        let TraitItemFn { attrs, sig, .. } = item;
        let Signature { ident, generics, .. } = sig;
        let self_scope = self.self_scope();
        let self_scope_holder = &self_scope.self_scope;
        let fn_self_scope = self_scope_holder.joined(ident);
        let self_type = fn_self_scope.to_type();
        let self_obj = ObjectKind::new_item(
            TypeModelKind::Fn(
                TypeModel::new(
                    self_type,
                    Some(generics.clone()),
                    Punctuated::new())),
            ScopeItemKind::Fn(
                sig.clone(),
                self_scope_holder.clone()));
        ScopeChain::func(
            Scope::new(fn_self_scope, self_obj),
            attrs,
            self.crate_ident_ref(),
            self
        )
    }
}

