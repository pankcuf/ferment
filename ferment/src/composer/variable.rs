use quote::ToTokens;
use syn::{parse_quote, Type};
use syn::punctuated::Punctuated;
use crate::composable::TypeComposition;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::conversion::{DictionaryTypeCompositionConversion, ObjectConversion, ScopeItemConversion, TypeCompositionConversion};
use crate::ext::{GenericNestedArg, Resolve, ResolveTrait, SpecialType, ToPath, ToType};
use crate::presentation::{FFIFullPath, FFIVariable, resolve_type_variable};

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
        println!("VariableComposer (compose): {} ({}) in {}", self.ty.to_token_stream(), full_ty.to_token_stream(), source.scope.fmt_short());
        let maybe_obj = source.maybe_object(&self.ty);
        let result = match <Type as Resolve<Option<SpecialType>>>::resolve(&full_ty, source) {
            Some(special) => match maybe_obj {
                Some(ObjectConversion::Item(fn_ty_conversion, ScopeItemConversion::Fn(..))) => {
                    println!("VariableComposer (Special Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
                    FFIVariable::MutPtr {
                        ty: match &source.scope.parent_object().unwrap() {
                            ObjectConversion::Type(ref ty_conversion) |
                            ObjectConversion::Item(ref ty_conversion, ..) =>
                                ty_conversion.ty().resolve(source),
                            _ => self.ty.clone()
                        }
                    }
                }
                Some(ObjectConversion::Item(TypeCompositionConversion::FnPointer(_), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::FnPointer(_), ..)) => {
                    println!("VariableComposer (Special FnPointer): {}", special.to_token_stream());
                    FFIVariable::Direct { ty: special.to_type() }
                }
                Some(ObjectConversion::Item(TypeCompositionConversion::Trait(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::TraitType(..), ..) |
                     ObjectConversion::Type(TypeCompositionConversion::LambdaFn(..), ..)) => {
                    println!("VariableComposer (Special Trait): {}", special.to_token_stream());
                    let ty = special.to_type();
                    FFIVariable::MutPtr { ty: parse_quote!(dyn #ty) }
                },
                Some(ObjectConversion::Type(TypeCompositionConversion::Bounds(bounds))) =>
                    bounds.resolve(source),
                _ => {
                    println!("VariableComposer (Special MutPtr): {}", special.to_token_stream());
                    FFIVariable::MutPtr { ty: special.to_type() }
                }
            }
            None => {
                println!("VariableComposer (NonSpecial): {} in {}", full_ty.to_token_stream(), source.scope.fmt_short());
                match maybe_obj {
                    Some(ObjectConversion::Item(fn_ty_conversion, ScopeItemConversion::Fn(..))) => {
                        println!("VariableComposer (Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
                        FFIVariable::MutPtr {
                            ty: match &source.scope.parent_object().unwrap() {
                                ObjectConversion::Type(ref ty_conversion) |
                                ObjectConversion::Item(ref ty_conversion, ..) => {
                                    let full_parent_ty: Type = Resolve::resolve(ty_conversion.ty(), source);
                                    println!("VariableComposer (Function Parent): {} ({}) in {}", ty_conversion.to_token_stream(), full_parent_ty.to_token_stream(), source.scope.fmt_short());
                                    match <Type as Resolve<Option<SpecialType>>>::resolve(&full_parent_ty, source) {
                                        Some(special) => special.to_type(),
                                        None => match ty_conversion {
                                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) =>
                                                ty.ty
                                                    .maybe_trait_object(source)
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
                                ty.ty
                                    .maybe_trait_object(source)
                                    .and_then(|oc| oc.type_conversion().cloned())
                            },
                            _ => Some(ty_conversion.clone()),
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
                                        // let nested_conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(nested_ty, source);
                                        // // println!("VariableComposer (Nested Boxed conversion): {}", nested_conversion);
                                        // let result = <TypeCompositionConversion as Resolve<FFIVariable>>::resolve(&nested_conversion, source);
                                        // println!("VariableComposer (Nested Boxed variable): {}", result.to_token_stream());



                                        // let conversion_ty = conversion.ty();
                                        let object = <Type as Resolve<Option<ObjectConversion>>>::resolve(nested_ty, source);
                                        println!("VariableComposer (Nested Boxed Type Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                        let var_ty = match object {
                                            Some(ObjectConversion::Item(.., ScopeItemConversion::Fn(..))) => {
                                                let parent_object = &source.scope.parent_object().unwrap();
                                                match parent_object {
                                                    ObjectConversion::Type(ref ty_conversion) |
                                                    ObjectConversion::Item(ref ty_conversion, ..) => {
                                                        match ty_conversion {
                                                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                                println!("VariableComposer (Nested Boxed Trait Fn Conversion): {}", ty);
                                                                ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned())
                                                            },
                                                            _ => {
                                                                None
                                                            },
                                                        }.unwrap_or_else(|| {
                                                            // println!("Type::<TypeCompositionConversion> Not a Trait So2 --> {}", parent_object.type_conversion().to_token_stream());
                                                            parent_object.type_conversion().cloned()
                                                        })
                                                    },
                                                    ObjectConversion::Empty => {
                                                        // println!("Type::<TypeCompositionConversion> Has no object2 --> {}", parent_object.type_conversion().to_token_stream());
                                                        None
                                                    }
                                                }
                                            },
                                            Some(ObjectConversion::Type(ref ty_conversion) |
                                                 ObjectConversion::Item(ref ty_conversion, ..)) => {
                                                match ty_conversion {
                                                    TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                        println!("VariableComposer (Nested Boxed Regular Trait Conversion): {}", ty);
                                                        ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned())
                                                    },
                                                    // TypeCompositionConversion
                                                    _ => {
                                                        None
                                                    },
                                                }.unwrap_or_else(|| {
                                                    println!("VariableComposer (Nested Boxed Regular Non-Trait Conversion): {}", ty.to_token_stream());
                                                    Some(ty_conversion.clone())
                                                })

                                            },
                                            _ => None,
                                        }.unwrap_or_else(|| {
                                            println!("VariableComposer (Nested Boxed Regular Unknown Conversion): {}", ty.to_token_stream());
                                            TypeCompositionConversion::Unknown(TypeComposition::new(nested_ty.clone(), None, Punctuated::new()))
                                        });
                                        let var_c_type = var_ty.to_type();
                                        let ffi_path: Option<FFIFullPath> = var_c_type.resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                        let result = resolve_type_variable(var_ty, source);

                                        // let result = resolve_type_variable(var_ty.to_type(), source);

                                        result
                                    }
                                }
                            }
                            TypeCompositionConversion::Unknown(TypeComposition { ty, .. }) => {
                                println!("VariableComposer (Unknown): {}", ty.to_token_stream());
                                FFIVariable::MutPtr { ty }
                            },
                            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::Primitive(TypeComposition { ty, .. })) => {
                                println!("VariableComposer (Dictionary Primitive): {}", ty.to_token_stream());
                                FFIVariable::Direct { ty }
                            },
                            TypeCompositionConversion::FnPointer(TypeComposition { ty, .. }, ..) => {
                                println!("VariableComposer (FnPointer Conversion): {}", ty.to_token_stream());
                                let result = FFIVariable::Direct {
                                    ty: <Type as Resolve<Option<SpecialType>>>::resolve(&ty, source)
                                        .map(|special| special.to_type())
                                        .unwrap_or(<Type as Resolve::<FFIFullPath>>::resolve(&ty, source)
                                            .to_type())
                                };
                                println!("VariableComposer (FnPointer Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeCompositionConversion::LambdaFn(TypeComposition { ty, .. }, ..) => {
                                println!("VariableComposer (LambdaFn Conversion): {}", ty.to_token_stream());
                                let result = FFIVariable::MutPtr {
                                    ty: <Type as Resolve::<FFIFullPath>>::resolve(&ty, source).to_type()
                                };
                                println!("VariableComposer (LambdaFn Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveFermentable(TypeComposition { ty, .. })) => {
                                // Dictionary generics and strings should be fermented
                                // Others should be treated as opaque
                                println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion): {}", ty.to_token_stream());
                                let maybe_ffi_full_path: Option<FFIFullPath> = ty.resolve(source);
                                println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion FFIFULLPATH?): {}", maybe_ffi_full_path.to_token_stream());
                                let result = resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or(parse_quote!(#ty)), source);
                                println!("VariableComposer (Dictionary NonPrimitiveFermentable Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveOpaque(..)) => {
                                // Dictionary generics should be fermented
                                // Others should be treated as opaque
                                println!("VariableComposer (Dictionary NonPrimitiveOpaque Conversion): {}", conversion.to_token_stream());
                                let result: FFIVariable = conversion.resolve(source);
                                println!("VariableComposer (Dictionary NonPrimitiveOpaque Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeCompositionConversion::Bounds(bounds) => {
                                bounds.resolve(source)
                            }

                            ref cnv => {
                                println!("VariableComposer (Regular Fermentable Conversion): {}", conversion);
                                // let result: FFIVariable = conversion.resolve(source);
                                // let conversion_ty = conversion.ty();
                                let object = <Type as Resolve<Option<ObjectConversion>>>::resolve(&self.ty, source);
                                println!("VariableComposer (Regular Fermentable Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                let var_ty = match object {
                                    Some(ObjectConversion::Item(.., ScopeItemConversion::Fn(..))) => {
                                        let parent_object = &source.scope.parent_object().unwrap();
                                        match parent_object {
                                            ObjectConversion::Type(ref ty_conversion) |
                                            ObjectConversion::Item(ref ty_conversion, ..) => {
                                                match ty_conversion {
                                                    TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                        println!("VariableComposer (Regular Fermentable Trait Fn Conversion): {}", conversion);
                                                        ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned())
                                                    },
                                                    _ => {
                                                        None
                                                    },
                                                }.unwrap_or_else(|| {
                                                    println!("VariableComposer (Regular Fermentable Non-Trait Fn Conversion): {}", conversion);
                                                    parent_object.type_conversion().cloned()
                                                })
                                            },
                                            ObjectConversion::Empty => {
                                                // println!("Type::<TypeCompositionConversion> Has no object2 --> {}", parent_object.type_conversion().to_token_stream());
                                                None
                                            }
                                        }
                                    },
                                    Some(ObjectConversion::Type(..) |
                                         ObjectConversion::Item(..)) => {
                                        // cnv
                                        match &cnv {
                                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                println!("VariableComposer (Regular Fermentable Trait Conversion): {}", conversion);
                                                ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned())
                                            },
                                            // TypeCompositionConversion::Bounds(bounds) =>
                                            //     bounds.resolve(source),

                                            // TypeCompositionConversion
                                            _ => {
                                                // println!("VariableComposer (Regular Fermentable Non-Trait Conversion): {}", conversion);
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            println!("VariableComposer (Regular Fermentable Non Trait Conversion): {}", cnv);
                                            Some(cnv.clone())
                                        })

                                    },
                                    _ => None,
                                }.unwrap_or_else(|| {
                                    println!("VariableComposer (Regular Fermentable Unknown Conversion): {}", cnv);
                                    cnv.clone()
                                    // TypeCompositionConversion::Unknown(TypeComposition::new(conversion_ty.clone(), None, Punctuated::new()))
                                });
                                println!("VariableComposer (Regular Fermentable Conversion): {}", var_ty.to_token_stream());
                                let var_c_type = var_ty.to_type();
                                let ffi_path: Option<FFIFullPath> = var_c_type.resolve(source);
                                let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                let result = resolve_type_variable(var_ty, source);

                                println!("VariableComposer (Regular Fermentable Variable): {}", result.to_token_stream());
                                result
                            }
                        }
                    },
                    _ => {
                        println!("UNKNOWN TOTALLY: {}", self.ty.to_token_stream());
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
        };println!("VariableComposer (compose) RESULT: {}", result.to_token_stream());result
    }
}

