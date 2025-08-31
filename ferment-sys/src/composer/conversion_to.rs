use std::fmt::Debug;
use quote::quote;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeContext, ScopeSearch};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GenericBoundsModel, ObjectKind, ScopeItemKind, SmartPointerModelKind, SpecialType, TypeModelKind};
use crate::ext::{AsType, ExpressionComposable, FFIObjectResolve, FFISpecialTypeResolve, GenericNestedArg, MaybeLambdaArgs, Primitive, Resolve, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

#[derive(Clone, Debug)]
pub struct ConversionToComposer<SPEC>
where SPEC: Specification {
    pub name: SPEC::Name,
    pub search: ScopeSearch,
    pub expr: Option<SPEC::Expr>,
}

impl<SPEC> ConversionToComposer<SPEC>
where SPEC: Specification {
    pub fn new(name: SPEC::Name, search: ScopeSearch, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search, expr }
    }
    pub fn key_expr_in_composer_scope(name: SPEC::Name, ty: &Type, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_key_in_composer_scope(ty), expr)
    }
    pub fn value_maybe_expr(name: SPEC::Name, ty: &Type, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_value(ty), expr)
    }
    pub fn key_in_composer_scope(name: SPEC::Name, ty: &Type) -> Self {
        Self::key_expr_in_composer_scope(name, ty, None)
    }
    pub fn value(name: SPEC::Name, ty: &Type) -> Self {
        Self::value_maybe_expr(name, ty, None)
    }
    pub fn value_expr(name: SPEC::Name, ty: &Type, expr: SPEC::Expr) -> Self {
        Self::value_maybe_expr(name, ty, Some(expr))
    }
}

impl<SPEC> SourceComposable for ConversionToComposer<SPEC>
where SPEC: Specification<Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        let field_path = expr.clone().unwrap_or_else(|| SPEC::Expr::simple(name));
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| search_key.to_type());
        let is_ref = search_key.maybe_originally_is_ref();
        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };
        let ffi_type = Resolve::<FFIFullPath<SPEC>>::resolve(&full_type, source).to_type();
        let result = match maybe_object {
            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object().unwrap() {
                ObjectKind::Type(ref ty_conversion) |
                ObjectKind::Item(ref ty_conversion, ..) => {
                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                    let maybe_special: Option<SpecialType<SPEC>> = full_parent_ty.maybe_resolve(source);
                    match maybe_special {
                        Some(SpecialType::Opaque(..)) if is_ref =>
                            Expression::boxed_tokens(quote!(#name.clone())),
                        Some(SpecialType::Opaque(..)) =>
                            Expression::boxed_tokens(name),
                        Some(SpecialType::Custom(custom_ty)) =>
                            Expression::cast_to(field_path, ConversionExpressionKind::Complex, custom_ty, full_parent_ty),
                        _ =>
                            Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_parent_ty),
                    }
                },
                _ => Expression::cast_to(if is_ref { field_path.cloned() } else { field_path }, ConversionExpressionKind::from(&full_type), ffi_type, full_type)
            },
            Some(ObjectKind::Item(ty_kind, ..) | ObjectKind::Type(ty_kind)) => {
                let maybe_special: Option<SpecialType<SPEC>> = full_type.maybe_special_type(source);
                match maybe_special {
                    Some(SpecialType::Opaque(..)) if search_key.maybe_originally_is_ptr() =>
                        field_path,
                    Some(SpecialType::Opaque(..)) if is_ref =>
                        Expression::boxed(field_path.cloned()),
                    Some(SpecialType::Opaque(..)) =>
                        Expression::boxed(field_path),
                    Some(SpecialType::Custom(custom_ty)) =>
                        Expression::cast_to(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
                    _ => match ty_kind {
                        TypeModelKind::FnPointer(..) |
                        TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                            Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type),
                        TypeModelKind::Optional(TypeModel { ty, .. }) => {
                            let nested_ty_kind = ty.maybe_first_nested_type_kind().unwrap();
                            let nested_ty = nested_ty_kind.to_type();
                            let maybe_nested_special: Option<SpecialType<SPEC>> = nested_ty.maybe_special_type(source);
                            let (expr_kind, ffi_type) = match maybe_nested_special {
                                Some(SpecialType::Custom(custom_ty)) => (ConversionExpressionKind::ComplexOpt, custom_ty),
                                Some(SpecialType::Opaque(opaque_ty)) => (ConversionExpressionKind::PrimitiveOpt, opaque_ty),
                                _ => (nested_ty_kind.is_primitive().then_some(ConversionExpressionKind::PrimitiveOpt).unwrap_or(ConversionExpressionKind::ComplexOpt), ffi_type)
                            };
                            Expression::cast_to(field_path, expr_kind, ffi_type, nested_ty)
                        }
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                            Expression::cast_to::<Type, Type>(if is_ref { field_path.cloned() } else { field_path }, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(i128)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                            Expression::cast_to::<Type, Type>(if is_ref { field_path.cloned() } else { field_path }, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(u128)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty, .. }))) =>
                            Expression::cast_to::<Type, Type>(field_path, ConversionExpressionKind::Complex, ffi_type, parse_quote!(&#ty)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ref ty, .. })))) => if let Some(nested_ty) = ty.maybe_first_nested_type_ref() {
                            match FFISpecialTypeResolve::<SPEC>::maybe_special_type(nested_ty, source) {
                                Some(SpecialType::Opaque(..)) =>
                                    Expression::deref_expr(field_path),
                                _ =>
                                    Expression::cast_to(Expression::deref_expr(field_path), ConversionExpressionKind::Complex, ffi_type, nested_ty.clone())
                            }
                        } else {
                            Expression::expression_to(ConversionExpressionKind::Primitive, field_path)
                        },
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ref ty, .. }))) => if let Some(nested_ty) = ty.maybe_first_nested_type_ref() {
                            match (FFISpecialTypeResolve::<SPEC>::maybe_special_type(nested_ty, source), nested_ty.maybe_object(source)) {
                                (Some(SpecialType::Opaque(..)), ..) =>
                                    Expression::boxed(Expression::cow_into_owned(field_path)),
                                (_, Some(ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..))))) =>
                                    Expression::cast_to(Expression::cow_into_owned(field_path), ConversionExpressionKind::Primitive, ffi_type, nested_ty.clone()),
                                _ =>
                                    Expression::cast_to(Expression::cow_into_owned(field_path), ConversionExpressionKind::Complex, ffi_type, nested_ty.clone())
                            }
                        } else {
                            Expression::expression_to(ConversionExpressionKind::Primitive, Expression::cow_into_owned(field_path))
                        },
                        TypeModelKind::Bounds(GenericBoundsModel { bounds, .. }) => match bounds.len() {
                            0 => field_path,
                            1 => if let Some(lambda_args) = MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(bounds.first().unwrap()) {
                                Expression::from_lambda(field_path, lambda_args)
                            } else {
                                Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)
                            }
                            _ =>
                                Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)
                        },
                        _ =>
                            Expression::cast_to(if is_ref { field_path.cloned() } else { field_path }, ConversionExpressionKind::from(&full_type), ffi_type, full_type)
                    }
                }
            }
            _ =>
                Expression::cast_to(if is_ref { field_path.cloned() } else { field_path }, ConversionExpressionKind::from(&full_type), ffi_type, full_type)
        };
        result
    }
}
