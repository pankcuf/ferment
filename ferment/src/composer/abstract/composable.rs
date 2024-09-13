use std::cell::Ref;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::{BasicComposer, FieldsOwnedSequenceComposer, FieldComposers, ComposerLink};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Aspect, BindingPresentableContext, Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DocPresentation, FFIObjectPresentation, InterfacePresentation};
use crate::shared::SharedAccess;

/// Composer common interfaces
pub trait BasicComposerOwner<CTX, LANG, SPEC, Gen>: Sized + 'static
    where CTX: Clone,
          LANG: Clone,
          Gen: LangGenSpecification<LANG>,
          SPEC: LangAttrSpecification<LANG>,
          Aspect<CTX>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>;
}
pub trait SourceAccessible {
    fn context(&self) -> &ComposerLink<ScopeContext>;
    fn source_ref(&self) -> Ref<ScopeContext> { self.context().borrow() }
}
// pub trait SourceFermentable: SourceAccessible {
//     fn ferment(&self) -> Depunctuated<Fermentate> { Depunctuated::new() }
// }
pub trait SourceFermentable2<LANG>: SourceAccessible {
    fn ferment(&self) -> Depunctuated<LANG> { Depunctuated::new() }
}
pub trait NameContext<CTX>
    where CTX: Clone,
          Aspect<CTX>: ScopeContextPresentable {
    fn name_context(&self) -> CTX { self.name_context_ref().clone() }
    fn name_context_ref(&self) -> &CTX;
    fn ffi_name_aspect(&self) -> Aspect<CTX> { Aspect::FFI(self.name_context()) }
    fn target_name_aspect(&self) -> Aspect<CTX> { Aspect::Target(self.name_context()) }
    fn raw_target_name_aspect(&self) -> Aspect<CTX> { Aspect::RawTarget(self.name_context()) }
}

pub trait NameComposable<CTX>
    where CTX: Clone,
          Aspect<CTX>: ScopeContextPresentable {
    fn compose_ffi_name(&self) -> <Aspect<CTX> as ScopeContextPresentable>::Presentation;
    fn compose_target_name(&self) -> <Aspect<CTX> as ScopeContextPresentable>::Presentation;
}
pub trait FieldsContext<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn field_types_ref(&self) -> &FieldComposers<LANG, SPEC>;
    fn field_types(&self) -> FieldComposers<LANG, SPEC> {
        self.field_types_ref()
            .clone()
    }
}
pub trait FieldsConversionComposable<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>
        where Self: Sized
            + 'static;
    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>
        where Self: Sized
            + 'static;
}
pub trait DocsComposable {
    fn compose_docs(&self) -> DocPresentation;
}

pub trait AttrComposable<SPEC> {
    fn compose_attributes(&self) -> SPEC;
}
pub trait GenericsComposable<T> {
    fn compose_generics(&self) -> T;
}
pub trait VariantComposable<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn compose_variants(&self) -> CommaPunctuated<SequenceOutput<LANG, SPEC>>;
}
pub trait ConversionComposable {
    fn compose_conversions(&self) -> Depunctuated<InterfacePresentation>;
}
pub trait FFIObjectComposable {
    fn compose_object(&self) -> FFIObjectPresentation;
}
pub trait BindingComposable<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC, Gen>>;
}
pub trait BasicComposable<Link, CTX, LANG, SPEC, Gen>
// : SourceFermentable2<LANG>
: NameComposable<CTX>
+ NameContext<CTX>
+ DocsComposable
+ AttrComposable<SPEC>
+ GenericsComposable<Gen>
    where Link: SharedAccess,
          CTX: Clone,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Aspect<CTX>: ScopeContextPresentable {}

/// Composer common implementations
// impl<Link, LANG, SPEC> NameContext<Context> for Link
//     where Link: BasicComposerOwner<Context, LANG, SPEC>,
//           LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     fn name_context_ref(&self) -> &Context {
//         self.base().name_context_ref()
//     }
// }
// impl<Link, LANG, SPEC> SourceAccessible for Link
//     where Link: BasicComposerOwner<Context, LANG, SPEC>,
//           LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     fn context(&self) -> &ComposerLink<ScopeContext> {
//         self.base().context()
//     }
// }

// impl<Link, LANG, SPEC> AttrComposable<SPEC> for Link
//     where Link: BasicComposerOwner<Context, LANG, SPEC>,
//           LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     fn compose_attributes(&self) -> SPEC {
//         self.base().compose_attributes()
//     }
// }
// impl<Link, LANG, SPEC> GenericsComposable<Option<Generics>> for Link
//     where Link: BasicComposerOwner<Context, LANG, SPEC>,
//           LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     fn compose_generics(&self) -> Option<Generics> {
//         self.base().compose_generics()
//     }
// }
impl<Link, CTX> NameComposable<CTX> for Link
    where Link: SourceAccessible + NameContext<CTX>,
          CTX: Clone,
          Aspect<CTX>: ScopeContextPresentable {
    fn compose_ffi_name(&self) -> <Aspect<CTX> as ScopeContextPresentable>::Presentation {
        self.ffi_name_aspect()
            .present(&self.source_ref())
    }
    fn compose_target_name(&self) -> <Aspect<CTX> as ScopeContextPresentable>::Presentation {
        self.target_name_aspect()
            .present(&self.source_ref())
    }
    // #[allow(unused)]
    // fn compose_raw_target_name(&self) -> <Aspect<CTX> as ScopeContextPresentable>::Presentation {
    //     self.raw_target_name_aspect()
    //         .present(&self.source_ref())
    // }
}

impl<T, LANG, SPEC, Gen> BasicComposable<ComposerLink<T>, Context, LANG, SPEC, Gen> for T
    where T: BasicComposerOwner<Context, LANG, SPEC, Gen>
            + SourceFermentable2<LANG>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + NameContext<Context>
            + NameComposable<Context>
            + DocsComposable,
            LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {}

