use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composer::{Composer, ContextComposer, DropConversionComposer, FFIConversionComposer, HasParent};
use crate::context::ScopeContext;
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::ScopeContextPresentable;
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIAspect {
    From,
    To,
    Destroy,
    Drop,
    // Bindings,
}

// impl ComposerAspect for FFIAspect {
//     type Context = ScopeContext;
//     type Presentation = TokenStream2;
// }
pub struct FFIComposer<Parent> where Parent: SharedAccess {
    pub parent: Option<Parent>,
    pub from_conversion_composer: FFIConversionComposer<Parent>,
    pub to_conversion_composer: FFIConversionComposer<Parent>,
    pub drop_composer: DropConversionComposer<Parent>,
    pub destroy_composer: ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, Parent>,
    // pub bindings_composer: FFIBindingsComposer<Parent>,
}

impl<Parent> HasParent<Parent> for FFIComposer<Parent> where Parent: SharedAccess {
    fn set_parent(&mut self, parent: &Parent) {
        // self.bindings_composer.set_parent(parent);
        self.from_conversion_composer.set_parent(parent);
        self.to_conversion_composer.set_parent(parent);
        self.destroy_composer.set_parent(parent);
        self.drop_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent> FFIComposer<Parent> where Parent: SharedAccess {
    pub const fn new(
        from_conversion_composer: FFIConversionComposer<Parent>,
        to_conversion_composer: FFIConversionComposer<Parent>,
        destroy_composer: ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, Parent>,
        drop_composer: DropConversionComposer<Parent>,
        // bindings_composer: FFIBindingsComposer<Parent>
    ) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, parent: None }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect, source: &ScopeContext) -> TokenStream2 {
        match aspect {
            FFIAspect::From =>
                self.from_conversion_composer.compose(&())
                    .present(source),
            FFIAspect::To =>
                self.to_conversion_composer.compose(&())
                    .present(source),
            FFIAspect::Destroy =>
                self.destroy_composer.compose(&())
                    .present(source),
            FFIAspect::Drop =>
                self.drop_composer.compose(&())
                    .present(source),
            // FFIAspect::Bindings =>
            //     self.bindings_composer.compose(&())
            //         .present(source),
        }.to_token_stream()
    }
}
