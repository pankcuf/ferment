use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;

#[allow(unused)]
pub enum LocalTypeKind {
    Bound(Type),
    Type(Type)
}

impl LocalTypeKind {
    pub fn ty(&self) -> &Type {
        match self {
            LocalTypeKind::Bound(ty) | LocalTypeKind::Type(ty) => ty
        }
    }
}

impl ToTokens for LocalTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}
impl Debug for LocalTypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalTypeKind::Bound(ty) =>
                f.write_fmt(format_args!("Bound({})", ty.to_token_stream())),
            LocalTypeKind::Type(ty) =>
                f.write_fmt(format_args!("Type({})", ty.to_token_stream())),
        }
    }
}

impl Display for LocalTypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for LocalTypeKind {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ty().to_token_stream()];
        let other_tokens = [other.ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for LocalTypeKind {}

impl Hash for LocalTypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().to_token_stream().to_string().hash(state);
    }
}
