use std::collections::HashSet;
use syn::Attribute;
use crate::ast::{BraceWrapped, CommaPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{SourceComposable, ComposerLink, GenericComposer, GenericComposerInfo};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::kind::{expand_attributes, MixinKind};
use crate::lang::RustSpecification;
use crate::presentable::{ScopeContextPresentable, TypeContext};
use crate::presentation::{DocPresentation, FFIObjectPresentation, present_struct, RustFermentate};

impl GenericComposer<RustSpecification> {
    pub fn mixin(context: (&MixinKind, &HashSet<Option<Attribute>>), scope_link: &ScopeContextLink) -> Option<ComposerLink<Self>> {
        let (mixin, attrs) = context;
        let attrs = expand_attributes(attrs);
        Self::new(mixin, TypeContext::mixin(mixin, attrs.cfg_attributes()), attrs, scope_link)
    }
}

impl SourceComposable for GenericComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<RustFermentate>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        self.wrapper
            .compose(source)
            .map(|GenericComposerInfo {
                      field_composers,
                      field_composer,
                      ffi_aspect,
                      attrs,
                      interfaces,
                      bindings
                  }| {
                let struct_body = BraceWrapped::new(CommaPunctuated::from_iter(field_composers.iter().map(field_composer)));
                let ffi_presentation = FFIObjectPresentation::Full(present_struct(ffi_aspect.present(source), &attrs, struct_body.present(source)));

                RustFermentate::Item {
                    attrs,
                    comment: DocPresentation::Empty,
                    ffi_presentation,
                    conversions: interfaces,
                    bindings: bindings.present(source),
                    traits: Default::default(),
                }
            })
    }
}
