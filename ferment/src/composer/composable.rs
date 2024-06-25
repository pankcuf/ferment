use std::cell::Ref;
use syn::{Generics, Type};
use crate::composer::{Depunctuated, FieldsOwnedComposer, FieldTypesContext, ParentComposer};
use crate::context::ScopeContext;
use crate::presentation::{BindingPresentation, DestroyPresentation, InterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{name, name::Aspect};
use crate::shared::SharedAccess;

pub trait SourceAccessible {
    fn context(&self) -> &ParentComposer<ScopeContext>;
    fn source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
}
pub trait SourceExpandable: SourceAccessible {
    fn expand(&self) -> Expansion { Expansion::Empty }
}

pub trait NameContext {
    fn name_context(&self) -> name::Context { self.name_context_ref().clone() }
    fn name_context_ref(&self) -> &name::Context;
    fn ffi_name_aspect(&self) -> Aspect { Aspect::FFI(self.name_context()) }
    fn target_name_aspect(&self) -> Aspect { Aspect::Target(self.name_context()) }
    fn raw_target_name_aspect(&self) -> Aspect { Aspect::RawTarget(self.name_context()) }

    fn compose_ffi_name(&self) -> Type where Self: SourceAccessible {
        self.ffi_name_aspect()
            .present(&self.source_ref())
    }
    fn compose_target_name(&self) -> Type where Self: SourceAccessible {
        self.target_name_aspect()
            .present(&self.source_ref())
    }
    fn compose_raw_target_name(&self) -> Type where Self: SourceAccessible {
        self.raw_target_name_aspect()
            .present(&self.source_ref())
    }
}
pub trait FieldsContext {
    fn field_types_ref(&self) -> &FieldTypesContext;
    fn field_types(&self) -> FieldTypesContext {
        self.field_types_ref()
            .clone()
    }
}
pub trait FieldsConversionComposable {
    fn fields_from(&self) -> &FieldsOwnedComposer<ParentComposer<Self>> where Self: Sized + 'static;
    fn fields_to(&self) -> &FieldsOwnedComposer<ParentComposer<Self>> where Self: Sized + 'static;
}

pub trait BasicComposable<Parent>: SourceExpandable + NameContext where Parent: SharedAccess {
    fn compose_attributes(&self) -> Depunctuated<Expansion>;
    fn compose_generics(&self) -> Option<Generics>;
    fn compose_docs(&self) -> DocPresentation;
}

pub trait DropComposable {
    fn compose_drop(&self) -> DropInterfacePresentation;
}

pub trait ConversionComposable<Parent> where Parent: SharedAccess {
    fn compose_conversion(&self) -> InterfacePresentation where Self: BasicComposable<Parent> {
        InterfacePresentation::Conversion {
            attrs: self.compose_attributes(),
            types: (
                self.compose_ffi_name(),
                self.compose_target_name()
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
