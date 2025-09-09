use syn::{Generics, Type};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use indexmap::IndexMap;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use crate::ast::CommaPunctuated;
use crate::composable::{TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::kind::ObjectKind;
use crate::ext::{AsType, MaybeLambdaArgs, ToType};
use crate::formatter::format_generic_scope_chain;
use crate::lang::Specification;
use crate::presentable::{Expression, ScopeContextPresentable};

#[derive(Clone)]
pub struct GenericBoundsModel {
    pub type_model: TypeModel,
    pub chain: IndexMap<ObjectKind, Vec<ObjectKind>>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
}

impl<'a> AsType<'a> for GenericBoundsModel {
    fn as_type(&'a self) -> &'a Type {
        self.type_model.as_type()
    }
}
impl ToType for GenericBoundsModel {
    fn to_type(&self) -> Type {
        self.type_model.to_type()
    }
}
impl TypeModeled for GenericBoundsModel {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        &mut self.type_model
    }

    fn type_model_ref(&self) -> &TypeModel {
        &self.type_model
    }
}

impl Debug for GenericBoundsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { type_model, chain, nested_arguments, .. } = self;
        f.write_fmt(format_args!("GenericBoundsModel(ty: {type_model}, chain: {}, nested_args: {})", format_generic_scope_chain(chain), nested_arguments.to_token_stream()))
    }
}

impl Display for GenericBoundsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl PartialEq for GenericBoundsModel {
    fn eq(&self, other: &Self) -> bool {
        let self_bounds = self.chain.iter().map(|(bounded_ty, bounds)| quote!(#bounded_ty: #(#bounds)*+));
        let other_bounds = other.chain.iter().map(|(bounded_ty, bounds)| quote!(#bounded_ty: #(#bounds)*+));
        let self_tokens = [self.as_type().to_token_stream(), quote!(#(#self_bounds),*)];
        let other_tokens = [other.as_type().to_token_stream(), quote!(#(#other_bounds),*)];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(ToString::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericBoundsModel {}

impl Hash for GenericBoundsModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_type().to_token_stream().to_string().hash(state);
        self.chain.iter().for_each(|(bounded_ty, bounds)| quote!(#bounded_ty: #(#bounds)*+).to_string().hash(state));
    }
}

impl GenericBoundsModel {
    pub fn new(ident: &Ident, chain: IndexMap<ObjectKind, Vec<ObjectKind>>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self {
            type_model: TypeModel::new_generic(ident.to_type(), generics, nested_arguments.clone()),
            chain,
            nested_arguments,
        }
    }
}

impl<SPEC> MaybeLambdaArgs<SPEC> for GenericBoundsModel
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        if self.is_lambda() {
            self.chain.first().and_then(|(_, bounds)| bounds.first().map(MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names))?
        } else {
            None
        }
    }
}
impl GenericBoundsModel {
    pub fn is_lambda(&self) -> bool {
        self.chain.values().any(|bounds| bounds.iter().any(|b| b.is_lambda()))
    }

    pub fn first_bound(&self) -> Option<&ObjectKind> {
        self.chain.first().and_then(|(_, bounds)| bounds.first())
    }

    pub fn expr_from<SPEC>(&self, field_path: SPEC::Expr) -> SPEC::Expr
        where SPEC: Specification<Expr=Expression<SPEC>>,
              SPEC::Expr: ScopeContextPresentable {
        if self.chain.is_empty() {
            Expression::from_primitive(field_path)
        } else if let Some(lambda_args) = MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(self) {
            Expression::from_lambda(field_path, lambda_args)
        } else {
            Expression::from_complex(field_path)
        }
    }

}
