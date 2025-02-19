use syn::{BareFnArg, TypeBareFn};
use crate::ast::CommaPunctuated;
use crate::lang::{LangFermentable, NameComposable, Specification};

pub trait MaybeLambdaArgs<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>>;
}

impl<LANG, SPEC> MaybeLambdaArgs<LANG, SPEC> for TypeBareFn
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        Some(CommaPunctuated::from_iter(self.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
            Some((ident, ..)) => SPEC::Name::ident(ident.clone()),
            None => SPEC::Name::unnamed_arg(index)
        })))
    }
}