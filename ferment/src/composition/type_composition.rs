use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{__private::TokenStream2, Type};
use crate::formatter::format_token_stream;

#[derive(Clone)]
pub enum TypeComposition {
    Single(Type),
    Composite(Type),
    Unknown(Type)
}
impl ToTokens for TypeComposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}

impl TypeComposition {
    pub fn ty(&self) -> &Type {
        match self {
            TypeComposition::Single(ty) => ty,
            TypeComposition::Composite(ty) => ty,
            TypeComposition::Unknown(ty) => ty
        }
    }
}
impl Debug for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeComposition::Single(ty) =>
                f.write_str(format!("Single({})", format_token_stream(ty)).as_str()),
            TypeComposition::Composite(ty) =>
                f.write_str(format!("Composite({})", format_token_stream(ty)).as_str()),
            TypeComposition::Unknown(ty) =>
                f.write_str(format!("Unknown({})", format_token_stream(ty)).as_str()),
        }
    }
}
impl Display for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ty().to_token_stream()];
        let other_tokens = [other.ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeComposition {}

impl Hash for TypeComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().to_token_stream().to_string().hash(state);
    }
}
