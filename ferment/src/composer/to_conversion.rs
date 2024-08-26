use quote::ToTokens;
use syn::Type;
use crate::composable::TypeModel;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{DictTypeModelKind, GenericTypeKind, ObjectKind, ScopeItemKind, TypeModelKind, TypeKind, DictFermentableModelKind, SmartPointerModelKind};
use crate::ext::{FFITypeModelKindResolve, FFIObjectResolve, FFISpecialTypeResolve, GenericNestedArg, Resolve, SpecialType, ToType, AsType};
use crate::presentable::Expression;
use crate::presentation::{InterfacesMethodExpr, Name};

#[derive(Clone, Debug)]
pub struct ToConversionComposer {
    pub name: Name,
    pub ty: Type,

    pub expr: Option<Expression>
}

// impl From<&FieldComposer> for ToConversionComposer {
//     fn from(value: &FieldComposer) -> Self {
//         Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
//     }
// }
impl ToConversionComposer {
    pub fn new(name: Name, ty: Type, expr: Option<Expression>) -> Self {
        Self { name, ty, expr }
    }
}

fn from_external(ty: &Type, field_path: Expression) -> Expression {
    match TypeKind::from(ty) {
        TypeKind::Primitive(_) =>
            field_path,
        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
            TypeKind::Primitive(_) => Expression::ToOptPrimitive(field_path.into()),
            _ => Expression::ToOpt(field_path.into()),
        }
        _ =>
            Expression::To(field_path.into())
    }
}

impl<'a> Composer<'a> for ToConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let Self { name, ty, expr } = self;
        let field_path = /*ty.conversion_to(*/expr.clone()
            .unwrap_or(Expression::Simple(name.to_token_stream()))/*)*/;
        match source.maybe_object_by_key(ty) {
            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object().unwrap() {
                ObjectKind::Type(ref ty_conversion) |
                ObjectKind::Item(ref ty_conversion, ..) => {
                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                    match <Type as Resolve<Option<SpecialType>>>::resolve(&full_parent_ty, source) {
                        Some(SpecialType::Opaque(..)) =>
                            Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(name.to_token_stream())),
                        Some(SpecialType::Custom(..)) =>
                            Expression::To(field_path.into()),
                        None => Expression::To(field_path.into()),
                    }
                },
                _ => from_external(ty, field_path)
            },
            Some(ObjectKind::Item(ty_conversion, ..) |
                 ObjectKind::Type(ty_conversion)) => {
                let full_type = ty_conversion.to_type();
                match full_type.maybe_special_type(source) {
                    Some(SpecialType::Opaque(..)) =>
                        Expression::Boxed(field_path.into()),
                        // Expression::InterfacesExpr(InterfacesMethodExpr::Boxed(name.to_token_stream())),
                    Some(SpecialType::Custom(..)) =>
                        Expression::To(field_path.into()),
                    None => match ty.type_model_kind(source) {
                        TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                            field_path,
                        TypeModelKind::Optional(ty) => match TypeKind::from(ty.as_type().first_nested_type().unwrap()) {
                            TypeKind::Primitive(_) => Expression::ToOptPrimitive(field_path.into()),
                            _ => Expression::ToOpt(field_path.into())
                        }
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ref ty, .. })))) => if let Some(nested_ty) = ty.first_nested_type() {
                            match (nested_ty.maybe_special_type(source),
                                   nested_ty.maybe_object(source)) {
                                (Some(SpecialType::Opaque(..)),
                                    Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) |
                                                                TypeModelKind::Trait(..) |
                                                                TypeModelKind::TraitType(..), ..) |
                                         ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) |
                                                                TypeModelKind::Trait(..) |
                                                                TypeModelKind::TraitType(..)))) =>
                                    Expression::DerefContext(field_path.into()),
                                (Some(SpecialType::Opaque(..)), _any_other) =>
                                    Expression::DerefContext(field_path.into()),
                                _ =>
                                    Expression::To(Expression::DerefContext(field_path.into()).into())
                            }
                        } else {
                            field_path
                        },
                        TypeModelKind::Bounds(bounds) => match bounds.bounds.len() {
                            0 => field_path,
                            1 => if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                // Expression::Simple(quote!(move |#lambda_args| unsafe { (&*#name).call(#lambda_args) }))
                                Expression::FromLambda(field_path.into(), lambda_args).into()

                            } else {
                                Expression::To(field_path.into())
                            }
                            _ =>
                                Expression::To(field_path.into())
                        },
                        _ => from_external(ty, field_path)
                    }
                }
            }
            _ => from_external(ty, field_path)
        }
    }
}