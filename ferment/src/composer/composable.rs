use std::cell::Ref;
use syn::Generics;
use crate::composer::{Depunctuated, ParentComposer};
use crate::context::ScopeContext;
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::presentation::context::name;
use crate::presentation::context::name::Aspect;
use crate::presentation::destroy_presentation::DestroyPresentation;

pub trait Composable {
    fn context(&self) -> &ParentComposer<ScopeContext>;
    fn as_source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
    fn name_context(&self) -> name::Context { self.name_context_ref().clone() }
    fn ffi_name_aspect(&self) -> Aspect { Aspect::FFI(self.name_context()) }
    fn target_name_aspect(&self) -> Aspect { Aspect::Target(self.name_context()) }
    fn name_context_ref(&self) -> &name::Context;
    fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation>;
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation>;
    fn compose_docs(&self) -> DocPresentation;
    fn compose_object(&self) -> FFIObjectPresentation;
    fn compose_drop(&self) -> DropInterfacePresentation;
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>);
    fn expand(&self) -> Expansion {
        let source = self.context().borrow();
        Expansion::Full {
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionInterfacePresentation::Interface {
                types: (self.ffi_name_aspect().present(&source), self.target_name_aspect().present(&source)),
                conversions: self.compose_interface_aspects()
            },
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            traits: self.compose_attributes()
        }
    }
}
