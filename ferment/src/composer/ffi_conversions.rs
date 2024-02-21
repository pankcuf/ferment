use syn::__private::TokenStream2;
use crate::composer::{Composer, ConversionComposer, FFIBindingsComposer, FieldTypesContext, HasParent, LocalConversionContext, OwnerIteratorLocalContext, SimpleContextComposer};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::shared::SharedAccess;

#[allow(dead_code)]
pub enum ComposerAspect {
    From,
    To,
    Destroy,
    Drop,
    Bindings,
}
pub struct FFIConversionComposer<Parent: SharedAccess> {
    pub parent: Option<Parent>,
    pub from_conversion_composer: ConversionComposer<Parent, LocalConversionContext, (TokenStream2, TokenStream2), OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
    pub to_conversion_composer: ConversionComposer<Parent, LocalConversionContext, (TokenStream2, TokenStream2), OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
    pub destroy_composer: SimpleContextComposer<Parent>,
    pub drop_composer: ConversionComposer<Parent, FieldTypesContext, TokenStream2, Vec<OwnedItemPresenterContext>, IteratorPresentationContext>,
    pub bindings_composer: FFIBindingsComposer<Parent>,
}

impl<Parent: SharedAccess> HasParent<Parent> for FFIConversionComposer<Parent> {
    fn set_parent(&mut self, parent: &Parent) {
        self.bindings_composer.set_parent(parent);
        self.from_conversion_composer.set_parent(parent);
        self.to_conversion_composer.set_parent(parent);
        self.destroy_composer.set_parent(parent);
        self.drop_composer.set_parent(parent);
        self.parent = Some(parent.clone_container());
    }
}

impl<Parent: SharedAccess> FFIConversionComposer<Parent> {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        from_conversion_composer: ConversionComposer<Parent, LocalConversionContext, (TokenStream2, TokenStream2), OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
        to_conversion_composer: ConversionComposer<Parent, LocalConversionContext, (TokenStream2, TokenStream2), OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
        destroy_composer: SimpleContextComposer<Parent>,
        drop_composer: ConversionComposer<Parent, FieldTypesContext, TokenStream2, Vec<OwnedItemPresenterContext>, IteratorPresentationContext>,
        bindings_composer: FFIBindingsComposer<Parent>) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, bindings_composer, parent: None }
    }
    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.bindings_composer.add_conversion(field_type.clone());
    }

    pub fn compose_aspect(&self, aspect: ComposerAspect, context: &ScopeContext) -> TokenStream2 {
        match aspect {
            ComposerAspect::From => self.from_conversion_composer.compose(context),
            ComposerAspect::To => self.to_conversion_composer.compose(context),
            ComposerAspect::Destroy => self.destroy_composer.compose(context),
            ComposerAspect::Drop => self.drop_composer.compose(context),
            ComposerAspect::Bindings => self.bindings_composer.compose(context),
        }
    }
}
