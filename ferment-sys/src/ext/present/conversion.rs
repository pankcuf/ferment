use syn::Type;
use crate::composable::FieldComposer;
use crate::composer::{SourceComposable, DestroyConversionComposer, FromConversionFullComposer, ToConversionComposer};
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, Name};

#[derive(Clone, Debug)]
pub enum ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    From(Name, Type, Option<SPEC::Expr>),
    To(ToConversionComposer<LANG, SPEC>),
    Destroy(DestroyConversionComposer<LANG, SPEC>),
    // Variable(VariableComposer)
}

impl<LANG, SPEC> ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG,
              Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn expr_from(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::From(composer.name.clone(), composer.ty().clone(), expr)
    }
    pub fn expr_to(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::To(ToConversionComposer::new(composer.name.clone(), composer.ty().clone(), expr))
    }
    pub fn expr_destroy(composer: &FieldComposer<LANG, SPEC>, expr: Option<SPEC::Expr>) -> Self {
        Self::Destroy(DestroyConversionComposer::new(composer.name.clone(), composer.ty().clone(), expr))
    }
}

impl<LANG, SPEC> SourceComposable for ConversionType<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType
{
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            ConversionType::From(name, ty, expr) =>
                FromConversionFullComposer::<LANG, SPEC>::key_in_scope_with_expr(name.clone(), ty, &source.scope, expr.clone()).compose(source),
            ConversionType::To(composer) =>
                composer.compose(source),
            ConversionType::Destroy(composer) =>
                composer.compose(source),
        }
    }
}
