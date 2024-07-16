use std::fmt::{Debug, Formatter};
use proc_macro2::Ident;
use syn::{FnArg, ItemFn, parse_quote, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn};
use quote::{quote, ToTokens};
use crate::ast::Depunctuated;
use crate::composable::{CfgAttributes, FieldComposer, FieldTypeConversionKind};
use crate::composer::{Composer, FromConversionComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::presentable::{Expression, OwnedItemPresentableContext};
use crate::presentation::{DictionaryName, Expansion, Name};

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn(ItemFn),
    Impl(Type, Option<Type>, Signature),
    Bare(Ident, TypeBareFn),
    TraitInner(Type, Option<Type>, Signature)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FnSignatureContext::ModFn(sig) =>
                format!("ModFn({})", sig.to_token_stream()),
            FnSignatureContext::Impl(self_ty, trait_ty, sig) =>
                format!("Impl(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::TraitInner(self_ty, trait_ty, sig) =>
                format!("TraitInner(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::Bare(ident, type_bare_fn) =>
                format!("Bare({}, {})", ident.to_token_stream(), type_bare_fn.to_token_stream()),
        }.as_str())
    }
}

impl FnSignatureContext {
    #[allow(unused)]
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::Impl(_, Some(_), _) => true,
            _ => false
        }
    }
}
#[derive(Clone, Debug)]
pub struct FnReturnTypeComposer {
    pub presentation: ReturnType,
    pub conversion: Expression,
}

#[derive(Clone, Debug)]
pub struct FnArgComposer {
    // pub name: Option<TokenStream2>,
    // pub name_type_original: OwnedFieldTypeComposerRef,
    // pub name_type_conversion: FieldTypePresentationContextPassRef,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: Expression,
}

impl FnArgComposer {
    pub fn new(original: OwnedItemPresentableContext, conversion: Expression) -> Self {
        Self {name_type_original: original, name_type_conversion: conversion }
    }
}

impl<'a> Composer<'a> for PatType {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (_ctx, source) = source;
        let PatType { ty, attrs, pat, .. } = self;
        let from_conversion_composer = FromConversionComposer::from(self);
        FnArgComposer::new(
            original(Name::Pat(*pat.clone()), ty.to_type(), attrs.cfg_attributes_expanded()),
            from_conversion_composer.compose(source))
    }
}

fn original(name: Name, ty: Type, attrs: Depunctuated<Expansion>) -> OwnedItemPresentableContext {
    OwnedItemPresentableContext::Named(FieldComposer::new(name, FieldTypeConversionKind::Type(ty), true, attrs), false)
}

impl<'a> Composer<'a> for FnArg {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        match self {
            FnArg::Receiver(receiver) =>
                receiver.compose(source),
            FnArg::Typed(pat_ty) =>
                pat_ty.compose(source),
        }
    }
}

impl<'a> Composer<'a> for Receiver {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let (ctx, source) = source;
        let Receiver { mutability, reference, attrs, .. } = self;
        match ctx {
            FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
            FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => {
                let (target_type, name_type_conversion) = match maybe_trait_ty {
                    Some(trait_ty) => (
                        <Type as Resolve<Type>>::resolve(trait_ty, &source),
                        Expression::SelfAsTrait(<Type as Resolve<Type>>::resolve(self_ty, &source).to_token_stream())
                    ),
                    None => (
                        <Type as Resolve<Type>>::resolve(self_ty, &source),
                        Expression::From(Expression::Self_.into())
                    )
                };
                let mangled_ident = target_type.mangle_ident_default();
                let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                let name_type_original = original(
                    Name::Dictionary(DictionaryName::Self_),
                    parse_quote!(* #access #mangled_ident),
                    attrs.cfg_attributes_expanded()
                );
                let name_type_conversion = if reference.is_some() {
                    Expression::AsRef(name_type_conversion.into())
                } else {
                    name_type_conversion
                };
                FnArgComposer::new(name_type_original, name_type_conversion)
            },
            _ => panic!("Receiver in regular fn")
        }
    }
}


