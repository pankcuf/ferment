use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_quote, ImplItemFn, Item, Path, PathSegment, Signature, TraitItemFn};
use crate::composable::TypeModel;
use crate::context::{Scope, ScopeChain, ScopeContext, ScopeInfo};
use crate::kind::{ObjectKind, ScopeItemKind};
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
                ScopeChain::object(ScopeInfo::new(attrs, self.crate_ident(), self_scope), self.clone()),
            Item::Trait(..) =>
                ScopeChain::r#trait(ScopeInfo::new(attrs, self.crate_ident(), self_scope), self.clone()),
            Item::Fn(..) =>
                ScopeChain::r#fn(ScopeInfo::new(attrs, self.crate_ident(), self_scope), self.clone()),
            Item::Impl(..) =>
                ScopeChain::r#impl(ScopeInfo::new(attrs, self.crate_ident(), self_scope), self.clone()),
            Item::Mod(..) =>
                ScopeChain::r#mod(ScopeInfo::new(attrs, self.crate_ident(), self_scope), self.clone()),
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
        ScopeChain::func(
            Scope::new(
                fn_self_scope,
                ObjectKind::new_fn_item(
                    TypeModel::new_non_nested(self_type, Some(generics.clone())),
                    ScopeItemKind::fn_ref(sig, self_scope_holder))),
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
        ScopeChain::func(
            Scope::new(
                fn_self_scope,
                ObjectKind::new_fn_item(
                    TypeModel::new_generic_non_nested(self_type, generics.clone()),
                    ScopeItemKind::fn_ref(sig, self_scope_holder))),
            attrs,
            self.crate_ident_ref(),
            self
        )
    }
}

impl Join<ImplItemFn> for ScopeContext {
    fn joined(&self, other: &ImplItemFn) -> Self {
        Self::with(self.scope.joined(other), self.context.clone())
    }
}

impl Join<TraitItemFn> for ScopeContext {
    fn joined(&self, other: &TraitItemFn) -> Self {
        Self::with(self.scope.joined(other), self.context.clone())
    }
}

impl Join<Ident> for Path {
    fn joined(&self, other: &Ident) -> Self {
        let mut segments = self.segments.clone();
        segments.push(PathSegment::from(other.clone()));
        Path { leading_colon: self.leading_colon, segments }
    }
}
impl Join<Path> for Path {
    fn joined(&self, other: &Path) -> Self {
        parse_quote!(#self::#other)
    }
}