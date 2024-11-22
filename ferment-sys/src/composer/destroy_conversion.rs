use std::fmt::Debug;
use quote::quote;
use syn::{parse_quote, Type, TypeReference};
use crate::composable::{FieldComposer, TypeModel};
use crate::composer::SourceComposable;
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{path_arguments_to_type_conversions, DictionaryType, FFISpecialTypeResolve, GenericNestedArg, MaybeLambdaArgs, Resolve, SpecialType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{ConversionExpressionKind, Expression, ExpressionComposable, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};


#[derive(Clone, Debug)]
pub struct DestroyFullConversionComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub name: SPEC::Name,
    pub search: ScopeSearch<'a>,
    pub expr: Option<SPEC::Expr>,
}
impl<'a, LANG, SPEC> DestroyFullConversionComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub fn new(name: SPEC::Name, search: ScopeSearch<'a>, expr: Option<SPEC::Expr>) -> Self {
        Self { name, search, expr }
    }

    pub fn value_expr(name: SPEC::Name, ty: &'a Type, expr: SPEC::Expr) -> Self {
        Self::new(name, ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()), Some(expr))
    }

}
impl<'a, LANG, SPEC> SourceComposable for DestroyFullConversionComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      FFIFullPath<LANG, SPEC>: ToType,
      FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = Option<SPEC::Expr>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, search, expr, .. } = self;
        let search_key = self.search.search_key();
        // println!("FromConversionFullComposer:: {}({}) -- {}", name,  name.to_token_stream(), search);

        let field_path = expr.clone().unwrap_or(SPEC::Expr::simple(name));
        let maybe_object = source.maybe_object_by_predicate_ref(search);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(search_key.to_type());
        // let is_ref = search_key.maybe_originally_is_ref();
        let full_type = match &full_type {
            Type::Reference(TypeReference { elem, .. }) => *elem.clone(),
            _ => full_type
        };
        let ffi_type = Resolve::<FFIFullPath<LANG, SPEC>>::resolve(&full_type, source).to_type();
        let composition = maybe_object.as_ref()
            .and_then(|kind| kind.maybe_trait_or_same_kind(source))
            .unwrap_or(TypeModelKind::unknown_type(search_key.to_type()));
        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_special_type(source);
        let expression = match maybe_special {
            Some(special) =>
                Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, special, full_type)),
            _ => {
                match composition {
                    TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => None,
                        // Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Primitive, ffi_type, full_type)),
                    TypeModelKind::FnPointer(..) => {
                        if let Some(..) = source.maybe_fn_sig(&full_type)
                            .and_then(|ty| MaybeLambdaArgs::<LANG, SPEC>::maybe_lambda_arg_names(&ty)) {
                            Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_type))
                        } else {
                            None
                        }
                    },
                    TypeModelKind::Optional(..) => {
                        let full_nested_ty = full_type.maybe_first_nested_type_kind()?;
                        match full_nested_ty {
                            TypeKind::Primitive(ty) =>
                                Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, ty)),
                            _ =>
                                Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, full_nested_ty.to_type()))
                        }
                    }
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel { ty: ref full_ty, .. }))) => {
                        let ty: Type = parse_quote!(&#full_ty);
                        Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, ty))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) => {
                        //TODO: Check if u8 here is correct
                        Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(i128)))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) => {
                        Some(SPEC::Expr::destroy_big_int(field_path, quote!([u8; 16]), quote!(u128)))
                    },
                    TypeModelKind::Dictionary(
                        DictTypeModelKind::NonPrimitiveFermentable(
                            DictFermentableModelKind::SmartPointer(
                                SmartPointerModelKind::Box(TypeModel { ty: ref full_ty, .. })))) => {
                        let full_nested_ty = full_ty.maybe_first_nested_type_ref().unwrap();
                        Some(SPEC::Expr::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, full_nested_ty.clone()))
                    },
                    TypeModelKind::Bounds(..) => {
                        Some(SPEC::Expr::destroy_complex(field_path))
                    },
                    TypeModelKind::Unknown(..) => {
                        destroy_other::<LANG, SPEC>(&search_key.to_type(), ffi_type, full_type, field_path)
                    },
                    TypeModelKind::Slice(TypeModel { ref ty, .. }) => {
                        let maybe_nested_ty = ty.maybe_first_nested_type_ref();
                        destroy_other::<LANG, SPEC>(&search_key.to_type(), ffi_type, parse_quote!(Vec<#maybe_nested_ty>), field_path)
                    },
                    _ => {
                        destroy_other::<LANG, SPEC>(&search_key.to_type(), ffi_type, full_type, field_path)
                    }
                }
            }
        };
        expression
    }
}


fn destroy_other<LANG, SPEC>(ty: &Type, ffi_type: Type, target_ty: Type, field_path: SPEC::Expr) -> Option<SPEC::Expr>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    match TypeKind::from(ty) {
        TypeKind::Primitive(_) =>
            None,
            // Some(Expression::cast_destroy(field_path, ConversionExpressionKind::Primitive, ffi_type, target_ty)),
        TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
            Some(TypeKind::Primitive(_)) =>
                Some(Expression::cast_destroy(field_path, ConversionExpressionKind::PrimitiveOpt, ffi_type, target_ty)),
            _ =>
                Some(Expression::cast_destroy(field_path, ConversionExpressionKind::ComplexOpt, ffi_type, target_ty)),
        }
        _ =>
            Some(Expression::cast_destroy(field_path, ConversionExpressionKind::Complex, ffi_type, target_ty))
    }
}

#[derive(Clone, Debug)]
pub struct DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub name: SPEC::Name,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
}
impl<LANG, SPEC> From<&FieldComposer<LANG, SPEC>> for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn from(value: &FieldComposer<LANG, SPEC>) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
    }
}
impl<LANG, SPEC> DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(name: SPEC::Name, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}

impl<LANG, SPEC> SourceComposable for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { name, ty, expr } = self;
        let maybe_object = source.maybe_object_by_key(ty);
        let full_type = maybe_object
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(ty.to_type());
        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_type.maybe_resolve(source);
        let ffi_type = Resolve::<FFIFullPath<LANG, SPEC>>::maybe_resolve(&full_type, source)
            .map_or(full_type.clone(), |full_path| full_path.to_type());
        let expr = expr.clone().unwrap_or(SPEC::Expr::name(name));
        match maybe_special {
            Some(SpecialType::Opaque(..) | SpecialType::Custom(..)) => {
                SPEC::Expr::destroy_complex(expr)
            }
            _ => {
                match &full_type {
                    Type::Path(type_path) => {
                        let last_segment = type_path.path.segments.last().unwrap();
                        let last_ident = &last_segment.ident;
                        if last_ident.is_primitive() {
                            SPEC::Expr::empty()
                        } else if matches!(last_ident.to_string().as_str(), "i128") {
                            SPEC::Expr::destroy_big_int(expr, quote!([u8; 16]), quote!(i128))
                        } else if matches!(last_ident.to_string().as_str(), "u128") {
                            SPEC::Expr::destroy_big_int(expr, quote!([u8; 16]), quote!(u128))
                        } else if last_ident.is_optional() {
                            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                                Some(TypeKind::Primitive(_)) =>
                                    Expression::empty(),
                                Some(kind) =>
                                    SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, ffi_type, kind.to_type()),
                                None =>
                                    unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                            }
                        } else if last_ident.is_string() {
                            SPEC::Expr::destroy_string(expr, &type_path.path)
                        } else if last_ident.is_str() {
                            SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                        } else {
                            SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::Complex, ffi_type, full_type)
                        }
                    }
                    Type::Ptr(ty) => match &*ty.elem {
                        Type::Path(type_path) => {
                            let last_segment = type_path.path.segments.last().unwrap();
                            let last_ident = &last_segment.ident;
                            if last_ident.is_primitive() {
                                SPEC::Expr::empty()
                            } else if last_ident.is_optional() {
                                match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                                    Some(TypeKind::Primitive(_)) => SPEC::Expr::empty(),
                                    Some(kind) => SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, ffi_type, kind.to_type()),


                                    // Expression::destroy_complex_opt(expr),
                                    None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                                }
                            } else if last_ident.is_string() {
                                SPEC::Expr::destroy_string(expr, &type_path.path)
                            } else if last_ident.is_str() {
                                SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                            } else {
                                SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::Complex, ffi_type, full_type)
                                //
                                // SPEC::Expr::destroy_complex(expr)
                            }

                        },
                        _ => panic!("Can't destroy_ptr: of type: {}", quote!(#ty)),
                    }
                    Type::Reference(ty) => match &*ty.elem {
                        Type::Path(type_path) => {
                            let last_segment = type_path.path.segments.last().unwrap();
                            let last_ident = &last_segment.ident;
                            if last_ident.is_primitive() {
                                SPEC::Expr::empty()
                            } else if last_ident.is_optional() {
                                match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                                    Some(TypeKind::Primitive(_)) => SPEC::Expr::empty(),
                                    Some(kind) => SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, ffi_type, kind.to_type()),
                                    None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                                }
                            } else if last_ident.is_string() {
                                SPEC::Expr::destroy_string(expr, &type_path.path)
                            } else if last_ident.is_str() {
                                SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                            } else {
                                SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, ffi_type, full_type)
                            }
                        }
                        Type::Slice(..) => SPEC::Expr::destroy_complex(expr),
                        _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#ty)),
                    }
                    _ => SPEC::Expr::destroy_complex(expr)
                }

            }
        }

    }
}