use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeReference, TypeTraitObject};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::composable::TypeModel;
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::conversion::{GenericTypeKind, ObjectKind, ScopeItemKind, TypeModelKind, TypeKind};
use crate::ext::{AsType, CrateExtension, DictionaryType, FFISpecialTypeResolve, Mangle, ResolveTrait, SpecialType, ToPath, ToType};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

// pub trait ResolveByValue<T> {
//     fn resolve_by_value(&self, source: &ScopeContext) -> T;
// }

pub trait Resolve<T> {
    fn resolve(&self, source: &ScopeContext) -> T;
}

impl Resolve<Type> for Type {
    fn resolve(&self, source: &ScopeContext) -> Type {
        // println!("<Type as Resolve<Type>>::resolve({})", self.to_token_stream());
        source.full_type_for(self)
    }
}
impl Resolve<Option<ObjectKind>> for Type {
    fn resolve(&self, source: &ScopeContext) -> Option<ObjectKind> {
        // println!("Type::<Option<ObjectKind>>::resolve({})", self.to_token_stream());
        source.maybe_object_by_key(self)
    }
}

impl Resolve<Option<SpecialType>> for Type {

    // #[ferment_macro::debug_io]
    fn resolve(&self, source: &ScopeContext) -> Option<SpecialType> {
        // println!("Type::<Option<SpecialType>>::resolve.1({}) -- {}", self.to_token_stream(), source.scope.fmt_short());
        let result = source.maybe_custom_conversion(self)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object(self)
                .map(SpecialType::Opaque));
        // println!("Type::<Option<SpecialType>>::resolve.2({})", result.to_token_stream());
        result
    }
}

impl<'a> Resolve<Option<SpecialType>> for ScopeSearchKey<'a> {
    fn resolve(&self, source: &ScopeContext) -> Option<SpecialType> {
        let ty = self.to_type();
        let result = source.maybe_custom_conversion(&ty)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object(&ty)
                .map(SpecialType::Opaque));
        result
    }
}

impl<'a> Resolve<TypeModelKind> for ScopeSearchKey<'a>  {
    fn resolve(&self, source: &ScopeContext) -> TypeModelKind {
        self.to_type().resolve(source)
    }
}

impl Resolve<TypeModelKind> for Type {
    fn resolve(&self, source: &ScopeContext) -> TypeModelKind {
        // println!("Type::<TypeModelKind>::resolve.1({}) in {}", self.to_token_stream(), source.scope.fmt_short());
        let result = <Type as Resolve<Option<ObjectKind>>>::resolve(self, source)
            .and_then(|ext_obj_kind| {
                match ext_obj_kind {
                    ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
                        source.scope.parent_object().and_then(|parent_obj| parent_obj.maybe_trait_or_regular_model_kind(source))
                    },
                    ObjectKind::Type(ref ty_conversion) |
                    ObjectKind::Item(ref ty_conversion, ..) => {
                        match ty_conversion {
                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) =>
                                ty.maybe_trait_object_maybe_model_kind(source),
                            _ => None,
                        }.unwrap_or_else(|| {
                            // println!("Type::<TypeModelKind> Not a Trait So --> {}", external_type.type_conversion().to_token_stream());
                            ext_obj_kind.maybe_type_model_kind_ref().cloned()
                        })
                    },
                    ObjectKind::Empty => {
                        // println!("Type::<TypeModelKind> Has no object --> {}", external_type.type_conversion().to_token_stream());
                        None
                    }
                }
            })
            .unwrap_or_else(|| {
                // println!("Type::<TypeModelKind> Default Unknown --> {}", self.to_token_stream());
                TypeModelKind::Unknown(TypeModel::new(self.clone(), None, Punctuated::new()))
            });
        // println!("Type::<TypeModelKind>::resolve.2({}) in {} --> {}", self.to_token_stream(), source.scope.fmt_short(), result);
        result
    }
}

impl Resolve<Option<FFIFullPath>> for Type {
    fn resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        //println!("Type::<Option<FFIFullPath>>::resolve({})",self.to_token_stream());
        let res = match self {
            Type::Path(TypePath { path, .. }) =>
                path.resolve(source),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.resolve(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) =>
                Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() }),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                match bounds.len() {
                    0 => unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject (Empty)"),
                    1 => match bounds.first().unwrap() {
                        TypeParamBound::Trait(TraitBound { path, .. }) => path.resolve(source),
                        TypeParamBound::Lifetime(_) => unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject (Lifetime)"),
                    },
                    _ => Some(FFIFullPath::Generic { ffi_name: bounds.mangle_ident_default().to_path() }),
                }

            },
            _ => None
        };
        // println!("Type::<Option<FFIFullPath>>::resolve...2({}", res.to_token_stream());
        res
    }
}
impl Resolve<FFIFullPath> for Type {
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath {
        // println!("Type::<FFIFullPath>::resolve({})", self.to_token_stream());
        <Self as Resolve<Option<FFIFullPath>>>::resolve(self, source)
            .unwrap_or_else(|| {
                // println!("Type::<FFIFullPath>::resolve else ({})", self.to_token_stream());

                FFIFullPath::External { path: parse_quote!(#self) }
            })
    }
}

impl Resolve<Option<SpecialType>> for GenericTypeKind {
    fn resolve(&self, source: &ScopeContext) -> Option<SpecialType> {
        self.ty()
            .and_then(|ty| ty.resolve(source))
    }
}
impl Resolve<FFIFullPath> for GenericTypeKind {
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath {
        println!("GenericTypeKind -> FFIFullPath --> {}", self);
        let result = match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::AnyOther(ty) =>
                single_generic_ffi_type(ty),
            GenericTypeKind::Callback(ty) |
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) =>
                FFIFullPath::Generic { ffi_name: ty.mangle_ident_default().to_path() },
            GenericTypeKind::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => FFIFullPath::Dictionary { path: FFIFullDictionaryPath::Void },
                1 => single_generic_ffi_type(tuple.elems.first().unwrap()),
                _ => FFIFullPath::Generic { ffi_name: tuple.mangle_ident_default().to_path() }
            }
            GenericTypeKind::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => match TypeKind::from(ty) {
                        TypeKind::Generic(gen) => gen.resolve(source),
                        _ => single_generic_ffi_type(ty),
                    },
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", segments.to_token_stream()),
                },
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::Generic { ffi_name: args.mangle_ident_default().to_path() },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeKind::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_type(elem),
            GenericTypeKind::TraitBounds(bounds) => {
                println!("GenericTypeKind (TraitBounds): {}", bounds.to_token_stream());
                match bounds.len() {
                    1 => FFIFullPath::Generic {
                        ffi_name: {
                            if let Some(TypeParamBound::Trait(TraitBound  { path, .. })) = bounds.first() {
                                let ty = path.to_type();
                                match ty.maybe_special_type(source) {
                                    Some(SpecialType::Opaque(..)) => {
                                        println!("GenericTypeKind (TraitBounds: Opaque): {}", path.to_token_stream());
                                        return FFIFullPath::External { path: path.clone() }
                                    },
                                    Some(SpecialType::Custom(..)) => {
                                        println!("GenericTypeKind (TraitBounds: Custom): {}", path.to_token_stream());
                                        return FFIFullPath::External { path: path.clone() }
                                    },
                                    None => {}
                                }
                            }

                            bounds.first().unwrap().mangle_ident_default().to_path()
                        }
                    },
                    _ => FFIFullPath::Generic { ffi_name: bounds.mangle_ident_default().to_path() }
                }
            },
            gen_ty =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", gen_ty),
        };
        println!("GenericTypeKind -> FFIFullPath <-- {}", result.to_token_stream());
        result
    }
}


impl Resolve<Option<SpecialType>> for TypeModelKind {
    fn resolve(&self, source: &ScopeContext) -> Option<SpecialType> {
        // println!("Type::<Option<SpecialType>>::resolve({})", self.to_token_stream());
        self.as_type().resolve(source)
    }
}

impl Resolve<Option<FFIFullPath>> for Path {
    fn resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        // println!("Path::<Option<FFIFullPath>>::resolve({})", self.to_token_stream());
        let segments = &self.segments;
        let first_segment = segments.first().unwrap();
        let last_segment = segments.last().unwrap();
        let first_ident = &first_segment.ident;
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(FFIFullPath::Dictionary { path: FFIFullDictionaryPath::CChar })
        } else if last_ident.is_special_generic() ||
            (last_ident.is_result() && segments.len() == 1) ||
            last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json") || last_ident.is_lambda_fn() {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else if last_ident.is_optional() || last_ident.is_box() {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                    args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) =>
                            ty.resolve(source),
                        _ => None
                    }),
                _ => None
            }
        } else if last_ident.is_smart_ptr() {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else {
            let chunk =  if let Some(
                ObjectKind::Type(TypeModelKind::Trait(tc, ..)) |
                ObjectKind::Type(TypeModelKind::TraitType(tc))
            ) = self.maybe_trait_object(source) {
                &tc.as_type().to_path().segments
            } else {
                segments
            };
            maybe_crate_ident_replacement(&chunk.first().unwrap().ident, source)
                .map(|crate_ident| {
                    let crate_local_segments = chunk.crate_and_ident_less();
                    FFIFullPath::Type {
                        crate_ident: crate_ident.clone(),
                        ffi_name: if crate_local_segments.is_empty() {
                            let ty: Type = parse_quote!(#crate_ident::#last_ident);
                            ty.mangle_ident_default().to_path()
                        } else {
                            let no_ident_segments = chunk.ident_less();
                            let ty: Type = parse_quote!(#no_ident_segments::#last_ident);
                            let mangled_ty = ty.mangle_ident_default();
                            parse_quote!(#crate_local_segments::#mangled_ty)
                        }
                    }
                })
                .or_else(|| {
                    let segments = chunk.ident_less();
                    Some(FFIFullPath::External {
                        path: if segments.is_empty() {
                            last_ident.to_path()
                        } else {
                            parse_quote!(#segments::#last_ident)
                        }
                    })
                })
        }

    }
}


fn single_generic_ffi_type(ty: &Type) -> FFIFullPath {
    let path: Path = parse_quote!(#ty);
    let first_segment = path.segments.first().unwrap();
    let mut cloned_segments = path.segments.clone();
    let first_ident = &first_segment.ident;
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        FFIFullPath::External { path: last_ident.to_path() }
    } else if last_ident.is_any_string() {
        FFIFullPath::Dictionary { path: FFIFullDictionaryPath::CChar }
    } else if last_ident.is_special_generic() ||
        (last_ident.is_result() && path.segments.len() == 1) ||
        // TODO: avoid this hardcode
        (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) ||
        last_ident.is_smart_ptr() ||
        last_ident.is_lambda_fn() {
        FFIFullPath::Generic { ffi_name: path.mangle_ident_default().to_path() }
    } else {
        let new_segments = cloned_segments
            .into_iter()
            .map(|segment| quote_spanned! { segment.span() => #segment })
            .collect::<Vec<_>>();
        FFIFullPath::External { path: parse_quote!(#(#new_segments)::*) }

    }
}
fn maybe_crate_ident_replacement<'a>(ident: &'a Ident, source: &'a ScopeContext) -> Option<&'a Ident> {
    let lock = source.context.read().unwrap();
    match ident.to_string().as_str() {
        "crate" | _ if lock.config.is_current_crate(ident) => Some(source.scope.crate_ident_ref()),
        _ if lock.config.contains_fermented_crate(ident) =>
            Some(ident),
        _ => None
    }
}

