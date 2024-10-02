use syn::{Generics, parse_quote, Type};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use crate::ast::CommaPunctuated;
use crate::composable::{TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeContext;
use crate::conversion::ObjectKind;
use crate::ext::{AsType, Mangle, ToType};
use crate::formatter::{format_obj_vec, format_predicates_obj_dict};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::Name;

#[derive(Clone)]
pub struct GenericBoundsModel {
    // 'T'
    type_model: TypeModel,
    // 'Fn(u32) -> Result<bool, ProtocolError>' or 'Clone + Debug + Smth'
    pub bounds: Vec<ObjectKind>,
    pub predicates: HashMap<Type, Vec<ObjectKind>>,
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
        f.write_str(format!(
            "GenericBoundsModel(ty: {}, bounds: {}, predicates: {}, nested_args: {})",
            self.type_model,
            format_obj_vec(&self.bounds),
            format_predicates_obj_dict(&self.predicates),
            self.nested_arguments.to_token_stream()
        ).as_str())
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
            .all(|(a, b)| {
                let x = a == b;
                // println!("GGGGG:::({}) {} ==== {}", x, a, b);
                x
            })
    }
}

impl Eq for GenericBoundsModel {}

impl Hash for GenericBoundsModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_type().to_token_stream().to_string().hash(state);
        self.bounds.iter().for_each(|bound| bound.to_token_stream().to_string().hash(state));
        // self.predicates.iter().for_each(||)
    }
}

impl GenericBoundsModel {
    pub fn new(ty: Type, bounds: Vec<ObjectKind>, predicates: HashMap<Type, Vec<ObjectKind>>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self {
            type_model: TypeModel::new(ty, Some(generics), nested_arguments.clone()),
            bounds,
            predicates,
            nested_arguments,
        }
    }

    pub fn ffi_full_dictionary_type_presenter(&self, _source: &ScopeContext) -> Type {
        // unimplemented!("")
        let ffi_name = self.mangle_ident_default();
        println!("GenericBound: ffi_full_dictionary_type_presenter: {} --- {}", ffi_name, self);
        parse_quote!(crate::fermented::generics::#ffi_name)
        // Determine mixin type
        //
    }

}
impl GenericBoundsModel {
    pub fn is_lambda(&self) -> bool {
        self.bounds.iter().find(|b| {
            match b {
                ObjectKind::Type(ty) |
                ObjectKind::Item(ty, _) => ty.is_lambda(),
                ObjectKind::Empty => false
            }
        }).is_some()
    }
    pub fn maybe_lambda_args(&self) -> Option<CommaPunctuated<Name>> {
        if self.is_lambda() {
            self.bounds.first().unwrap().maybe_lambda_args()
        } else {
            None
        }
    }

    pub fn expr_from<LANG, SPEC>(&self, field_path: Expression<LANG, SPEC>) -> Expression<LANG, SPEC>
        where LANG: LangFermentable,
              SPEC: Specification<LANG, Var: ToType>,
              Aspect<SPEC::TYC>: ScopeContextPresentable {
        if self.bounds.is_empty() {
            Expression::from_primitive(field_path)
        } else if let Some(lambda_args) = self.maybe_lambda_args() {
            Expression::from_lambda(field_path, lambda_args)
        } else {
            Expression::from_complex(field_path)
        }
    }
}
