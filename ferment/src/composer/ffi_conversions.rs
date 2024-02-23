use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composer::{Composer, FFIBindingsComposer, HasParent, SimpleContextComposer};
use crate::composer::item::{StructConversionComposer, StructDropComposer};
use crate::context::ScopeContext;
use crate::presentation::ScopeContextPresentable;
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum FFIConversionAspect {
    From,
    To,
    Destroy,
    Drop,
    Bindings,
}

// impl ComposerAspect for FFIConversionAspect {
//     type Context = ScopeContext;
//     type Presentation = TokenStream2;
// }
#[allow(non_camel_case_types)]
pub struct FFIConversionComposer<Parent> where Parent: SharedAccess
{
    pub parent: Option<Parent>,
    pub from_conversion_composer: StructConversionComposer<Parent>,
    pub to_conversion_composer: StructConversionComposer<Parent>,
    pub drop_composer: StructDropComposer<Parent>,


    pub destroy_composer: SimpleContextComposer<Parent>,
    pub bindings_composer: FFIBindingsComposer<Parent>,
}

#[allow(non_camel_case_types)]
impl<Parent> HasParent<Parent>
for FFIConversionComposer<Parent> where Parent: SharedAccess {
    fn set_parent(&mut self, parent: &Parent) {
        self.bindings_composer.set_parent(parent);
        self.from_conversion_composer.set_parent(parent);
        self.to_conversion_composer.set_parent(parent);
        self.destroy_composer.set_parent(parent);
        self.drop_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}

#[allow(non_camel_case_types)]
impl<Parent> FFIConversionComposer<Parent> where Parent: SharedAccess {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        from_conversion_composer: StructConversionComposer<Parent>,
        to_conversion_composer: StructConversionComposer<Parent>,
        destroy_composer: SimpleContextComposer<Parent>,
        drop_composer: StructDropComposer<Parent>,
        bindings_composer: FFIBindingsComposer<Parent>) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, bindings_composer, parent: None }
    }

    pub fn compose_aspect(&self, aspect: FFIConversionAspect, context: &ScopeContext) -> TokenStream2 {
        match aspect {
            FFIConversionAspect::From => self.from_conversion_composer.compose(context).present(context).to_token_stream(),
            FFIConversionAspect::To => self.to_conversion_composer.compose(context).present(context).to_token_stream(),
            FFIConversionAspect::Destroy => self.destroy_composer.compose(context).present(context).to_token_stream(),
            FFIConversionAspect::Drop => self.drop_composer.compose(context).present(context).to_token_stream(),
            FFIConversionAspect::Bindings => self.bindings_composer.compose(context).present(context).to_token_stream(),
        }
    }
}
