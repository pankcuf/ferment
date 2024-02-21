use syn::__private::TokenStream2;
use crate::composer::{Composer, ComposerPresenter, FieldTypeComposer, FieldTypesContext, LocalConversionContext, OwnerIteratorLocalContext, SharedComposer};
use crate::context::ScopeContext;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;
use crate::shared::{HasParent, SharedAccess};

pub struct OwnedComposer<Parent: SharedAccess, C1, C2, C3, C4> {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<C3, C4>,
    context_composer: SharedComposer<Parent, C1>,
    conversion_presenter: ComposerPresenter<C2, TokenStream2>,
    field_presenter: FieldTypeComposer,
}

impl<Parent: SharedAccess, C1, C2, C3, C4> OwnedComposer<Parent, C1, C2, C3, C4> {
    pub const fn new(
        root_composer: ComposerPresenter<C3, C4>,
        context_composer: SharedComposer<Parent, C1>,
        conversion_presenter: ComposerPresenter<C2, TokenStream2>,
        field_presenter: FieldTypeComposer) -> Self {
        Self { parent: None, root_composer, context_composer, conversion_presenter, field_presenter }
    }
}

impl<Parent: SharedAccess, C1, C2, C3, C4> HasParent<Parent> for OwnedComposer<Parent, C1, C2, C3, C4> {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent: SharedAccess> Composer<Parent> for OwnedComposer<Parent, FieldTypesContext, TokenStream2, Vec<OwnedItemPresenterContext>, IteratorPresentationContext> {
    type Item = IteratorPresentationContext;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = context.iter()
            .map(|field_type| {
                let from_to = (self.field_presenter)(field_type);
                let local_context = from_to.present(source);
                let conversion = (self.conversion_presenter)(local_context);
                OwnedItemPresenterContext::Conversion(conversion)
            })
            .collect();
        (self.root_composer)(local_context)
    }
}

impl<Parent: SharedAccess> Composer<Parent> for OwnedComposer<Parent, LocalConversionContext, (TokenStream2, TokenStream2), OwnerIteratorLocalContext, OwnerIteratorPresentationContext> {
    type Item = OwnerIteratorPresentationContext;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = context.1.iter().map(|field_type| {
            let from_to = (self.field_presenter)(field_type);
            let local_context = (field_type.name(), from_to.present(source));
            let conversion = (self.conversion_presenter)(local_context);
            OwnedItemPresenterContext::Conversion(conversion)
        }).collect();
        let local_context = (context.0, local_context);
        (self.root_composer)(local_context)
    }
}


// impl<Parent: SharedAccess, C1, C2, C3, C4> Composer<Parent> for OwnedComposer<Parent, C1, C2, C3, C4> {
//     type Item = C4;
//     type Source = ScopeContext;
//
//     fn compose(&self, source: &Self::Source) -> Self::Item {
//         let parent = self.parent.as_ref().unwrap();
//         let (owner, owned_items) = parent.access(self.context_composer);
//         let ctx = (owner, owned_items.iter().map(|field_type| {
//             let from_to = (self.field_presenter)(field_type);
//             let conversion = (self.conversion_presenter)((field_type.name(), from_to.present(source)));
//             OwnedItemPresenterContext::Conversion(conversion)
//         }).collect());
//         (self.root_composer)(ctx)
//     }
// }
