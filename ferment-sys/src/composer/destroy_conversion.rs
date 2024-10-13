use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::{parse_quote, Type};
use crate::composable::FieldComposer;
use crate::composer::SourceComposable;
use crate::context::ScopeContext;
use crate::conversion::{ObjectKind, TypeKind};
use crate::ext::{DictionaryType, path_arguments_to_type_conversions, Resolve, SpecialType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

#[derive(Clone, Debug)]
pub struct DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: SPEC::Name,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
}
impl<LANG, SPEC> From<&FieldComposer<LANG, SPEC>> for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable, Name=Name<LANG, SPEC>, Var: ToType>,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &FieldComposer<LANG, SPEC>) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
    }
}
impl<LANG, SPEC> DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable, Name=Name<LANG, SPEC>, Var: ToType>,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name<LANG, SPEC>, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}

impl<LANG, SPEC> SourceComposable for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          FFIFullPath<LANG, SPEC>: ToType,
          FFIFullDictionaryPath<LANG, SPEC>: ToType,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
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
        let ffi_type = <Type as Resolve::<FFIFullPath<LANG, SPEC>>>::maybe_resolve(&full_type, source).map_or(full_type.clone(), |full_path| full_path.to_type());
        let expr = expr.clone().unwrap_or(SPEC::Expr::name(name));
        match maybe_special {
            Some(SpecialType::Opaque(..)) => {
                Expression::destroy_complex(expr)
            }
            Some(SpecialType::Custom(..)) => {
                Expression::destroy_complex(expr)
                // SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::Complex, custom_ty, full_type)
                // Expression::destroy_complex(expr)
            }
            _ => {
                match &full_type {
                    Type::Path(type_path) => {
                        let last_segment = type_path.path.segments.last().unwrap();
                        let last_ident = &last_segment.ident;
                        if last_ident.is_primitive() {
                            SPEC::Expr::empty()
                        } else if matches!(last_ident.to_string().as_str(), "i128") {
                            SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, parse_quote!([u8; 16]), parse_quote!(i128))
                        } else if matches!(last_ident.to_string().as_str(), "u128") {
                            SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, parse_quote!([u8; 16]), parse_quote!(u128))
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
                                    // Some(kind) => Expression::destroy_complex_opt(expr),
                                    None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                                }
                            } else if last_ident.is_string() {
                                SPEC::Expr::destroy_string(expr, &type_path.path)
                            } else if last_ident.is_str() {
                                SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                            } else {
                                SPEC::Expr::cast_destroy(expr, ConversionExpressionKind::ComplexOpt, ffi_type, full_type)
                                // SPEC::Expr::destroy_complex(expr)
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