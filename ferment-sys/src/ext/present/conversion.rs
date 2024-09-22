use syn::Type;
use crate::composable::FieldComposer;
use crate::composer::{Composer, DestroyConversionComposer, FromConversionFullComposer, ToConversionComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::lang::Specification;
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::Name;

#[derive(Clone)]
pub enum ConversionType<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    From(Name, Type, Option<SPEC::Expr>),
    To(ToConversionComposer<LANG, SPEC>),
    Destroy(DestroyConversionComposer<LANG, SPEC>),
    // Variable(VariableComposer)
}

impl<LANG, SPEC> ConversionType<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG,
              Expr=Expression<LANG, SPEC>>,
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

impl<'a, LANG, SPEC> Composer<'a> for ConversionType<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        match self {
            ConversionType::From(name, ty, expr) => {
                FromConversionFullComposer::<LANG, SPEC>::new(name.clone(), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope), expr.clone()).compose(source)
            },
            ConversionType::To(composer) =>
                composer.compose(source),
            ConversionType::Destroy(composer) =>
                composer.compose(source),
        }
    }
}
