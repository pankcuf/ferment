use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use quote::ToTokens;
use crate::context::ScopeContext;
use crate::conversion::{PathConversion, TypeConversion};
use crate::formatter::format_token_stream;
use crate::holder::PathHolder;

#[derive(Clone)]
pub struct GenericConversion(pub TypeConversion);

impl<'a> From<&'a TypeConversion> for GenericConversion {
    fn from(value: &'a TypeConversion) -> Self {
        GenericConversion::new(value.clone())
    }
}

impl std::fmt::Debug for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for GenericConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.0.to_token_stream()];
        let other_tokens = [other.0.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericConversion {}

impl Hash for GenericConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
    }
}

impl GenericConversion {
    pub fn new(full_type: TypeConversion) -> Self {
        Self(full_type)
    }

    pub fn used_imports(&self) -> HashSet<PathHolder> {
        generic_imports(self.0.ty())
    }

    pub fn expand(&self, context: &ScopeContext) -> TokenStream2 {
        let Self { 0: full_type } = self;
        // println!("GenericConversion::expand: {}", full_type);
        let path: Path = parse_quote!(#full_type);
        match PathConversion::from(path) {
            PathConversion::Generic(generic_conversion) =>
                generic_conversion.expand(full_type, context),
            conversion =>
                unimplemented!("non-generic PathConversion: {}", format_token_stream(conversion.as_path()))
        }
    }
}

fn generic_imports(ty: &Type) -> HashSet<PathHolder> {
    match ty {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) => segments.iter()
            .flat_map(|PathSegment { arguments, .. }| match arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .iter()
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None
                    })
                    .flat_map(generic_imports)
                    .collect(),
                _ => HashSet::new(),
            })
            .collect(),
        _ => HashSet::new(),
    }
}
