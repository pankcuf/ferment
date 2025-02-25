use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use quote::ToTokens;
use syn::{Generics, Lifetime};
use syn::token::{Brace, Paren};
use ferment_macro::ComposerBase;
use crate::ast::{DelimiterTrait, Depunctuated, Void};
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerLink, BasicComposerOwner, BindingComposable, CommaArgComposers, CommaPunctuatedFields, ComposerLink, DocsComposable, FFIAspect, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectComposable, FFIObjectSpec, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposerLink, GenericsComposable, InterfaceComposable, ItemComposerSpec, Linkable, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, NameKind, NameKindComposable, SeqKindComposerLink, SourceAccessible, SourceComposable, SourceFermentable, TypeAspect, ArgKindPairs, LifetimesComposable, VariableComposer};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{BindingPresentableContext, ScopeContextPresentable, SeqKind};
use crate::presentation::{DocComposer, DocPresentation, FFIObjectPresentation, InterfacePresentation, RustFermentate};


#[derive(ComposerBase)]
pub struct ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
    pub fields_from_composer: FieldsOwnedSequenceComposerLink<LANG, SPEC, Self>,
    pub fields_to_composer: FieldsOwnedSequenceComposerLink<LANG, SPEC, Self>,
    pub field_composers: CommaArgComposers<LANG, SPEC>,

    pub ffi_object_composer: Option<SeqKindComposerLink<LANG, SPEC, Self>>,
    pub ffi_conversions_composer: MaybeFFIComposerLink<LANG, SPEC, Self>,
    pub bindings_composer: MaybeFFIBindingsComposerLink<LANG, SPEC, Self, ArgKindPairs<LANG, SPEC>>,
}


impl<LANG, SPEC, I> ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized {

    pub(crate) fn new<C>(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink) -> ComposerLink<Self>
        where C: FFIFieldsSpec<LANG, SPEC, ComposerLink<Self>>
        + FFIObjectSpec<LANG, SPEC, ComposerLink<Self>>
        + FFIBindingsSpec<LANG, SPEC, ComposerLink<Self>, ArgKindPairs<LANG, SPEC>>
        + FFIConversionsSpec<LANG, SPEC, ComposerLink<Self>>
        + ItemComposerSpec<LANG, SPEC> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics.clone()), LifetimesModel::new(lifetimes), Rc::clone(context)),
            fields_from_composer: <C as FFIFieldsSpec<LANG, SPEC, ComposerLink<Self>>>::FROM,
            fields_to_composer: <C as FFIFieldsSpec<LANG, SPEC, ComposerLink<Self>>>::TO,
            bindings_composer: <C as FFIBindingsSpec<LANG, SPEC, ComposerLink<Self>, ArgKindPairs<LANG, SPEC>>>::COMPOSER,
            ffi_conversions_composer: <C as FFIConversionsSpec<LANG, SPEC, ComposerLink<Self>>>::COMPOSER,
            ffi_object_composer: <C as FFIObjectSpec<LANG, SPEC, ComposerLink<Self>>>::COMPOSER,
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
}

impl<LANG, SPEC, I> FieldsContext<LANG, SPEC> for ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized {
    fn field_composers_ref(&self) -> &CommaArgComposers<LANG, SPEC> {
        &self.field_composers
    }
}

impl<LANG, SPEC, I> FieldsConversionComposable<LANG, SPEC> for ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposerLink<LANG, SPEC, Self> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposerLink<LANG, SPEC, Self> {
        &self.fields_to_composer
    }
}

impl<LANG, SPEC> NameKindComposable for ItemComposer<LANG, SPEC, Paren>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Unnamed
    }
}
impl<LANG, SPEC> NameKindComposable for ItemComposer<LANG, SPEC, Brace>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Named
    }
}
impl<LANG, SPEC> NameKindComposable for ItemComposer<LANG, SPEC, Void>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Unit
    }
}

impl<LANG, SPEC, I> DocsComposable for ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<LANG, SPEC, I> FFIObjectComposable for ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
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

impl<LANG, SPEC, I> BindingComposable<LANG, SPEC> for ItemComposer<LANG, SPEC, I>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          I: DelimiterTrait + ?Sized,
          VariableComposer<LANG, SPEC>: SourceComposable<Source = ScopeContext, Output = SPEC::Var> {

          // Type: Resolve<SPEC::Var> {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>> {
        let source = self.source_ref();
        self.bindings_composer
            .as_ref()
            .map(|c| c.compose(&source))
            .unwrap_or(Depunctuated::new())
    }
}

impl<SPEC, I> InterfaceComposable<SPEC::Interface> for ItemComposer<RustFermentate, SPEC, I>
    where SPEC: RustSpecification,
          I: DelimiterTrait + ?Sized,
          Self: GenericsComposable<SPEC::Gen>
            + LifetimesComposable<SPEC::Lt>
            + AttrComposable<SPEC::Attr>
            + TypeAspect<SPEC::TYC>
            + NameKindComposable {

    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
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

impl<SPEC, I> SourceFermentable<RustFermentate> for ItemComposer<RustFermentate, SPEC, I>
    where SPEC: RustSpecification,
          I: DelimiterTrait + ?Sized,
          Self: NameKindComposable {
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






