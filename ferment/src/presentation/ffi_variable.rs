use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use ferment_macro::Display;
use syn::punctuated::Punctuated;
use crate::ast::AddPunctuated;
use crate::composable::{GenericBoundComposition, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::{DictionaryTypeCompositionConversion, GenericTypeConversion, ObjectConversion, ScopeItemConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{Accessory, DictionaryType, GenericNestedArg, Mangle, path_arguments_to_type_conversions, Resolve, ResolveTrait, SpecialType, ToPath, ToType};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

#[derive(Clone, Display, Debug)]
pub enum FFIVariable {
    Direct { ty: Type },
    ConstPtr { ty: Type },
    MutPtr { ty: Type },
}

impl ToTokens for FFIVariable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

impl ToType for FFIVariable {
    fn to_type(&self) -> Type {
        match self {
            FFIVariable::Direct { ty } => ty.to_type(),
            FFIVariable::ConstPtr { ty } => ty.joined_const(),
            FFIVariable::MutPtr { ty } => ty.joined_mut()
        }
    }
}

impl Resolve<FFIVariable> for Path {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        // println!("Path::<FFIVariable>::resolve({})", self.to_token_stream());
        let first_segment = self.segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = self.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            FFIVariable::Direct { ty: self.to_type() }
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeConversion::Primitive(ty)) => FFIVariable::MutPtr {
                    ty: ty.clone()
                },
                Some(TypeConversion::Generic(generic_ty)) => FFIVariable::MutPtr {
                    ty: <GenericTypeConversion as Resolve<FFIFullPath>>::resolve(generic_ty, source).to_type()
                },
                Some(TypeConversion::Complex(Type::Path(TypePath { path, .. }))) =>
                    path.resolve(source),
                _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
            }
        } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
            FFIVariable::MutPtr {
                ty: source.scope_type_for_path(self)
                    .map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default())
                    .to_type()
            }
        } else {
            FFIVariable::MutPtr {
                ty: self.to_type()
            }
        }
    }
}

pub fn resolve_type_variable(ty: Type, source: &ScopeContext) -> FFIVariable {
    // println!("resolve_type_variable: {}", ty.to_token_stream());
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.resolve(source),
        Type::Array(TypeArray { elem, len, .. }) => FFIVariable::MutPtr {
            ty: parse_quote!([#elem; #len])
        },
        Type::Reference(TypeReference { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) =>
            elem.resolve(source),
        Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
            match *elem {
                Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                    "c_void" => match (star_token, const_token, mutability) {
                        (_, Some(_const_token), None) => FFIVariable::ConstPtr { ty: FFIFullDictionaryPath::Void.to_type() },
                        (_, None, Some(_mut_token)) => FFIVariable::MutPtr { ty: FFIFullDictionaryPath::Void.to_type() },
                        _ => panic!("<Type as Resolve<FFIVariable>>::resolve: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                    },
                    _ => FFIVariable::MutPtr {
                        ty: path.to_type()
                    }
                },
                Type::Ptr(..) => FFIVariable::MutPtr {
                    ty: elem.to_type(),
                },
                ty => mutability.as_ref()
                    .map_or( FFIVariable::ConstPtr { ty: ty.clone() }, |_| FFIVariable::MutPtr { ty: ty.clone() })
            },
        Type::TraitObject(TypeTraitObject { dyn_token: _, bounds, .. }) => {
            bounds.resolve(source)
        }
        Type::ImplTrait(TypeImplTrait { impl_token: _, bounds, .. }) =>
            bounds.resolve(source),
        ty => FFIVariable::Direct { ty: ty.mangle_ident_default().to_type() }
    }
}

impl Resolve<FFIVariable> for AddPunctuated<TypeParamBound> {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        // println!("AddPunctuated<TypeParamBound>::<FFIVariable>::resolve({})", self.to_token_stream());
        let bound = self.iter().find_map(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
            TypeParamBound::Lifetime(_) => None
        }).unwrap();
        bound.resolve(source)
    }
}

impl Resolve<FFIVariable> for Type {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        // println!("Type::<FFIVariable>::resolve.1({})", self.to_token_stream());
        let full_ty = <Type as Resolve<Type>>::resolve(self, source);
        // println!("Type::<FFIVariable>::resolve.2({})", full_ty.to_token_stream());
        let maybe_special = <Type as Resolve<Option<SpecialType>>>::resolve(&full_ty, source);
        // println!("Type::<FFIVariable>::resolve.3({})", maybe_special.to_token_stream());
        let refined = maybe_special
            .map(|ty| FFIFullPath::External { path: ty.to_path() })
            .or(<Type as Resolve<TypeCompositionConversion>>::resolve(self, source)
                .to_type()
                .resolve(source))
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or(parse_quote!(#self))
            .to_type();
        resolve_type_variable(refined, source)
    }
}

impl Resolve<FFIVariable> for TypeCompositionConversion {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        println!("TypeCompositionConversion::<FFIVariable>::resolve({}) in {}", self, source.scope.fmt_short());
        let result = match self  {
            // TODO: For now we assume that every callback defined as fn pointer is opaque
            TypeCompositionConversion::FnPointer(TypeComposition { ty, .. }, ..) => FFIVariable::Direct {
                ty: <Type as Resolve<Option<SpecialType>>>::resolve(ty, source)
                    .map(|special| special.to_type())
                    .unwrap_or(<Type as Resolve::<FFIFullPath>>::resolve(ty, source)
                        .to_type())
            },
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::LambdaFn(TypeComposition { ty, .. }, ..)) => FFIVariable::MutPtr {
                ty: <Type as Resolve::<FFIFullPath>>::resolve(ty, source).to_type()
            },
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::Primitive(composition)) => FFIVariable::Direct {
                ty: composition.to_type()
            },
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveFermentable(TypeComposition { ty, .. })) |
            TypeCompositionConversion::Dictionary(DictionaryTypeCompositionConversion::NonPrimitiveOpaque(TypeComposition { ty, .. })) |
            TypeCompositionConversion::Trait(TypeComposition { ty, .. }, ..) |
            TypeCompositionConversion::TraitType(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Object(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Optional(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Array(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Slice(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Tuple(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Unknown(TypeComposition { ty, .. }) |
            TypeCompositionConversion::LocalOrGlobal(TypeComposition { ty, .. }) => {
                println!("TypeCompositionConversion::Regular: {}", ty.to_token_stream());
                // let ffi_path = match <Type as Resolve<Option<SpecialType>>>::resolve(ty, source) {
                //     Some(ty) =>
                //         ty.to_type().resolve(source),
                //         // FFIFullPath::External { path: ty.to_path() }.res,
                //     None => {
                //         let ty = <Type as Resolve<TypeCompositionConversion>>::resolve(ty, source);
                //         let ffi_path: Option<FFIFullPath> = ty.resolve(source);
                //         resolve_type_variable(match ffi_path {
                //             Some(ffi_path) =>
                //                 ffi_path.to_type(),
                //             None =>
                //                 ty.to_type()
                //                 // FFIFullPath::External { path: ty.to_path() }
                //         }, source)
                //     }
                // };

                // resolve_type_variable(ffi_path.to_type(), source)
                //


                <Type as Resolve<Option<SpecialType>>>::resolve(ty, source)
                    .map(|ty| resolve_type_variable(FFIFullPath::External { path: ty.to_path() }.to_type(), source))
                    .unwrap_or_else(|| {
                        resolve_type_variable(<Type as Resolve<Option<ObjectConversion>>>::resolve(ty, source)
                            .and_then(|external_type| {
                                match external_type {
                                    ObjectConversion::Item(.., ScopeItemConversion::Fn(..)) => {
                                        // println!("Type::<TypeCompositionConversion> IT's a FUNCTION --> {} {} {}", sig.ident, path.to_token_stream(), ty_conversion);
                                        let parent_object = &source.scope.parent_object().unwrap();
                                        // println!("Type::<TypeCompositionConversion> Parent Object: {}", parent_object);
                                        match parent_object {
                                            ObjectConversion::Type(ref ty_conversion) |
                                            ObjectConversion::Item(ref ty_conversion, ..) => {
                                                match ty_conversion {
                                                    TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                        // println!("Type::<TypeCompositionConversion> It's a Trait So2 --> {}", parent_object.type_conversion().to_token_stream());
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
                                    ObjectConversion::Type(ref ty_conversion) |
                                    ObjectConversion::Item(ref ty_conversion, ..) => {
                                        match ty_conversion {
                                            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) => {
                                                // println!("Type::<TypeCompositionConversion> It's a Trait So --> {}", external_type.type_conversion().to_token_stream());
                                                ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned())
                                            },
                                            _ => {
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            // println!("Type::<TypeCompositionConversion> Not a Trait So --> {}", external_type.type_conversion().to_token_stream());
                                            external_type.type_conversion().cloned()
                                        })
                                    },
                                    ObjectConversion::Empty => {
                                        // println!("Type::<TypeCompositionConversion> Has no object --> {}", external_type.type_conversion().to_token_stream());
                                        None
                                    }
                                }
                            })
                            .unwrap_or_else(|| {
                                println!("Type::<TypeCompositionConversion> Default Unknown --> {}", ty.to_token_stream());
                                TypeCompositionConversion::Unknown(TypeComposition::new(ty.clone(), None, Punctuated::new()))
                            }).to_type(), source)


                        // .map(|ffi_path| ffi_path.to_type())
                        //     .unwrap_or(parse_quote!(#ty))
                        //     .resolve(source)
                    })
                // <Type as Resolve<Option<SpecialType>>>::resolve(ty, source)
                //     .map(|ty| FFIFullPath::External { path: ty.to_path() }.to_type())
                //     .or_else(|| <Type as Resolve<TypeCompositionConversion>>::resolve(ty, source)
                //         .to_type()
                //         .resolve(source))
                //     .map(|ffi_path| ffi_path.to_type())
                //     .unwrap_or(parse_quote!(#ty))
                //     .resolve(source)
            },
            TypeCompositionConversion::Fn(TypeComposition { ty, .. }, ..) => {
                // ty.to_path().popped()
                panic!("error: Arg conversion (Fn) ({}) not supported", ty.to_token_stream())
            },
            TypeCompositionConversion::Boxed(TypeComposition { ty, .. }, ..) => {
                // println!("TypeCompositionConversion::Boxed: {}", ty.to_token_stream());
                match ty.first_nested_type() {
                    Some(nested_full_ty) => {
                        // println!("Nested: {}", nested_full_ty.to_token_stream());
                        resolve_type_variable(match <Type as Resolve<Option<SpecialType>>>::resolve(nested_full_ty, source) {
                            Some(special) => special.to_type(),
                            None => {
                                let conversion = <Type as Resolve<TypeCompositionConversion>>::resolve(nested_full_ty, source);
                                <Type as Resolve<Option<FFIFullPath>>>::resolve(&conversion.to_type(), source)
                                    .map(|full_path| full_path.to_type())
                                    .unwrap_or_else(|| nested_full_ty.clone())
                            }
                        }, source)
                    }
                    None => panic!("error: Arg conversion ({}) not supported", ty.to_token_stream())
                }
            },

            TypeCompositionConversion::Bounds(bounds) => {
                // println!("TypeCompositionConversion::Bounds: {}", bounds);
                bounds.resolve(source)
            },
            ty =>
                panic!("error: Arg conversion ({}) not supported", ty),
        };
        // println!("TypeCompositionConversion::<FFIVariable>::resolve.2({}) --> {}", self, result.to_token_stream());
        result
    }
}

impl Resolve<FFIVariable> for GenericBoundComposition {
    fn resolve(&self, _source: &ScopeContext) -> FFIVariable {
        let ffi_name = self.mangle_ident_default();
        // println!("GenericBoundComposition::<FFIVariable>::resolve({})", self);
        FFIVariable::MutPtr { ty: parse_quote!(crate::fermented::generics::#ffi_name) }
    }
}