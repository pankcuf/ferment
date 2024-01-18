use std::hash::{Hash, Hasher};
use syn::{Path, Type};
use quote::ToTokens;

#[derive(Clone)]
pub struct TypeAndPathHolder(pub Type, pub Path);

impl PartialEq for TypeAndPathHolder {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.0.to_token_stream(), self.1.to_token_stream()];
        let other_tokens = [other.0.to_token_stream(), other.1.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeAndPathHolder {}

impl Hash for TypeAndPathHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
        self.1.to_token_stream().to_string().hash(state);
    }
}
