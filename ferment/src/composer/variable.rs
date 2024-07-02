use quote::ToTokens;
use syn::{parse_quote, Type};
use crate::composable::TypeComposition;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{GenericNestedArg, Resolve, SpecialType, ToType};
use crate::presentation::FFIVariable;

pub struct VariableComposer {
    pub ty: Type,
}

impl From<&Type> for VariableComposer {
    fn from(value: &Type) -> Self {
        Self { ty: value.clone() }
    }
}

// impl VariableComposer {
//     pub const fn new(ty: Type) -> Self {
//         Self { ty }
//     }
// }

impl<'a> Composer<'a> for VariableComposer {
    type Source = ScopeContext;
    type Result = FFIVariable;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let full_ty: Type = Resolve::resolve(&self.ty, source);
        match <Type as Resolve<Option<SpecialType>>>::resolve(&full_ty, source) {
            Some(special) => match source.maybe_object(&self.ty) {
                Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                    println!("VariableComposer (Special FnPointer): {}", special.to_token_stream());
                    FFIVariable::Direct { ty: special.to_type() }
                }
                Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..)) => {
                    println!("VariableComposer (Special Trait): {}", special.to_token_stream());
                    let ty = special.to_type();
                    FFIVariable::MutPtr { ty: parse_quote!(dyn #ty) }
                },
                _ => {
                    println!("VariableComposer (Special MutPtr): {}", special.to_token_stream());
                    FFIVariable::MutPtr { ty: special.to_type() }
                }
            }
            None => {
                println!("VariableComposer (Regular): {}", full_ty.to_token_stream());
                let conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(&self.ty, source);
                match conversion {
                    // TypeCompositionConversion::Optional(_) |
                    TypeCompositionConversion::Boxed(TypeComposition { ref ty, .. }) => {
                        println!("VariableComposer (Boxed conversion): {}", conversion);
                        let nested_ty = ty.first_nested_type().unwrap();
                        // println!("OwnedItemPresentableContext::Named (Boxed conversion): Nested Type: {}", nested_ty.to_token_stream());
                        match <Type as Resolve<Option<SpecialType>>>::resolve(nested_ty, source) {
                            Some(special) => {
                                println!("VariableComposer (Special Boxed conversion): Nested Type: {}", special.to_token_stream());
                                match source.maybe_object(nested_ty) {
                                    Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                                         ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                                        println!("VariableComposer (Special Boxed conversion): Nested Special FnPointer: {}", nested_ty.to_token_stream());
                                        FFIVariable::Direct { ty: special.to_type() }
                                    }
                                    Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                                         ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..)) => {
                                        println!("VariableComposer (Special Boxed conversion): Nested Special Trait: {}", nested_ty.to_token_stream());
                                        let ty = special.to_type();
                                        FFIVariable::MutPtr { ty: parse_quote!(dyn #ty) }
                                    },
                                    _ => {
                                        println!("VariableComposer (Boxed conversion): Nested Special MutPtr: {}", nested_ty.to_token_stream());
                                        FFIVariable::MutPtr { ty: special.to_type() }
                                    }
                                }
                            }
                            None => {
                                let nested_conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(nested_ty, source);
                                let result = <TypeCompositionConversion as Resolve<FFIVariable>>::resolve(&nested_conversion, source);
                                println!("VariableComposer (Boxed conversion): Nested Variable: {}", result.to_token_stream());
                                result
                            }
                        }
                    }
                    _ => {
                        println!("VariableComposer (Regular type conversion): {}", conversion);
                        let result = <TypeCompositionConversion as Resolve<FFIVariable>>::resolve(&conversion, source);
                        println!("VariableComposer (Regular type variable): {}", result.to_token_stream());
                        result
                    }
                }
            }
        }
    }
}

