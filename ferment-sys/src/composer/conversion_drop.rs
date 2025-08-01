use std::fmt::Debug;
use quote::quote;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, SpecialType, TypeKind, TypeModelKind};
use crate::ext::{FFISpecialTypeResolve, GenericNestedArg, Primitive, Resolve, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionExpressionKind, Expression, ExpressionComposable, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};


#[derive(Clone, Debug)]
pub struct ConversionDropComposer<'a, SPEC>
where SPEC: Specification {
    pub name: SPEC::Name,
    pub search: ScopeSearch<'a>,
    pub expr: Option<SPEC::Expr>,
}
impl<'a, SPEC> ConversionDropComposer<'a, SPEC>
where SPEC: Specification {
    fn new(name: SPEC::Name, search: ScopeSearch<'a>, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search, expr }
    }

    pub fn key_expr(name: SPEC::Name, ty: &'a Type, scope: &'a ScopeChain, expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope), expr)
    }

    #[allow(unused)]
    pub fn value(name: SPEC::Name, ty: &'a Type) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), None)
    }
    #[allow(unused)]
    pub fn value_expr(name: SPEC::Name, ty: &'a Type, expr: SPEC::Expr) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), Some(expr))
    }

}
impl<'a, SPEC> SourceComposable for ConversionDropComposer<'a, SPEC>
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
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or_else(|| TypeModelKind::unknown_type(search_key.to_type()));
        let maybe_special: Option<SpecialType<SPEC>> = full_type.maybe_special_type(source);
        let expression = match maybe_special {
            Some(special) =>
                Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, special, full_type)),
            _ => {
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        None,
                    TypeModelKind::FnPointer(..) =>
                        source.maybe_lambda_args::<SPEC>(&full_type)
                            .map(|_| SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)),
                    TypeModelKind::Optional(..) =>
                        full_type.maybe_first_nested_type()
                            .map(|target_ty| SPEC::Expr::cast_destroy(field_path, full_type.is_primitive().then_some(ConversionExpressionKind::PrimitiveOpt).unwrap_or(ConversionExpressionKind::ComplexOpt), ffi_type, target_ty)),
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) => {
                        Some(SPEC::Expr::destroy_string(field_path, quote!(&#full_ty)))
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::String(TypeModel { ty: ref full_ty, .. }))) => {
                        Some(SPEC::Expr::destroy_string(field_path, full_ty))
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) => {
                        Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(i128)))
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) => {
                        Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(u128)))
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_ty.maybe_first_nested_type().unwrap()))
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ty: ref full_ty, .. }))) if full_ty.maybe_first_nested_type_ref().unwrap().is_primitive() => {
                        None
                    },
                    TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ty: ref full_ty, .. }))) => {
                        Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_ty.maybe_first_nested_type().unwrap()))
                    },
                    TypeModelKind::Bounds(..) => {
                        Some(SPEC::Expr::destroy_complex(field_path))
                    },
                    TypeModelKind::Slice(TypeModel { ref ty, .. }) => {
                        let maybe_nested_ty = ty.maybe_first_nested_type_ref();
                        destroy_other::<SPEC>(&search_key.to_type(), ffi_type, parse_quote!(Vec<#maybe_nested_ty>), field_path)
                    },
                    _ => {
                        destroy_other::<SPEC>(&search_key.to_type(), ffi_type, full_type, field_path)
                    }
                }
            }
        };
        expression
    }
}


fn destroy_other<SPEC>(ty: &Type, ffi_type: Type, target_ty: Type, field_path: SPEC::Expr) -> Option<SPEC::Expr>
where SPEC: Specification<Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    match TypeKind::from(ty) {
        TypeKind::Primitive(_) =>
            None,
        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
            Some(TypeKind::Primitive(_)) =>
                Some(Expression::cast_destroy(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, target_ty)),
            _ =>
                Some(Expression::cast_destroy(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, target_ty)),
        }
        TypeKind::Generic(GenericTypeKind::Cow(ty)) => match ty.maybe_first_nested_type_kind() {
            Some(TypeKind::Primitive(_)) =>
                None,
            _ =>
                Some(Expression::cast_destroy(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, target_ty)),
        }
        _ =>
            Some(Expression::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, target_ty))
    }
}
