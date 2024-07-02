use quote::ToTokens;
use syn::{parse_quote, Type};
use crate::composable::{FieldComposer, TypeComposition};
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{GenericNestedArg, Resolve, SpecialType};
use crate::presentable::Expression;
use crate::presentation::{FFIVariable, Name};

pub enum FromConversionType {
    FromPrimitive,
    FromComplex,
}

#[derive(Clone, Debug)]
pub struct FromConversionComposer {
    pub name: Name,
    pub ty: Type,
}

impl From<&FieldComposer> for FromConversionComposer {
    fn from(value: &FieldComposer) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone() }
    }
}
impl FromConversionComposer {
    pub fn new(name: Name, ty: Type) -> Self {
        Self { name, ty }
    }
}

impl<'a> Composer<'a> for FromConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {

        // match TypeConversion::from(&self.ty) {
        //     TypeConversion::Primitive(_) =>
        //         Expression::Simple(self.name.to_token_stream()),
        //     TypeConversion::Complex(_) => Expression::From(Expression::Simple(self.name.to_token_stream()).into()),
        //     TypeConversion::Generic(generic_ty) => match generic_ty {
        //         GenericTypeConversion::Optional(_) => Expression::FromOpt(Expression::Simple(self.name.to_token_stream()).into()),
        //         _ => Expression::From(Expression::Simple(self.name.to_token_stream()).into())
        //     }
        // }

        let full_ty: Type = Resolve::resolve(&self.ty, source);
        match <Type as Resolve<Option<SpecialType>>>::resolve(&full_ty, source) {
            Some(special) => match source.maybe_object(&self.ty) {
                Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                    println!("FromConversionComposer (Special FnPointer): {}", special.to_token_stream());
                    Expression::Simple(self.name.to_token_stream())
                }
                Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..)) => {
                    println!("FromConversionComposer (Special Trait): {}", special.to_token_stream());
                    Expression::Simple(self.name.to_token_stream())
                    // Expression::From(Expression::Simple(self.name.to_token_stream()).into())
                },
                _ => {
                    println!("FromConversionComposer (Special MutPtr): {}", special.to_token_stream());
                    Expression::Simple(self.name.to_token_stream())
                    // Expression::From(Expression::Simple(self.name.to_token_stream()).into())
                }
            }
            None => {
                println!("FromConversionComposer (Regular): {}", full_ty.to_token_stream());
                let conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(&self.ty, source);
                match conversion {
                    TypeCompositionConversion::Optional(_)  => {
                        let nested_ty = self.ty.first_nested_type().unwrap();
                        Expression::FromOpt(Expression::Simple(self.name.to_token_stream()).into())
                    }
                    TypeCompositionConversion::Boxed(TypeComposition { ref ty, .. }) => {
                        println!("FromConversionComposer (Boxed conversion): {}", conversion);
                        let nested_ty = ty.first_nested_type().unwrap();
                        // println!("OwnedItemPresentableContext::Named (Boxed conversion): Nested Type: {}", nested_ty.to_token_stream());
                        match <Type as Resolve<Option<SpecialType>>>::resolve(nested_ty, source) {
                            Some(special) => {
                                println!("FromConversionComposer (Special Boxed conversion): Nested Type: {}", special.to_token_stream());
                                match source.maybe_object(nested_ty) {
                                    Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                                         ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                                        println!("FromConversionComposer (Special Boxed conversion): Nested Special FnPointer: {}", nested_ty.to_token_stream());
                                        Expression::IntoBox(Expression::Simple(self.name.to_token_stream()).into())
                                    }
                                    Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                                         ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..)) => {
                                        println!("FromConversionComposer (Special Boxed conversion): Nested Special Trait: {}", nested_ty.to_token_stream());
                                        Expression::IntoBox(Expression::Simple(self.name.to_token_stream()).into())
                                    },
                                    _ => {
                                        println!("FromConversionComposer (Boxed conversion): Nested Special MutPtr: {}", nested_ty.to_token_stream());
                                        Expression::IntoBoxRaw(Expression::Simple(self.name.to_token_stream()).into())
                                    }
                                }
                            }
                            None => {
                                // let nested_conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(nested_ty, source);
                                Expression::From(Expression::IntoBox(Expression::Simple(self.name.to_token_stream()).into()).into())
                            }
                        }
                    }
                    _ => {
                        match TypeConversion::from(&self.ty) {
                            TypeConversion::Primitive(_) =>
                                Expression::Simple(self.name.to_token_stream()),
                            TypeConversion::Complex(_) => Expression::From(Expression::Simple(self.name.to_token_stream()).into()),
                            TypeConversion::Generic(generic_ty) => match generic_ty {
                                GenericTypeConversion::Optional(_) => Expression::FromOpt(Expression::Simple(self.name.to_token_stream()).into()),
                                _ => Expression::From(Expression::Simple(self.name.to_token_stream()).into())
                            }
                        }
                    }
                }
            }
        }
    }
}