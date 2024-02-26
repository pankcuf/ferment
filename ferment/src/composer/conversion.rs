use crate::composer::{Composer, SharedComposer, OwnedComposer, ComposerPresenter, ComposerPresenterByRef};
use crate::presentation::presentable::ScopeContextPresentable;
use crate::shared::{HasParent, SharedAccess};

pub struct ConversionComposer<Parent, L1CTX, L2CTX, L2MAP, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        L1CTX: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<(CTX, LOUT), OUT>,
    context_composer: SharedComposer<Parent, CTX>,
    local_context_composer: OwnedComposer<Parent, L1CTX, L2CTX, L2MAP, L1OUT, LOUT>,
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> HasParent<Parent> for ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    fn set_parent(&mut self, parent: &Parent) {
        self.local_context_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> Composer<Parent> for ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    type Source = ();
    type Result = OUT;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        (self.root_composer)((
            self.parent.as_ref().unwrap().access(self.context_composer),
            self.local_context_composer.compose(&())))
    }
}
impl<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT> ConversionComposer<Parent, C0, C1, C2, L1OUT, LOUT, CTX, OUT>
    where
        Parent: SharedAccess,
        C0: Clone,
        LOUT: ScopeContextPresentable,
        OUT: ScopeContextPresentable {
    pub const fn new(
        root_composer: ComposerPresenter<(CTX, LOUT), OUT>,
        context_composer: SharedComposer<Parent, CTX>,
        local_root_composer: ComposerPresenter<L1OUT, LOUT>,
        local_context_composer: SharedComposer<Parent, C0>,
        local_context_iterator_item_composer: ComposerPresenterByRef<C1, C2>,
        local_context_iterator_root_composer: ComposerPresenter<(C0, ComposerPresenterByRef<C1, C2>), L1OUT>,
    ) -> Self {
        Self {
            parent: None,
            root_composer,
            context_composer,
            local_context_composer: OwnedComposer::new(
                local_root_composer,
                local_context_composer,
                local_context_iterator_root_composer,
                local_context_iterator_item_composer
            )
        }
    }
}
