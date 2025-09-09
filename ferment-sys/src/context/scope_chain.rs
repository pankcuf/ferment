use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Generics, parse_quote, Path, Type, PathSegment};
use crate::composable::CfgAttributes;
use crate::context::{GenericChain, Scope, ScopeInfo};
use crate::kind::{ObjectKind, TypeModel};
use crate::ext::{CRATE, ResolveAttrs, ToPath, ToType, Join, GenericBoundKey, PathTransform, CrateBased};
use crate::formatter::{format_attrs, format_token_stream};


#[derive(Clone, Eq)]
#[repr(u8)]
pub enum ScopeChain {
    CrateRoot {
        info: ScopeInfo,
    },
    Mod {
        info: ScopeInfo,
        parent: Box<ScopeChain>,
    },
    Trait {
        info: ScopeInfo,
        parent: Box<ScopeChain>,
    },
    Fn {
        info: ScopeInfo,
        parent: Box<ScopeChain>,
    },
    Object {
        info: ScopeInfo,
        parent: Box<ScopeChain>,
    },
    Impl {
        info: ScopeInfo,
        parent: Box<ScopeChain>,
    },
}

impl ScopeChain {
    pub fn root(info: ScopeInfo) -> Self {
        Self::CrateRoot { info }
    }
    pub fn root_with(attrs: Vec<Attribute>, crate_ident: Ident, self_scope: Scope) -> Self {
        Self::CrateRoot { info: ScopeInfo::new(attrs, crate_ident, self_scope) }
    }
    pub fn object(info: ScopeInfo, parent: ScopeChain) -> Self {
        Self::Object { info, parent: parent.into() }
    }
    pub fn r#trait(info: ScopeInfo, parent: ScopeChain) -> Self {
        Self::Trait { info, parent: parent.into() }
    }
    pub fn r#impl(info: ScopeInfo, parent: ScopeChain) -> Self {
        Self::Impl { info, parent: parent.into() }
    }
    pub fn r#mod(info: ScopeInfo, parent: ScopeChain) -> Self {
        Self::Mod { info, parent: parent.into() }
    }
    pub fn r#fn(info: ScopeInfo, parent: ScopeChain) -> Self {
        Self::Fn { info, parent: parent.into() }
    }
    pub fn func(self_scope: Scope, attrs: &[Attribute], crate_ident: &Ident, parent: &ScopeChain) -> Self {
        Self::r#fn(ScopeInfo::new(attrs.to_owned(), crate_ident.clone(), self_scope), parent.clone())
    }

    pub fn obj_scope_priority(&self) -> u8 {
        match self {
            ScopeChain::CrateRoot { .. } => 0,
            ScopeChain::Mod { .. } => 1,
            ScopeChain::Trait { .. } => 4,
            ScopeChain::Fn { .. } => 3,
            ScopeChain::Object { .. } => 5,
            ScopeChain::Impl { .. } => 2,
        }
    }
}

impl PartialEq<Self> for ScopeChain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScopeChain::Impl { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Impl { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::CrateRoot { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::CrateRoot { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::Mod { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Mod { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::Trait { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Trait { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::Fn { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Fn { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) |
            (ScopeChain::Object { info: ScopeInfo { crate_ident, self_scope, .. }, .. },
                ScopeChain::Object { info: ScopeInfo { crate_ident: other_crate_ident, self_scope: other_self_scope, .. }, .. }) =>
                self_scope.eq(other_self_scope) && crate_ident.eq(other_crate_ident),
            _ => false
        }
    }
}
impl Hash for ScopeChain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.variant_code().hash(state);
        self.self_scope_ref().hash(state);
    }
}

impl Debug for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.fmt_long().as_str())
    }
}
impl Display for ScopeChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ScopeChain {
    pub fn variant_code(&self) -> u8 {
        match self {
            ScopeChain::CrateRoot { .. } => 0,
            ScopeChain::Mod { .. } => 1,
            ScopeChain::Fn { .. } => 2,
            ScopeChain::Object { .. } => 3,
            ScopeChain::Impl { .. } => 4,
            ScopeChain::Trait { .. } => 5
        }
    }
    pub fn info(&self) -> &ScopeInfo {
        match self {
            ScopeChain::CrateRoot { info, .. } |
            ScopeChain::Mod { info, .. } |
            ScopeChain::Fn { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } |
            ScopeChain::Trait { info, .. } => info
        }
    }

    #[allow(unused)]
    pub fn fmt_short(&self) -> String {
        match self {
            ScopeChain::CrateRoot { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "CrateRoot", info.fmt_export_type()),
            ScopeChain::Mod { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "Mod", info.fmt_export_type()),
            ScopeChain::Fn { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "Fn", info.fmt_export_type()),
            ScopeChain::Object { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "Object", info.fmt_export_type()),
            ScopeChain::Impl { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "Impl", info.fmt_export_type()),
            ScopeChain::Trait { info, .. } => format!("[{}({} + {})]", format_token_stream(info.self_path()), "Trait", info.fmt_export_type()),
        }
    }
    #[allow(unused)]
    pub fn fmt_mid(&self) -> String {
        match self {
            ScopeChain::CrateRoot { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "CrateRoot", info.fmt_export_type(), info.self_scope.object),
            ScopeChain::Mod { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "Mod", info.fmt_export_type(), info.self_scope.object),
            ScopeChain::Fn { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "Fn", info.fmt_export_type(), info.self_scope.object),
            ScopeChain::Object { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "Object", info.fmt_export_type(), info.self_scope.object),
            ScopeChain::Impl { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "Impl", info.fmt_export_type(), info.self_scope.object),
            ScopeChain::Trait { info, .. } => format!("[{}({} + {} + {})]", format_token_stream(info.self_path()), "Trait", info.fmt_export_type(), info.self_scope.object),
        }
    }
    #[allow(unused)]
    pub fn fmt_long(&self) -> String {
        match self {
            ScopeChain::CrateRoot { info } =>
                format!("[{}] :: {} + {} (CrateRoot)", format_attrs(&info.attrs), info.crate_ident, info.self_scope),
            ScopeChain::Mod { info, parent } =>
                format!("[{}] :: {} + {} (Mod) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent),
            ScopeChain::Trait { info, parent } =>
                format!("[{}] :: {} + {} (Trait) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent),
            ScopeChain::Fn { info, parent } =>
                format!("[{}] :: {} + {} (Fn) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent),
            ScopeChain::Object { info, parent } =>
                format!("[{}] :: {} + {} (Object) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent),
            ScopeChain::Impl { info, parent } =>
                format!("[{}] :: {} + {} (Impl) (parent: {:?})", format_attrs(&info.attrs), info.crate_ident, info.self_scope, parent),
        }
    }
}

impl ToTokens for ScopeChain {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.self_path_ref()
            .to_tokens(tokens)
    }
}

impl ToType for ScopeChain {
    fn to_type(&self) -> Type {
        self.self_path_ref().to_type()
    }
}

impl ScopeChain {

    fn crate_root_with(attrs: Vec<Attribute>, crate_ident: Ident, path: Path) -> Self {
        Self::root_with(attrs, crate_ident, Scope::empty(path))
    }
    fn new_mod(crate_ident: Ident, self_scope: Scope, parent_scope: &ScopeChain, attrs: Vec<Attribute>) -> Self {
        Self::r#mod(ScopeInfo::new(attrs, crate_ident, self_scope), parent_scope.clone())
    }

    pub fn crate_root_with_ident(crate_ident: Ident, attrs: Vec<Attribute>) -> Self {
        let path = crate_ident.to_path();
        Self::crate_root_with(attrs, crate_ident, path)
    }
    pub fn crate_root(crate_ident: Ident, attrs: Vec<Attribute>) -> Self {
        Self::crate_root_with(attrs, crate_ident, parse_quote!(crate))
    }

    pub fn child_mod(attrs: Vec<Attribute>, crate_ident: Ident, name: &Ident, parent_scope: &ScopeChain) -> Self {
        Self::new_mod(crate_ident, Scope::empty(parent_scope.self_path_ref().joined(name)), parent_scope, attrs)
    }

    pub fn crate_name(&self) -> TokenStream2 {
        if self.is_crate_root() {
            self.crate_ident_ref().to_token_stream()
        } else {
            self.head().to_token_stream()
        }
    }
    pub fn crate_ident_ref(&self) -> &Ident {
        &self.info().crate_ident
    }
    pub fn crate_ident(&self) -> Ident {
        self.crate_ident_ref().clone()
    }
    pub fn crate_ident_as_path(&self) -> Path {
        self.crate_ident_ref().to_path()
    }
    pub fn self_scope_ref(&self) -> &Scope {
        &self.info().self_scope
    }

    pub fn joined_path(&self, ident: &Ident) -> Path {
        let scope = self.self_path_ref();
        let mut full_fn_path = scope.joined(ident);
        if scope.is_crate_based() {
            full_fn_path.replace_first_with(&self.crate_ident_ref().to_path())
        }
        full_fn_path
    }

    pub fn self_path_ref(&self) -> &Path {
        &self.self_scope_ref().self_scope
    }
    pub fn self_object_ref(&self) -> &ObjectKind {
        &self.self_scope_ref().object
    }
    pub fn self_object(&self) -> ObjectKind {
        self.self_object_ref().clone()
    }
    pub fn parent_scope(&self) -> Option<&ScopeChain> {
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Fn { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } => Some(parent),
        }
    }
    pub fn parent_object(&self) -> Option<&ObjectKind> {
        self.parent_scope()
            .map(|scope| scope.self_object_ref())
    }
    pub fn obj_root_chain(&self) -> Option<&Self> {
        match self {
            ScopeChain::Fn { parent, .. } => parent.obj_root_chain(),
            ScopeChain::Trait { .. } |
            ScopeChain::Object { .. } |
            ScopeChain::Impl { .. } => Some(self),
            _ => None,
        }
    }

    pub fn obj_root_model_composer(&self) -> fn(TypeModel) -> ObjectKind {
        match self.obj_root_chain() {
            Some(ScopeChain::Trait { .. }) =>
                ObjectKind::trait_model_type,
            Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                ObjectKind::object_model_type,
            _ =>
                ObjectKind::unknown_model_type
        }
    }

    pub(crate) fn is_crate_root(&self) -> bool {
        if let ScopeChain::CrateRoot { info, .. } = self {
            if let Some(PathSegment { ident, .. }) = info.self_path().segments.last() {
                return ident.eq(CRATE)
            }
        }
        false
    }

    pub fn head(&self) -> Ident {
        self.self_path_ref().segments.last().expect("Should have last segment here").ident.clone()
    }

    pub fn has_same_parent(&self, other: &ScopeChain) -> bool {
        match self {
            ScopeChain::CrateRoot { info, .. } |
            ScopeChain::Mod { info, .. } => info.crate_ident.eq(other.crate_ident_ref()) && info.self_scope.eq(other.self_scope_ref()),
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Fn { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } => other.eq(parent),
        }
    }

    pub fn maybe_generic_bound_for_path(&self, path: &GenericBoundKey) -> Option<(Generics, GenericChain)> {
        match self {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } => None,
            ScopeChain::Trait { info, .. } |
            ScopeChain::Object { info, .. } |
            ScopeChain::Impl { info, .. } =>
                info.maybe_generic_bound_for_path(path),
            ScopeChain::Fn { info, parent, .. } =>
                info.maybe_generic_bound_for_path(path)
                    .or_else(|| parent.maybe_generic_bound_for_path(path)),
        }
    }
}

impl ResolveAttrs for ScopeChain {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ScopeChain::CrateRoot { info, .. } =>
                info.attrs.cfg_attributes_or_none(),
            ScopeChain::Mod { info, parent, .. } |
            ScopeChain::Trait { info, parent, .. } |
            ScopeChain::Fn { info, parent, .. } |
            ScopeChain::Object { info, parent, .. } |
            ScopeChain::Impl { info, parent, .. } => {
                let mut inherited_attrs = parent.resolve_attrs();
                inherited_attrs.extend(info.attrs.cfg_attributes_or_none());
                inherited_attrs
            }
        }
    }
}

impl ToPath for ScopeChain {
    fn to_path(&self) -> Path {
        self.self_path_ref().clone()
    }
}