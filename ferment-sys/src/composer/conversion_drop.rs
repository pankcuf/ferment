use std::fmt::Debug;
use quote::quote;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeContext, ScopeSearch};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{ExpressionComposable, FFISpecialTypeResolve, GenericNestedArg, Primitive, Resolve, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};


#[derive(Clone, Debug)]
pub struct ConversionDropComposer<SPEC>
where SPEC: Specification {
    pub name: SPEC::Name,
    pub search: ScopeSearch,
    pub expr: Option<SPEC::Expr>,
}
impl<SPEC> ConversionDropComposer<SPEC>
where SPEC: Specification {
    fn new(name: SPEC::Name, search: ScopeSearch, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search, expr }
    }
    #[allow(unused)]
    pub fn key_expr_in_composer_scope(name: SPEC::Name, ty: &Type, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_key_in_composer_scope(ty), expr)
    }
    fn value_expr_in_composer_scope(name: SPEC::Name, ty: &Type, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_value(ty), expr)
    }

    #[allow(unused)]
    pub fn value(name: SPEC::Name, ty: &Type) -> Self {
        Self::value_expr_in_composer_scope(name, ty, None)
    }
    #[allow(unused)]
    pub fn value_expr(name: SPEC::Name, ty: &Type, expr: SPEC::Expr) -> Self {
        Self::value_expr_in_composer_scope(name, ty, Some(expr))
    }

}
impl<SPEC> SourceComposable for ConversionDropComposer<SPEC>
where SPEC: Specification<Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType {
    type Source = ScopeContext;
    type Output = Option<SPEC::Expr>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        let field_path = expr.clone().unwrap_or_else(|| SPEC::Expr::simple(name));
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| search_key.to_type());
        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };
        let ffi_type = Resolve::<FFIFullPath<SPEC>>::resolve(&full_type, source).to_type();
        match FFISpecialTypeResolve::<SPEC>::maybe_special_type(&full_type, source) {
            Some(special) =>
                Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, special, full_type)),
            _ => match maybe_object
                .and_then(|kind| kind.maybe_trait_or_same_kind(source))
                .unwrap_or_else(|| TypeModelKind::unknown_type(search_key.to_type())) {
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    None,
                TypeModelKind::FnPointer(..) =>
                    source.maybe_lambda_args::<SPEC>(&full_type)
                        .map(|_| SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)),
                TypeModelKind::Optional(..) =>
                    full_type.maybe_first_nested_type()
                        .map(|target_ty| SPEC::Expr::cast_destroy(field_path, full_type.is_primitive().then_some(ConversionExpressionKind::PrimitiveOpt).unwrap_or(ConversionExpressionKind::ComplexOpt), ffi_type, target_ty)),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) =>
                    Some(SPEC::Expr::destroy_string(field_path, quote!(&#full_ty))),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::String(TypeModel { ty: ref full_ty, .. }))) =>
                    Some(SPEC::Expr::destroy_string(field_path, full_ty)),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                    Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(i128))),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                    Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(u128))),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) =>
                    full_ty.maybe_first_nested_type()
                        .map(|first_nested_ty| SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, first_nested_ty)),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ty: ref full_ty, .. }))) => if let Some(first_nested_type_ref) = full_ty.maybe_first_nested_type_ref() {
                    (!first_nested_type_ref.is_primitive())
                        .then(|| SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, first_nested_type_ref.clone()))
                } else {
                    None
                },
                TypeModelKind::Bounds(..) =>
                    Some(SPEC::Expr::destroy_complex(field_path)),
                TypeModelKind::Slice(TypeModel { ref ty, .. }) =>
                    ty.maybe_first_nested_type_ref()
                        .and_then(|first_nested_ty| destroy_other::<SPEC, _>(search_key, ffi_type, parse_quote!(Vec<#first_nested_ty>), field_path)),
                _ =>
                    destroy_other::<SPEC, _>(search_key, ffi_type, full_type, field_path)
            }
        }
    }
}


fn destroy_other<SPEC, T: ToType>(ty: &T, ffi_type: Type, target_ty: Type, field_path: SPEC::Expr) -> Option<SPEC::Expr>
where SPEC: Specification<Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    match TypeKind::from(ty.to_type()) {
        TypeKind::Primitive(_) =>
            None,
        TypeKind::Generic(GenericTypeKind::Optional(ty) | GenericTypeKind::Cow(ty)) => Some(match ty.maybe_first_nested_type_kind() {
            Some(TypeKind::Primitive(_)) => ConversionExpressionKind::PrimitiveOpt,
            _ => ConversionExpressionKind::ComplexOpt,
        }),
        _ =>
            Some(ConversionExpressionKind::Complex)
    }.map(|kind| Expression::cast_destroy(field_path, kind, ffi_type, target_ty))
}
