use std::rc::Rc;
use std::cell::RefCell;
use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composer::{Composer, ComposerPresenter, ItemComposer};
use crate::context::ScopeContext;

pub struct FFIContextComposer<T: ToTokens = TokenStream2> {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: ComposerPresenter<T, T>,
    context_presenter: ComposerPresenter<ItemComposer, T>,
}

impl<T: ToTokens> FFIContextComposer<T> {
    pub const fn new(composer: ComposerPresenter<T, T>, context_presenter: ComposerPresenter<ItemComposer, T>) -> Self {
        Self { parent: None, composer, context_presenter }
    }
}

impl<T: ToTokens> Composer for FFIContextComposer<T> {
    type Item = T;
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    #[allow(clippy::redundant_closure_call)]
    fn compose(&self, _context: &ScopeContext) -> Self::Item {
        (self.composer)(
            &(self.context_presenter)(
                &self.parent.as_ref().unwrap().borrow()))
    }
}


// composer_impl!(FFIContextComposer, TokenStream2, |context_composer: &FFIContextComposer, _context: &ScopeContext|
//     (context_composer.composer)(
//         (context_composer.context_presenter)(
//             &context_composer.parent.as_ref().unwrap().borrow())));
//
