use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;

#[allow(unused)]
pub enum LocalTypeConversion {
    Bound(Type),
    Type(Type)
    // Trait(TypeModel, TraitDecompositionPart1),
    // TraitType(TypeModel),
    // TraitAssociatedType(TypeModel),
    // Object(TypeModel),
    // Primitive(TypeModel),
    // Bounds(GenericBoundsModel),
    // SmartPointer(TypeModel),
    // Fn(TypeModel),
    // Unknown(TypeModel),
}

impl LocalTypeConversion {
    pub fn ty(&self) -> &Type {
        match self {
            LocalTypeConversion::Bound(ty) | LocalTypeConversion::Type(ty) => ty
        }
    }
}

impl ToTokens for LocalTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty().to_tokens(tokens)
    }
}
impl Debug for LocalTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalTypeConversion::Bound(ty) =>
                f.write_str(format!("Bound({})", ty.to_token_stream()).as_str()),
            LocalTypeConversion::Type(ty) =>
                f.write_str(format!("Type({})", ty.to_token_stream()).as_str()),
        }
    }
}

impl Display for LocalTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for LocalTypeConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ty().to_token_stream()];
        let other_tokens = [other.ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for LocalTypeConversion {}

impl Hash for LocalTypeConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().to_token_stream().to_string().hash(state);
    }
}
