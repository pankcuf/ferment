use std::fmt::{Display, Formatter};
use quote::ToTokens;
use syn::{PatType, Type};
use crate::composable::TypeComposition;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{FFICompositionResolve, FFIObjectResolve, FFISpecialTypeResolve, FFITypeResolve, GenericNestedArg, Resolve, SpecialType};
use crate::presentable::Expression;
use crate::presentation::Name;

#[derive(Clone, Debug)]
pub struct FromConversionComposer {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<Expression>,
}

impl Display for FromConversionComposer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} --- {} --- {}", self.name, self.ty.to_token_stream(), self.expr.as_ref().map_or("None".to_string(), |e| format!("{}", e))).as_str())
    }
}

// impl From<&FieldComposer> for FromConversionComposer {
//     fn from(value: &FieldComposer) -> Self {
//         Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
//     }
// }
//
#[allow(unused)]
impl From<&PatType> for FromConversionComposer {
    fn from(value: &PatType) -> Self {
        let PatType { ty, pat, .. } = value;
        Self { name: Name::Pat(*pat.clone()), ty: *ty.clone(), expr: None }
    }
}
//
//
#[allow(unused)]
impl FromConversionComposer {
    pub fn new(name: Name, ty: Type, expr: Option<Expression>) -> Self {
        Self { name, ty, expr }
    }
}
impl<'a> Composer<'a> for FromConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let Self { name, ty, expr } = self;
        println!("FromConversionComposer:: {}({}) -- {} -- {:?}", name,  name.to_token_stream(), ty.to_token_stream(), expr);
        // let field_path = Expression::Simple(name.to_token_stream());
        // let field_path = ty.conversion_from(expr.clone().unwrap_or(Expression::Simple(name.to_token_stream())));
        let field_path = expr.clone().unwrap_or(Expression::Simple(name.to_token_stream()));


        let full_type = ty.full_type(source);
        println!("FromConversionComposer:: {} ", full_type.to_token_stream());
        let composition = ty.composition(source);
        let expression = match full_type.maybe_special_type(source) {
            Some(SpecialType::Opaque(..)) => {
                println!("FromConversionComposer:: Opaque: {}({})", ty.to_token_stream(), full_type.to_token_stream());
                match composition {
                    TypeCompositionConversion::FnPointer(..) |
                    TypeCompositionConversion::LambdaFn(..) => field_path,
                    _ => match ty {
                        Type::Ptr(_) => field_path,
                        _ =>
                            Expression::FromPtrClone(field_path.into())
                    }
                }
            },
            Some(SpecialType::Custom(..)) =>
                Expression::From(field_path.into()),
            None => {
                println!("FromConversionComposer (Non Special): {} ({}) --- {}", ty.to_token_stream(), full_type.to_token_stream(), ty.composition(source));
                match composition {
                    TypeCompositionConversion::FnPointer(..) | TypeCompositionConversion::LambdaFn(..) =>
                        field_path,
                    TypeCompositionConversion::Optional(ty) => match TypeConversion::from(ty.ty.first_nested_type().unwrap()) {
                        TypeConversion::Primitive(_) =>
                            Expression::FromOptPrimitive(field_path.into()),
                        _ =>
                            Expression::FromOpt(field_path.into()),
                    }
                    TypeCompositionConversion::Boxed(TypeComposition { ty: ref full_ty, .. }) => {
                        let nested_ty = ty.first_nested_type().unwrap();
                        let full_nested_ty = full_ty.first_nested_type().unwrap();
                        println!("FromConversionComposer (Non Special Boxed): {} ({}) --- {} ---- {}", nested_ty.to_token_stream(), full_nested_ty.to_token_stream(), <Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source).to_token_stream(), nested_ty.maybe_object(source).to_token_stream());
                        match (<Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source),
                               nested_ty.maybe_object(source)) {
                            (Some(SpecialType::Opaque(..)),
                                Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_) | TypeCompositionConversion::LambdaFn(_),..) |
                                     ObjectConversion::Type(TypeCompositionConversion::FnPointer(_) | TypeCompositionConversion::LambdaFn(_)))
                            ) =>
                                Expression::IntoBox(field_path.into()),
                            (Some(SpecialType::Opaque(..)), _any_other) =>
                                Expression::FromRawBox(field_path.into()),
                            (Some(SpecialType::Custom(..)), _any) =>
                                Expression::IntoBox(Expression::From(field_path.into()).into()),
                            (_, Some(obj)) => {
                                println!("FromConversionComposer (Non Special Boxed Lambda): {}", obj);
                                Expression::IntoBox(match obj.maybe_lambda_args() {
                                    Some(lambda_args) =>
                                        Expression::FromLambda(field_path.into(), lambda_args),
                                    None =>
                                        Expression::From(field_path.into())
                                }.into())
                            }
                            _ =>
                                Expression::IntoBox(Expression::From(field_path.into()).into())
                        }
                    },
                    TypeCompositionConversion::Bounds(bounds) => match bounds.bounds.len() {
                        0 => field_path,
                        1 => if let Some(lambda_args) = bounds.bounds.first().unwrap().maybe_lambda_args() {
                            Expression::FromLambda(field_path.into(), lambda_args)
                        } else {
                            Expression::From(field_path.into())
                        }
                        _ =>
                            Expression::From(field_path.into())
                    },
                    _ => {
                        println!("FromConversionComposer (Regular): {}", ty.to_token_stream());
                        match TypeConversion::from(ty) {
                            TypeConversion::Primitive(_) =>
                                field_path,
                            TypeConversion::Generic(GenericTypeConversion::Optional(ty)) => match TypeConversion::from(ty.first_nested_type().unwrap()) {
                                TypeConversion::Primitive(_) => Expression::FromOptPrimitive(field_path.into()),
                                _ => Expression::FromOpt(field_path.into()),
                            }
                            _ =>
                                Expression::From(field_path.into())
                        }
                    }
                }
            }
        };
        // let expression = match ty {
        //     Type::Reference(_) => {
        //         println!("FromConversionComposer::RESULT (REF) {}({}) -- {} === {} == {}", name, name.to_token_stream(), ty.to_token_stream(), expression, expression.present(source));
        //         Expression::AsRef(expression.into())
        //     },
        //     _ => {
        //         println!("FromConversionComposer::RESULT {}({}) -- {} === {} == {}", name, name.to_token_stream(), ty.to_token_stream(), expression, expression.present(source));
        //         expression
        //     }
        // };
        // println!("FromConversionComposer::RESULT {}({}) -- {} === {} == {}", name, name.to_token_stream(), ty.to_token_stream(), result, result.present(source));
        expression
    }
}