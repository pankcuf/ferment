use std::cell::Ref;
use quote::ToTokens;
use syn::Generics;
use crate::composer::{Depunctuated, ParentComposer};
use crate::context::ScopeContext;
use crate::presentation::{BindingPresentation, InterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::name;
use crate::presentation::context::name::Aspect;
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::shared::SharedAccess;


pub trait SourceExpandable {
    fn context(&self) -> &ParentComposer<ScopeContext>;
    fn source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
    fn expand(&self) -> Expansion { Expansion::Empty }
}

pub trait NameContext {
    fn name_context(&self) -> name::Context { self.name_context_ref().clone() }
    fn name_context_ref(&self) -> &name::Context;
    fn ffi_name_aspect(&self) -> Aspect { Aspect::FFI(self.name_context()) }
    fn target_name_aspect(&self) -> Aspect { Aspect::Target(self.name_context()) }
}
pub trait BasicComposable<Parent>: SourceExpandable + NameContext where Parent: SharedAccess {
    fn compose_attributes(&self) -> Depunctuated<Expansion>;
    fn compose_docs(&self) -> DocPresentation;
    // fn base(&self) -> &BasicComposer<Parent>;
    // fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation> {
    //     self.base().compose_attributes()
    // }
    //
    // fn compose_docs(&self) -> DocPresentation {
    //     self.base().compose_docs()
    // }
}

pub trait DropComposable {
    fn compose_drop(&self) -> DropInterfacePresentation;
}

pub trait ConversionComposable<Parent> where Parent: SharedAccess {
    fn compose_conversion(&self) -> InterfacePresentation where Self: BasicComposable<Parent> {
        let source = self.context().borrow();
        InterfacePresentation::Conversion {
            attrs: self.compose_attributes().to_token_stream(),
            types: (
                self.ffi_name_aspect().present(&source),
                self.target_name_aspect().present(&source)
            ),
            conversions: self.compose_interface_aspects()
        }
    }
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>);
}

pub trait FFIObjectComposable {
    fn compose_object(&self) -> FFIObjectPresentation;
}

pub trait BindingComposable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation>;
}
