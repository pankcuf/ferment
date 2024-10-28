use ferment_macro::Display;
use crate::composer::{DropSequenceMixer, FFIConversionsMixer, LinkedContextComposer};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ArgKind, Aspect, Expression, ScopeContextPresentable, SeqKind};
use crate::shared::SharedAccess;

#[derive(Clone, Debug, Display)]
pub enum InterfaceKind<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    From(FFIConversionsMixer<Link, LANG, SPEC>),
    To(FFIConversionsMixer<Link, LANG, SPEC>),
    Destroy(LinkedContextComposer<Link, SeqKind<LANG, SPEC>, SeqKind<LANG, SPEC>>),
    Drop(DropSequenceMixer<Link, LANG, SPEC>),
}