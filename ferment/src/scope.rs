use std::hash::{Hash, Hasher};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, parse_quote, Path, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::punctuated::Punctuated;
use crate::context::Context;
use crate::helper::{ffi_mangled_ident, ffi_struct_name, path_arguments_to_types};

pub const EMPTY: Scope = Scope { path: Path { leading_colon: None, segments: Punctuated::new() } };

#[derive(Clone)]
pub struct Scope {
    pub path: Path,
}


impl<'a> From<&'a Path> for Scope {
    fn from(value: &'a Path) -> Self {
        Self::new(value.clone())
    }
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.path.to_token_stream().to_string().eq(&other.path.to_token_stream().to_string())
    }
}

impl Eq for Scope {}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.to_token_stream().to_string().hash(state);
    }
}


impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.path.to_token_stream().to_string().split_whitespace().collect::<String>().as_str())
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ParseQuote for Scope {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Path::parse(input)
            .map(Scope::new)
    }
}

impl ToTokens for Scope {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.path.to_tokens(tokens)
    }
}

impl Scope {

    pub fn crate_root() -> Self {
        Self::new(parse_quote!(crate))
    }
    pub fn ffi_expansion_scope() -> Self {
        Self::crate_and(quote!(fermented))
    }
    pub fn ffi_generics_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(generics))
    }
    pub fn ffi_types_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(types))
    }

    pub fn crate_and(path: TokenStream2) -> Self {
        Self::crate_root().joined_path(parse_quote!(#path))
    }
    pub fn ffi_types_and(path: TokenStream2) -> Self {
        Self::ffi_types_scope().joined_path(parse_quote!(#path))
    }

    pub fn new(path: Path) -> Self {
        Scope { path }
    }

    pub fn extract_type_scope(ty: &Type) -> Scope {
        match ty {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
                let new_segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                if new_segments.is_empty() {
                    EMPTY
                } else {
                    let scope_path = quote!(#(#new_segments)::*);
                    Scope::new(parse_quote!(#scope_path))
                }
            },
            _ => EMPTY
        }
    }

    pub fn ffi_type_converted_or_same(ty: &Type) -> Type {
        Self::ffi_type_converted(ty)
            .unwrap_or(ty.clone())
    }

    pub fn ffi_type_converted(ty: &Type) -> Option<Type> {
        match ty {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
                let first_segment = segments.first().unwrap();
                let first_ident = &first_segment.ident;

                let last_segment = segments.iter().last().unwrap();
                let last_ident = &last_segment.ident;

                match last_ident.to_string().as_str() {
                    "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                    | "isize" | "usize" | "bool" => None,
                    "UInt128" => Some(parse_quote!([u8; 16])),
                    "UInt160" => Some(parse_quote!([u8; 20])),
                    "UInt256" => Some(parse_quote!([u8; 32])),
                    "UInt384" => Some(parse_quote!([u8; 48])),
                    "UInt512" => Some(parse_quote!([u8; 64])),
                    "UInt768" => Some(parse_quote!([u8; 96])),
                    "VarInt" => None,
                    "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
                    "Vec" | "BTreeMap" | "HashMap" => {
                        let ffi_name = ffi_mangled_ident(ty);
                        Some(parse_quote!(crate::fermented::generics::#ffi_name))
                    },
                    "Option" => path_arguments_to_types(&last_segment.arguments)
                        .first()
                        .cloned()
                        .and_then(Self::ffi_type_converted),
                    _ => {
                        let segments: Vec<_> = match first_ident.to_string().as_str() {
                            "crate" => segments.iter().take(segments.len() - 1).skip(1).collect(),
                            _ => segments.iter().take(segments.len() - 1).collect()
                        };
                        let new_ident = ffi_struct_name(last_ident);
                        let middle = if segments.len() == 0 {
                            quote!(#new_ident)
                        } else {
                            quote!(#(#segments)::*::#new_ident)
                        };
                        Some(parse_quote!(crate::fermented::types::#middle))
                    }

                }
            },
            _ => None
        }
    }

    pub fn ffi_external_type_converted_or_same(ty: &Type, context: &Context) -> Type {
        Self::ffi_external_type_converted(ty, context)
            .unwrap_or(ty.clone())
    }

    pub fn ffi_external_type_converted(ty: &Type, context: &Context) -> Option<Type> {
        match ty {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
                let first_segment = segments.first().unwrap();
                let first_ident = &first_segment.ident;

                let last_segment = segments.iter().last().unwrap();
                let last_ident = &last_segment.ident;

                match last_ident.to_string().as_str() {
                    "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                    | "isize" | "usize" | "bool" => None,
                    "UInt128" => Some(parse_quote!([u8; 16])),
                    "UInt160" => Some(parse_quote!([u8; 20])),
                    "UInt256" => Some(parse_quote!([u8; 32])),
                    "UInt384" => Some(parse_quote!([u8; 48])),
                    "UInt512" => Some(parse_quote!([u8; 64])),
                    "UInt768" => Some(parse_quote!([u8; 96])),
                    "VarInt" => None,
                    "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
                    "Vec" | "BTreeMap" | "HashMap" => {
                        let ffi_name = ffi_mangled_ident(ty);
                        Some(parse_quote!(crate::fermented::generics::#ffi_name))
                    },
                    "Option" => path_arguments_to_types(&last_segment.arguments)
                        .first()
                        .cloned()
                        .and_then(|ty| Self::ffi_external_type_converted(ty, context)),
                    _ => {
                        let new_ident = ffi_struct_name(last_ident);

                        match first_ident.to_string().as_str() {
                            "crate" => {
                                let segments: Vec<_> = segments.iter().skip(1).take(segments.len() - 2).collect();
                                let middle = if segments.is_empty() {
                                    quote!(#new_ident)
                                } else {
                                    quote!(#(#segments)::*::#new_ident)
                                };
                                Some(parse_quote!(crate::fermented::types::#middle))
                            },
                            _ if context.contains_fermented_crate(first_ident) => {
                                let segments: Vec<_> = segments.iter().skip(1).take(segments.len() - 2).collect();
                                let middle = if segments.is_empty() {
                                    quote!(#new_ident)
                                } else {
                                    quote!(#(#segments)::*::#new_ident)
                                };
                                Some(parse_quote!(#first_ident::fermented::types::#middle))
                            },
                            _ => {
                                let segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                                let middle = if segments.is_empty() {
                                    quote!(#new_ident)
                                } else {
                                    quote!(#(#segments)::*::#new_ident)
                                };
                                Some(parse_quote!(#middle))
                            }
                        }
                    }

                }
            },
            _ => None
        }
    }

    pub fn is_crate(&self) -> bool {
        self.path.segments.last().unwrap().ident == format_ident!("crate")
    }

    pub fn has_belong_to_current_crate(&self) -> bool {
        self.path.segments.first().unwrap().ident == format_ident!("crate")
    }

    pub fn root_ident(&self) -> Ident {
        self.path.segments.first().unwrap().ident.clone()
    }
    pub fn head(&self) -> Ident {
        self.path.segments.last().unwrap().ident.clone()
    }

    pub fn joined(&self, name: &Ident) -> Scope {
        let mut segments = self.path.segments.clone();
        segments.push(PathSegment::from(name.clone()));
        Scope::new(Path { leading_colon: None, segments })
    }

    pub fn joined_path(&self, path: Path) -> Scope {
        parse_quote!(#self::#path)
    }

    pub fn popped(&self) -> Scope {
        let segments = self.path.segments.clone();
        let n = segments.len() - 1;
        Scope::new(Path { leading_colon: None, segments: Punctuated::from_iter(segments.into_iter().take(n)) })
    }
}

