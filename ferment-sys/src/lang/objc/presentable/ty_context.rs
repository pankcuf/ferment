use std::fmt::{Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, ItemFn, ItemTrait, Path, TypeBareFn};
use crate::composable::{CfgAttributes, FnSignatureContext};
use crate::conversion::MixinKind;
use crate::ext::ToPath;
use crate::presentable::NameTreeContext;

#[derive(Clone, Debug)]
pub enum TypeContext {
    Enum {
        ident: Ident,
        prefix: String,
        attrs: Vec<Attribute>,
    },
    EnumVariant {
        parent: Box<TypeContext>,
        prefix: String,
        ident: Ident,
        variant_ident: Ident,
        attrs: Vec<Attribute>
    },
    Struct {
        ident: Ident,
        prefix: String,
        attrs: Vec<Attribute>,
    },
    Fn {
        parent: Option<Box<TypeContext>>,
        prefix: String,
        path: Path,
        sig_context: FnSignatureContext,
        attrs: Vec<Attribute>,
    },
    Trait {
        path: Path,
        prefix: String,
        attrs: Vec<Attribute>,
    },
    Impl {
        path: Path,
        prefix: String,
        attrs: Vec<Attribute>,
    },
    Mixin {
        mixin_kind: MixinKind,
        prefix: String,
        attrs: Vec<Attribute>,
    }
}
impl TypeContext {
    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        match self {
            TypeContext::Mixin { attrs, .. } |
            TypeContext::Enum { attrs, .. } |
            TypeContext::EnumVariant { attrs, .. } |
            TypeContext::Struct { attrs, .. } |
            TypeContext::Fn { attrs, .. } |
            TypeContext::Trait { attrs, .. } |
            TypeContext::Impl { attrs, .. } => attrs,
        }
    }
    pub(crate) fn prefix(&self) -> &String {
        match self {
            TypeContext::Mixin { prefix, .. } |
            TypeContext::Enum { prefix, .. } |
            TypeContext::EnumVariant { prefix, .. } |
            TypeContext::Struct { prefix, .. } |
            TypeContext::Fn { prefix, .. } |
            TypeContext::Trait { prefix, .. } |
            TypeContext::Impl { prefix, .. } => prefix,
        }
    }

    pub fn r#struct(ident: &Ident, prefix: &str, attrs: Vec<Attribute>) -> Self {
        Self::Struct { ident: ident.clone(), prefix: prefix.to_string(), attrs }
    }
    pub fn r#enum(ident: &Ident, prefix: &str, attrs: Vec<Attribute>) -> Self {
        Self::Enum { ident: ident.clone(), prefix: prefix.to_string(), attrs }
    }

    pub fn r#impl(path: Path, prefix: &str, attrs: Vec<Attribute>) -> Self {
        Self::Impl { path, attrs, prefix: prefix.to_string() }
    }
    pub fn mixin(kind: &MixinKind, prefix: &str, attrs: Vec<Attribute>) -> Self {
        Self::Mixin { mixin_kind: kind.clone(), prefix: prefix.to_string(), attrs }
    }
    pub fn mod_fn(path: Path, prefix: &str, item: &ItemFn) -> Self {
        Self::Fn { parent: None, path, prefix: prefix.to_string(), sig_context: FnSignatureContext::ModFn(item.clone()), attrs: item.attrs.cfg_attributes() }
    }
    pub fn callback(path: Path, ident: &Ident, prefix: &str, item: &TypeBareFn, attrs: &Vec<Attribute>) -> Self {
        Self::Fn { parent: None, path, prefix: prefix.to_string(), sig_context: FnSignatureContext::Bare(ident.clone(), item.clone()), attrs: attrs.cfg_attributes() }
    }
    pub fn r#trait(item: &ItemTrait, prefix: &str) -> Self {
        Self::Trait { path: item.ident.to_path(), prefix: prefix.to_string(), attrs: item.attrs.cfg_attributes() }
    }
}
impl Display for TypeContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeContext::Enum { ident, .. } |
            TypeContext::Struct { ident, .. } =>
                ident.to_string(),
            TypeContext::EnumVariant { ident, variant_ident, .. } =>
                format!("{ident}_{variant_ident}"),
            TypeContext::Fn { path, .. } |
            TypeContext::Trait { path, .. } =>
                path.to_token_stream().to_string(),
            TypeContext::Impl { path, .. } =>
                path.to_token_stream().to_string(),
            TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. } =>
                kind.to_token_stream().to_string(),
            TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. } =>
                model.to_string(),
        }.as_str())
    }
}
impl NameTreeContext for TypeContext {
    fn join_fn(&self, path: Path, sig_context: FnSignatureContext, attrs: Vec<Attribute>) -> Self {
        match self {
            TypeContext::Trait { prefix, .. } |
            TypeContext::Impl { prefix, .. } =>
                TypeContext::Fn { path, sig_context, prefix: prefix.clone(), attrs, parent: Some(Box::new(self.clone())) },
            _ => panic!()
        }
    }

    fn join_variant(&self, ident: Ident, variant_ident: Ident, attrs: Vec<Attribute>) -> Self {
        match self {
            TypeContext::Enum { prefix, .. } =>
                TypeContext::EnumVariant { attrs, ident, prefix: prefix.clone(), variant_ident, parent: Box::new(self.clone()) },
            _ => panic!()
        }
    }
}
