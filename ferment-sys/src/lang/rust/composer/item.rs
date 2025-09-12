use std::clone::Clone;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composer::{AspectPresentable, AttrComposable, DocsComposable, FFIAspect, FFIObjectComposable, GenericsComposable, InterfaceComposable, NameKindComposable, SourceAccessible, SourceFermentable, TypeAspect, LifetimesComposable, ItemComposer};
#[cfg(feature = "accessors")]
use crate::composer::BindingComposable;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{InterfacePresentation, RustFermentate};

impl<I> InterfaceComposable<<RustSpecification as Specification>::Interface> for ItemComposer<RustSpecification, I>
    where I: DelimiterTrait + ?Sized,
          Self: GenericsComposable<<RustSpecification as Specification>::Gen>
            + LifetimesComposable<<RustSpecification as Specification>::Lt>
            + AttrComposable<<RustSpecification as Specification>::Attr>
            + TypeAspect<<RustSpecification as Specification>::TYC>
            + NameKindComposable {

    fn compose_interfaces(&self) -> Depunctuated<<RustSpecification as Specification>::Interface> {
        let generics = self.compose_generics();
        let lifetimes = self.compose_lifetimes();
        let attrs = self.compose_attributes();
        let source = self.source_ref();
        let from = self.compose_aspect(FFIAspect::From).present(&source);
        let to = self.compose_aspect(FFIAspect::To).present(&source);
        let drop = self.compose_aspect(FFIAspect::Drop).present(&source);
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, from, &generics, &lifetimes),
            InterfacePresentation::conversion_to(&attrs, &types, to, &generics, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_type, drop)
        ])
    }
}

impl<I> SourceFermentable<RustFermentate> for ItemComposer<RustSpecification, I>
    where I: DelimiterTrait + ?Sized,
          Self: NameKindComposable {
    fn ferment(&self) -> RustFermentate {
        let conversions = self.ffi_conversions_composer
            .as_ref()
            .map(|_| self.compose_interfaces())
            .unwrap_or_default();
        let comment = self.ffi_object_composer
            .as_ref()
            .map(|_| self.compose_docs())
            .unwrap_or_default();
        RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment,
            ffi_presentation: self.compose_object(),
            conversions,
            #[cfg(feature = "accessors")]
            bindings: self.compose_bindings().present(&self.source_ref()),
            #[cfg(not(feature = "accessors"))]
            bindings: Default::default(),
            traits: Depunctuated::new()
        }
    }
}






