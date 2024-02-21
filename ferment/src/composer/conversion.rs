use syn::__private::TokenStream2;
use crate::composer::{Composer, SimplePairConversionComposer, SharedComposer, LocalConversionContext, OwnedComposer, OwnerIteratorLocalContext, FieldTypesContext};
use crate::context::ScopeContext;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::presentable::ScopeContextPresentable;
use crate::shared::{HasParent, SharedAccess};


pub struct ConversionComposer<Parent: SharedAccess, C1, C2, C3, C4> {
    parent: Option<Parent>,
    root_composer: SimplePairConversionComposer,
    context_composer: SharedComposer<Parent, TokenStream2>,
    local_context_composer: OwnedComposer<Parent, C1, C2, C3, C4>,
}
// pub struct ConversionComposer<Parent: SharedAccess, ChildContext, ChildConversionContext> {
//     parent: Option<Parent>,
//     root_composer: SimplePairConversionComposer,
//     context_composer: SharedComposer<Parent, TokenStream2>,
//     local_context_composer: OwnedComposer<
//         Parent,
//         ChildContext,
//         ChildConversionContext,
//         OwnerIteratorLocalContext,
//         OwnerIteratorPresentationContext>,
// }
impl<Parent: SharedAccess, C1, C2, C3, C4> HasParent<Parent>
for ConversionComposer<Parent, C1, C2, C3, C4> {
    fn set_parent(&mut self, parent: &Parent) {
        self.local_context_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent: SharedAccess, C1, C2, C3, C4> ConversionComposer<Parent, C1, C2, C3, C4> {
    pub const fn new(
        root_composer: SimplePairConversionComposer,
        context_composer: SharedComposer<Parent, TokenStream2>,
        local_context_composer: OwnedComposer<Parent, C1, C2, C3, C4>,
    ) -> Self {
        Self { parent: None, root_composer, context_composer, local_context_composer }
    }
}

impl<Parent: SharedAccess> Composer<Parent>
for ConversionComposer<
    Parent,
    LocalConversionContext,
    (TokenStream2, TokenStream2),
    OwnerIteratorLocalContext,
    OwnerIteratorPresentationContext> {
    type Item = TokenStream2;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = self.local_context_composer.compose(source);
        let presentation = (context, local_context.present(source));
        let presentable = (self.root_composer)(presentation);
        let result = presentable.present(source);
        result
    }
}

impl<Parent: SharedAccess> Composer<Parent> for ConversionComposer<Parent, FieldTypesContext, TokenStream2, Vec<OwnedItemPresenterContext>, IteratorPresentationContext> {
    type Item = TokenStream2;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = self.local_context_composer.compose(source);
        let presentation = (context, local_context.present(source));
        let presentable = (self.root_composer)(presentation);
        let result = presentable.present(source);
        result
    }
}

// impl<Parent: SharedAccess, C1, C2, C3, C4: ScopeContextPresentable> Composer<Parent> for ConversionComposer<Parent, C1, C2, C3, C4> {
//     type Item = TokenStream2;
//     type Source = ScopeContext;
//
//     fn compose(&self, source: &Self::Source) -> Self::Item {
//         let parent = self.parent.as_ref().unwrap();
//         let context = parent.access(self.context_composer);
//         let local_context = self.local_context_composer.compose(source);
//         let presentation = (context, local_context.present(source));
//         let presentable = (self.root_composer)(presentation);
//         let result = presentable.present(source);
//         result
//     }
// }
//
