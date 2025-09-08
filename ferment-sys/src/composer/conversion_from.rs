use quote::ToTokens;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeContext, ScopeSearch};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, ObjectKind, SmartPointerModelKind, SpecialType, TypeModelKind};
use crate::ext::{Accessory, ExpressionComposable, FFISpecialTypeResolve, GenericNestedArg, MaybeLambdaArgs, Primitive, Resolve, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[derive(Clone)]
pub struct ConversionFromComposer<SPEC>
    where SPEC: Specification {
    pub name: SPEC::Name,
    pub search: ScopeSearch,
    pub field_expr: Option<SPEC::Expr>,
}

impl<SPEC> ConversionFromComposer<SPEC>
    where SPEC: Specification {
    fn new(name: SPEC::Name, search: ScopeSearch, field_expr: Option<SPEC::Expr>) -> Self {
        Self { name, search , field_expr }
    }
    pub fn key_expr_in_composer_scope(name: SPEC::Name, ty: &Type, field_expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_key_in_composer_scope(ty), field_expr)
    }
    pub fn key_ref_expr_in_composer_scope(name: &SPEC::Name, ty: &Type, field_expr: Option<SPEC::Expr>) -> Self {
        Self::key_expr_in_composer_scope(name.clone(), ty, field_expr)
    }
    pub fn value_maybe_expr(name: SPEC::Name, ty: &Type, field_expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::type_ref_value(ty), field_expr)
    }
    pub fn value_ref_maybe_expr(name: &SPEC::Name, ty: &Type, field_expr: Option<SPEC::Expr>) -> Self {
        Self::value_maybe_expr(name.clone(), ty, field_expr)
    }
    pub fn key_in_composer_scope(name: SPEC::Name, ty: &Type) -> Self {
        Self::key_expr_in_composer_scope(name, ty, None)
    }
    pub fn value_expr(name: SPEC::Name, ty: &Type, field_expr: SPEC::Expr) -> Self {
        Self::value_maybe_expr(name, ty, Some(field_expr))
    }
    pub fn value_ref_expr(name: &SPEC::Name, ty: &Type, field_expr: SPEC::Expr) -> Self {
        Self::value_expr(name.clone(), ty, field_expr)
    }
    pub fn value(name: SPEC::Name, ty: &Type) -> Self {
        Self::value_maybe_expr(name, ty, None)
    }
    pub fn value_ref(name: &SPEC::Name, ty: &Type) -> Self {
        Self::value(name.clone(), ty)
    }
}
impl<SPEC> ConversionFromComposer<SPEC> where SPEC: Specification<Name=Name<SPEC>>, SPEC::Name: ToTokens {
    pub fn value_pat_tokens<T: ToTokens>(name: T, ty: &Type) -> Self {
        Self::value_maybe_expr(SPEC::Name::pat_tokens(name), ty, None)
    }

}

impl<SPEC> SourceComposable for ConversionFromComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          FFIFullPath<SPEC>: ToType,
          FFIFullDictionaryPath<SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, field_expr, .. } = self;
        let field_path = field_expr.clone().unwrap_or_else(|| SPEC::Expr::simple(name));
        let search_key = self.search.search_key();
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| search_key.to_type());
        let is_ref = search_key.maybe_originally_is_ref();
        let is_mut_ref = search_key.maybe_originally_is_mut_ref();
        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };
        let ffi_type = Resolve::<FFIFullPath<SPEC>>::resolve(&full_type, source).to_type();
        let type_model_kind = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or_else(|| TypeModelKind::unknown_type(search_key.to_type()));
        if name.to_string().contains("should_continue") {
            println!("ffi_type: {}", ffi_type.to_token_stream());
        }
        let maybe_special: Option<SpecialType<SPEC>> = full_type.maybe_special_type(source);
        let is_opaque = matches!(maybe_special, Some(SpecialType::Opaque(..)));
        let should_leak = is_ref && !is_opaque;
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => match type_model_kind {
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from::<SPEC>(field_path),
                TypeModelKind::FnPointer(..) |
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                _ if search_key.maybe_originally_is_ptr() =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                _ if is_mut_ref =>
                    Expression::deref_mut_ref(field_path),
                _ if is_ref =>
                    Expression::deref_ref(field_path),
                _ =>
                    Expression::from_ptr_read(field_path),
            },
            Some(SpecialType::Custom(custom_ty)) =>
                Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
            _ => match type_model_kind {
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                TypeModelKind::FnPointer(..) => if let Some(lambda_args) = source.maybe_lambda_args::<SPEC>(&full_type) {
                    Expression::from_lambda(field_path, lambda_args)
                } else {
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)
                },
                TypeModelKind::Optional(..) => match full_type.maybe_first_nested_type_kind() {
                    None => Expression::cast_from(field_path, ConversionExpressionKind::from(search_key.to_type()), ffi_type, full_type),
                    Some(full_nested_ty_kind) => {
                        let (expr_kind, ffi_type) = match full_nested_ty_kind.maybe_special_type(source) {
                            Some(SpecialType::Custom(custom_ffi_type)) => (ConversionExpressionKind::ComplexOpt, custom_ffi_type),
                            Some(SpecialType::Opaque(..)) => (ConversionExpressionKind::OpaqueOpt, ffi_type),
                            _ if full_nested_ty_kind.is_primitive() => (ConversionExpressionKind::PrimitiveOpt, ffi_type),
                            _ => (ConversionExpressionKind::ComplexOpt, ffi_type)
                        };
                        Expression::cast_from(field_path, expr_kind, ffi_type, full_nested_ty_kind.to_type())
                    }
                },
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) =>
                    Expression::cast_from::<Type, Type>(field_path, ConversionExpressionKind::Complex, ffi_type, full_ty.joined_ref()),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                    Expression::cast_from::<Type, Type>(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(i128)),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                    Expression::cast_from::<Type, Type>(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(u128)),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                    if let Some(full_nested_ty) = full_ty.maybe_first_nested_type_ref() {
                        match (Resolve::<SpecialType<SPEC>>::maybe_resolve(full_nested_ty, source), source.maybe_object_by_value(full_nested_ty)) {
                            (Some(SpecialType::Opaque(..)), Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                                                 ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::new_box(field_path),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::from_raw_box(field_path),
                            (Some(SpecialType::Custom(custom_ty)), _any) =>
                                Expression::new_box(Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_nested_ty.clone())),
                            (_, Some(obj)) =>
                                Expression::new_box(match MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(&obj) {
                                    Some(lambda_args) =>
                                        Expression::from_lambda(field_path, lambda_args),
                                    None =>
                                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())
                                }),
                            _ =>
                                Expression::new_box(Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())),
                        }
                    } else {
                        Expression::new_box(field_path)
                    }
                },
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ty: ref full_ty, .. }))) => {
                    if let Some(full_nested_ty) = full_ty.maybe_first_nested_type_ref() {
                        match (Resolve::<SpecialType<SPEC>>::maybe_resolve(full_nested_ty, source), source.maybe_object_by_value(full_nested_ty)) {
                            (Some(SpecialType::Opaque(..)), Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                                                 ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                            ) =>
                                Expression::new_cow(field_path),
                            (Some(SpecialType::Opaque(..)), ..) =>
                                Expression::new_cow(Expression::from_ptr_read(field_path)),
                            (Some(SpecialType::Custom(custom_ty)), _any) =>
                                Expression::new_cow(Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_nested_ty.clone())),
                            (_, Some(ObjectKind::Item(TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)), ..) |
                                     ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..))))) =>
                                Expression::new_cow(Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_nested_ty.clone())),

                            (_, Some(obj)) =>
                                Expression::new_cow(match MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(&obj) {
                                    Some(lambda_args) =>
                                        Expression::from_lambda(field_path, lambda_args),
                                    None =>
                                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())
                                }),
                            _ =>
                                Expression::new_cow(Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())),
                        }
                    } else {
                        Expression::new_cow(field_path)
                    }
                },
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from::<SPEC>(field_path),
                TypeModelKind::Slice(TypeModel { ref ty, .. }) => {
                    let maybe_nested_ty = ty.maybe_first_nested_type_ref();
                    Expression::cast_from::<Type, Type>(field_path, ConversionExpressionKind::from(search_key.to_type()), ffi_type, parse_quote!(Vec<#maybe_nested_ty>))
                },
                _ =>
                    Expression::cast_from(field_path, ConversionExpressionKind::from(search_key.to_type()), ffi_type, full_type)
            }
        };
        if should_leak {
            SPEC::Expr::leak_box(expression)
        } else {
            SPEC::Expr::simple_expr(expression)
        }
    }
}
