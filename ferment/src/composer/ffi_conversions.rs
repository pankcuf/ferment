use std::rc::Rc;
use std::cell::RefCell;
use quote::quote;
use syn::__private::TokenStream2;
use crate::composer::{Composer, ConversionComposer, DropComposer, FFIBindingsComposer, FFIContextComposer, ItemComposer};
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::Conversion;
use crate::interface::MapPresenter;

pub enum ComposerAspect {
    From,
    To,
    Destroy,
    Drop,
    Bindings,
}
pub struct FFIConversionComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub from_conversion_composer: ConversionComposer,
    pub to_conversion_composer: ConversionComposer,
    pub destroy_composer: FFIContextComposer,
    pub drop_composer: DropComposer,
    pub bindings_composer: FFIBindingsComposer,

    from_presenter: MapPresenter,
    to_presenter: MapPresenter,
    destructor_presenter: MapPresenter,
}

impl FFIConversionComposer {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        from_conversion_composer: ConversionComposer,
        to_conversion_composer: ConversionComposer,
        destroy_composer: FFIContextComposer<TokenStream2>,
        drop_composer: DropComposer,
        from_presenter: MapPresenter,
        to_presenter: MapPresenter,
        bindings_composer: FFIBindingsComposer,
        destructor_presenter: MapPresenter) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, from_presenter, to_presenter, bindings_composer, destructor_presenter, parent: None }
    }
    pub(crate) fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.bindings_composer.set_parent(root);
        self.from_conversion_composer.set_parent(root);
        self.to_conversion_composer.set_parent(root);
        self.destroy_composer.set_parent(root);
        self.drop_composer.set_parent(root);
        self.parent = Some(Rc::clone(root));
    }
    pub fn add_conversion(&mut self, field_type: FieldTypeConversion, context: &Rc<RefCell<ScopeContext>>) {
        let field_path_to = (self.to_presenter)(&field_type.name());
        let field_path_from = (self.from_presenter)(&field_type.name());
        let field_path_destroy = (self.destructor_presenter)(&field_type.name());
        let context = context.borrow();
        println!("add_conversion: {}: {}", quote!(#field_path_to), field_type);
        self.to_conversion_composer.add_conversion(field_type.name(), field_type.to(field_path_to, &context));
        self.from_conversion_composer.add_conversion(field_type.name(), field_type.from(field_path_from, &context));
        self.drop_composer.add_conversion(&field_type.destroy(field_path_destroy, &context));
        self.bindings_composer.add_conversion(field_type);
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
