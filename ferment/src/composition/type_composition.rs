use std::fmt::{Debug, Display, Formatter};
use syn::{Generics, Type};
use crate::formatter::format_token_stream;

#[derive(Clone)]
pub struct TypeComposition {
    pub ty: Type,
    pub generics: Option<Generics>,
}

impl TypeComposition {
    pub fn new(ty: Type, generics: Option<Generics>) -> Self {
        Self { ty, generics }
    }
    pub fn new_default(ty: Type) -> Self {
        Self::new(ty, None)
    }
}

impl Debug for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("TypeComposition({})",
                    format_token_stream(&self.ty),
                    // self.generics.as_ref().map_or(format!("None"), format_token_stream)
                ).as_str())
    }
}

impl Display for TypeComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
