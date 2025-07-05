use std::cell::Ref;
use std::fmt::Debug;
use quote::ToTokens;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::{BasicComposerLink, ComposerLinkRef, CommaArgComposers, FieldsOwnedSequenceComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, NameTreeContext, ScopeContextPresentable, SeqKind};
use crate::presentation::{DocPresentation, FFIObjectPresentation};

/// Composer common interfaces
/// SPEC: Language specification providing presenters for particular language
pub trait BasicComposerOwner<SPEC>: Sized + 'static
    where SPEC: Specification {
    fn base(&self) -> &BasicComposerLink<SPEC, Self>;
}
/// Provides access to stack information in scope
pub trait SourceAccessible {
    fn context(&self) -> &ScopeContextLink;
    fn source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
}
/// Ferments to specific language representation using stack information in scope
pub trait SourceFermentable<LANG> {
    fn ferment(&self) -> LANG;
}
/// Provides different aspects for types
pub trait TypeAspect<TYC>
    where TYC: NameTreeContext {
    fn type_context(&self) -> TYC { self.type_context_ref().clone() }
    fn type_context_ref(&self) -> &TYC;
    fn ffi_type_aspect(&self) -> Aspect<TYC> { Aspect::FFI(self.type_context()) }
    fn target_type_aspect(&self) -> Aspect<TYC> { Aspect::Target(self.type_context()) }
    fn raw_target_type_aspect(&self) -> Aspect<TYC> { Aspect::RawTarget(self.type_context()) }
}

/// Presents types using different aspects
pub trait AspectPresentable<TYC>: TypeAspect<TYC>
    where TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    fn present_ffi_aspect(&self) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation;
    fn present_target_aspect(&self) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation;
    #[allow(unused)]
    fn present_ffi_aspect_ref(ref_self: &ComposerLinkRef<Self>) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation {
        ref_self.present_ffi_aspect()
    }
    #[allow(unused)]
    fn present_target_aspect_ref(ref_self: &ComposerLinkRef<Self>) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation {
        ref_self.present_target_aspect()
    }
}

/// Access to set of field or arg sequence composers
pub trait FieldsContext<SPEC>
    where SPEC: Specification {
    fn field_composers_ref(&self) -> &CommaArgComposers<SPEC>;
    #[allow(unused)]
    fn field_composers(&self) -> CommaArgComposers<SPEC> {
        self.field_composers_ref()
            .clone()
    }
    #[allow(unused)]
    fn field_composers_by_ref(by_ref: &ComposerLinkRef<Self>) -> CommaArgComposers<SPEC> {
        by_ref.field_composers()
    }
}

pub trait FieldsConversionComposable<SPEC>
    where SPEC: Specification {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposerLink<SPEC, Self>
        where Self: Sized + 'static;
    fn fields_to(&self) -> &FieldsOwnedSequenceComposerLink<SPEC, Self>
        where Self: Sized + 'static;
}
pub trait DocsComposable {
    fn compose_docs(&self) -> DocPresentation;
}

#[derive(Clone, Debug)]
pub enum NameKind {
    Named,
    Unnamed,
    Unit
}
pub trait NameKindComposable {
    fn compose_name_kind(&self) -> NameKind;
}
pub trait AttrComposable<T> {
    fn compose_attributes(&self) -> T;
}
pub trait GenericsComposable<T> {
    fn compose_generics(&self) -> T;
}
pub trait LifetimesComposable<T> {
    fn compose_lifetimes(&self) -> T;
}
pub trait VariantComposable<SPEC>
    where SPEC: Specification {
    fn compose_variants(&self) -> CommaPunctuated<SeqKind<SPEC>>;
}
pub trait InterfaceComposable<T> where T: ToTokens {
    fn compose_interfaces(&self) -> Depunctuated<T>;
}
pub trait FFIObjectComposable {
    fn compose_object(&self) -> FFIObjectPresentation;
}
pub trait BindingComposable<SPEC>
    where SPEC: Specification {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<SPEC>>;
}

impl<Link, TYC> AspectPresentable<TYC> for Link
    where Link: SourceAccessible + TypeAspect<TYC>,
          TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    fn present_ffi_aspect(&self) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation {
        self.ffi_type_aspect()
            .present(&self.source_ref())
    }
    fn present_target_aspect(&self) -> <Aspect<TYC> as ScopeContextPresentable>::Presentation {
        self.target_type_aspect()
            .present(&self.source_ref())
    }
}
