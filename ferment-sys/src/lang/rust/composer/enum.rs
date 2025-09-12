use quote::quote;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::{AspectPresentable, AttrComposable, DocsComposable, FFIAspect, FFIObjectComposable, GenericsComposable, InterfaceComposable, ItemComposerWrapper, SourceAccessible, SourceFermentable, TypeAspect, NameKindComposable, LifetimesComposable, EnumComposer};
#[cfg(feature = "accessors")]
use crate::composer::BindingComposable;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{TypeContext, ArgKind, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, InterfacePresentation, RustFermentate};

impl InterfaceComposable<<RustSpecification as Specification>::Interface> for EnumComposer<RustSpecification>
    where Self: SourceAccessible
            + NameKindComposable
            + TypeAspect<TypeContext>
            + AttrComposable<<RustSpecification as Specification>::Attr>
            + GenericsComposable<<RustSpecification as Specification>::Gen>
            + LifetimesComposable<<RustSpecification as Specification>::Lt> {
    fn compose_interfaces(&self) -> Depunctuated<<RustSpecification as Specification>::Interface> {
        let source = self.source_ref();
        let generics = self.compose_generics();
        let lifetimes = self.compose_lifetimes();
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());

        let variant_conversion_composer = |composer: &ItemComposerWrapper<RustSpecification>, aspect: FFIAspect|
            ArgKind::AttrSequence(composer.compose_aspect(aspect), composer.compose_attributes());

        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = CommaPunctuated::new();

        self.variant_composers.iter()
            .for_each(|variant_composer| {
                from_conversions.push(variant_conversion_composer(variant_composer, FFIAspect::From));
                to_conversions.push(variant_conversion_composer(variant_composer, FFIAspect::To));
                destroy_conversions.push(variant_conversion_composer(variant_composer, FFIAspect::Drop));
            });
        to_conversions.push(ArgKind::AttrExhaustive(vec![]));
        destroy_conversions.push(ArgKind::AttrExhaustive(vec![]));

        let from_body = DictionaryExpr::MatchFields(quote!(ffi_ref), from_conversions.present(&source));
        let to_body = DictionaryExpr::MatchFields(quote!(obj), to_conversions.present(&source));
        let drop_body = DictionaryExpr::MatchFields(quote!(self), destroy_conversions.present(&source));

        Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &generics, &lifetimes),
            InterfacePresentation::conversion_to_boxed(&attrs, &types, to_body, &generics, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_type, drop_body)
        ])
    }
}

impl SourceFermentable<RustFermentate> for EnumComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        #[cfg(feature = "accessors")]
        let bindings = self.compose_bindings().present(&self.source_ref());
        #[cfg(not(feature = "accessors"))]
        let bindings = Default::default();
        RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversions: self.compose_interfaces(),
            bindings,
            traits: Depunctuated::new()
        }
    }
}

