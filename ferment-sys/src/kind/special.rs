use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Type};
use crate::ext::{ToPath, ToType};
use crate::lang::Specification;

#[derive(Debug)]
pub enum SpecialType<SPEC>
where SPEC: Specification {
    Custom(Type),
    Opaque(Type),
    Phantom(PhantomData<SPEC>)
}

impl<SPEC> Display for SpecialType<SPEC>
where SPEC: Specification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SpecialType::Custom(ty) => format!("Custom({})", ty.to_token_stream()),
            SpecialType::Opaque(ty) => format!("Opaque({})", ty.to_token_stream()),
            SpecialType::Phantom(..) => "Phantom".to_string(),
        }.as_str())
    }
}

impl<SPEC> ToTokens for SpecialType<SPEC>
where SPEC: Specification {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}
impl<SPEC> ToType for SpecialType<SPEC>
where SPEC: Specification {
    fn to_type(&self) -> Type {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.clone(),
            _ => panic!("")
        }
    }
}
impl<SPEC> ToPath for SpecialType<SPEC>
where SPEC: Specification {
    fn to_path(&self) -> Path {
        match self {
            SpecialType::Custom(ty) |
            SpecialType::Opaque(ty) => ty.to_path(),
            _ => panic!()
        }
    }
}
