use quote::ToTokens;
use syn::{parse_quote, Type};
use crate::composable::TypeComposition;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::ext::{GenericNestedArg, Resolve, ResolveTrait, SpecialType, ToPath, ToType};
use crate::presentation::{FFIFullPath, FFIVariable};

#[derive(Clone, Debug)]
pub struct VariableComposer {
    pub ty: Type,
}

impl From<&Type> for VariableComposer {
    fn from(value: &Type) -> Self {
        Self { ty: value.clone() }
    }
}


impl<'a> Composer<'a> for VariableComposer {
    type Source = ScopeContext;
    type Result = FFIVariable;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let full_ty: Type = Resolve::resolve(&self.ty, source);
        // println!("VariableComposer (compose): {} ({}) in {}", self.ty.to_token_stream(), full_ty.to_token_stream(), source.scope.fmt_short());
        let result = match <Type as Resolve<Option<SpecialType>>>::resolve(&full_ty, source) {
            Some(special) => match source.maybe_object(&self.ty) {
                Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                    // println!("VariableComposer (Special FnPointer): {}", special.to_token_stream());
                    FFIVariable::Direct { ty: special.to_type() }
                }
                Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::LambdaFn(..), ..)) => {
                    // println!("VariableComposer (Special Trait): {}", special.to_token_stream());
                    let ty = special.to_type();
                    FFIVariable::MutPtr { ty: parse_quote!(dyn #ty) }
                },
                _ => {
                    // println!("VariableComposer (Special MutPtr): {}", special.to_token_stream());
                    FFIVariable::MutPtr { ty: special.to_type() }
                }
            }
            None => {
                // println!("VariableComposer (NonSpecial): {} in {}", full_ty.to_token_stream(), source.scope.fmt_short());
                match source.maybe_object(&self.ty) {
                    Some(ObjectConversion::Item(.., ScopeItemConversion::Fn(..))) => {
                        // println!("VariableComposer (Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
                        FFIVariable::MutPtr {
                            ty: match &source.scope.parent_object().unwrap() {
                                ObjectConversion::Type(ref ty_conversion) |
                                ObjectConversion::Item(ref ty_conversion, ..) => {
                                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.ty(), source);
                                    // println!("VariableComposer (Function Parent): {} ({}) in {}", ty_conversion.to_token_stream(), full_parent_ty.to_token_stream(), source.scope.fmt_short());
                                    match <Type as Resolve<Option<SpecialType>>>::resolve(&full_parent_ty, source) {
                                        Some(special) => special.to_type(),
                                        None => match ty_conversion {
                                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) =>
                                                ty.ty.maybe_trait_object(source)
                                                    .and_then(|oc| oc.type_conversion().map(|c| c.to_type()))
                                                    .unwrap_or(ty_conversion.to_type()),
                                            _ => ty_conversion.to_type()
                                        }
                                    }
                                },
                                _ => self.ty.clone()
                            }
                        }
                    },
                    Some(ObjectConversion::Type(ref ty_conversion)) |
                    Some(ObjectConversion::Item(ref ty_conversion, ..)) => {
                        let conversion = match ty_conversion {
                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                println!("Type::<TypeCompositionConversion> It's a Trait So --> {}", ty_conversion.to_token_stream());
                                ty.ty.maybe_trait_object(source).and_then(|oc| oc.type_conversion().cloned())
                            },
                            _ =>
                                Some(ty_conversion.clone()),
                        }.unwrap_or_else(|| {
                            println!("Type::<TypeCompositionConversion> Not a Trait So --> {}", ty_conversion.to_token_stream());
                            ty_conversion.clone()
                        });
                        match conversion {
                            // TypeCompositionConversion::Optional(_) |
                            TypeCompositionConversion::Boxed(TypeComposition { ref ty, .. }) => {
                                // println!("VariableComposer (Boxed conversion): {}", conversion);
                                // let nested_ty = ty.first_nested_type().unwrap();
                                let nested_ty = self.ty.first_nested_type().unwrap();
                                let full_nested_ty = ty.first_nested_type().unwrap();
                                match <Type as Resolve<Option<SpecialType>>>::resolve(full_nested_ty, source) {
                                    Some(special) => {
                                        // println!("VariableComposer (Special Boxed conversion): Nested Type: {}", special.to_token_stream());
                                        match source.maybe_object(nested_ty) {
                                            Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                                                 ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special FnPointer: {}", nested_ty.to_token_stream());
                                                FFIVariable::Direct { ty: special.to_type() }
                                            }
                                            Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                                                 ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..) |
                                                 ObjectConversion::Type(TypeCompositionConversion::LambdaFn(..), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special Trait: {}", nested_ty.to_token_stream());
                                                let ty = special.to_type();
                                                FFIVariable::MutPtr { ty: parse_quote!(dyn #ty) }
                                            },
                                            _ => {
                                                // println!("VariableComposer (Boxed conversion): Nested Special MutPtr: {}", nested_ty.to_token_stream());
                                                FFIVariable::MutPtr { ty: special.to_type() }
                                            }
                                        }
                                    }
                                    None => {
                                        // println!("VariableComposer (Nested Boxed ty): {}", nested_ty.to_token_stream());
                                        let nested_conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(nested_ty, source);
                                        // println!("VariableComposer (Nested Boxed conversion): {}", nested_conversion);
                                        let result = <TypeCompositionConversion as Resolve<FFIVariable>>::resolve(&nested_conversion, source);
                                        // println!("VariableComposer (Nested Boxed variable): {}", result.to_token_stream());
                                        result
                                    }
                                }
                            }
                            _ => {
                                // println!("VariableComposer (Regular type conversion): {}", conversion);
                                let result = <TypeCompositionConversion as Resolve<FFIVariable>>::resolve(&conversion, source);
                                // println!("VariableComposer (Regular type variable): {}", result.to_token_stream());
                                result
                            }
                        }
                    },
                    _ => {
                        // println!("UNKNOWN TOTALLY: {}", self.ty.to_token_stream());
                        // FFIVariable::MutPtr { ty: self.ty.clone() }
                            <Type as Resolve<Option<SpecialType>>>::resolve(&self.ty, source)
                            .map(|ty| FFIFullPath::External { path: ty.to_path() })
                            .or(<Type as Resolve<TypeCompositionConversion>>::resolve(&self.ty, source)
                                .to_type()
                                .resolve(source))
                            .map(|ffi_path| ffi_path.to_type())
                            .unwrap_or(self.ty.clone())
                            .resolve(source)

                    }
                }

                // let conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(&self.ty, source);

            }
        };/*println!("VariableComposer: {} ---> {} ----> {}", self.ty.to_token_stream(), full_ty.to_token_stream(), result.to_token_stream());*/result
    }
}

