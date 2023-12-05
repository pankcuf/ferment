use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use crate::path_conversion::PathConversion;
use crate::helper::ffi_mangled_ident;
use crate::scope::Scope;

#[derive(Clone)]
pub struct TypePathComposition(pub Type, pub Path);

impl PartialEq for TypePathComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.0.to_token_stream(), self.1.to_token_stream()];
        let other_tokens = [other.0.to_token_stream(), other.1.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypePathComposition {}

impl Hash for TypePathComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
        self.1.to_token_stream().to_string().hash(state);
    }
}

pub fn add_generic_type(field_type: &Type, generics: &mut HashSet<TypePathComposition>) {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if let PathConversion::Generic(generic_path_conversion) = PathConversion::from(path) {
            generics.insert(TypePathComposition(field_type.clone(), generic_path_conversion.path()));
        }
    }
}

#[derive(Clone)]
pub struct GenericConversion {
    pub full_type: Type,
}

impl<'a> From<&'a Type> for GenericConversion {
    fn from(value: &'a Type) -> Self {
        GenericConversion::new(value.clone())
    }
}
impl std::fmt::Debug for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_type.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for GenericConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.full_type.to_token_stream()];
        let other_tokens = [other.full_type.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericConversion {}

impl Hash for GenericConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_type.to_token_stream().to_string().hash(state);
    }
}

impl ToTokens for GenericConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { full_type } = self;
        let path: Path = parse_quote!(#full_type);
        // PathConversion::from(path)
        //     .into_mangled_generic_ident();
        match PathConversion::from(path) {
            PathConversion::Generic(generic_conversion) =>
                generic_conversion.expand(ffi_mangled_ident(full_type)),
            conversion =>
                unimplemented!("non-generic PathConversion: {}", conversion.as_path().to_token_stream())
        }.to_tokens(tokens)
    }
}

impl GenericConversion {
    pub fn new(full_type: Type) -> Self {
        Self { full_type }
    }

    pub fn used_imports(&self) -> HashSet<Scope> {
        generic_imports(&self.full_type)
    }

}

fn generic_imports(ty: &Type) -> HashSet<Scope> {
    match ty {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) => segments.iter()
            .flat_map(|PathSegment { arguments, .. }| match arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .iter()
                    .filter_map(|arg| match arg { GenericArgument::Type(ty) => Some(ty), _ => None })
                    .flat_map(generic_imports)
                    .collect(),
                _ => HashSet::new(),
            })
            .collect(),
        _ => HashSet::new(),
    }
}