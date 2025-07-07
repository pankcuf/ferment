use std::fmt::Debug;
use quote::ToTokens;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, FromConversionFullComposer, ToConversionFullComposer, DestroyFullConversionComposer};
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::Specification;
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[derive(Clone, Debug)]
pub enum ConversionType<SPEC>
    where SPEC: Specification {
    From(FieldComposer<SPEC>, Option<SPEC::Expr>),
    To(FieldComposer<SPEC>, Option<SPEC::Expr>),
    Destroy(FieldComposer<SPEC>, Option<SPEC::Expr>),
    // Destroy(DestroyConversionComposer<LANG, SPEC>),
}

impl<SPEC> ConversionType<SPEC>
    where SPEC: Specification {
    pub fn expr_from(composer: &FieldComposer<SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::From(composer.clone(), expr)
    }
    pub fn expr_to(composer: &FieldComposer<SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::To(composer.clone(), expr)
    }
    pub fn expr_destroy(composer: &FieldComposer<SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::Destroy(composer.clone(), expr)
    }
}

impl<SPEC> SourceComposable for ConversionType<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens,
          FFIFullPath<SPEC>: ToType,
          FFIFullDictionaryPath<SPEC>: ToType
{
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            ConversionType::From(composer, expr) =>
                FromConversionFullComposer::<SPEC>::key_in_scope_with_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            ConversionType::To(composer, expr) =>
                ToConversionFullComposer::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            ConversionType::Destroy(composer, expr) =>
                DestroyFullConversionComposer::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source).unwrap_or_default(),
                // DestroyConversionComposer::new(composer.name.clone(), composer.ty().clone(), expr.clone()).compose(source),
        }
    }
}
