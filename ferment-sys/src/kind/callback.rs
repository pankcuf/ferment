use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Type;
use crate::ext::ToType;

#[derive(Clone, PartialEq, Eq)]
pub enum CallbackKind {
    FnOnce(Type),
    Fn(Type),
    FnMut(Type),
    FnPointer(Type),
}
impl Debug for CallbackKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("CallbackKind::{}({})", match self {
            CallbackKind::FnOnce(_) => "FnOnce",
            CallbackKind::Fn(_) => "Fn",
            CallbackKind::FnMut(_) => "FnMut",
            CallbackKind::FnPointer(_) => "FnPointer",
        }, self.to_token_stream()))
    }
}

impl Display for CallbackKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for CallbackKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty.to_tokens(tokens),
        }
    }
}

impl ToType for CallbackKind {
    fn to_type(&self) -> Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty.clone(),
        }
    }
}

impl CallbackKind {
    pub fn ty(&self) -> &Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty,
        }
    }
    pub fn ty_mut(&mut self) -> &mut Type {
        match self {
            CallbackKind::FnOnce(ty) |
            CallbackKind::Fn(ty) |
            CallbackKind::FnMut(ty) |
            CallbackKind::FnPointer(ty) => ty,
        }
    }
}
