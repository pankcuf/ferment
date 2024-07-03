use quote::{quote, ToTokens};
use syn::{ParenthesizedGenericArguments, Type};
use crate::ast::CommaPunctuated;
use crate::composable::{FieldComposer, TypeComposition};
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{FFICompositionResolve, FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, SpecialType};
use crate::presentable::Expression;
use crate::presentation::{InterfacesMethodExpr, Name};

#[derive(Clone, Debug)]
pub struct ToConversionComposer {
    pub name: Name,
    pub ty: Type,
}

impl From<&FieldComposer> for ToConversionComposer {
    fn from(value: &FieldComposer) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone() }
    }
}
impl ToConversionComposer {
    pub fn new(name: Name, ty: Type) -> Self {
        Self { name, ty }
    }
}
impl<'a> Composer<'a> for ToConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let Self{ name, ty } = self;
        let field_path = Expression::Simple(name.to_token_stream());
        let full_type = ty.full_type(source);
        let expression = match full_type.maybe_special_type(source) {
            Some(SpecialType::Opaque(..)) =>
                Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(name.to_token_stream())),
            Some(SpecialType::Custom(..)) =>
                Expression::To(field_path.into()),
            None => match ty.composition(source) {
                TypeCompositionConversion::FnPointer(..) =>
                    field_path,
                TypeCompositionConversion::Optional(..)  =>
                    Expression::ToOpt(field_path.into()),
                TypeCompositionConversion::Boxed(TypeComposition { ref ty, .. }) => if let Some(nested_ty) = ty.first_nested_type() {
                    match (nested_ty.maybe_special_type(source),
                           nested_ty.maybe_object(source)) {
                        (Some(SpecialType::Opaque(..)),
                            Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_) |
                                                        TypeCompositionConversion::Trait(..) |
                                                        TypeCompositionConversion::TraitType(..), ..) |
                                 ObjectConversion::Type(TypeCompositionConversion::FnPointer(_) |
                                                        TypeCompositionConversion::Trait(..) |
                                                        TypeCompositionConversion::TraitType(..)))) =>
                            Expression::DerefContext(field_path.into()),
                        (Some(SpecialType::Opaque(..)), _any_other) =>
                            Expression::DerefContext(field_path.into()),
                        _ =>
                            Expression::To(Expression::DerefContext(field_path.into()).into())
                    }
                } else {
                    field_path
                },
                TypeCompositionConversion::Bounds(bounds) => match bounds.bounds.len() {
                    0 => field_path,
                    1 => if let Some(ParenthesizedGenericArguments { inputs, .. }) = bounds.maybe_bound_is_callback(bounds.bounds.first().unwrap()) {
                        let lambda_args = CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)));
                        Expression::Simple(quote!(|#lambda_args| unsafe { (&*#name).call(#lambda_args) }))
                    } else {
                        Expression::To(field_path.into())
                    }
                    _ =>
                        Expression::To(field_path.into())
                },
                _ => match TypeConversion::from(ty) {
                    TypeConversion::Primitive(_) =>
                        field_path,
                    TypeConversion::Generic(GenericTypeConversion::Optional(_)) =>
                        Expression::ToOpt(field_path.into()),
                    _ =>
                        Expression::To(field_path.into())
                }
            }
        };
        expression
        // match self {
        //     Type::Array(ty) =>
        //         expression,
        //     Type::Path(ty) =>
        //         ty.conversion_to(expr),
        //     Type::Ptr(ty) =>
        //         ty.conversion_to(expr),
        //     Type::Reference(ty) =>
        //         ty.conversion_to(expr),
        //     Type::Slice(ty) =>
        //         Expression::To(Expression::ToVec(expression.into()).into())
        //     Type::TraitObject(ty) =>
        //         ty.conversion_to(expr),
        //     Type::Tuple(ty) =>
        //         ty.conversion_to(expr),
        //     Type::ImplTrait(ty) =>
        //         ty.conversion_to(expr),
        //     _ => unimplemented!("No conversions for {}", self.to_token_stream())
        // }

        // ty.conversion_to(expression)
    }
}