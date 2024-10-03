use quote::quote;
use syn::Type;
use crate::composable::FieldComposer;
use crate::composer::SourceComposable;
use crate::context::ScopeContext;
use crate::conversion::TypeKind;
use crate::ext::{DictionaryType, path_arguments_to_type_conversions, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::Name;

#[derive(Clone, Debug)]
pub struct DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
}
impl<LANG, SPEC> From<&FieldComposer<LANG, SPEC>> for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &FieldComposer<LANG, SPEC>) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
    }
}
impl<LANG, SPEC> DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}

impl<LANG, SPEC> SourceComposable for DestroyConversionComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = SPEC::Expr;

    fn compose(&self, _source: &Self::Source) -> Self::Output {
        let Self { name, ty, expr } = self;
        let expr = expr.clone().unwrap_or(SPEC::Expr::name(name));
        match ty {
            Type::Path(type_path) => {
                let last_segment = type_path.path.segments.last().unwrap();
                let last_ident = &last_segment.ident;
                if last_ident.is_primitive() {
                    SPEC::Expr::empty()
                } else if last_ident.is_optional() {
                    match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                        Some(TypeKind::Primitive(_)) => Expression::empty(),
                        Some(_) => Expression::destroy_complex_opt(expr),
                        None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                    }
                } else if last_ident.is_string() {
                    SPEC::Expr::destroy_string(expr, &type_path.path)
                } else if last_ident.is_str() {
                    SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                } else {
                    SPEC::Expr::destroy_complex(expr)
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
                            Some(_) => Expression::destroy_complex_opt(expr),
                            None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                        }
                    } else if last_ident.is_string() {
                        SPEC::Expr::destroy_string(expr, &type_path.path)
                    } else if last_ident.is_str() {
                        SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                    } else {
                        SPEC::Expr::destroy_complex(expr)
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
                            Some(_) => Expression::destroy_complex_opt(expr),
                            None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                        }
                    } else if last_ident.is_string() {
                        SPEC::Expr::destroy_string(expr, &type_path.path)
                    } else if last_ident.is_str() {
                        SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                    } else {
                        SPEC::Expr::destroy_complex(expr)
                    }
                }
                Type::Slice(..) => SPEC::Expr::destroy_complex(expr),
                _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#ty)),
            }
            _ => SPEC::Expr::destroy_complex(expr)
        }
    }
}