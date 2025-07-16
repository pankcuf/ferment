use std::fmt::Debug;
use quote::ToTokens;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, ConversionFromComposer, ConversionToComposer, ConversionDropComposer};
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::Specification;
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[derive(Clone, Debug)]
pub enum Conversion<SPEC>
    where SPEC: Specification {
    From(FieldComposer<SPEC>, Option<SPEC::Expr>),
    To(FieldComposer<SPEC>, Option<SPEC::Expr>),
    Destroy(FieldComposer<SPEC>, Option<SPEC::Expr>),
}

impl<SPEC> Conversion<SPEC>
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

impl<SPEC> SourceComposable for Conversion<SPEC>
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
            Conversion::From(composer, expr) =>
                ConversionFromComposer::<SPEC>::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            Conversion::To(composer, expr) =>
                ConversionToComposer::<SPEC>::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source),
            Conversion::Destroy(composer, expr) =>
                ConversionDropComposer::<SPEC>::key_expr(composer.name.clone(), composer.ty(), &source.scope, expr.clone()).compose(source).unwrap_or_default(),
        }
    }
}
