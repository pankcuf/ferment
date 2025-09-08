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
use crate::formatter::{format_obj_vec, format_predicates_obj_dict};
use crate::lang::Specification;
use crate::presentable::{Expression, ScopeContextPresentable};

#[derive(Clone)]
pub struct GenericBoundsModel {
    // 'T'
    pub type_model: TypeModel,
    // 'Fn(u32) -> Result<bool, ProtocolError>' or 'Clone + Debug + Smth'
    // pub bounded_ty: Type,
    // pub bounds: Vec<ObjectKind>,
    // pub predicates: IndexMap<Type, Vec<ObjectKind>>,
    pub bounds: Vec<ObjectKind>,
    pub predicates: IndexMap<Type, Vec<ObjectKind>>,
    // pub bounds: Vec<Path>,
    // pub predicates: HashMap<Type, Vec<Path>>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
    // pub nested_arguments: HashMap<Path, CommaPunctuated<NestedArgument>>,
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
        f.write_fmt(format_args!("GenericBoundsModel(ty: {}, bounded_ty: {}, bounds: {:?}, nested_args: {})", self.type_model, format_obj_vec(&self.bounds), format_predicates_obj_dict(&self.predicates), self.nested_arguments.to_token_stream()))
    }
}

impl Display for GenericBoundsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl PartialEq for GenericBoundsModel {
    fn eq(&self, other: &Self) -> bool {
        let self_bounds = self.bounds.iter().map(|b| b.to_token_stream());
        let other_bounds = other.bounds.iter().map(|b| b.to_token_stream());
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
        // self.bounded_ty.to_token_stream().to_string().hash(state);
        self.bounds.iter().for_each(|bound| bound.to_token_stream().to_string().hash(state));
        // self.predicates.iter().for_each(||)
    }
}

impl GenericBoundsModel {
    // pub fn new(ident: &Ident, bounded_ty: Type, bounds: Vec<ObjectKind>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
    //     Self {
    //         type_model: TypeModel::new_generic(ident.to_type(), generics, nested_arguments.clone()),
    //         bounded_ty,
    //         bounds,
    //         nested_arguments,
    //     }
    // }
    pub fn new(ident: &Ident, bounds: Vec<ObjectKind>, predicates: IndexMap<Type, Vec<ObjectKind>>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self {
            type_model: TypeModel::new_generic_ident(ident, generics, nested_arguments.clone()),
            bounds,
            predicates,
            nested_arguments,
        }
    }
}

impl<SPEC> MaybeLambdaArgs<SPEC> for GenericBoundsModel
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        if self.is_lambda() {
            self.bounds.first().map(MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names)?
        } else {
            None
        }
    }
}
impl GenericBoundsModel {
    pub fn is_lambda(&self) -> bool {
        self.bounds.iter().find(|b| match b {
            ObjectKind::Type(ty) |
            ObjectKind::Item(ty, _) => ty.is_lambda(),
            ObjectKind::Empty => false
        }).is_some()
    }
}
impl GenericBoundsModel {
    pub fn expr_from<SPEC>(&self, field_path: SPEC::Expr) -> SPEC::Expr
        where SPEC: Specification<Expr=Expression<SPEC>>,
              SPEC::Expr: ScopeContextPresentable {
        // println!("GenericBoundsModel::expr_from: {} /// {}",
        //          Vec::from_iter(self.bounds.iter().map(|obj| obj.to_token_stream().to_string())).join(" + "),
        //          Vec::from_iter(self.predicates.iter().map(|(ty, bb)| format!("{}: {}", ty.to_token_stream(), Vec::from_iter(bb.iter().map(|b| b.to_token_stream().to_string())).join(" + ")))).join(" + "));

        if self.bounds.is_empty() {
            Expression::from_primitive(field_path)
        } else if let Some(lambda_args) = MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(self) {
            Expression::from_lambda(field_path, lambda_args)
        } else {
            Expression::from_complex(field_path)
        }
    }

}
