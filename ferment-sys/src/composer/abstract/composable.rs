use std::cell::Ref;
use std::fmt::Debug;
use quote::ToTokens;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::{BasicComposerLink, ComposerLinkRef, FieldComposers, FieldsOwnedSequenceComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{DocPresentation, FFIObjectPresentation};

/// Composer common interfaces
/// LANG: Fermentate Type,
/// SPEC: Language specification providing presenters for particular language
pub trait BasicComposerOwner<LANG, SPEC>: Sized + 'static
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposerLink<Self, LANG, SPEC>;
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
    where TYC: NameTreeContext,
          Aspect<TYC>: ScopeContextPresentable {
    fn type_context(&self) -> TYC { self.type_context_ref().clone() }
    fn type_context_ref(&self) -> &TYC;

    fn ffi_type_aspect(&self) -> Aspect<TYC> { Aspect::FFI(self.type_context()) }
    fn target_type_aspect(&self) -> Aspect<TYC> { Aspect::Target(self.type_context()) }
    fn raw_target_type_aspect(&self) -> Aspect<TYC> { Aspect::RawTarget(self.type_context()) }
}

/// Presents types using different aspects
pub trait AspectPresentable<TYC>
    : TypeAspect<TYC>
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
pub trait FieldsContext<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn field_composers_ref(&self) -> &FieldComposers<LANG, SPEC>;
    #[allow(unused)]
    fn field_composers(&self) -> FieldComposers<LANG, SPEC> {
        self.field_composers_ref()
            .clone()
    }
    #[allow(unused)]
    fn field_composers_by_ref(by_ref: &ComposerLinkRef<Self>) -> FieldComposers<LANG, SPEC> {
        by_ref.field_composers()
    }
}

pub trait FieldsConversionComposable<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable<Presentation: Clone> {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposerLink<Self, LANG, SPEC>
        where Self: Sized + 'static;
    fn fields_to(&self) -> &FieldsOwnedSequenceComposerLink<Self, LANG, SPEC>
        where Self: Sized + 'static;
}
pub trait DocsComposable {
    fn compose_docs(&self) -> DocPresentation;
}

pub trait AttrComposable<T> {
    fn compose_attributes(&self) -> T;
}
pub trait GenericsComposable<T> {
    fn compose_generics(&self) -> T;
}
pub trait VariantComposable<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_variants(&self) -> CommaPunctuated<SeqKind<LANG, SPEC>>;
}
pub trait InterfaceComposable<T> where T: ToTokens {
    fn compose_interfaces(&self) -> Depunctuated<T>;
}
pub trait FFIObjectComposable {
    fn compose_object(&self) -> FFIObjectPresentation;
}
pub trait BindingComposable<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>>;
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
