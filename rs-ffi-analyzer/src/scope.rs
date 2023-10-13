use std::hash::{Hash, Hasher};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, parse_quote, Path, PathSegment, Type, TypePath};
use syn::__private::{Span, TokenStream2};
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::punctuated::Punctuated;
use crate::helper::{ffi_struct_name, mangle_type, path_arguments_to_types};
use crate::scope_conversion::ImportType;

fn ffi_generic_path(ty: &Type) -> Path {
    let mangled_ident = mangle_type(ty);
    let name = ffi_struct_name(&mangled_ident);
    parse_quote!(crate::ffi_expansions::generics::#name)
}

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
        f.write_str(self.path.to_token_stream().to_string().split_whitespace().collect::<String>().as_str())
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
        Self::crate_and(quote!(ffi_expansions))
    }
    pub fn ffi_generics_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(generics))
    }
    pub fn ffi_types_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(types))
    }
    pub fn standard_scope() -> Self {
        Self::new(parse_quote!(std))
    }

    pub fn crate_and(path: TokenStream2) -> Self {
        Self::crate_root().joined_path(parse_quote!(#path))
    }
    pub fn ffi_types_and(path: TokenStream2) -> Self {
        Self::ffi_types_scope().joined_path(parse_quote!(#path))
    }

    pub fn std_collections() -> Self {
        Self::standard_and(quote!(collections))
    }
    pub fn standard_and(path: TokenStream2) -> Self {
        Self::standard_scope().joined_path(parse_quote!(#path))
    }

    pub fn ffi_generics_and(path: TokenStream2) -> Self {
        Self::ffi_generics_scope().joined_path(parse_quote!(#path))
    }

    pub fn new(path: Path) -> Self {
        Scope { path }
    }
    pub fn ffi_name(&self) -> Ident {
        if self.is_crate() {
            format_ident!("types")
        } else {
            self.head()
        }
    }

    pub fn ffi_generic_import(ty: &Type) -> Scope {
        Scope::new(ffi_generic_path(ty))
    }

    // pub fn is_globally_visible(ty: &Type) -> bool {
    //     match ty {
    //
    //     }
    // }
    pub fn ffi_type_converted_or_same(ty: &Type) -> Type {
        Self::ffi_type_converted(ty).unwrap_or(ty.clone())
    }

    // pub fn ffi_type_path_converted(type_path: TypePath) -> Option<TypePath> {
    //     let TypePath { path: Path { segments, .. }, .. } = type_path;
    // }
    pub fn ffi_type_converted(ty: &Type) -> Option<Type> {
        let converted = match ty {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
                let first_segment = segments.first().unwrap();
                let first_ident = &first_segment.ident;

                let last_segment = segments.iter().last().unwrap();
                let last_ident = &last_segment.ident;

                match last_ident.to_string().as_str() {
                    "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                    | "isize" | "usize" | "bool" |
                    "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" |
                    "VarInt" => None,
                    "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
                    "Vec" | "BTreeMap" | "HashMap" => {
                        let ffi_name = ffi_struct_name(&mangle_type(ty));
                        Some(parse_quote!(crate::ffi_expansions::generics::#ffi_name))
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
                        // let new_ident = last_ident;
                        let middle = if segments.len() == 0 {
                            quote!(#new_ident)
                        } else {
                            quote!(#(#segments)::*::#new_ident)
                        };
                        Some(parse_quote!(crate::ffi_expansions::types::#middle))
                    }

                }
            },
            _ => None
        };
        println!("ffi_type_converted:::: {}: {}", ty.to_token_stream(), converted.clone().map(|t| t.to_token_stream()).unwrap_or(quote!()));
        converted
    }

    pub fn ffi_type_import(ty: &Type) -> Option<Scope> {

        let s = Self::ffi_type_converted(ty)
            .map(|ty| Scope::new(parse_quote!(#ty)));
        // println!("ffi_type_import: {} => {}", quote!(#ty), quote!(#s));
        s
    }

    pub fn as_ffi_scope(&self) -> Scope {
        // println!("as_ffi_scope: {}", self);
        let target_segments = self.path.segments.clone();
        let mut ffi_segments = vec![
            target_segments.first().unwrap().clone(),
            PathSegment {
                ident: Ident::new("ffi_expansions", Span::call_site()),
                arguments: Default::default(),
            }
        ];
        ffi_segments.extend(target_segments.into_iter().skip(1));
        Scope::new(Path {
            leading_colon: self.path.leading_colon,
            segments: Punctuated::from_iter(ffi_segments),
        })
    }

    pub fn is_crate(&self) -> bool {
        self.path.segments.last().unwrap().ident == format_ident!("crate")
    }

    pub fn has_belong_to_current_crate(&self) -> bool {
        self.path.segments.first().unwrap().ident == format_ident!("crate")
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

    pub fn determine_import_type(&self) -> ImportType {
        if self.is_crate() {
            ImportType::Original
        } else {
            ImportType::External
        }
        // self.path.segments.first()

        //match &self.path {  }
    }
}

