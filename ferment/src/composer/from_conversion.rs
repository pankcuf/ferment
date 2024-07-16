use quote::{quote, ToTokens};
use syn::{ParenthesizedGenericArguments, PatType, Type};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, TypeComposition};
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{FFICompositionResolve, FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, SpecialType};
use crate::presentable::Expression;
use crate::presentation::Name;

#[derive(Clone, Debug)]
pub struct FromConversionComposer {
    pub name: Name,
    pub ty: Type,
}

impl From<&FieldComposer> for FromConversionComposer {
    fn from(value: &FieldComposer) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone() }
    }
}

#[allow(unused)]
impl From<&PatType> for FromConversionComposer {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone() }
    }
}


#[allow(unused)]
impl FromConversionComposer {
    pub fn new(name: Name, ty: Type) -> Self {
        Self { name, ty }
    }
}
impl<'a> Composer<'a> for FromConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let Self { name, ty } = self;
        let field_path = Expression::Simple(name.to_token_stream());
        let full_type = ty.full_type(source);
        let expression = match full_type.maybe_special_type(source) {
            Some(SpecialType::Opaque(..)) =>
                field_path,
            Some(SpecialType::Custom(..)) =>
                Expression::From(field_path.into()),
            None => match ty.composition(source) {
                TypeCompositionConversion::FnPointer(..) | TypeCompositionConversion::LambdaFn(..) =>
                    field_path,
                TypeCompositionConversion::Optional(..)  =>
                    Expression::FromOpt(field_path.into()),
                TypeCompositionConversion::Boxed(TypeComposition { ref ty, .. }) => if let Some(nested_ty) = ty.first_nested_type() {
                    match (nested_ty.maybe_special_type(source),
                           nested_ty.maybe_object(source)) {
                        (Some(SpecialType::Opaque(..)),
                            Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_) |
                                                        TypeCompositionConversion::LambdaFn(_) |
                                                        TypeCompositionConversion::Trait(..) |
                                                        TypeCompositionConversion::TraitType(..), ..) |
                                 ObjectConversion::Type(TypeCompositionConversion::FnPointer(_) |
                                                        TypeCompositionConversion::LambdaFn(_) |
                                                        TypeCompositionConversion::Trait(..) |
                                                        TypeCompositionConversion::TraitType(..)))) =>
                            Expression::IntoBox(field_path.into()),
                        (Some(SpecialType::Opaque(..)), _any_other) =>
                            Expression::FromRawBox(field_path.into()),
                        (Some(SpecialType::Custom(..)), _any) =>
                            Expression::IntoBox(Expression::From(field_path.into()).into()),
                        _ =>
                            Expression::From(Expression::IntoBox(field_path.into()).into())
                    }
                } else {
                    field_path
                },
                TypeCompositionConversion::Bounds(bounds) => match bounds.bounds.len() {
                    0 => field_path,
                    1 => if let Some(ParenthesizedGenericArguments { inputs, .. }) = bounds.maybe_bound_is_callback(bounds.bounds.first().unwrap()) {
                        let lambda_args = CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)));
                        Expression::Simple(quote!(move |#lambda_args| unsafe { (&*#name).call(#lambda_args) }))
                    } else {
                        Expression::From(field_path.into())
                    }
                    _ =>
                        Expression::From(field_path.into())
                },
                _ => match TypeConversion::from(ty) {
                    TypeConversion::Primitive(_) =>
                        field_path,
                    TypeConversion::Generic(GenericTypeConversion::Optional(_)) =>
                        Expression::FromOpt(field_path.into()),
                    _ =>
                        Expression::From(field_path.into())
                }
            }
        };
        match ty {
            Type::Reference(_) =>
                Expression::AsRef(expression.into()),
            _ =>
                expression
        }
    }
}