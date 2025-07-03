use std::fmt::{Display, Formatter};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, ItemFn, ItemTrait, Path, TypeBareFn};
use crate::composable::{CfgAttributes, FnSignatureContext};
use crate::conversion::MixinKind;
use crate::ext::{AsType, ToPath};
use crate::presentable::NameTreeContext;

#[derive(Clone, Debug)]
pub enum TypeContext {
    Enum {
        ident: Ident,
        generics: Generics,
        attrs: Vec<Attribute>,
    },
    EnumVariant {
        parent: Box<TypeContext>,
        ident: Ident,
        variant_ident: Ident,
        attrs: Vec<Attribute>
    },
    Struct {
        ident: Ident,
        generics: Generics,
        attrs: Vec<Attribute>,
    },
    Fn {
        parent: Option<Box<TypeContext>>,
        path: Path,
        sig_context: FnSignatureContext,
        attrs: Vec<Attribute>,
    },
    Trait {
        path: Path,
        attrs: Vec<Attribute>,
    },
    Impl {
        path: Path,
        trait_: Option<Path>,
        attrs: Vec<Attribute>,
    },
    Mixin {
        mixin_kind: MixinKind,
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

    pub fn r#struct(ident: &Ident, attrs: Vec<Attribute>, generics: Generics) -> Self {
        Self::Struct { ident: ident.clone(), attrs, generics }
    }
    pub fn r#enum(ident: &Ident, attrs: Vec<Attribute>, generics: Generics) -> Self {
        Self::Enum { ident: ident.clone(), attrs, generics }
    }

    pub fn r#impl(path: Path, trait_: Option<Path>, attrs: Vec<Attribute>) -> Self {
        Self::Impl { path, attrs, trait_ }
    }
    pub fn mixin(kind: &MixinKind, attrs: Vec<Attribute>) -> Self {
        Self::Mixin { mixin_kind: kind.clone(), attrs }
    }
    pub fn mod_fn(path: Path, item: &ItemFn) -> Self {
        Self::Fn { parent: None, path, sig_context: FnSignatureContext::ModFn(item.clone()), attrs: item.attrs.cfg_attributes() }
    }
    pub fn callback(path: Path, ident: &Ident, item: &TypeBareFn, attrs: &Vec<Attribute>) -> Self {
        Self::Fn { parent: None, path, sig_context: FnSignatureContext::Bare(ident.clone(), item.clone()), attrs: attrs.cfg_attributes() }
    }
    pub fn r#trait(item: &ItemTrait) -> Self {
        Self::Trait { path: item.ident.to_path(), attrs: item.attrs.cfg_attributes() }
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
impl ToTokens for TypeContext {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeContext::Enum { ident, .. } |
            TypeContext::Struct { ident, .. } =>
                ident.to_tokens(tokens),
            TypeContext::EnumVariant { ident, variant_ident, .. } =>
                quote!(#ident::#variant_ident).to_tokens(tokens),
            TypeContext::Fn { path, .. } |
            TypeContext::Trait { path, .. } |
            TypeContext::Impl { path, .. } =>
                path.to_tokens(tokens),
            TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. } =>
                kind.to_tokens(tokens),
            TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. } =>
                model.as_type().to_tokens(tokens),
        }    }
}
impl NameTreeContext for TypeContext {
    fn join_fn(&self, path: Path, sig_context: FnSignatureContext, attrs: Vec<Attribute>) -> Self {
        match self {
            TypeContext::Trait { .. } | TypeContext::Impl { .. } =>
                TypeContext::Fn { path, sig_context, attrs, parent: Some(Box::new(self.clone())) },
            _ => panic!()
        }
    }

    fn join_variant(&self, ident: Ident, variant_ident: Ident, attrs: Vec<Attribute>) -> Self {
        match self {
            TypeContext::Enum { .. } =>
                TypeContext::EnumVariant { attrs, ident, variant_ident, parent: Box::new(self.clone()) },
            _ => panic!()
        }
    }
}

