use std::fmt::Debug;
use quote::ToTokens;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, FromConversionFullComposer, ToConversionFullComposer, DestroyFullConversionComposer};
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[derive(Clone, Debug)]
pub enum ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    From(FieldComposer<LANG, SPEC>, Option<SPEC::Expr>),
    To(FieldComposer<LANG, SPEC>, Option<SPEC::Expr>),
    Destroy(FieldComposer<LANG, SPEC>, Option<SPEC::Expr>),
    // Destroy(DestroyConversionComposer<LANG, SPEC>),
}

impl<LANG, SPEC> ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn expr_from(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::From(composer.clone(), expr)
    }
    pub fn expr_to(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::To(composer.clone(), expr)
    }
    pub fn expr_destroy(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::Destroy(composer.clone(), expr)
    }
}

impl<LANG, SPEC> SourceComposable for ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType
{
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            ConversionType::From(composer, expr) =>
                FromConversionFullComposer::<LANG, SPEC>::key_in_scope_with_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            ConversionType::To(composer, expr) =>
                ToConversionFullComposer::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            ConversionType::Destroy(composer, expr) =>
                DestroyFullConversionComposer::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source).unwrap_or(Expression::Empty),
                // DestroyConversionComposer::new(composer.name.clone(), composer.ty().clone(), expr.clone()).compose(source),
        }
    }
}
