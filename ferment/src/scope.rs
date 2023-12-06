use std::hash::{Hash, Hasher};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, parse_quote, Path, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::punctuated::Punctuated;

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

    pub fn to_type(&self) -> Type {
        parse_quote!(#self)
    }
}

