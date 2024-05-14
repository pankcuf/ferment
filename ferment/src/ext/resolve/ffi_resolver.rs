use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::composition::TypeComposition;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion};
use crate::ext::{Accessory, CrateExtension, DictionaryType, Mangle, ResolveTrait, ToPath, ToType};
use crate::helper::path_arguments_to_paths;

pub trait FFIResolve where Self: Sized + ToTokens + Parse {
    fn ffi_resolve(&self, source: &ScopeContext) -> Option<Self>;
    fn ffi_resolve_or_same(&self, source: &ScopeContext) -> Self {
        self.ffi_resolve(source)
            .unwrap_or(parse_quote!(#self))
    }
}

pub trait FFITypeResolve {
    fn to_custom_or_ffi_type(&self, source: &ScopeContext) -> Type;
    fn to_custom_or_ffi_type_mut_ptr(&self, source: &ScopeContext) -> Type {
        self.to_custom_or_ffi_type(source).joined_mut()
    }
}

impl FFITypeResolve for Type where Self: FFIResolve {
    fn to_custom_or_ffi_type(&self, source: &ScopeContext) -> Self {
        source.maybe_custom_conversion(self)
            .unwrap_or(self.ffi_resolve_or_same(source))
    }
}

impl FFITypeResolve for GenericTypeConversion {
    fn to_custom_or_ffi_type(&self, source: &ScopeContext) -> Type {
        self.ty()
            .and_then(|ty| source.maybe_custom_conversion(ty)
                .map(|ty| ty.clone()))
            .unwrap_or(self.to_ffi_type())
    }
}


pub trait FFIResolveExtended: FFIResolve where Self: ResolveTrait {
    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self>;
    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self;
    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self;
    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self;
    fn ffi_full_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
        // println!("ffi_full_dictionary_type_presenter: {}", self.to_token_stream());
        self.ffi_custom_or_internal_type(source)
            .ffi_dictionary_type_presenter(source)
    }

}

impl FFIResolve for Path {
    fn ffi_resolve(&self, source: &ScopeContext) -> Option<Self> {
        // println!("Path::ffi_resolve.1: {}", self.to_token_stream());
        let segments = &self.segments;
        let first_segment = segments.first().unwrap();
        let last_segment = segments.last().unwrap();
        let first_ident = &first_segment.ident;
        let last_ident = &last_segment.ident;
        let result = if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(parse_quote!(std::os::raw::c_char))
        } else if last_ident.is_special_generic() || (last_ident.is_result() && segments.len() == 1) {
            let ffi_name = self.mangle_ident_default();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        } else if last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json") {
            let ffi_name = self.mangle_ident_default();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        } else if last_ident.is_optional() {
            path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|path| path.ffi_resolve(source))
        } else if last_ident.is_box() {
            path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|item| {
                    // let path = match item {
                    //     Type::TraitObject(TypeTraitObject { bounds , .. }) => {
                    //         bounds.iter().find_map(|bound| match bound {
                    //             TypeParamBound::Trait(TraitBound { path, .. }) => {
                    //                 let p: Path = parse_quote!(#path);
                    //                 Some(p)
                    //             }
                    //             TypeParamBound::Lifetime(_) => None
                    //         }).unwrap()
                    //     },
                    //     _ => parse_quote!(#item)
                    // };
                    Some(item.ffi_custom_or_internal_type(source))
                })
        } else {
            let ffi_type = if let Some(
                ObjectConversion::Type(TypeCompositionConversion::Trait(tc, ..)) |
                ObjectConversion::Type(TypeCompositionConversion::TraitType(tc))
            ) = self.trait_ty(source) {
                ffi_chunk_converted(&tc.ty.to_path().segments)
            } else {
                ffi_chunk_converted(segments)
            };
            // println!("[{}] [{}] resolve_ffi_path: {} ----> {}", self.scope.crate_scope(), self.scope.self_path_holder(), ty.to_token_stream(), ffi_type.to_token_stream());
            Some(ffi_type.to_path())
        };
        // println!("Path::ffi_resolve.2: {} --> {}", self.to_token_stream(), result.to_token_stream());
        result
        // "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
        // "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
    }
}

impl FFIResolve for TypePath {
    fn ffi_resolve(&self, source: &ScopeContext) -> Option<Self> {
        self.path.ffi_resolve(source)
            .map(|ffi_path| parse_quote!(#ffi_path))
    }
}

impl FFIResolve for Type {
    fn ffi_resolve(&self, source: &ScopeContext) -> Option<Self> {
        match self {
            Type::Path(type_path) =>
                type_path.ffi_resolve(source).map(TypePath::into),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.ffi_resolve(source),
            Type::TraitObject(TypeTraitObject { bounds: _, .. }) => {
                unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject")
            },
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) => {
                let ffi_chunk = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_chunk))
            }
            _ => None
        }
    }
}


impl FFIResolveExtended for Path {

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        // println!("Path::ffi_external_path_converted.1: {}", self.to_token_stream());
        let segments = &self.segments;
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let result = if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(parse_quote!(std::os::raw::c_char))
        } else if last_ident.is_special_generic() || (last_ident.is_result() && segments.len() == 1) {
            let ffi_name = self.mangle_ident_default();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        } else if last_ident.to_string().eq("Map") || first_ident.to_string().eq("serde_json") {
            let ffi_name = self.mangle_ident_default();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        } else if last_ident.is_optional() || last_ident.is_box() {
            path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|path| path.ffi_external_path_converted(source))
        } else {
            let lock = source.context.read().unwrap();
            let crate_ident = source.scope.crate_ident();
            let first_segment = segments.first().unwrap();
            let first_ident = &first_segment.ident;

            match first_ident.to_string().as_str() {
                "crate" | _ if lock.config.is_current_crate(first_ident) =>
                    Some(ffi_external_chunk(crate_ident, segments)),
                _ if lock.config.contains_fermented_crate(first_ident) =>
                    Some(ffi_external_chunk(first_ident, segments)),
                _ => {
                    let segments = segments.ident_less();
                    Some(if segments.is_empty() { last_ident.to_path() } else { parse_quote!(#segments::#last_ident) })
                }
            }
        };
        // println!("Path::ffi_external_path_converted.2: {}", self.to_token_stream());
        result
        // "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
        // "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
    }

    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self {
        // println!("Path::ffi_internal_type_for: {}", self.to_token_stream());
        let lock = source.context.read().unwrap();
        let ty = self.to_type();
        let tyty = lock.maybe_type(&ty, &source.scope)
            .and_then(|external_type| {
                match external_type.type_conversion() {
                    Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
                        ty.ty.trait_ty(source)
                            .map(|oc| oc.type_conversion().cloned()),
                    _ => None
                }.unwrap_or(external_type.type_conversion().cloned())
            }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(ty, None, Punctuated::new())));

        tyty.to_ty()
            .ffi_external_path_converted(source)
            .map_or(self.clone(), |ty| ty.to_path())
        // let tyty = lock.maybe_scope_type(ty, &self.scope)
        //     .and_then(|external_type| {
        //         match external_type.type_conversion() {
        //             Some(TypeCompositionConversion::Trait(ty, _decomposition)) =>
        //                 self.trait_ty(&ty.ty)
        //                     .map(|oc| oc.type_conversion().cloned()),
        //             _ => None
        //         }.unwrap_or(external_type.type_conversion().cloned())
        //
        //     }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(ty.clone(), None)));
        // let result = match tyty.ty() {
        //     Type::Path(TypePath { path, .. }) =>
        //         path.ffi_external_path_converted(source)
        //             .map(|path| parse_quote!(#path)),
        //     _ => None
        // };
        // let result = result.unwrap_or(ty.clone());
    }

    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self {
        let lock = source.context.read().unwrap();
        // println!("Path::ffi_custom_or_internal_type: {}", self.to_token_stream());
        let ty: Type = parse_quote!(#self);
        let full_ty = source.full_type_for(&ty);
        lock.custom.maybe_conversion(&full_ty)
            .map_or(self.ffi_internal_type_for(source), |ty| ty.to_path())
    }

    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
        // println!("Path::ffi_dictionary_type_presenter: {}", self.to_token_stream());
        ffi_dictionary_type(self, source)
            .to_path()
    }
}

impl FFIResolveExtended for Type {

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        // println!("Type::ffi_external_path_converted: {}", self.to_token_stream());
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.ffi_external_path_converted(source)
                    .map(|path| path.to_type()),
            Type::Reference(TypeReference { elem, .. }) => elem.ffi_external_path_converted(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) => {
                let ffi_chunk = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_chunk))
            }
            _ => None
        }
    }

    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self {
        // println!("Type::ffi_internal_type_for: {}", self.to_token_stream());
        let lock = source.context.read().unwrap();
        let tyty = lock.maybe_type(self, &source.scope)
            .and_then(|external_type| {
                match external_type.type_conversion() {
                    Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
                        ty.ty.trait_ty(source)
                            .map(|oc| oc.type_conversion().cloned()),
                    _ => None
                }.unwrap_or(external_type.type_conversion().cloned())
            }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(self.clone(), None, Punctuated::new())));

        tyty.to_ty()
            .ffi_external_path_converted(source)
            .unwrap_or(self.clone())
    }

    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self {
        let lock = source.context.read().unwrap();

        let full_ty = source.full_type_for(self);
        // println!("Type::ffi_custom_or_internal_type: {}   ({})", self.to_token_stream(), full_ty.to_token_stream());
        lock.custom.maybe_conversion(&full_ty)
            .unwrap_or(self.ffi_internal_type_for(source))
    }

    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
        // println!("Type::ffi_dictionary_type_presenter: {}", self.to_token_stream());
        match self {
            Type::Path(TypePath { path, .. }) =>
                ffi_dictionary_type(path, source),
            Type::Array(TypeArray { elem, len, .. }) =>
                parse_quote!(*mut [#elem; #len]),
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.ffi_dictionary_type_presenter(source),
            Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
                match &**elem {
                    Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                        "c_void" => match (star_token, const_token, mutability) {
                            (_, Some(_const_token), None) => parse_quote!(OpaqueContext_FFI),
                            (_, None, Some(_mut_token)) => parse_quote!(OpaqueContextMut_FFI),
                            _ => panic!("ffi_dictionary_type_presenter: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                        },
                        _ => parse_quote!(*mut #path)
                    },
                    Type::Ptr(type_ptr) => parse_quote!(*mut #type_ptr),
                    _ => self.clone()
                },
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                let bound = bounds.iter().find_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
                    TypeParamBound::Lifetime(_) => None
                }).unwrap();
                bound.ffi_dictionary_type_presenter(source)
            },
            Type::Tuple(type_tuple) =>
                type_tuple.mangle_ident_default()
                    .to_type(),
            Type::ImplTrait(type_impl_trait) =>
                type_impl_trait.mangle_ident_default()
                    .to_type(),
            _ => panic!("ffi_dictionary_type_presenter: type not supported: {}", self.to_token_stream())
        }
    }
}

pub fn ffi_chunk_converted(segments: &Punctuated<PathSegment, Colon2>) -> Type {
    let crate_local_segments = match segments.first().unwrap().ident.to_string().as_str() {
        "crate" => segments.crate_and_ident_less(),
        _ => segments.ident_less()
    };
    let mangled_segments_ident = segments.mangle_ident_default();
    let ffi_path_chunk = if crate_local_segments.is_empty() {
        mangled_segments_ident
            .to_token_stream()
    } else {
        quote!(#crate_local_segments::#mangled_segments_ident)
    };
    parse_quote!(crate::fermented::types::#ffi_path_chunk)
}
pub fn ffi_external_chunk<T: FFIResolveExtended>(crate_ident: &Ident, segments: &Punctuated<PathSegment, Colon2>) -> T {
    let crate_local_segments = segments.crate_and_ident_less();
    let last_ident = &segments.iter().last().unwrap().ident;

    let ffi_chunk_path = if crate_local_segments.is_empty() {
        let ty: Type = parse_quote!(#crate_ident::#last_ident);
        let mangled_ty = ty.mangle_ident_default();
        mangled_ty.to_token_stream()
    } else {
        let no_ident_segments = segments.ident_less();
        let ty: Type = parse_quote!(#no_ident_segments::#last_ident);
        let mangled_ty = ty.mangle_ident_default();
        quote!(#crate_local_segments::#mangled_ty)
    };
    parse_quote!(crate::fermented::types::#crate_ident::#ffi_chunk_path)
}
pub fn ffi_dictionary_type(path: &Path, source: &ScopeContext) -> Type {
    let first_segment = path.segments.first().unwrap();
    let first_ident = &first_segment.ident;
    let last_segment = path.segments.last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        path.to_type()
    } else if last_ident.is_optional() {
        ffi_dictionary_type(path_arguments_to_paths(&last_segment.arguments).first().unwrap(), source)
    } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
        source.scope_type_for_path(path)
            .map_or(path.to_token_stream(), |full_type| full_type.mangle_ident_default().to_token_stream())
            .joined_mut()
            .to_type()
    } else {
        path.to_type()
            .joined_mut()
    }
}
