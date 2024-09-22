use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use quote::ToTokens;
use syn::Generics;
use ferment_macro::ComposerBase;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composable::{AttrsModel, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedFields, Composer, ComposerLink, constants, ConstructorFieldsContext, DocsComposable, FFIAspect, FFIBindingsComposer, FFIComposer, FFIObjectComposable, FieldComposers, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, GenericsComposable, InterfaceComposable, Linkable, SequenceOutputComposer, SourceAccessible, SourceFermentable, TypeAspect, PresentableArgumentComposerRef, AspectSequenceComposer, OwnerCommaIteratorConversionComposer, BindingCtorComposer, ConstructorArgComposerRef, FieldsComposerRef, PresentableExpressionComposerRef, SharedComposer, ItemComposerLink};
use crate::context::ScopeContext;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{DocPresentation, FFIObjectPresentation, InterfacePresentation, RustFermentate};
use crate::shared::SharedAccess;

pub trait FFIObjectSpecification<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    const COMPOSER: Option<SequenceOutputComposer<Link, LANG, SPEC>>;
}
pub trait FFIConversionsSpecification<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<FFIComposer<Link, LANG, SPEC>>;
}
pub trait FFIBindingsSpecification<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<FFIBindingsComposer<Link, LANG, SPEC>>;
}
pub trait FFIFieldsSpecification<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldsOwnedSequenceComposer<Link, LANG, SPEC>;
    const TO: FieldsOwnedSequenceComposer<Link, LANG, SPEC>;
}
impl<T, I, LANG, SPEC> FFIFieldsSpecification<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> for T
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          T: ItemComposerSpecification<LANG, SPEC> {
    const FROM: FieldsOwnedSequenceComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> = constants::fields_from_composer(Self::ROOT_PRESENTER, Self::FIELD_PRESENTER);
    const TO: FieldsOwnedSequenceComposer<ItemComposerLink<I, LANG, SPEC>, LANG, SPEC> = constants::fields_to_composer(Self::ROOT_PRESENTER, Self::FIELD_PRESENTER);
}

pub trait CtorSpecification<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable {
    const CTOR_ROOT: BindingCtorComposer<LANG, SPEC>;
    const CTOR_CONTEXT: SharedComposer<Link, ConstructorFieldsContext<LANG, SPEC>>;
    const CTOR_ARG: ConstructorArgComposerRef<LANG, SPEC>;
}
pub trait ItemComposerSpecification<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: AspectSequenceComposer<LANG, SPEC>;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC>;
    const ROOT_CONVERSION_PRESENTER: OwnerCommaIteratorConversionComposer<LANG, SPEC>;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC>;
    fn get_field_composers(fields: &CommaPunctuatedFields) -> FieldComposers<LANG, SPEC> {
        Self::FIELD_COMPOSERS(fields)
    }
}

pub trait ItemComposerExpressionSpecification<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    const CONVERSION: PresentableExpressionComposerRef<LANG, SPEC>;
    const DESTROY: PresentableExpressionComposerRef<LANG, SPEC>;
}

#[derive(ComposerBase)]
pub struct ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposer<ComposerLink<Self>, LANG, SPEC>,
    pub fields_from_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC>,
    pub fields_to_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC>,
    pub field_composers: FieldComposers<LANG, SPEC>,

    pub ffi_object_composer: Option<SequenceOutputComposer<ComposerLink<Self>, LANG, SPEC>>,
    pub ffi_conversions_composer: Option<FFIComposer<ComposerLink<Self>, LANG, SPEC>>,
    pub bindings_composer: Option<FFIBindingsComposer<ComposerLink<Self>, LANG, SPEC>>,
    // #[cfg(feature = "objc")]
    // pub objc_composer: crate::lang::objc::composer::ItemComposer<ComposerLink<Self>>
}


impl<I, LANG, SPEC> ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {

    pub(crate) fn new<ISPEC>(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>) -> ComposerLink<Self>
        where ISPEC: FFIFieldsSpecification<ComposerLink<Self>, LANG, SPEC>
        + FFIObjectSpecification<ComposerLink<Self>, LANG, SPEC>
        + FFIBindingsSpecification<ComposerLink<Self>, LANG, SPEC>
        + FFIConversionsSpecification<ComposerLink<Self>, LANG, SPEC>
        + ItemComposerSpecification<LANG, SPEC> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: <ISPEC as FFIFieldsSpecification<ComposerLink<Self>, LANG, SPEC>>::FROM,
            fields_to_composer: <ISPEC as FFIFieldsSpecification<ComposerLink<Self>, LANG, SPEC>>::TO,
            bindings_composer: <ISPEC as FFIBindingsSpecification<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            ffi_conversions_composer: <ISPEC as FFIConversionsSpecification<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            ffi_object_composer: <ISPEC as FFIObjectSpecification<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            field_composers: ISPEC::get_field_composers(fields),
            // #[cfg(feature = "objc")]
            // objc_composer,
        }));
        {
            root.borrow_mut().setup_composers(&root);
        }
        root

    }

    fn setup_composers(&mut self, root: &ComposerLink<Self>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        if let Some(ref mut composer) = self.bindings_composer {
            composer.link(root);
        }
        if let Some(ref mut composer) = self.ffi_object_composer {
            composer.link(root)
        }
        if let Some(ref mut composer) = self.ffi_conversions_composer {
            composer.link(root);
        }
        // #[cfg(feature = "objc")]
        // self.objc_composer.link(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> PresentableSequence<LANG, SPEC> {
        self.ffi_conversions_composer
            .as_ref()
            .expect("Composer doesn't exist")
            .compose(&aspect)
    }
    pub(crate) fn present_aspect(&self, aspect: FFIAspect) -> <PresentableSequence<LANG, SPEC> as ScopeContextPresentable>::Presentation {
        self.compose_aspect(aspect)
            .present(&self.source_ref())
    }
}

impl<I, LANG, SPEC> FieldsContext<LANG, SPEC> for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable{
    fn field_composers_ref(&self) -> &FieldComposers<LANG, SPEC> {
        &self.field_composers
    }
}

impl<I, LANG, SPEC> FieldsConversionComposable<LANG, SPEC> for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC> {
        &self.fields_to_composer
    }
}

impl<I, LANG, SPEC> DocsComposable for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I, LANG, SPEC> FFIObjectComposable for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        if let Some(ref obj_composer) = self.ffi_object_composer {
            FFIObjectPresentation::Full(obj_composer.compose(&())
                .present(&self.source_ref())
                .to_token_stream())
        } else {
            FFIObjectPresentation::Empty
        }
    }
}

impl<I, LANG, SPEC> BindingComposable<LANG, SPEC> for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>> {
        self.bindings_composer.as_ref()
            .map(|c| c.compose(&self.source_ref()))
            .unwrap_or(Depunctuated::new())
    }
}

impl<I, SPEC> InterfaceComposable<SPEC::Interface> for ItemComposer<I, RustFermentate, SPEC>
    where I: DelimiterTrait + ?Sized,
          SPEC: RustSpecification,
          Self: GenericsComposable<SPEC::Gen>
            + AttrComposable<SPEC::Attr>
            + TypeAspect<SPEC::TYC> {

    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let generics = self.compose_generics();
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, self.present_aspect(FFIAspect::From), &generics),
            InterfacePresentation::conversion_to(&attrs, &types, self.present_aspect(FFIAspect::To), &generics),
            InterfacePresentation::conversion_destroy(&attrs, &types, self.present_aspect(FFIAspect::Destroy), &generics),
            InterfacePresentation::drop(&attrs, ffi_type, self.present_aspect(FFIAspect::Drop))
        ])
    }
}

impl<I, SPEC> SourceFermentable<RustFermentate> for ItemComposer<I, RustFermentate, SPEC>
    where I: DelimiterTrait + ?Sized,
          SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let conversions = self.ffi_conversions_composer.as_ref().map(|_| self.compose_interfaces()).unwrap_or(Depunctuated::new());
        let comment = self.ffi_object_composer.as_ref().map(|_| self.compose_docs()).unwrap_or(DocPresentation::Empty);
        RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment,
            ffi_presentation: self.compose_object(),
            conversions,
            bindings: self.compose_bindings().present(&self.source_ref()),
            traits: Depunctuated::new()
        }
        // #[cfg(feature = "objc")]
        // fermentate.extend(self.objc_composer.ferment-sys(&self.context()));
        // fermentate
    }
}






