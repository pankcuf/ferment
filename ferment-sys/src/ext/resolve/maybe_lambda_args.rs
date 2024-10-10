use quote::ToTokens;
use syn::{BareFnArg, TypeBareFn};
use crate::ast::CommaPunctuated;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::Name;

pub trait MaybeLambdaArgs<T: ToTokens> {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<T>>;
}

impl<LANG, SPEC> MaybeLambdaArgs<Name<LANG, SPEC>> for TypeBareFn
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<Name<LANG, SPEC>>> {
        Some(CommaPunctuated::from_iter(self.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
            Some((ident, ..)) => Name::Ident(ident.clone()),
            None => Name::UnnamedArg(index)
        })))
    }
}