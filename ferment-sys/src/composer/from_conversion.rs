use quote::ToTokens;
use syn::{parse_quote, PatType, Type, TypeReference};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, MaybeLambdaArgs, Primitive, Resolve, SpecialType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ConversionExpressionKind, Expression, ExpressionComposable, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};



#[allow(unused)]
#[derive(Clone)]
pub struct FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub name: SPEC::Name,
    pub search: ScopeSearch<'a>,
    pub field_expr: Option<SPEC::Expr>,
}

impl<'a, LANG, SPEC> FromConversionFullComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
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
    pub fn value_expr(name: SPEC::Name, ty: &'a Type, field_expr: SPEC::Expr) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), Some(field_expr))
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
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, field_expr: expr, .. } = self;
        let field_path = expr.clone().unwrap_or(SPEC::Expr::simple(name));

        let search_key = self.search.search_key();
        println!("FromConversionFullComposer::search_key {} ", search_key);
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
        // let ffi_type = full_type.mangle_tokens_default().to_type();

        println!("FromConversionFullComposer::maybe_object {} ", maybe_object.as_ref().map_or("None".to_string(), ObjectKind::to_string));
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type(search_key.to_type()));

        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_special_type(source);
        println!("FromConversionFullComposer::maybe_special {} ", maybe_special.as_ref().map_or("None".to_string(), SpecialType::to_string));
        let expression = match maybe_special {
            Some(SpecialType::Opaque(..)) => match composition {
                TypeModelKind::Bounds(bounds) =>
                    bounds.expr_from(field_path),
                TypeModelKind::FnPointer(..) |
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                _ if search_key.maybe_originally_is_ptr() =>
                    Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                _ =>
                    Expression::from_ptr_clone(field_path),
            },
            Some(SpecialType::Custom(custom_ty)) =>
                Expression::cast_from(field_path, ConversionExpressionKind::Complex, custom_ty, full_type),
            _ => {
                // println!("FromConversionFullComposer (Non Special): {} ({})", search_key, full_type.to_token_stream());
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) =>
                        Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                    TypeModelKind::FnPointer(..) => {
                        // println!("FromConversionFullComposer (Non Special FnPointer): {} --- {}", search_key, maybe_object.to_token_stream());
                        if let Some(lambda_args) = source.maybe_fn_sig(&full_type)
                            .and_then(|ty| MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(&ty)) {
                            Expression::from_lambda(field_path, lambda_args)
                        } else {
                            Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)
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
                                Expression::new_box(match MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(&obj) {
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
                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, full_type),
                                _ =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_type),
                            }
                            TypeKind::Generic(..) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_type),
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)
                        }
                    },
                    TypeModelKind::Slice(TypeModel { ref ty, .. }) => {
                        let maybe_nested_ty = ty.maybe_first_nested_type_ref();
                        let target_type = parse_quote!(Vec<#maybe_nested_ty>);
                        /*Expression::AsRef(*/match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, target_type),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, target_type),
                                _ =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, target_type),
                            }
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, target_type)
                        }/*.into())*/
                    },
                    _ => {
                        match TypeKind::from(search_key.to_type()) {
                            TypeKind::Primitive(_) =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type),
                            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, full_type),
                                _ =>
                                    Expression::cast_from(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_type),
                            }
                            _ =>
                                Expression::cast_from(field_path, ConversionExpressionKind::Complex, ffi_type, full_type)
                        }
                    }
                }
            }
        };
        println!("FromConversionFullComposer ==> {:?}", expression);
        if is_ref {
            Expression::LeakBox(expression.into())
        } else {
            expression
        }
        // expression
    }
}

#[derive(Clone)]
pub struct FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub name: SPEC::Name,
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
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone(), expr: None }
    }
}

#[allow(unused)]
impl<LANG, SPEC> FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(name: SPEC::Name , ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}
impl<LANG, SPEC> SourceComposable for FromConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
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
                        .and_then(|bare| MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(&bare)) {
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
                            Expression::new_box(match MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(&obj) {
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