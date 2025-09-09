use std::fmt::{Debug, Formatter};
use proc_macro2::Ident;
use syn::{ItemFn, Signature, Type, TypeBareFn};
use quote::ToTokens;

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn(ItemFn),
    Impl(Signature, Type),
    TraitImpl(Signature, Type, Type),
    TraitAsType(Signature, Type, Type),
    Bare(Ident, TypeBareFn),
    TraitInner(Signature, Type, Type)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FnSignatureContext::Bare(ident, type_bare_fn) =>
                f.write_fmt(format_args!("Bare({}, {})", ident.to_token_stream(), type_bare_fn.to_token_stream())),
            FnSignatureContext::ModFn(sig) =>
                f.write_fmt(format_args!("ModFn({})", sig.to_token_stream())),
            FnSignatureContext::Impl(sig, self_ty) =>
                f.write_fmt(format_args!("Impl(self: {}, sig: {})", self_ty.to_token_stream(), sig.to_token_stream())),
            FnSignatureContext::TraitImpl(sig, self_ty, trait_ty) =>
                f.write_fmt(format_args!("TraitImpl(self: {}, trait: {}, sig: {})", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream())),
            FnSignatureContext::TraitAsType(sig, self_ty, trait_ty) =>
                f.write_fmt(format_args!("TraitAsType(self: {}, trait: {}, sig: {})", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream())),
            FnSignatureContext::TraitInner(sig, self_ty, trait_ty) =>
                f.write_fmt(format_args!("TraitInner(self: {}, trait: {}, sig: {})", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream())),
        }
    }
}

impl FnSignatureContext {
    #[allow(unused)]
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::TraitImpl(..) => true,
            FnSignatureContext::TraitAsType(..) => true,
            _ => false
        }
    }

    pub fn maybe_signature(&self) -> Option<&Signature> {
        match self {
            FnSignatureContext::ModFn(ItemFn { sig, .. }) |
            FnSignatureContext::Impl(sig, ..) |
            FnSignatureContext::TraitImpl(sig, ..) |
            FnSignatureContext::TraitAsType(sig, ..) |
            FnSignatureContext::TraitInner(sig, ..) => Some(sig),
            FnSignatureContext::Bare(.., _) => None
        }
    }

    pub fn receiver_ty(&self) -> &Type {
        match self {
            FnSignatureContext::TraitAsType(.., trait_ty) |
            FnSignatureContext::TraitImpl(.., trait_ty) |
            FnSignatureContext::TraitInner(.., trait_ty) => trait_ty,
            FnSignatureContext::Impl(.., self_ty) => self_ty,
            _ => panic!("Receiver in mod fn")
        }
    }

    pub fn ident(&self) -> &Ident {
        match self {
            FnSignatureContext::ModFn(ItemFn { sig, .. }) |
            FnSignatureContext::Impl(sig, ..) |
            FnSignatureContext::TraitImpl(sig, ..) |
            FnSignatureContext::TraitAsType(sig, ..) |
            FnSignatureContext::TraitInner(sig, ..) => &sig.ident,
            FnSignatureContext::Bare(ident, ..) => ident,
        }
    }
}
