use quote::quote;
use syn::Type;
use crate::composable::FieldComposer;
use crate::composer::{Composer, FFIAspect};
use crate::context::ScopeContext;
use crate::conversion::TypeKind;
use crate::ext::{DictionaryType, path_arguments_to_type_conversions};
use crate::lang::Specification;
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::Name;

#[derive(Clone)]
pub struct DestroyConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<SPEC::Expr>,
}
impl<LANG, SPEC> From<&FieldComposer<LANG, SPEC>> for DestroyConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &FieldComposer<LANG, SPEC>) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
    }
}
impl<LANG, SPEC> DestroyConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(name: Name, ty: Type, expr: Option<SPEC::Expr>) -> Self {
        Self { name, ty, expr }
    }
}

// impl<'a> Composer<'a> for TypePath {
//     type Source = ExprSource<'a>;
//     type Result = Expression;
//
//     fn compose(&self, source: &'a Self::Source) -> Self::Result {
//         let (expr, source) = source;
//         let last_segment = self.path.segments.last().unwrap();
//         let last_ident = &last_segment.ident;
//         // let is_string = last_ident.is_string();
//         // let is_str = last_ident.is_str();
//         if last_ident.is_primitive() {
//             Expression::Empty
//         } else if last_ident.is_string() {
//             Expression::DestroyString(expr.into(), self.path.to_token_stream())
//         } else if last_ident.is_str() {
//             Expression::DestroyString(expr.into(), quote!(&#self))
//         } else if last_ident.is_optional() {
//             match path_arguments_to_type_conversions(&last_segment.arguments).first() {
//                 Some(TypeKind::Primitive(_)) =>
//                     Expression::DestroyOptPrimitive(expr.into()),
//                 _ =>
//                     Expression::DestroyOpt(expr.into()),
//             }
//         } else {
//             Expression::UnboxAnyTerminated(expr.into())
//         }
//     }
// }
//
//
impl<'a, LANG, SPEC> Composer<'a> for DestroyConversionComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    type Source = ScopeContext;
    type Output = <SPEC as Specification<LANG>>::Expr;

    fn compose(&self, _source: &'a Self::Source) -> Self::Output {
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
                        Some(_) => Expression::ConversionExpr(FFIAspect::Destroy, ConversionExpressionKind::ComplexOpt, expr.into()),
                        None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                    }
                } else if last_ident.is_string() {
                    SPEC::Expr::destroy_string(expr, &type_path.path)
                } else if last_ident.is_str() {
                    SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                } else {
                    SPEC::Expr::unbox_any_expr_terminated(expr)
                }

            }

                // ty.conversion_destroy(expr),
            Type::Ptr(ty) => match &*ty.elem {
                // Type::Ptr(type_ptr) => type_ptr.conversion_destroy(expr),
                Type::Path(type_path) => {
                    let last_segment = type_path.path.segments.last().unwrap();
                    let last_ident = &last_segment.ident;
                    if last_ident.is_primitive() {
                        SPEC::Expr::empty()
                    } else if last_ident.is_optional() {
                        match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                            Some(TypeKind::Primitive(_)) => SPEC::Expr::empty(),
                            Some(_) => Expression::ConversionExpr(FFIAspect::Destroy, ConversionExpressionKind::ComplexOpt, expr.into()),
                            None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                        }
                    } else if last_ident.is_string() {
                        SPEC::Expr::destroy_string(expr, &type_path.path)
                    } else if last_ident.is_str() {
                        SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                    } else {
                        SPEC::Expr::unbox_any_expr_terminated(expr)
                    }

                },
                _ => panic!("Can't destroy_ptr: of type: {}", quote!(#ty)),
            }
                // ty.conversion_destroy(expr),
            Type::Reference(ty) => match &*ty.elem {
                Type::Path(type_path) => {
                    {
                        let last_segment = type_path.path.segments.last().unwrap();
                        let last_ident = &last_segment.ident;
                        if last_ident.is_primitive() {
                            SPEC::Expr::empty()
                        } else if last_ident.is_optional() {
                            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                                Some(TypeKind::Primitive(_)) => SPEC::Expr::empty(),
                                Some(_) => Expression::ConversionExpr(FFIAspect::Destroy, ConversionExpressionKind::ComplexOpt, expr.into()),
                                None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                            }
                        } else if last_ident.is_string() {
                            SPEC::Expr::destroy_string(expr, &type_path.path)
                        } else if last_ident.is_str() {
                            SPEC::Expr::destroy_string(expr, quote!(&#type_path))
                        } else {
                            SPEC::Expr::unbox_any_expr_terminated(expr)
                        }

                    }
                }
                Type::Slice(..) => SPEC::Expr::unbox_any_expr(expr),
                _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#ty)),
            }
            _ => SPEC::Expr::unbox_any_expr(expr)
        }

        // println!("DestroyConversionComposer:: {} -- {}", name, ty.to_token_stream());
        // ty.conversion_destroy(expr)
        // match ty {
        //     Type::Path(type_path) =>
        //         type_path.compose(source),
        //     Type::Ptr(TypePtr { elem, .. }) => match &*elem {
        //         Type::Ptr(TypePtr { elem, .. }) => match &*elem {
        //             Type::Path(type_path) =>
        //                 type_path.compose(source),
        //             _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        //         },
        //         Type::Path(type_path) =>
        //             type_path.compose(source),
        //         _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        //     },
        //     Type::Reference(TypeReference { elem, ..}) => match &*elem {
        //         Type::Path(type_path) =>
        //             type_path.compose(source),
        //         _ => Expression::UnboxAny(expr.into()),
        //     },
        //     Type::Array(..) |
        //     Type::Slice(..) |
        //     Type::TraitObject(..) |
        //     Type::Tuple(..) |
        //     Type::ImplTrait(..) =>
        //         Expression::UnboxAny(expr.into()),
        //     _ => unimplemented!("No conversions for {}", self.to_token_stream())
        // }
        //
    }
}