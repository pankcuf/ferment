use quote::ToTokens;
use syn::Path;
use syn::__private::TokenStream2;
use ferment_macro::Parent;
use crate::composer::Composer;
use crate::shared::SharedAccess;

#[derive(Parent)]
pub struct NameComposer<Parent: SharedAccess> {
    pub parent: Option<Parent>,
    pub name: Path,
}

impl<Parent: SharedAccess> NameComposer<Parent> {
    pub const fn new(name: Path) -> Self {
        Self { parent: None, name }
    }
}

impl<Parent: SharedAccess> Composer<Parent> for NameComposer<Parent> {
    type Source = ();
    type Result = TokenStream2;

    fn compose(&self, _source: &Self::Source) -> Self::Result {
        self.name.to_token_stream()
    }
}
