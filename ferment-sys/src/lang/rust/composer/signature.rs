use syn::ItemFn;
use crate::composable::FnSignatureContext;
use crate::composer::{DocsComposable, FnImplContext, SigComposer, SourceAccessible, SourceFermentable, TypeAspect, compose_trait_impl_fn, compose_mod_fn, compose_impl_fn, compose_bare_fn, compose_trait_inner_fn};
use crate::lang::RustSpecification;
use crate::presentable::{ScopeContextPresentable, TypeContext};
use crate::presentation::RustFermentate;

impl SourceFermentable<RustFermentate> for SigComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let source = self.source_ref();
        let binding = match self.type_context_ref() {
            TypeContext::Fn { path, sig_context, attrs, .. } => {
                let signature_aspect = (attrs.clone(), vec![], None);
                match &sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) =>
                        compose_mod_fn(path, signature_aspect, self.target_type_aspect(), sig, &source),
                    FnSignatureContext::Impl(sig, self_ty) =>
                        compose_impl_fn(path, signature_aspect, FnImplContext::TypeImpl { self_ty, aspect: self.ffi_type_aspect() }, sig, &source),
                    FnSignatureContext::TraitImpl(sig, self_ty, trait_ty) =>
                        compose_trait_impl_fn(path, self_ty, trait_ty, signature_aspect, sig, &source),
                    FnSignatureContext::TraitAsType(sig, self_ty, trait_ty) =>
                        compose_impl_fn(path, signature_aspect, FnImplContext::TraitImpl { self_ty, trait_ty }, sig, &source),
                    FnSignatureContext::TraitInner(sig, _, trait_ty) =>
                        compose_trait_inner_fn(trait_ty, signature_aspect, sig, &source),
                    FnSignatureContext::Bare(_, type_bare_fn) =>
                        compose_bare_fn(path, signature_aspect, self.ffi_type_aspect(), type_bare_fn, &source)
                }
            }
            _ => panic!("Wrong name context for fn")
        };
        RustFermentate::Function {
            comment: self.compose_docs(),
            binding: binding.present(&source)
        }
    }
}