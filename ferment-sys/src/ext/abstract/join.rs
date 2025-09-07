use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_quote, ImplItemFn, Item, Path, PathSegment, Signature, TraitItemFn, Type, TypePath};
use crate::ast::Colon2Punctuated;
use crate::composable::TypeModel;
use crate::context::{Scope, ScopeChain, ScopeContext, ScopeInfo};
use crate::kind::{ObjectKind, ScopeItemKind};
use crate::ext::{GenericBoundKey, MaybeAttrs, ToType};

pub trait Join<T: ToTokens> {
    fn joined(&self, other: &T) -> Self;
}
#[macro_export]
macro_rules! impl_parseable_join {
    ($SelfTy:ty, $JoinTy:ty) => {
        impl Join<$JoinTy> for $SelfTy {
            fn joined(&self, other: &$JoinTy) -> Self {
                parse_quote!(#self::#other)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_parseable_reverse_join {
    ($SelfTy:ty, $JoinTy:ty) => {
        impl Join<$JoinTy> for $SelfTy {
            fn joined(&self, other: &$JoinTy) -> Self {
                parse_quote!(#other::#self)
            }
        }
    };
}

impl Join<Item> for ScopeChain {
    fn joined(&self, item: &Item) -> Self {
        let attrs = item.maybe_attrs().cloned().unwrap_or_default();
        let self_scope = self.self_scope_ref().joined(item);
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
        let self_path = self.self_path_ref();
        let fn_self_scope = self_path.joined(ident);
        let self_type = fn_self_scope.to_type();
        ScopeChain::func(
            Scope::new(fn_self_scope, ObjectKind::new_fn_item(TypeModel::new_generic_non_nested(self_type, generics), ScopeItemKind::fn_ref(sig, self_path))),
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
        let self_path = self.self_path_ref();
        let fn_self_scope = self_path.joined(ident);
        let self_type = fn_self_scope.to_type();
        ScopeChain::func(
            Scope::new(fn_self_scope, ObjectKind::new_fn_item(TypeModel::new_generic_non_nested(self_type, generics), ScopeItemKind::fn_ref(sig, self_path))),
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
        let mut new_path = self.clone();
        new_path.segments.push(PathSegment::from(other.clone()));
        new_path
    }
}
impl_parseable_join!(Path, Path);
impl_parseable_join!(Path, Type);
impl_parseable_join!(Type, Path);
impl_parseable_join!(Type, Colon2Punctuated<PathSegment>);
impl_parseable_join!(Path, Colon2Punctuated<PathSegment>);
impl_parseable_join!(Colon2Punctuated<PathSegment>, Colon2Punctuated<PathSegment>);
impl_parseable_join!(Colon2Punctuated<PathSegment>, Ident);
impl_parseable_join!(Colon2Punctuated<PathSegment>, Path);
impl_parseable_join!(TypePath, Colon2Punctuated<PathSegment>);
impl_parseable_join!(TypePath, Ident);


impl Join<GenericBoundKey> for Path {
    fn joined(&self, other: &GenericBoundKey) -> Self {
        match other {
            GenericBoundKey::Ident(ident) => self.joined(ident),
            GenericBoundKey::Path(path) => self.joined(path)
        }
    }
}
