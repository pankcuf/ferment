use std::fmt::Debug;
use quote::quote;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::{FFIAspect, SourceComposable};
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, FFIObjectResolve, FFISpecialTypeResolve, GenericNestedArg, MaybeLambdaArgs, Resolve, SpecialType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ConversionExpressionKind, Expression, ExpressionComposable, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

#[derive(Clone, Debug)]
pub struct ToConversionFullComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub name: SPEC::Name,
    pub search: ScopeSearch<'a>,
    pub expr: Option<SPEC::Expr>,
}

impl<'a, LANG, SPEC> ToConversionFullComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub fn new(name: SPEC::Name, search: ScopeSearch<'a>, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search, expr }
    }
    pub fn key(name: SPEC::Name, ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::key_expr(name, ty, scope, None)
    }
    pub fn key_expr(name: SPEC::Name, ty: &'a Type, scope: &'a ScopeChain, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope), expr)
    }
    pub fn value(name: SPEC::Name, ty: &'a Type) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), None)
    }
    pub fn value_expr(name: SPEC::Name, ty: &'a Type, expr: SPEC::Expr) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), Some(expr))
    }
}

impl<'a, LANG, SPEC> SourceComposable for ToConversionFullComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      FFIFullPath<LANG, SPEC>: ToType,
      FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        let field_path = expr.clone().unwrap_or(SPEC::Expr::simple(name));
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(search_key.to_type());
        let is_ref = search_key.maybe_originally_is_ref();
        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };
        let ffi_type = Resolve::<FFIFullPath<LANG, SPEC>>::resolve(&full_type, source).to_type();
        match maybe_object {
            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object().unwrap() {
                ObjectKind::Type(ref ty_conversion) |
                ObjectKind::Item(ref ty_conversion, ..) => {
                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                    let maybe_special: Option<SpecialType<LANG, SPEC>> = full_parent_ty.maybe_resolve(source);
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
                _ => from_external::<LANG, SPEC>(&full_type, ffi_type, if is_ref { Expression::Clone(field_path.into()) } else { field_path })
            },
            Some(ObjectKind::Item(ty_kind, ..) | ObjectKind::Type(ty_kind)) => {
                let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_special_type(source);
                match maybe_special {
                    Some(SpecialType::Opaque(..)) if search_key.maybe_originally_is_ptr() =>
                        field_path,
                    Some(SpecialType::Opaque(..)) if is_ref =>
                        Expression::boxed(Expression::Clone(field_path.into())),
                    Some(SpecialType::Opaque(..)) =>
                        Expression::boxed(field_path),
                    Some(SpecialType::Custom(custom_ty)) =>
                        Expression::cast_to(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
                    _ => match ty_kind {
                        TypeModelKind::FnPointer(..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                            Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type),
                        TypeModelKind::Optional(TypeModel { ty, .. }) => {
                            let nested_ty = ty.maybe_first_nested_type_kind().unwrap();
                            let maybe_special = <Type as FFISpecialTypeResolve<LANG, SPEC>>::maybe_special_type(&nested_ty.to_type(), source);
                            let (expr_kind, ffi_type) = match maybe_special {
                                Some(SpecialType::Custom(custom_ty)) => {
                                    (ConversionExpressionKind::ComplexOpt, custom_ty)
                                },
                                Some(SpecialType::Opaque(custom_ty)) => {
                                    (ConversionExpressionKind::PrimitiveOpt, custom_ty)
                                },
                                _ => (match &nested_ty {
                                    TypeKind::Primitive(..) =>
                                        ConversionExpressionKind::PrimitiveOpt,
                                    _ =>
                                        ConversionExpressionKind::ComplexOpt,
                                }, ffi_type)
                            };
                            Expression::cast_to(field_path, expr_kind, ffi_type, nested_ty.to_type())
                        }
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                            Expression::cast_to(if is_ref { Expression::Clone(field_path.into()) } else { field_path }, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(i128)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                            Expression::cast_to(if is_ref { Expression::Clone(field_path.into()) } else { field_path }, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(u128)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty, .. }))) =>
                            Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, parse_quote!(&#ty)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ref ty, .. })))) => if let Some(nested_ty) = ty.maybe_first_nested_type_ref() {
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
                                    Expression::cast_to(Expression::deref_expr(field_path), ConversionExpressionKind::Complex, ffi_type, nested_ty.clone())
                            }
                        } else {
                            Expression::ConversionExpr(FFIAspect::To, ConversionExpressionKind::Primitive, field_path.into())
                        },
                        TypeModelKind::Bounds(bounds) => match bounds.bounds.len() {
                            0 => field_path,
                            1 => if let Some(lambda_args) = MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(bounds.bounds.first().unwrap()) {
                                Expression::from_lambda(field_path, lambda_args)
                            } else {
                                Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type.clone())
                            }
                            _ =>
                                Expression::cast_to(field_path, ConversionExpressionKind::Complex, ffi_type, full_type.clone())
                        },
                        _ => from_external::<LANG, SPEC>(&full_type, ffi_type, if is_ref { Expression::Clone(field_path.into()) } else { field_path })
                    }
                }
            }
            _ => from_external::<LANG, SPEC>(&full_type, ffi_type, if is_ref { Expression::Clone(field_path.into()) } else { field_path })
        }
    }
}
fn from_external<LANG, SPEC>(ty: &Type, ffi_ty: Type, field_path: SPEC::Expr) -> SPEC::Expr
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    let (kind, ty) = match TypeKind::from(ty) {
        TypeKind::Primitive(ty) => (ConversionExpressionKind::Primitive, ty),
        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
            Some(TypeKind::Primitive(ty)) => (ConversionExpressionKind::PrimitiveOpt, ty),
            _ => (ConversionExpressionKind::ComplexOpt, ty),
        }
        _ => (ConversionExpressionKind::Complex, ty.clone()),
    };
    Expression::cast_to(field_path, kind, ffi_ty, ty)
}
