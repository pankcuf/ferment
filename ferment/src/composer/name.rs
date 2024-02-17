use quote::ToTokens;
use std::rc::Rc;
use std::cell::RefCell;
use syn::Path;
use syn::__private::TokenStream2;
use crate::composer::ItemComposer;
use crate::context::ScopeContext;
use crate::composer_impl;

pub struct NameComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub name: Path,
}

impl NameComposer {
    pub const fn new(name: Path) -> Self {
        Self { parent: None, name }
    }
}
composer_impl!(NameComposer, TokenStream2, |itself: &NameComposer, _context: &ScopeContext|
    itself.name.to_token_stream());

