use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::composition::TypeComposition;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{Accessory, CrateExtension, Mangle, ResolveTrait, ToPath, ToType};
use crate::helper::path_arguments_to_paths;

pub trait FFIResolve where Self: Sized + ToTokens + Parse {
    fn resolve(&self, source: &ScopeContext) -> Option<Self>;
    fn resolve_or_same(&self, source: &ScopeContext) -> Self {
        self.resolve(source)
            .unwrap_or(parse_quote!(#self))
    }
}

pub trait FFIResolveExtended: FFIResolve where Self: ResolveTrait {
    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self>;
    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self;
    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self;
    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self;
    fn ffi_full_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
        self.ffi_custom_or_internal_type(source)
            .ffi_dictionary_type_presenter(source)
    }
}

impl FFIResolve for Path {
    fn resolve(&self, source: &ScopeContext) -> Option<Self> {
        let segments = &self.segments;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let result = match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128"
            | "isize" | "usize" | "bool" => None,
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|path| path.resolve(source)),
            "Box" => path_arguments_to_paths(&last_segment.arguments)
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
                }),
            _ => {
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
            }
        };
        // println!("[{}] [{}] FFIResolver::resolve (Path): {} ----> {}", source.scope.crate_scope(), source.scope.self_path_holder(), self.to_token_stream(), result.to_token_stream());
        result
    }
}

impl FFIResolve for TypePath {
    fn resolve(&self, source: &ScopeContext) -> Option<Self> {
        self.path.resolve(source)
            .map(|ffi_path| parse_quote!(#ffi_path))
    }
}

impl FFIResolve for Type {
    fn resolve(&self, source: &ScopeContext) -> Option<Self> {
        match self {
            Type::Path(type_path) =>
                type_path.resolve(source).map(TypePath::into),
            Type::Array(TypeArray { elem , ..}) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.resolve(source),
            Type::TraitObject(TypeTraitObject { bounds: _, .. }) => {
                unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject")
            },
            Type::Tuple(type_tuple) => {
                let ffi_chunk = type_tuple.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_chunk))
            },
            _ => None
        }
    }
}


impl FFIResolveExtended for Path {

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        let segments = &self.segments;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let result = match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
            "isize" | "usize" | "bool" => None,
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|path| path.ffi_external_path_converted(source)),
            _ => {
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
            }
        };
        // println!("[{}] [{}] FFIResolver::ffi_external_path_converted (Path): {} ----> {}", source.scope.crate_scope(), source.scope.self_path_holder(), self.to_token_stream(), result.to_token_stream());

        result
    }

    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self {
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
        lock.custom.maybe_conversion(&self.to_type())
            .map_or(self.ffi_internal_type_for(source), |ty| ty.to_path())
    }

    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
        ffi_dictionary_type(self, source)
            .to_path()
    }
}

impl FFIResolveExtended for Type {

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.ffi_external_path_converted(source)
                    .map(|path| path.to_type()),
            Type::Array(TypeArray { elem , ..}) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => elem.ffi_external_path_converted(source),
            Type::Tuple(type_tuple) => {
                let ffi_chunk = type_tuple.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_chunk))
            }
            _ => None
        }
    }

    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self {
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
        lock.custom.maybe_conversion(self)
            .unwrap_or(self.ffi_internal_type_for(source))
    }

    fn ffi_dictionary_type_presenter(&self, source: &ScopeContext) -> Self {
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
                            _ => panic!("extract_struct_field: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
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
            _ => panic!("FFI_DICTIONARY_TYPE_PRESENTER: type not supported: {}", self.to_token_stream())
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
    // println!("ffi_dictionary_type: {}", format_token_stream(path));
    match path.segments.last().unwrap().ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
        "isize" | "usize" | "bool" =>
            path.to_type(),
        "OpaqueContext" =>
            parse_quote!(ferment_interfaces::OpaqueContext_FFI),
        "OpaqueContextMut" =>
            parse_quote!(ferment_interfaces::OpaqueContextMut_FFI),
        "Option" =>
            ffi_dictionary_type(path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap(), source),
        "Vec" | "BTreeMap" | "HashMap" => {
            source.scope_type_for_path(path)
                .map_or(path.to_token_stream(), |full_type| full_type.mangle_ident_default().to_token_stream())
                .joined_mut()
                .to_type()
        },
        "Result" /*if path.segments.len() == 1*/ => {
            source.scope_type_for_path(path)
                .map_or(path.to_token_stream(), |full_type| full_type.mangle_ident_default().to_token_stream())
                .joined_mut()
                .to_type()
        },
        _ =>
            path.to_type()
                .joined_mut()
    }
}
