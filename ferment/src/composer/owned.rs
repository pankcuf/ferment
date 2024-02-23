use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, SharedComposer};
use crate::context::ScopeContext;
use crate::shared::{HasParent, SharedAccess};

// pub type OwnedConversion<Parent> = OwnedComposer<       // <Item = OwnerIteratorPresentationContext, Source = ScopeContext>
//     Parent,
//     LocalConversionContext,                             // C0 -> IN
//     OwnedItemPresenterContext,                          // C1 -> CTX
//     (TokenStream2, FieldTypePresentationContext),       // LMAP -> MAP
//     OwnerIteratorLocalContext,                          // C1OUT -> OUT
//     OwnerIteratorPresentationContext>;                  // OUT
// pub type ConversionIteratorComposer = IteratorComposer< // Item: OwnerIteratorLocalContext = (TokenStream2, Vec<OwnedItemPresenterContext>), Source: LocalConversionContext
//     LocalConversionContext,                             // IN
//     (TokenStream2, FieldTypePresentationContext),       // CTX
//     FieldTypePresentationContext,                       // MAP
//     OwnerIteratorLocalContext>;                         // OUT
//
// pub type DropOwnedConversion<Parent> = OwnedComposer<   // <Item = IteratorPresentationContext, Source = ScopeContext>
//     Parent,
//     FieldTypesContext,                                  // C0 -> IN
//     FieldTypePresentationContext,                       // C1 -> CTX
//     FieldTypePresentationContext,                       // LMAP -> MAP
//     Vec<OwnedItemPresenterContext>,                     // C1OUT -> OUT
//     IteratorPresentationContext>;                       // OUT
//
// pub type DropIteratorComposer = IteratorComposer<       // Item = Vec<OwnedItemPresenterContext>, Source = FieldTypesContext
//     FieldTypesContext,                                  // IN
//     FieldTypePresentationContext,                       // CTX
//     FieldTypePresentationContext,                       // MAP
//     Vec<OwnedItemPresenterContext>>;                    // OUT
//
// IteratorComposer<FieldTypesContext, OwnedItemPresenterContext, FieldTypePresentationContext, Vec<OwnedItemPresenterContext>>

// OwnedComposer<Parent, C0, C1, LMAP, C1OUT, OUT>
// IteratorComposer<C0, C1, LMAP, C1OUT>

pub struct IteratorComposer<IN: Clone, CTX, MAP, OUT> {
    root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
    item_composer: ComposerPresenterByRef<CTX, MAP>
}

impl<IN: Clone, CTX, MAP, OUT> IteratorComposer<IN, CTX, MAP, OUT> {
    pub const fn new(
        root_composer: ComposerPresenter<(IN, ComposerPresenterByRef<CTX, MAP>), OUT>,
        item_composer: ComposerPresenterByRef<CTX, MAP>,
    ) -> Self {
        Self { root_composer, item_composer }
    }
}
impl<Parent: SharedAccess, IN: Clone, CTX, MAP, OUT> Composer<Parent>
for IteratorComposer<IN, CTX, MAP, OUT> where {
    type Item = OUT;
    type Source = IN;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        (self.root_composer)((source.clone(), self.item_composer))
    }
}


impl<Parent: SharedAccess, IN: Clone, CTX, MAP, OUT> HasParent<Parent> for IteratorComposer<IN, CTX, MAP, OUT> {
    fn set_parent(&mut self, _parent: &Parent) {}
}

pub struct OwnedComposer<Parent, CTX, L1CTX, L1MAP, L1OUT, OUT>
    where Parent: SharedAccess, CTX: Clone {
    parent: Option<Parent>,
    context_composer: SharedComposer<Parent, CTX>,
    local_context_composer: IteratorComposer<CTX, L1CTX, L1MAP, L1OUT>,
    root_composer: ComposerPresenter<L1OUT, OUT>,
}

impl<Parent, CTX, L1CTX, LMAP, L1OUT, OUT> OwnedComposer<Parent, CTX, L1CTX, LMAP, L1OUT, OUT>
    where Parent: SharedAccess, CTX: Clone {
    pub const fn new(
        root_composer: ComposerPresenter<L1OUT, OUT>,
        context_composer: SharedComposer<Parent, CTX>,
        root_iterator_composer: ComposerPresenter<(CTX, ComposerPresenterByRef<L1CTX, LMAP>), L1OUT>,
        item_iterator_composer: ComposerPresenterByRef<L1CTX, LMAP>,
    ) -> Self {
        Self {
            parent: None,
            root_composer,
            context_composer,
            local_context_composer: IteratorComposer::new(root_iterator_composer, item_iterator_composer),
        }
    }
}

impl<Parent: SharedAccess, CTX: Clone, C1, LMAP, L1OUT, OUT> HasParent<Parent> for OwnedComposer<Parent, CTX, C1, LMAP, L1OUT, OUT> {
    fn set_parent(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent: SharedAccess, CTX: Clone, C1, LMAP, L1OUT, OUT> Composer<Parent> for OwnedComposer<Parent, CTX, C1, LMAP, L1OUT, OUT> {
    type Item = OUT;
    type Source = ScopeContext;

    fn compose(&self, _source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = <IteratorComposer<CTX, C1, LMAP, L1OUT> as Composer<Parent>>::compose(&self.local_context_composer, &context);
        let out = (self.root_composer)(local_context);
        out
    }
}
