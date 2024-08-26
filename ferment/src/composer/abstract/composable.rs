use std::cell::Ref;
use syn::{Attribute, Generics, Type};
use syn::__private::TokenStream2;
use crate::ast::Depunctuated;
use crate::composer::{BasicComposer, Composer, FieldsOwnedSequenceComposer, FieldTypesContext, ParentComposer};
use crate::context::ScopeContext;
use crate::presentable::{Aspect, Context, ScopeContextPresentable};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, InterfacePresentation};
use crate::shared::SharedAccess;

pub trait BasicComposerOwner: Sized + 'static {
    fn base(&self) -> &BasicComposer<ParentComposer<Self>>;
}

impl<T> NameContext for T where Self: BasicComposerOwner {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<T> SourceAccessible for T where T: BasicComposerOwner {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base().context()
    }
}

impl<T> BasicComposable<ParentComposer<T>> for T where T: BasicComposerOwner + SourceExpandable + DocsComposable {
    fn compose_attributes(&self) -> Vec<Attribute> {
        self.base().compose_attributes()
    }
    fn compose_generics(&self) -> Option<Generics> {
        self.base().generics.compose(self.context())
    }
}


pub trait SourceAccessible {
    fn context(&self) -> &ParentComposer<ScopeContext>;
    fn source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
}
pub trait SourceExpandable: SourceAccessible {
    fn expand(&self) -> Expansion { Expansion::Empty }
}

pub trait NameContext {
    fn name_context(&self) -> Context { self.name_context_ref().clone() }
    fn name_context_ref(&self) -> &Context;
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
    #[allow(unused)]
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
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> where Self: Sized + 'static;
    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> where Self: Sized + 'static;
}

pub trait DocsComposable {
    fn compose_docs(&self) -> DocPresentation;
}

pub trait BasicComposable<Parent>: SourceExpandable + NameContext + DocsComposable where Parent: SharedAccess {
    fn compose_attributes(&self) -> Vec<Attribute>;
    fn compose_generics(&self) -> Option<Generics>;
}

pub trait DropComposable {
    fn compose_drop(&self) -> DropInterfacePresentation;
}

pub trait ConversionComposable<Parent> where Parent: SharedAccess {
    fn compose_conversions(&self) -> Depunctuated<InterfacePresentation> where Self: BasicComposable<Parent> {
        let generics = self.compose_generics();
        let attrs = self.compose_attributes();
        let types = (self.compose_ffi_name(), self.compose_target_name());
        Depunctuated::from_iter([
            InterfacePresentation::ConversionFrom {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (self.compose_interface_from(), generics.clone())
            },
            InterfacePresentation::ConversionTo {
                attrs: attrs.clone(),
                types: types.clone(),
                conversions: (self.compose_interface_to(), generics.clone())
            },
            InterfacePresentation::ConversionDestroy {
                attrs,
                types,
                conversions: (self.compose_interface_destroy(), generics)
            }
        ])
    }
    fn compose_interface_from(&self) -> TokenStream2;
    fn compose_interface_to(&self) -> TokenStream2;
    fn compose_interface_destroy(&self) -> TokenStream2;
    // fn compose_interface_generics(&self) -> Option<Generics>;
    // fn compose_interface_aspects(&self) -> (TokenStream2, TokenStream2, TokenStream2, Option<Generics>);
}

pub trait FFIObjectComposable {
    fn compose_object(&self) -> FFIObjectPresentation;
}

pub trait BindingComposable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation>;
}

