use std::fmt::{Debug, Formatter};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{FnArg, ItemFn, ParenthesizedGenericArguments, parse_quote, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn};
use quote::{quote, ToTokens};
use syn::token::RArrow;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{CfgAttributes, FieldTypeComposition, FieldTypeConversionKind};
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{Conversion, FFIVariableResolve, FFITypeResolve, Mangle, Resolve};
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

#[derive(Clone)]
pub struct FnArgComposer {
    pub name: Option<TokenStream2>,
    // pub name_type_original: OwnedFieldTypeComposerRef,
    // pub name_type_conversion: FieldTypePresentationContextPassRef,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: Expression,
}

impl FnArgComposer {
    pub fn new(name: Option<TokenStream2>, original: OwnedItemPresentableContext, conversion: Expression) -> Self {
        Self { name, name_type_original: original, name_type_conversion: conversion }
    }
}

impl Debug for FnArgComposer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnArgComposition")
            .field("name", &format!("{}", self.name.to_token_stream()))
            .field("name_type_original", &format!("{}", self.name_type_original))
            .field("name_type_conversion", &format!("{}", self.name_type_conversion))
            .finish()
    }
}
impl<'a> Composer<'a> for ReturnType {
    type Source = (bool, &'a ScopeContext);
    type Result = FnReturnTypeComposer;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (bare, source) = source;
        match (bare, self) {
            (false, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: Expression::LineTermination
            },
            (false, ReturnType::Type(_, ty)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(RArrow::default(), Box::new(ty.to_full_ffi_variable(source))),
                conversion: ty.conversion_to(Expression::Obj)
            },
            (true, ReturnType::Type(token, field_type)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(token.clone(), Box::new(field_type.to_full_ffi_variable(source))),
                conversion: Expression::Empty
            },
            (true, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: Expression::Empty,
            }
        }
    }
}

impl<'a> Composer<'a> for PatType {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (_ctx, source) = source;
        let PatType { ty, attrs, pat, .. } = self;
        let maybe_obj = source.maybe_object(ty);
        let ident = match &**pat {
            Pat::Ident(PatIdent { ident, .. }) => ident,
            _ =>
                panic!("error: Arg conversion not supported: {}", quote!(#ty)),
        };

        let (conversion_type, name_type_conversion) = match maybe_obj {
            Some(ObjectConversion::Item(composition, ..) |
                 ObjectConversion::Type(composition)) => {
                let ty = composition.ty().clone();
                println!("PatType::compose({})", composition);
                match composition {
                    // TODO: For now we assume that every callback defined as fn pointer is opaque
                    TypeCompositionConversion::FnPointer(_) =>
                        (ty.special_or_to_ffi_full_path_type(source), Expression::Simple(quote!(#ident))),
                    TypeCompositionConversion::Primitive(_) |
                    TypeCompositionConversion::Trait(_, _, _) |
                    TypeCompositionConversion::TraitType(_) |
                    TypeCompositionConversion::Object(_) |
                    TypeCompositionConversion::Optional(_) |
                    TypeCompositionConversion::Array(_) |
                    TypeCompositionConversion::Slice(_) |
                    TypeCompositionConversion::Tuple(_) |
                    TypeCompositionConversion::Unknown(_) |
                    TypeCompositionConversion::LocalOrGlobal(_) => (
                        ty.to_full_ffi_variable(source),
                        {
                            let conversion = ty.resolve(source).conversion_from(Expression::Simple(quote!(#ident)));
                            match ty {
                                Type::Reference(..) =>
                                    Expression::AsRef(conversion.into()),
                                _ => conversion
                            }
                        }
                    ),
                    TypeCompositionConversion::Imported(_, _) => panic!("error: Arg conversion (Imported) not supported: {}", quote!(#ty)),
                    TypeCompositionConversion::Bounds(bounds) => (
                        bounds.ffi_full_dictionary_type_presenter(source),

                        match bounds.bounds.len() {
                            0 => Expression::Simple(quote!(#ident)),
                            1 => {
                                println!("TypeCompositionConversion::Bounds:::: {}", bounds);
                                if let Some(ParenthesizedGenericArguments { inputs, .. }) = bounds.maybe_bound_is_callback(bounds.bounds.first().unwrap()) {
                                    let lambda_args = inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)).collect::<CommaPunctuated<_>>();
                                    Expression::Simple(quote!(|#lambda_args| unsafe { (&*#ident).call(#lambda_args) }))
                                } else {
                                    Expression::From(Expression::Simple(quote!(#ident)).into())
                                }
                            }
                            _ =>
                                unimplemented!("Mixin as fn arg..."),
                        }
                    ),
                    ty =>
                        panic!("error: Arg conversion ({ty}) not supported"),
                }
            }
            _ => panic!("ObjectConversion::None or Empty"),
        };
        FnArgComposer::new(
            Some(pat.to_token_stream()),
            original(Name::Pat(*pat.clone()), conversion_type, attrs.cfg_attributes_expanded()),
            name_type_conversion)
    }
}

fn original(name: Name, ty: Type, attrs: Depunctuated<Expansion>) -> OwnedItemPresentableContext {
    OwnedItemPresentableContext::Named(FieldTypeComposition::new(name, FieldTypeConversionKind::Type(ty), true, attrs), false)
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
                let (mangled_ident, name_type_conversion) = match maybe_trait_ty {
                    Some(trait_ty) => (
                        trait_ty.resolve(&source).mangle_ident_default(),
                        Expression::SelfAsTrait(self_ty.resolve(&source).to_token_stream())
                    ),
                    None => (
                        self_ty.resolve(&source).mangle_ident_default(),
                        Expression::From(Expression::Self_.into())
                    )
                };
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
                FnArgComposer::new(None, name_type_original, name_type_conversion)
            },
            _ => panic!("Receiver in regular fn")
        }
    }
}


