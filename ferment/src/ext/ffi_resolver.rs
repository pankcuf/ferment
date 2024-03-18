use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, PathSegment, Type, TypeArray, TypePath, TypeReference, TypeSlice, TypeTraitObject};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::composition::TypeComposition;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{Mangle, ResolveTrait};
use crate::helper::path_arguments_to_paths;

pub trait FFIResolver where Self: Sized + ToTokens + Parse + ResolveTrait {
    fn resolve(&self, source: &ScopeContext) -> Option<Self>;
    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self>;
    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self;
    fn ffi_path_converted_or_same(&self, source: &ScopeContext) -> Self {
        self.resolve(source)
            .unwrap_or(parse_quote!(#self))
    }

    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self;
}
impl FFIResolver for Path {

    fn resolve(&self, source: &ScopeContext) -> Option<Self> {
        let segments = &self.segments;
        let last_segment = segments.iter().last().unwrap();
        let last_ident = &last_segment.ident;
        let result = match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128"
            | "isize" | "usize" | "bool" => None,
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = self.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
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
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|item| item.resolve(source)),
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            _ => {
                let ffi_type = if let Some(
                    ObjectConversion::Type(TypeCompositionConversion::Trait(tc, ..)) |
                    ObjectConversion::Type(TypeCompositionConversion::TraitType(tc))
                ) = self.trait_ty(source) {
                    let trait_ty = &tc.ty;
                    let trait_path: Path = parse_quote!(#trait_ty);
                    println!("resolve::Complex Trait: {}", trait_path.to_token_stream());
                    ffi_chunk_converted(&trait_path.segments)
                } else {
                    println!("resolve::Complex Obj: {}", self.to_token_stream());
                    ffi_chunk_converted(&self.segments)
                };
                // println!("[{}] [{}] resolve_ffi_path: {} ----> {}", self.scope.crate_scope(), self.scope.self_path_holder(), ty.to_token_stream(), ffi_type.to_token_stream());
                Some(parse_quote!(#ffi_type))
            }
        };
        // println!("[{}] [{}] FFIResolver::resolve (Path): {} ----> {}", source.scope.crate_scope(), source.scope.self_path_holder(), self.to_token_stream(), result.to_token_stream());
        result
    }

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        let path = self;
        let crate_scope = source.scope.crate_scope();
        let lock = source.context.read().unwrap();
        let segments = &path.segments;
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;

        let last_segment = segments.iter().last().unwrap();
        let last_ident = &last_segment.ident;

        let result = match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
            "isize" | "usize" | "bool" => None,
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = path.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = path.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|ty| ty.ffi_external_path_converted(source)),
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            _ => {
                match first_ident.to_string().as_str() {
                    "crate" | _ if lock.config.is_current_crate(first_ident) =>
                        Some(ffi_external_chunk(crate_scope, segments)),
                    _ if lock.config.contains_fermented_crate(first_ident) =>
                        Some(ffi_external_chunk(first_ident, segments)),
                    _ => {
                        let segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                        Some(if segments.is_empty() { parse_quote!(#last_ident) } else { parse_quote!(#(#segments)::*::#last_ident) })
                    }
                }
            }
        };
        // println!("[{}] [{}] FFIResolver::ffi_external_path_converted (Path): {} ----> {}", source.scope.crate_scope(), source.scope.self_path_holder(), self.to_token_stream(), result.to_token_stream());

        result
    }

    fn ffi_internal_type_for(&self, source: &ScopeContext) -> Self {
        let lock = source.context.read().unwrap();
        let ty: Type = parse_quote!(#self);
        let tyty = lock.maybe_type(&ty, &source.scope)
            .and_then(|external_type| {
                match external_type.type_conversion() {
                    Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
                        ty.ty.trait_ty(source)
                            .map(|oc| oc.type_conversion().cloned()),
                    _ => None
                }.unwrap_or(external_type.type_conversion().cloned())
            }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(ty.clone(), None)));

        tyty.ty()
            .ffi_external_path_converted(source)
            .map_or(self.clone(), |ty| parse_quote!(#ty))
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
        let ty: Type = parse_quote!(#self);
        lock.custom.maybe_conversion(&ty)
            .map_or(self.ffi_internal_type_for(source), |ty| parse_quote!(#ty))
    }
}

impl FFIResolver for Type {
    fn resolve(&self, source: &ScopeContext) -> Option<Self> {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.resolve(source)
                    .map(|ffi_path| parse_quote!(#ffi_path)),
            Type::Array(TypeArray { elem , ..}) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.resolve(source),
            Type::TraitObject(TypeTraitObject { .. }) => {
                unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject")
            },
            Type::Tuple(type_tuple) => {
                let ffi_chunk = type_tuple.mangle_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_chunk))
            }
            // Type::Tuple(TypeTuple { elems, .. }) => {
            //     let ty = elems.iter().filter_map(|ty| ty.resolve(source).map(|ty| ty.to_mangled_ident_default())).collect::<Punctuated<Ident, Underscore>>();
            //     println!("FFIResolver::resolve::Tuple:: {}", ty.to_token_stream());
            //     Some(parse_quote!(crate::fermented::generics::#ty))
            // }
            _ => None
        }
    }

    fn ffi_external_path_converted(&self, source: &ScopeContext) -> Option<Self> {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.ffi_external_path_converted(source)
                    .map(|ffi_path| parse_quote!(#ffi_path)),
            Type::Array(TypeArray { elem , ..}) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => elem.ffi_external_path_converted(source),
            // Type::TraitObject(_) => {}
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
            }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(self.clone(), None)));

        tyty.ty()
            .ffi_external_path_converted(source)
            .unwrap_or(self.clone())
    }

    fn ffi_custom_or_internal_type(&self, source: &ScopeContext) -> Self {
        let lock = source.context.read().unwrap();
        lock.custom.maybe_conversion(self)
            .unwrap_or(self.ffi_internal_type_for(source))
    }
}

pub fn ffi_chunk_converted(segments: &Punctuated<PathSegment, Colon2>) -> Type {
    let crate_local_segments: Vec<_> = match segments.first().unwrap().ident.to_string().as_str() {
        "crate" => segments.iter().take(segments.len() - 1).skip(1).collect(),
        _ => segments.iter().take(segments.len() - 1).collect()
    };
    let ffi_path_chunk = if crate_local_segments.is_empty() {
        segments.mangle_ident_default()
            .to_token_stream()
    } else {
        let mangled_ty = segments.mangle_ident_default();
        quote!(#(#crate_local_segments)::*::#mangled_ty)
    };
    parse_quote!(crate::fermented::types::#ffi_path_chunk)
}
pub fn ffi_external_chunk<T: FFIResolver>(crate_ident: &Ident, segments: &Punctuated<PathSegment, Colon2>) -> T {
    let crate_local_segments: Vec<_> = segments.iter().take(segments.len() - 1).skip(1).collect();
    let last_ident = &segments.iter().last().unwrap().ident;

    let ffi_chunk_path = if crate_local_segments.is_empty() {
        let ty: Type = parse_quote!(#crate_ident::#last_ident);
        let mangled_ty = ty.mangle_ident_default();
        mangled_ty.to_token_stream()
    } else {
        let no_ident_segments = segments.iter().take(segments.len() - 1).collect::<Vec<_>>();
        let ty: Type = parse_quote!(#(#no_ident_segments)::*::#last_ident);
        let mangled_ty = ty.mangle_ident_default();
        quote!(#(#crate_local_segments)::*::#mangled_ty)
    };
    parse_quote!(crate::fermented::types::#crate_ident::#ffi_chunk_path)
}
