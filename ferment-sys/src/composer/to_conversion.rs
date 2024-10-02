use std::marker::PhantomData;
use quote::ToTokens;
use syn::Type;
use crate::composable::TypeModel;
use crate::composer::{SourceComposable, FFIAspect};
use crate::context::ScopeContext;
use crate::conversion::{DictTypeModelKind, GenericTypeKind, ObjectKind, ScopeItemKind, TypeModelKind, TypeKind, DictFermentableModelKind, SmartPointerModelKind};
use crate::ext::{FFITypeModelKindResolve, FFIObjectResolve, FFISpecialTypeResolve, GenericNestedArg, Resolve, SpecialType, ToType, AsType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, Name};

#[derive(Clone, Debug)]
pub struct ToConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name,
    pub ty: Type,

    pub expr: Option<SPEC::Expr>,
    _marker: PhantomData<(LANG, SPEC)>,
}

impl<LANG, SPEC> ToConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr, _marker: Default::default() }
    }
}

fn from_external<LANG, SPEC>(ty: &Type, field_path: SPEC::Expr) -> SPEC::Expr
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    let kind = match TypeKind::from(ty) {
        TypeKind::Primitive(_) => ConversionExpressionKind::Primitive,
        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty.first_nested_type().unwrap()) {
            TypeKind::Primitive(_) => ConversionExpressionKind::PrimitiveOpt,
            _ => ConversionExpressionKind::ComplexOpt,
        }
        _ => ConversionExpressionKind::Complex,
    };
    Expression::ConversionExpr(FFIAspect::To, kind, field_path.into())
}

impl<LANG, SPEC> SourceComposable for ToConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, ty, expr, .. } = self;
        let (_by_ptr, by_ref) = match ty {
            Type::Ptr(_) => (true, false),
            Type::Reference(_) => (false, true),
            _ => (false, false)
        };
        let field_path = /*ty.conversion_to(*/expr.clone()
            .unwrap_or(Expression::ffi_to_primitive_tokens(name.to_token_stream()))/*)*/;
        match source.maybe_object_by_key(ty) {
            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object().unwrap() {
                ObjectKind::Type(ref ty_conversion) |
                ObjectKind::Item(ref ty_conversion, ..) => {
                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                    match <Type as Resolve<SpecialType<LANG, SPEC>>>::maybe_resolve(&full_parent_ty, source) {
                        Some(SpecialType::Opaque(..)) =>
                            Expression::boxed_tokens(name),
                        Some(SpecialType::Custom(..)) =>
                            Expression::ffi_to_complex(field_path),
                        _ =>
                            Expression::ffi_to_complex(field_path),
                    }
                },
                _ => from_external::<LANG, SPEC>(ty, if by_ref { Expression::Clone(field_path.into()) } else { field_path })
            },
            Some(ObjectKind::Item(ty_conversion, ..) |
                 ObjectKind::Type(ty_conversion)) => {
                let full_type = ty_conversion.to_type();
                match <Type as FFISpecialTypeResolve<LANG, SPEC>>::maybe_special_type(&full_type, source) {
                    Some(SpecialType::Opaque(..)) =>
                        Expression::boxed(field_path),
                    Some(SpecialType::Custom(..)) =>
                        Expression::ffi_to_complex(field_path),
                    _ => match ty.type_model_kind(source) {
                        TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                            Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Primitive, field_path.into()),
                        TypeModelKind::Optional(ty) => match TypeKind::from(ty.as_type().first_nested_type().unwrap()) {
                            TypeKind::Primitive(_) =>
                                Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::PrimitiveOpt, field_path.into()),
                            _ =>
                                Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::ComplexOpt, field_path.into()),
                        }
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::String(..))) =>
                            Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, if by_ref { Expression::Clone(field_path.into()) } else { field_path }.into()),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(..))) =>
                            Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, field_path.into()),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ref ty, .. })))) => if let Some(nested_ty) = ty.first_nested_type() {
                            match (<Type as FFISpecialTypeResolve<LANG, SPEC>>::maybe_special_type(&nested_ty, source),
                                   nested_ty.maybe_object(source)) {
                                (Some(SpecialType::Opaque(..)),
                                    Some(ObjectKind::Item(
                                             TypeModelKind::FnPointer(..) |
                                             TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) |
                                             TypeModelKind::Trait(..) |
                                             TypeModelKind::TraitType(..), ..) |
                                         ObjectKind::Type(
                                             TypeModelKind::FnPointer(..) |
                                             TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) |
                                             TypeModelKind::Trait(..) |
                                             TypeModelKind::TraitType(..)))) =>
                                    Expression::deref_expr(field_path),
                                (Some(SpecialType::Opaque(..)), _any_other) =>
                                    Expression::deref_expr(field_path),
                                _ =>
                                    Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, Expression::deref_expr(field_path).into()),
                            }
                        } else {
                            Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Primitive, field_path.into())
                        },
                        TypeModelKind::Bounds(bounds) => match bounds.bounds.len() {
                            0 => field_path,
                            1 => if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                                Expression::from_lambda(field_path, lambda_args)
                            } else {
                                Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, field_path.into())
                            }
                            _ =>
                                Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Complex, field_path.into())
                        },
                        _ => from_external::<LANG, SPEC>(ty, if by_ref { Expression::Clone(field_path.into()) } else { field_path })
                    }
                }
            }
            _ => from_external::<LANG, SPEC>(ty, if by_ref { Expression::Clone(field_path.into()) } else { field_path })
        }
    }
}