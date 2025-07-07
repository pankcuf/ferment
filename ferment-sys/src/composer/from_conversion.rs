use syn::{parse_quote, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{FFISpecialTypeResolve, GenericNestedArg, MaybeLambdaArgs, Resolve, SpecialType, ToType};
use crate::lang::Specification;
use crate::presentable::{ConversionExpressionKind, Expression, ExpressionComposable, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

#[derive(Clone)]
pub struct FromConversionFullComposer<'a, SPEC>
    where SPEC: Specification {
    pub name: SPEC::Name,
    pub search: ScopeSearch<'a>,
    pub field_expr: Option<SPEC::Expr>,
}

impl<'a, SPEC> FromConversionFullComposer<'a, SPEC>
    where SPEC: Specification {
    fn new(name: SPEC::Name, search: ScopeSearch<'a>, field_expr: Option<SPEC::Expr>) -> Self {
        Self { name, search , field_expr }
    }
    pub fn expr_less(name: SPEC::Name, search: ScopeSearch<'a>) -> Self {
        Self::new(name, search, None)
    }
    pub fn key_in_scope_with_expr(name: SPEC::Name, ty: &'a Type, scope: &'a ScopeChain, field_expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope), field_expr)
    }
    pub fn key_in_scope(name: SPEC::Name, ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::expr_less(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }
    pub fn value(name: SPEC::Name, ty: &'a Type) -> Self {
        Self::expr_less(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
    #[allow(unused)]
    pub fn value_expr(name: SPEC::Name, ty: &'a Type, field_expr: SPEC::Expr) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), Some(field_expr))
    }
}

impl<'a, SPEC> SourceComposable for FromConversionFullComposer<'a, SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          FFIFullPath<SPEC>: ToType,
          FFIFullDictionaryPath<SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, field_expr: expr, .. } = self;
        let field_path = expr.clone().unwrap_or_else(|| SPEC::Expr::simple(name));
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
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or_else(|| TypeModelKind::unknown_type(search_key.to_type()));

        let maybe_special: Option<SpecialType<SPEC>> = full_type.maybe_special_type(source);
        let mut wrap_to_box = is_ref;
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => {
                wrap_to_box = false;
                match composition {
                    TypeModelKind::Bounds(bounds) =>
                        bounds.expr_from(field_path),
                    TypeModelKind::FnPointer(..) |
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                    _ if search_key.maybe_originally_is_ptr() =>
                        Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                    _ if is_mut_ref =>
                        Expression::DerefMutRef(field_path.into()),
                    _ if is_ref =>
                        Expression::DerefRef(field_path.into()),
                    _ =>
                        Expression::from_ptr_clone(field_path),
                }
            },
            Some(SpecialType::Custom(custom_ty)) =>
                Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
            _ => {
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                    TypeModelKind::FnPointer(..) => {
                        if let Some(lambda_args) = source.maybe_fn_sig(&full_type)
                            .and_then(|ty| MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(&ty)) {
                            Expression::from_lambda(field_path, lambda_args)
                        } else {
                            Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)
                        }
                    },
                    TypeModelKind::Optional(..) => {
                        let full_nested_ty = full_type.maybe_first_nested_type_kind().unwrap();
                        let maybe_special = <Type as FFISpecialTypeResolve<SPEC>>::maybe_special_type(&full_nested_ty.to_type(), source);
                        let (expr_kind, ffi_type, target_type) = match maybe_special {
                            Some(SpecialType::Custom(ty)) => {
                                (ConversionExpressionKind::ComplexOpt, ty, full_nested_ty.to_type())
                            },
                            Some(SpecialType::Opaque(..)) => {
                                wrap_to_box = true;
                                (ConversionExpressionKind::ComplexOpt, ffi_type, full_nested_ty.to_type())
                            },
                            _ => match full_nested_ty {
                                TypeKind::Primitive(ty) =>
                                    (ConversionExpressionKind::PrimitiveOpt, ffi_type, ty.clone()),
                                _ =>
                                    (ConversionExpressionKind::ComplexOpt, ffi_type, full_nested_ty.to_type())
                            }
                        };
                        Expression::cast_from(field_path, expr_kind, ffi_type, target_type)
                    }
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, parse_quote!(&#full_ty))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(i128))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(u128))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(
                            DictFermentableModelKind::SmartPointer(
                                SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let full_nested_ty = full_ty.maybe_first_nested_type_ref().unwrap();
                        let maybe_special: Option<SpecialType<SPEC>> = full_nested_ty.maybe_resolve(source);
                        let maybe_object = source.maybe_object_by_value(full_nested_ty);
                        match (maybe_special, maybe_object) {
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
                                Expression::new_box(
                                    Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())),
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        bounds.expr_from(field_path)
                    },
                    TypeModelKind::Unknown(..) => {
                        let expr_kind = match TypeKind::from(search_key.to_type()) {
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    ConversionExpressionKind::PrimitiveOpt,
                                _ =>
                                    ConversionExpressionKind::ComplexOpt
                            }
                            TypeKind::Generic(..) =>
                                ConversionExpressionKind::Complex,
                            _ =>
                                ConversionExpressionKind::Primitive
                        };
                        Expression::cast_from(field_path, expr_kind, ffi_type, full_type)
                    },
                    TypeModelKind::Slice(TypeModel { ref ty, .. }) => {
                        let maybe_nested_ty = ty.maybe_first_nested_type_ref();
                        let target_type = parse_quote!(Vec<#maybe_nested_ty>);
                        let expr_kind = match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                ConversionExpressionKind::Primitive,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    ConversionExpressionKind::PrimitiveOpt,
                                _ =>
                                    ConversionExpressionKind::ComplexOpt,
                            }
                            _ =>
                                ConversionExpressionKind::Complex
                        };
                        Expression::cast_from(field_path, expr_kind, ffi_type, target_type)
                    },
                    _ => {
                        let expr_kind = match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                ConversionExpressionKind::Primitive,
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    ConversionExpressionKind::PrimitiveOpt,
                                _ =>
                                    ConversionExpressionKind::ComplexOpt,
                            }
                            _ =>
                                ConversionExpressionKind::Complex,
                        };
                        Expression::cast_from(field_path, expr_kind, ffi_type, full_type)
                    }
                }
            }
        };
        if wrap_to_box {
            Expression::LeakBox(expression.into())
        } else {
            expression
        }
    }
}
