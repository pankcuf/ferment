use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, Generics, Path, PathArguments, PathSegment, Type, TypePath};
use quote::{quote, ToTokens};
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::holder::PathHolder;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub struct GenericConversion(pub ObjectConversion);

impl<'a> From<&'a ObjectConversion> for GenericConversion {
    fn from(value: &'a ObjectConversion) -> Self {
        GenericConversion::new(value.clone())
    }
}
impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
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
    pub fn new(full_type: ObjectConversion) -> Self {
        Self(full_type)
    }

    pub fn used_imports(&self) -> HashSet<PathHolder> {
        generic_imports(self.0.to_ty().as_ref())
    }
}

impl ScopeContextPresentable for GenericConversion {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let Self { 0: obj } = self;
        println!("GenericConversion::present: {}", obj);
        match obj {
            ObjectConversion::Type(type_conversion) |
            ObjectConversion::Item(type_conversion, _) => {
                match TypeConversion::from(type_conversion.to_ty()) {
                    TypeConversion::Generic(generic_conversion) =>
                        generic_conversion.expand(type_conversion, source),
                    conversion =>
                        unimplemented!("non-generic GenericConversion: {}", conversion.to_token_stream())
                }
            },
            ObjectConversion::Empty => {
                unimplemented!("expand: ObjectConversion::Empty")
            }
        }
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
    // pub qs: TypeComposition,
    pub generics: Generics,

}

impl ToTokens for GenericsComposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { generics, .. } = self;
        quote!(#generics).to_tokens(tokens)
    }
}
