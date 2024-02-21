use syn::__private::TokenStream2;
use ferment_macro::Parent;
use crate::composer::{ComposerPresenter, Composer, SharedComposer, OwnedFieldTypeComposer, OwnerIteratorLocalContext, LocalConversionContext, FieldTypesContext};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::presentation::presentable::ScopeContextPresentable;
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::shared::SharedAccess;

pub const LOCAL_CONTEXT_PRESENTER: ComposerPresenter<(OwnedFieldTypeComposer, LocalConversionContext), OwnerIteratorLocalContext> =
    |(field_presenter, (context, fields))| {
        (context, fields.iter().map(|field_type| (field_presenter)(field_type.clone())).collect())
    };

#[derive(Parent)]
pub struct FieldsComposer<Parent: SharedAccess> {
    parent: Option<Parent>,
    root_composer: ComposerPresenter<OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
    context_composer: SharedComposer<Parent, TokenStream2>,

    pub field_presenter: OwnedFieldTypeComposer,
    pub fields: FieldTypesContext,
}


impl<Parent: SharedAccess> FieldsComposer<Parent> {
    pub const fn new(
        root_composer: ComposerPresenter<OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
        context_composer: SharedComposer<Parent, TokenStream2>,
        field_presenter: OwnedFieldTypeComposer
    ) -> Self {
        Self { parent: None, root_composer, context_composer, field_presenter, fields: vec![] }
    }

    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.fields.push(field_type);
    }
}

impl<Parent: SharedAccess> Composer<Parent> for FieldsComposer<Parent> {
    type Item = TokenStream2;
    type Source = ScopeContext;

    fn compose(&self, source: &Self::Source) -> Self::Item {
        let parent = self.parent.as_ref().unwrap();
        let context = parent.access(self.context_composer);
        let local_context = LOCAL_CONTEXT_PRESENTER((self.field_presenter, (context, self.fields.clone())));
        let presentable = (self.root_composer)(local_context);
        let result = presentable.present(source);
        result
    }
}
