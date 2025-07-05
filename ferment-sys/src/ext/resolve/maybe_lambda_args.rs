use syn::{BareFnArg, TypeBareFn};
use crate::ast::CommaPunctuated;
use crate::lang::{NameComposable, Specification};

pub trait MaybeLambdaArgs<SPEC>
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>>;
}

impl<SPEC> MaybeLambdaArgs<SPEC> for TypeBareFn
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        Some(CommaPunctuated::from_iter(self.inputs.iter().enumerate().map(|(index, BareFnArg { name, ..})| match name {
            Some((ident, ..)) => SPEC::Name::ident(ident.clone()),
            None => SPEC::Name::unnamed_arg(index)
        })))
    }
}