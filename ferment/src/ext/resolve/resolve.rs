use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, Type, TypeArray, TypePath, TypeReference, TypeTraitObject};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use crate::composable::TypeComposition;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{CrateExtension, DictionaryType, Mangle, ResolveTrait, ToPath};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

pub trait Resolve<T> {
    fn resolve(&self, source: &ScopeContext) -> T;
}

impl Resolve<Type> for Type {
    fn resolve(&self, source: &ScopeContext) -> Type {
        source.full_type_for(self)
    }
}

// impl Resolve<ObjectConversion> for Type {
//     fn resolve(&self, source: &ScopeContext) -> ObjectConversion {
//         match source.maybe_object(self) {
//             Some(object) => object,
//             other => panic!("<Type as Resolve<ObjectConversion>>::resolve: {:?}", other)
//         }
//     }
// }

impl Resolve<TypeCompositionConversion> for Type {
    fn resolve(&self, source: &ScopeContext) -> TypeCompositionConversion {
        let lock = source.context.read().unwrap();
        lock.maybe_object(self, &source.scope)
            .and_then(|external_type| match external_type.type_conversion() {
                Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
                    ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned()),
                _ => None
            }.unwrap_or(external_type.type_conversion().cloned()))
            .unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(self.clone(), None, Punctuated::new())))
    }
}

impl Resolve<FFIFullPath> for Type {
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath {
        <Self as Resolve<Option<FFIFullPath>>>::resolve(self, source)
            .unwrap_or(FFIFullPath::External { path: parse_quote!(#self) })
    }
}

impl Resolve<Option<FFIFullPath>> for Type {
    fn resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        match self {
            Type::Path(TypePath{ path, .. }) =>
                path.resolve(source),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.resolve(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) =>
                Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() }),
            Type::TraitObject(TypeTraitObject { bounds: _, .. }) => {
                unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject")
            },
            _ => None
        }
    }
}
impl Resolve<Option<FFIFullPath>> for Path {
    fn resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
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
        } else if last_ident.is_optional() {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                    args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) =>
                            ty.resolve(source),
                        _ => None
                    }),
                _ => None
            }
        } else if last_ident.is_box() {
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
                ObjectConversion::Type(TypeCompositionConversion::Trait(tc, ..)) |
                ObjectConversion::Type(TypeCompositionConversion::TraitType(tc))
            ) = self.maybe_trait_object(source) {
                &tc.ty.to_path().segments
            } else {
                segments
            };
            crate_ident_replacement(&chunk.first().unwrap().ident, source)
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
                .or({
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


impl Resolve<FFIFullPath> for GenericTypeConversion {
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::AnyOther(ty) =>
                single_generic_ffi_type(ty),
            GenericTypeConversion::Callback(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) =>
                FFIFullPath::Generic { ffi_name: ty.mangle_ident_default().to_path() },
            GenericTypeConversion::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => single_generic_ffi_type(tuple.elems.first().unwrap()),
                _ => FFIFullPath::Generic { ffi_name: tuple.mangle_ident_default().to_path() }
            }
            GenericTypeConversion::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => match TypeConversion::from(ty) {
                        TypeConversion::Generic(gen) => gen.resolve(source),
                        _ => single_generic_ffi_type(ty),
                    },
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", segments.to_token_stream()),
                },
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::Generic { ffi_name: args.mangle_ident_default().to_path() },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeConversion::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_type(elem),
            gen_ty =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", gen_ty.to_token_stream()),
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
fn crate_ident_replacement<'a>(ident: &'a Ident, source: &'a ScopeContext) -> Option<&'a Ident> {
    let lock = source.context.read().unwrap();
    match ident.to_string().as_str() {
        "crate" | _ if lock.config.is_current_crate(ident) => Some(source.scope.crate_ident()),
        _ if lock.config.contains_fermented_crate(ident) =>
            Some(ident),
        _ => None
    }
}

