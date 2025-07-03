use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, Generics, Path, PathArguments, PathSegment, Type, TypePath};
use quote::{quote, ToTokens};
use crate::ast::PathHolder;
use crate::conversion::{GenericTypeKind, ObjectKind};

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct GenericConversion {
    pub object: ObjectKind,
}

 impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.object.to_token_stream().to_string().as_str())
    }
}

impl PartialEq for GenericConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.object.to_token_stream()];
        let other_tokens = [other.object.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(ToString::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericConversion {}

impl Hash for GenericConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.object.to_token_stream().to_string().hash(state);
    }
}

#[allow(unused)]
impl GenericConversion {
    pub fn new(object: ObjectKind) -> Self {
        Self { object }
    }

    pub fn used_imports(&self) -> HashSet<PathHolder> {
        generic_imports(self.object.maybe_type().as_ref())
    }
    pub fn maybe_generic_type_kind(&self) -> Option<GenericTypeKind> {
        self.object.maybe_generic_type_kind()
    }
}

fn generic_imports(ty: Option<&Type>) -> HashSet<PathHolder> {
    match ty {
        Some(Type::Path(TypePath { path: Path { segments, .. }, .. })) => segments.iter()
            .flat_map(|PathSegment { arguments, .. }| match arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .iter()
                    .map(|arg| match arg {
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


#[derive(Clone, Debug)]
pub struct GenericsComposition {
    pub generics: Generics,

}

impl ToTokens for GenericsComposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { generics, .. } = self;
        quote!(#generics).to_tokens(tokens)
    }
}
