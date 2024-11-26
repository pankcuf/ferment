use std::fmt::{Debug, Formatter};
use proc_macro2::Ident;
use syn::{ItemFn, Signature, Type, TypeBareFn};
use quote::ToTokens;

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn(ItemFn),
    Impl(Type, Option<Type>, Signature),
    TraitAsType(Type, Type, Signature),
    Bare(Ident, TypeBareFn),
    TraitInner(Type, Option<Type>, Signature)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FnSignatureContext::ModFn(sig) =>
                format!("ModFn({})", sig.to_token_stream()),
            FnSignatureContext::Impl(self_ty, trait_ty, sig) =>
                format!("Impl(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::TraitAsType(self_ty, trait_ty, sig) =>
                format!("TraitAsType(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::TraitInner(self_ty, trait_ty, sig) =>
                format!("TraitInner(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::Bare(ident, type_bare_fn) =>
                format!("Bare({}, {})", ident.to_token_stream(), type_bare_fn.to_token_stream()),
        }.as_str())
    }
}

impl FnSignatureContext {
    #[allow(unused)]
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::Impl(_, Some(_), _) => true,
            FnSignatureContext::TraitAsType(..) => true,
            _ => false
        }
    }

    pub fn maybe_signature(&self) -> Option<&Signature> {
        match self {
            FnSignatureContext::ModFn(ItemFn { sig, .. }) |
            FnSignatureContext::Impl(_, _, sig) |
            FnSignatureContext::TraitAsType(_, _, sig) |
            FnSignatureContext::TraitInner(_, _, sig) => Some(sig),
            FnSignatureContext::Bare(.., _) => None
        }
    }

    pub fn receiver_ty(&self) -> &Type {
        match self {
            FnSignatureContext::Impl(_, Some(trait_ty), ..) |
            FnSignatureContext::TraitAsType(_, trait_ty, ..) |
            FnSignatureContext::TraitInner(_, Some(trait_ty), ..) => trait_ty,
            FnSignatureContext::Impl(self_ty, ..) |
            FnSignatureContext::TraitInner(self_ty, ..) => self_ty,
            _ => panic!("Receiver in mod fn")
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            FnSignatureContext::ModFn(ItemFn { sig, .. }) |
            FnSignatureContext::Impl(_, _, sig) |
            FnSignatureContext::TraitAsType(_, _, sig) |
            FnSignatureContext::TraitInner(_, _, sig) => &sig.ident,
            FnSignatureContext::Bare(ident, _) => ident,
        }
    }
}
