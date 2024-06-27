use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use ferment_macro::Display;
use crate::composable::TypeComposition;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, ObjectConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{Accessory, CrateExtension, DictionaryType, Mangle, Resolve, ResolveTrait, ToPath, ToType};
use crate::ext::item::path_arguments_to_type_conversions;

#[allow(unused)]
#[derive(Display)]
pub enum GlobalType {
    /// Traits
    Clone, // For types that can be cloned.
    Copy, // For types that can be copied.
    Debug, // For types that can be formatted using {, //?}.
    Default, // For types that have a default value.
    Drop, // For types that need to run code on destruction.
    Eq, // For types that can be compared for equality.
    PartialEq, // For types that can be compared for partial equality.
    Ord, // For types that can be compared for ordering.
    PartialOrd, // For types that can be compared for partial ordering.
    Hash, // For types that can be hashed.
    From, // For types that can be created from another type.
    Into, // For types that can be converted into another type.
    AsRef, // For types that can be referenced as another type.
    AsMut, // For types that can be mutably referenced as another type.
    Borrow, // For types that can be borrowed as another type.
    BorrowMut, // For types that can be mutably borrowed as another type.
    Deref, // For types that can be dereferenced to another type.
    DerefMut, // For types that can be mutably dereferenced to another type.
    Iterator, // For types that can be iterated over.
    DoubleEndedIterator, // For iterators that can be iterated from both ends.
    ExactSizeIterator, // For iterators with a known exact length.
    Fn,
    FnMut,
    FnOnce, // For types that can be called as functions.

    /// Types
    Box, // For heap-allocated values.
    Vec, // For growable arrays.
    String, // For heap-allocated strings.
    Option, // For optional values.
    Result, // For error handling.
}

pub enum FFIFullDictionaryPath {
    Void,
    CChar
}
#[allow(unused)]
pub enum FFIFullDictionaryVariable {
    Void,
    CChar
}
impl ToType for FFIFullDictionaryVariable {
    fn to_type(&self) -> Type {
        match self {
            FFIFullDictionaryVariable::Void => FFIFullDictionaryPath::Void.to_type(),
            FFIFullDictionaryVariable::CChar => FFIFullDictionaryPath::CChar.to_type(),
        }
    }
}
impl ToPath for FFIFullDictionaryVariable {
    fn to_path(&self) -> Path {
        self.to_type()
            .to_path()
    }
}

impl ToType for FFIFullDictionaryPath {
    fn to_type(&self) -> Type {
        match self {
            FFIFullDictionaryPath::Void => parse_quote!(std::os::raw::c_void),
            FFIFullDictionaryPath::CChar => parse_quote!(std::os::raw::c_char),
        }
    }
}
impl ToPath for FFIFullDictionaryPath {
    fn to_path(&self) -> Path {
        self.to_type()
            .to_path()
    }
}

pub enum FFIFullPath {
    Type {
        crate_ident: Ident,
        ffi_name: Path,
    },
    Generic {
        ffi_name: Path
    },
    External {
        path: Path,
    },
    Dictionary {
        path: FFIFullDictionaryPath
    },
}

impl ToPath for FFIFullPath {
    fn to_path(&self) -> Path {
        match self {
            FFIFullPath::Type { crate_ident, ffi_name } =>
                parse_quote!(crate::fermented::types::#crate_ident::#ffi_name),
            FFIFullPath::Generic { ffi_name } =>
                parse_quote!(crate::fermented::generics::#ffi_name),
            FFIFullPath::External { path } =>
                parse_quote!(#path),
            FFIFullPath::Dictionary { path } =>
                path.to_path(),
        }
    }
}
impl ToType for FFIFullPath {
    fn to_type(&self) -> Type {
        match self {
            FFIFullPath::Type { crate_ident, ffi_name } =>
                parse_quote!(crate::fermented::types::#crate_ident::#ffi_name),
            FFIFullPath::Generic { ffi_name } =>
                parse_quote!(crate::fermented::generics::#ffi_name),
            FFIFullPath::External { path } =>
                parse_quote!(#path),
            FFIFullPath::Dictionary { path } =>
                path.to_type(),
        }
    }
}

pub trait ToFFIFullPath {
    fn to_ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath;
    fn to_ffi_full_type(&self, source: &ScopeContext) -> Type {
        self.to_ffi_full_path(source)
            .to_type()
    }
}

pub trait FFIResolve: Sized + ToTokens + Parse {
    fn maybe_ffi_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath>;
}

/// Types that are exported with [ferment_macro::export] or can be mixin types from exported crates
pub trait FFIInternalType {
    fn maybe_trait_or_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath>;
}

pub trait FFISpecialTypeResolve {
    /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
    /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type>;
}

pub trait FFIFullPathResolve: FFIResolve + ResolveTrait + Clone {
    fn maybe_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath>;
    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath>;
}

pub trait FFITypeResolve: FFISpecialTypeResolve + ToFFIFullPath {
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        self.maybe_special_type(source)
            .unwrap_or(self.to_ffi_full_type(source))
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
pub trait FFIVariableResolve: FFIFullPathResolve {
    fn to_ffi_variable(&self, source: &ScopeContext) -> Type;
    fn to_full_ffi_variable(&self, source: &ScopeContext) -> Type {
        self.maybe_special_or_trait_ffi_full_path(source)
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or(parse_quote!(#self))
            .to_type()
            .to_ffi_variable(source)
    }
}

// pub trait FFIFnArgResolve {
//     fn to_
// }

impl FFIInternalType for Type {
    fn maybe_trait_or_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        let lock = source.context.read().unwrap();
        lock.maybe_object(self, &source.scope)
            .and_then(|external_type| match external_type.type_conversion() {
                Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
                    ty.ty.maybe_trait_object(source).map(|oc| oc.type_conversion().cloned()),
                _ => None
                }.unwrap_or(external_type.type_conversion().cloned()))
            .unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(self.clone(), None, Punctuated::new())))
            .to_ty()
            .maybe_ffi_full_path(source)
    }
}

impl FFIInternalType for Path {
    fn maybe_trait_or_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        self.to_type()
            .maybe_trait_or_ffi_full_path(source)
    }
}


impl ToFFIFullPath for Type {
    fn to_ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath {
        self.maybe_ffi_resolve(source)
            .unwrap_or(FFIFullPath::External { path: parse_quote!(#self) })
    }
}

impl ToFFIFullPath for GenericTypeConversion {
    fn to_ffi_full_path(&self, source: &ScopeContext) -> FFIFullPath {
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
                        TypeConversion::Generic(gen) => gen.to_ffi_full_path(source),
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

impl FFISpecialTypeResolve for Type {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type> {
        source.maybe_custom_conversion(self)
            .or_else(|| source.maybe_opaque_object(self))
    }
}
impl FFISpecialTypeResolve for GenericTypeConversion {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type> {
        self.ty()
            .and_then(|ty| ty.maybe_special_type(source))
    }
}

impl FFITypeResolve for Type where Self: FFIResolve {}
impl FFITypeResolve for GenericTypeConversion {}

impl FFIResolve for Path {
    fn maybe_ffi_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
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
                            ty.maybe_ffi_resolve(source),
                        _ => None
                    }),
                _ => None
            }
        } else if last_ident.is_box() {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                    args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) =>
                            ty.maybe_special_or_trait_ffi_full_path(source),
                        _ => None
                    }),
                _ => None
            }
        } else {
            let chunk =  if let Some(
                ObjectConversion::Type(TypeCompositionConversion::Trait(tc, ..)) |
                ObjectConversion::Type(TypeCompositionConversion::TraitType(tc))
            ) = self.maybe_trait_object(source) {
                &tc.ty.to_path().segments
            } else {
                segments
            };
            let crate_local_segments = match chunk.first().unwrap().ident.to_string().as_str() {
                "crate" => chunk.crate_and_ident_less(),
                _ => chunk.ident_less()
            };
            let mangled_segments_ident = chunk.mangle_ident_default();
            let ffi_path_chunk = if crate_local_segments.is_empty() {
                mangled_segments_ident.to_token_stream()
            } else {
                quote!(#crate_local_segments::#mangled_segments_ident)
            };
            Some(FFIFullPath::External { path: parse_quote!(crate::fermented::types::#ffi_path_chunk) })
        }
    }
}

impl FFIResolve for TypePath {
    fn maybe_ffi_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        self.path.maybe_ffi_resolve(source)
    }
}

impl FFIResolve for Type {
    fn maybe_ffi_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        match self {
            Type::Path(type_path) =>
                type_path.maybe_ffi_resolve(source),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.maybe_ffi_resolve(source),
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


impl FFIFullPathResolve for Path {
    fn maybe_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        let segments = &self.segments;
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(FFIFullPath::Dictionary { path: FFIFullDictionaryPath::CChar })
        } else if last_ident.is_special_generic() ||
            (last_ident.is_result() && segments.len() == 1) ||
            (last_ident.to_string().eq("Map") || first_ident.to_string().eq("serde_json"))  {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else if last_ident.is_optional() || last_ident.is_box() {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                    args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) =>
                            ty.maybe_ffi_full_path(source),
                        _ => None
                    }),
                _ => None
            }
        } else if last_ident.is_smart_ptr() {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else {
            crate_ident_replacement(&segments.first().unwrap().ident, source)
                .map(|crate_ident| {
                    let crate_local_segments = segments.crate_and_ident_less();
                    FFIFullPath::Type {
                        crate_ident: crate_ident.clone(),
                        ffi_name: if crate_local_segments.is_empty() {
                            let ty: Type = parse_quote!(#crate_ident::#last_ident);
                            ty.mangle_ident_default().to_path()
                        } else {
                            let no_ident_segments = segments.ident_less();
                            let ty: Type = parse_quote!(#no_ident_segments::#last_ident);
                            let mangled_ty = ty.mangle_ident_default();
                            parse_quote!(#crate_local_segments::#mangled_ty)
                        }
                    }
                })
                .or({
                    let segments = segments.ident_less();
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

    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        self.to_type()
            .resolve(source)
            .maybe_special_type(source)
            .map(|ty| FFIFullPath::External { path: ty.to_path() })
            .or(self.maybe_trait_or_ffi_full_path(source))
    }

}

impl FFIVariableResolve for Path {
    fn to_ffi_variable(&self, source: &ScopeContext) -> Type {
        let first_segment = self.segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = self.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            self.to_type()
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeConversion::Primitive(ty)) =>
                    ty.to_path().to_full_ffi_variable(source)
                        .joined_mut(),
                Some(TypeConversion::Generic(generic_ty)) =>
                    generic_ty.to_ffi_full_type(source)
                        .joined_mut(),
                Some(TypeConversion::Complex(Type::Path(TypePath { path, .. }))) =>
                    path.to_ffi_variable(source),
                _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
            }
        } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
            source.scope_type_for_path(self)
                .map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default())
                .to_type()
                .joined_mut()
        } else {
            self.to_type()
                .joined_mut()
        }
    }
}

impl FFIFullPathResolve for Type {
    fn maybe_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.maybe_ffi_full_path(source)
                    .map(|path| path),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.maybe_ffi_full_path(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) =>
                Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() }),
            _ => None
        }
    }

    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        self.resolve(source)
            .maybe_special_type(source)
            .map(|ty| FFIFullPath::External { path: ty.to_path() })
            .or(self.maybe_trait_or_ffi_full_path(source))
    }
}

impl FFIVariableResolve for Type {
    fn to_ffi_variable(&self, source: &ScopeContext) -> Type {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.to_ffi_variable(source),
            Type::Array(TypeArray { elem, len, .. }) =>
                parse_quote!(*mut [#elem; #len]),
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.to_ffi_variable(source),
            Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
                match &**elem {
                    Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                        "c_void" => match (star_token, const_token, mutability) {
                            (_, Some(_const_token), None) => parse_quote!(ferment_interfaces::OpaqueContext),
                            (_, None, Some(_mut_token)) => parse_quote!(ferment_interfaces::OpaqueContextMut),
                            _ => panic!("ffi_dictionary_type_presenter: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                        },
                        _ => parse_quote!(*mut #path)
                    },
                    Type::Ptr(type_ptr) =>
                        parse_quote!(*mut #type_ptr),
                    _ => self.clone()
                },
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                let bound = bounds.iter().find_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
                    TypeParamBound::Lifetime(_) => None
                }).unwrap();
                bound.to_ffi_variable(source)
            },
            ty =>
                ty.mangle_ident_default().to_type()
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

