use std::fmt::Debug;
use quote::ToTokens;
use syn::{parse_quote, PatType, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictTypeModelKind, GenericTypeKind, ObjectKind, TypeModelKind, TypeKind, DictFermentableModelKind, SmartPointerModelKind};
use crate::ext::{FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, Primitive, Resolve, SpecialType, AsType, ToType, MaybeLambdaArgs};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[allow(unused)]
#[derive(Clone)]
pub struct FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name<LANG, SPEC> ,
    pub search: ScopeSearch<'a>,
    pub field_expr: Option<SPEC::Expr>,
}

impl<'a, LANG, SPEC> FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn new(name: Name<LANG, SPEC> , search: ScopeSearch<'a>, field_expr: Option<SPEC::Expr>) -> Self {
        Self { name, search , field_expr }
    }
    pub fn expr_less(name: Name<LANG, SPEC> , search: ScopeSearch<'a>) -> Self {
        Self::new(name, search, None)
    }
    pub fn key_in_scope_with_expr(name: Name<LANG, SPEC> , ty: &'a Type, scope: &'a ScopeChain, field_expr: Option<SPEC::Expr>) -> Self {
        Self::new(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope), field_expr)
    }
    pub fn key_in_scope(name: Name<LANG, SPEC> , ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::expr_less(name, ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }
}
// impl<'a, LANG, SPEC> Display for FromConversionFullComposer<'a, LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("{} --- {} --- {}", self.name, self.search, self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
//     }
// }

impl<'a, LANG, SPEC> SourceComposable for FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, field_expr: expr, .. } = self;
        let search_key = self.search.search_key();
        // println!("FromConversionFullComposer:: {}({}) -- {}", name,  name.to_token_stream(), search);

        let field_path = expr.clone().unwrap_or(SPEC::Expr::Simple(name.to_token_stream()));
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(search_key.to_type());

        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };

        let ffi_type = <Type as Resolve::<FFIFullPath<LANG, SPEC>>>::resolve(&full_type, source).to_type();
        // let ffi_type = full_type.mangle_tokens_default().to_type();

        // println!("FromConversionFullComposer::maybe_object {} ", maybe_object.as_ref().map_or("None".to_string(), ObjectKind::to_string));
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type(search_key.to_type()));

        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_special_type(source);
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => match composition {
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from(field_path),
                TypeModelKind::FnPointer(..) |
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                // Expression::from_primitive(field_path),
                _ if search_key.maybe_originally_is_ptr() =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                // Expression::from_primitive(field_path),
                _ =>
                    Expression::from_ptr_clone(field_path),
            },
            Some(SpecialType::Custom(custom_ty)) =>
                Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
                // Expression::from_complex(field_path),
            _ => {
                // println!("FromConversionFullComposer (Non Special): {} ({})", search_key, full_type.to_token_stream());
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                    // Expression::from_primitive(field_path),
                    TypeModelKind::FnPointer(..) => {
                        // println!("FromConversionFullComposer (Non Special FnPointer): {} --- {}", search_key, maybe_object.to_token_stream());
                        if let Some(lambda_args) = source.maybe_fn_sig(&full_type)
                            .and_then(|bare| bare.maybe_lambda_arg_names()) {
                            Expression::from_lambda(field_path, lambda_args)
                        } else {
                            Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)
                            // Expression::from_primitive(field_path)
                        }
                    },
                    TypeModelKind::Optional(..) => {
                        let full_nested_ty = full_type.maybe_first_nested_type_kind().unwrap();
                        match full_nested_ty {
                            TypeKind::Primitive(ty) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, ty),
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_nested_ty.to_type())
                        }
                        // let first_nested_type = full_type.maybe_first_nested_type_kind().unwrap();
                        // if let Some(first_nested_type) = full_type.maybe_first_nested_type_ref() {
                        //
                        // } else {
                        //
                        // }
                        //
                        // if ty.as_type().maybe_first_nested_type_ref().unwrap().is_primitive() {
                        //     Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, full_type.maybe_first_nested_type_ref().cloned().unwrap())
                        //     // Expression::from_primitive_opt(field_path)
                        // } else {
                        //     Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_type.maybe_first_nested_type_ref().cloned().unwrap())
                        //     // Expression::from_complex_opt(field_path)
                        // }
                    }
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, parse_quote!(&#full_ty))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(TypeModel { ty: ref full_ty, .. }))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(i128))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(TypeModel { ty: ref full_ty, .. }))) => {
                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, parse_quote!([u8; 16]), parse_quote!(u128))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(
                            DictFermentableModelKind::SmartPointer(
                                SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let full_nested_ty = full_ty.maybe_first_nested_type_ref().unwrap();
                        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_nested_ty.maybe_resolve(source);
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
                                Expression::new_box(match obj.maybe_lambda_arg_names() {
                                    Some(lambda_args) =>
                                        Expression::from_lambda(field_path, lambda_args),
                                    None =>
                                        Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())

                                    // Expression::from_complex(field_path)
                                }),
                            _ =>
                                Expression::new_box(
                                    Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone())),

                                    // Expression::from_complex(field_path)),
                        }
                    },
                    TypeModelKind::Bounds(bounds) => {
                        // println!("FromConversionFullComposer (Bounds): {}", bounds);
                        bounds.expr_from(field_path)
                    },
                    TypeModelKind::Unknown(..) => {
                        // println!("FromConversionFullComposer (Unknown): {}", search_key);

                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, full_type),

                                // Expression::from_primitive_opt(field_path),
                                _ =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_type),

                                // Expression::from_complex_opt(field_path),
                            }
                            TypeKind::Generic(..) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_type),

                            // Expression::from_complex(field_path),
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)

                            // Expression::from_primitive(field_path),
                        }
                    },
                    _ => {
                        // println!("FromConversionFullComposer (Regular): {}", composition);
                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),

                            // Expression::from_primitive(field_path),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, full_type),

                                // Expression::from_primitive_opt(field_path),
                                _ =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_type),

                                // Expression::from_complex_opt(field_path),
                            }
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)

                            // Expression::from_complex(field_path)
                        }
                    }
                }
            }
        };
        // println!("FromConversionFullComposer ==> {:?}", expression);
        expression
    }
}

#[derive(Clone)]
pub struct FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name<LANG, SPEC> ,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
}

// impl<LANG, SPEC> Display for FromConversionComposer<LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug  {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("{} --- {} --- {}", self.name, self.ty.to_token_stream(), self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
//     }
// }

#[allow(unused)]
impl<LANG, SPEC> From<&PatType> for FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone(), expr: None }
    }
}

#[allow(unused)]
impl<LANG, SPEC> FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name<LANG, SPEC> , ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}
impl<LANG, SPEC> SourceComposable for FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG,
              Attr: Debug,
              Expr=Expression<LANG, SPEC>,
              Name=Name<LANG, SPEC>,
              Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, ty, expr, .. } = self;
        let field_path = expr.clone()
            .unwrap_or(Expression::Simple(name.to_token_stream()));
        let full_type = ty.full_type(source);
        let maybe_object: Option<ObjectKind> = ty.maybe_resolve(source);
        let ty_model_kind = maybe_object
            .as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type_ref(ty));

        let maybe_special = <Type as FFISpecialTypeResolve<LANG, SPEC>>::maybe_special_type(&full_type, source);
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => match ty_model_kind {
                TypeModelKind::FnPointer(..) |
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    field_path,
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from(field_path),
                _ => Expression::deref_expr(field_path)
            },
            Some(SpecialType::Custom(..)) =>
                Expression::from_complex(field_path),
            _ => match ty_model_kind {
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    Expression::from_primitive(field_path),
                TypeModelKind::FnPointer(..) => {
                    if let Some(lambda_args) = source.maybe_fn_sig(&full_type)
                        .and_then(|bare| bare.maybe_lambda_arg_names()) {
                        Expression::from_lambda(field_path, lambda_args)
                    } else {
                        Expression::from_primitive(field_path)
                    }
                },
                TypeModelKind::Optional(ty) => if ty.as_type().maybe_first_nested_type_ref().unwrap().is_primitive() {
                    Expression::from_primitive_opt(field_path)
                } else {
                    Expression::from_complex_opt(field_path)
                }
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                    let nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                    let full_nested_ty = full_ty.maybe_first_nested_type_ref().unwrap();
                    let maybe_nested_special: Option<SpecialType<LANG, SPEC>> = full_nested_ty.maybe_resolve(source);
                    let maybe_nested_object = nested_ty.maybe_object(source);
                    match (maybe_nested_special, maybe_nested_object) {
                        (Some(SpecialType::Opaque(..)),
                            Some(ObjectKind::Item(TypeModelKind::FnPointer(..) |
                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..) |
                                 ObjectKind::Type(TypeModelKind::FnPointer(..) |
                                                  TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..))))
                        ) =>
                            Expression::new_box(field_path),
                        (Some(SpecialType::Opaque(..)), _any_other) =>
                            Expression::from_raw_box(field_path),
                        (Some(SpecialType::Custom(..)), _any) =>
                            Expression::new_box(Expression::from_complex(field_path)),
                        (_, Some(obj)) => {
                            Expression::new_box(match obj.maybe_lambda_arg_names() {
                                Some(lambda_args) =>
                                    Expression::from_lambda(field_path, lambda_args),
                                None =>
                                    Expression::from_complex(field_path)
                            })
                        }
                        _ =>
                            Expression::new_box(Expression::from_complex(field_path))
                    }
                },
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from(field_path),
                TypeModelKind::Unknown(..) => {
                    match TypeKind::from(ty) {
                        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind().unwrap() {
                            TypeKind::Primitive(_) =>
                                Expression::from_primitive_opt(field_path),
                            _ =>
                                Expression::from_complex_opt(field_path),
                        }
                        TypeKind::Generic(..) =>
                            Expression::from_complex(field_path),
                        _ =>
                            Expression::from_primitive(field_path),
                    }
                },
                _ => {
                    match TypeKind::from(ty) {
                        TypeKind::Primitive(_) =>
                            Expression::from_primitive(field_path),
                        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind().unwrap() {
                            TypeKind::Primitive(_) =>
                                Expression::from_primitive_opt(field_path),
                            _ =>
                                Expression::from_complex_opt(field_path),
                        }
                        _ =>
                            Expression::from_complex(field_path)
                    }
                }
            }
        };

        expression
    }
}