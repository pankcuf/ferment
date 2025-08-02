use syn::ItemFn;
use crate::composable::FnSignatureContext;
use crate::composer::{DocsComposable, LifetimesComposable, SourceAccessible, SourceFermentable, TypeAspect, SigComposer,  compose_trait_impl_fn, compose_trait_impl_fn_as_trait_type, compose_mod_fn, compose_impl_fn, compose_bare_fn, compose_trait_inner_fn};
use crate::lang::RustSpecification;
use crate::presentable::{ScopeContextPresentable, TypeContext};
use crate::presentation::RustFermentate;

impl SourceFermentable<RustFermentate> for SigComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let source = self.source_ref();
        let binding = match self.type_context_ref() {
            TypeContext::Fn { path, sig_context, attrs, .. } => match &sig_context {
                FnSignatureContext::ModFn(ItemFn { sig, .. }) =>
                    compose_mod_fn(path, self.target_type_aspect(), attrs, None, sig, &source),
                FnSignatureContext::Impl(sig, self_ty) =>
                    compose_impl_fn(path, self_ty, self.ffi_type_aspect(), attrs, None, sig, &source),
                FnSignatureContext::TraitImpl(sig, self_ty, trait_ty) =>
                    compose_trait_impl_fn(path, self_ty, trait_ty, attrs, None, sig, &source),
                FnSignatureContext::TraitAsType(sig, self_ty, trait_ty) =>
                    compose_trait_impl_fn_as_trait_type(path, self_ty, trait_ty, attrs, None, sig, &source),
                FnSignatureContext::TraitInner(sig, _, trait_ty) =>
                    compose_trait_inner_fn(trait_ty, attrs, sig, &source),
                FnSignatureContext::Bare(_, type_bare_fn) =>
                    compose_bare_fn(path, self.ffi_type_aspect(), type_bare_fn, attrs, None, self.compose_lifetimes(), &source)
            }
            _ => panic!("Wrong name context for fn")
        };
        RustFermentate::Function { comment: self.compose_docs(), binding: binding.present(&source) }
    }
}