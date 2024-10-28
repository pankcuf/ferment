use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Debug;
use quote::ToTokens;
use syn::{Generics, Type};
use ferment_macro::ComposerBase;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composable::{AttrsModel, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedFields, SourceComposable, ComposerLink, constants, ConstructorFieldsContext, DocsComposable, FFIAspect, FFIBindingsComposer, FFIComposer, FFIObjectComposable, FieldComposers, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, GenericsComposable, InterfaceComposable, Linkable, SourceAccessible, SourceFermentable, TypeAspect, PresentableArgumentComposerRef, ConversionSequenceComposer, BindingCtorComposer, ConstructorArgComposerRef, FieldsComposerRef, PresentableExprComposerRef, SharedComposer, FieldPathResolver, SequenceOutputComposerLink, FieldsOwnedSequenceComposerLink, BasicComposerLink, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, MaybeSequenceOutputComposer, AspectSequenceComposer, FieldsSequenceMixer, AspectPresentableArguments, GenericAspect, Composer};
use crate::context::ScopeContextLink;
use crate::ext::{Resolve, ToType};
use crate::lang::{LangFermentable, PresentableSpecification, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, ScopeContextPresentable, SeqKind, Expression, InterfaceKind};
use crate::presentation::{DocPresentation, FFIObjectPresentation, InterfacePresentation, RustFermentate};
use crate::shared::SharedAccess;

pub trait FFIObjectSpec<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    const COMPOSER: MaybeSequenceOutputComposer<Link, LANG, SPEC>;
}
pub trait FFIConversionsSpec<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<FFIComposer<Link, LANG, SPEC>>;
}
pub trait FFIBindingsSpec<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const COMPOSER: Option<FFIBindingsComposer<Link, LANG, SPEC>>;
}
pub trait FFIFieldsSpec<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldsOwnedSequenceComposer<Link, LANG, SPEC>;
    const TO: FieldsOwnedSequenceComposer<Link, LANG, SPEC>;
}
impl<T, C, LANG, SPEC> FFIFieldsSpec<ComposerLink<T>, LANG, SPEC> for C
    where T: AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + TypeAspect<SPEC::TYC>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC>
            + SourceAccessible
            + 'static,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          C: ItemComposerSpec<LANG, SPEC> {
    const FROM: FieldsOwnedSequenceComposerLink<T, LANG, SPEC> =
        constants::field_owned_sequence_composer::<T, C, LANG, SPEC>(
            |c| ((T::ffi_type_aspect(c), T::compose_generics(c)), T::field_composers(c)));
    const TO: FieldsOwnedSequenceComposerLink<T, LANG, SPEC> =
        constants::field_owned_sequence_composer::<T, C, LANG, SPEC>(
            |c| ((T::target_type_aspect(c), T::compose_generics(c)), T::field_composers(c)));
}

pub trait CtorSpec<Link, LANG, SPEC>
    where Link: SharedAccess,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT: BindingCtorComposer<LANG, SPEC>;
    const CONTEXT: SharedComposer<Link, ConstructorFieldsContext<LANG, SPEC>>;
    const ARG: ConstructorArgComposerRef<LANG, SPEC>;
}
pub trait ItemComposerSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const ROOT_PRESENTER: ConversionSequenceComposer<LANG, SPEC>;
    const FIELD_PRESENTER: PresentableArgumentComposerRef<LANG, SPEC>;
    const FROM_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC>;
    const TO_ROOT_CONVERSION_PRESENTER: AspectSequenceComposer<LANG, SPEC>;
    const FIELD_COMPOSERS: FieldsComposerRef<LANG, SPEC>;
}

// pub trait FieldPathResolveSpec<LANG, SPEC>
//     where LANG: LangFermentable,
//           SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
//           SPEC::Expr: ScopeContextPresentable,
//           Aspect<SPEC::TYC>: ScopeContextPresentable,
//           SeqKind<LANG, SPEC>: ScopeContextPresentable,
//           ArgKind<LANG, SPEC>: ScopeContextPresentable {
//     const RESOLVER: FieldPathResolver<LANG, SPEC>;
// }

pub trait FieldPathConversionResolveSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    const FROM: FieldPathResolver<LANG, SPEC>;
    const TO: FieldPathResolver<LANG, SPEC>;
    const DROP: FieldPathResolver<LANG, SPEC>;
}
pub trait FFIInterfaceMethodSpec<LANG, SPEC, SEP>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
      SPEC::Expr: ScopeContextPresentable,
      Aspect<SPEC::TYC>: ScopeContextPresentable,
      SeqKind<LANG, SPEC>: ScopeContextPresentable,
      ArgKind<LANG, SPEC>: ScopeContextPresentable,
      SEP: ToTokens {
    const RESOLVER: FieldPathResolver<LANG, SPEC>;
    const SEQ: AspectSequenceComposer<LANG, SPEC, SEP>;
    const EXPR: PresentableExprComposerRef<LANG, SPEC>;
}

pub trait ItemComposerExprSpec<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: PresentableSpecification<LANG>,
          SPEC::Expr: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    const FROM_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
    const TO_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
    const DROP_CONVERSION: PresentableExprComposerRef<LANG, SPEC>;
}

#[derive(ComposerBase)]
pub struct ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    pub fields_from_composer: FieldsOwnedSequenceComposerLink<Self, LANG, SPEC>,
    pub fields_to_composer: FieldsOwnedSequenceComposerLink<Self, LANG, SPEC>,
    pub field_composers: FieldComposers<LANG, SPEC>,

    pub ffi_object_composer: Option<SequenceOutputComposerLink<Self, LANG, SPEC>>,
    pub ffi_conversions_composer: MaybeFFIComposerLink<Self, LANG, SPEC>,
    pub bindings_composer: MaybeFFIBindingsComposerLink<Self, LANG, SPEC>,
}


impl<I, LANG, SPEC> ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {

    pub(crate) fn new<C>(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink) -> ComposerLink<Self>
        where C: FFIFieldsSpec<ComposerLink<Self>, LANG, SPEC>
        + FFIObjectSpec<ComposerLink<Self>, LANG, SPEC>
        + FFIBindingsSpec<ComposerLink<Self>, LANG, SPEC>
        + FFIConversionsSpec<ComposerLink<Self>, LANG, SPEC>
        + ItemComposerSpec<LANG, SPEC> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: <C as FFIFieldsSpec<ComposerLink<Self>, LANG, SPEC>>::FROM,
            fields_to_composer: <C as FFIFieldsSpec<ComposerLink<Self>, LANG, SPEC>>::TO,
            bindings_composer: <C as FFIBindingsSpec<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            ffi_conversions_composer: <C as FFIConversionsSpec<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            ffi_object_composer: <C as FFIObjectSpec<ComposerLink<Self>, LANG, SPEC>>::COMPOSER,
            field_composers: C::FIELD_COMPOSERS(fields),
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
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> SeqKind<LANG, SPEC> {
        self.ffi_conversions_composer
            .as_ref()
            .expect("Composer doesn't exist")
            .compose(&aspect)
    }
    pub(crate) fn present_aspect(&self, aspect: FFIAspect) -> <SeqKind<LANG, SPEC> as ScopeContextPresentable>::Presentation {
        self.compose_aspect(aspect)
            .present(&self.source_ref())
    }
}

impl<I, LANG, SPEC> FieldsContext<LANG, SPEC> for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable{
    fn field_composers_ref(&self) -> &FieldComposers<LANG, SPEC> {
        &self.field_composers
    }
}

impl<I, LANG, SPEC> FieldsConversionComposable<LANG, SPEC> for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposerLink<Self, LANG, SPEC> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposerLink<Self, LANG, SPEC> {
        &self.fields_to_composer
    }
}

impl<I, LANG, SPEC> DocsComposable for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I, LANG, SPEC> FFIObjectComposable for ItemComposer<I, LANG, SPEC>
    where I: DelimiterTrait + ?Sized,
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
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
          LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Type: Resolve<<SPEC as Specification<LANG>>::Var> {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>> {
        let source = self.source_ref();
        self.bindings_composer
            .as_ref()
            .map(|c| c.compose(&source))
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
        let conversions = self.ffi_conversions_composer
            .as_ref()
            .map(|_| self.compose_interfaces())
            .unwrap_or(Depunctuated::new());
        let comment = self.ffi_object_composer
            .as_ref()
            .map(|_| self.compose_docs())
            .unwrap_or(DocPresentation::Empty);
        RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment,
            ffi_presentation: self.compose_object(),
            conversions,
            bindings: self.compose_bindings().present(&self.source_ref()),
            traits: Depunctuated::new()
        }
    }
}






