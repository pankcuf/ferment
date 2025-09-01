use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use quote::ToTokens;
use syn::{Generics, Lifetime};
use syn::token::{Brace, Paren};
use ferment_macro::ComposerBase;
use crate::ast::{DelimiterTrait, Depunctuated, Void};
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, BindingComposable, CommaArgComposers, CommaPunctuatedFields, ComposerLink, DocComposer, DocsComposable, FFIAspect, FFIBindingsSpec, FFIConversionsSpec, FFIFieldsSpec, FFIObjectComposable, FFIObjectSpec, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposerLink, ItemComposerSpec, Linkable, MaybeFFIBindingsComposerLink, MaybeFFIComposerLink, NameKind, NameKindComposable, SeqKindComposerLink, SourceAccessible, SourceComposable, ArgKindPairs};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentable::{BindingPresentableContext, ScopeContextPresentable, SeqKind};
use crate::presentation::{DocPresentation, FFIObjectPresentation};


#[derive(ComposerBase)]
pub struct ItemComposer<SPEC, I>
    where SPEC: Specification + 'static,
          I: DelimiterTrait + 'static + ?Sized {
    pub base: BasicComposerLink<SPEC, Self>,
    pub fields_from_composer: FieldsOwnedSequenceComposerLink<SPEC, Self>,
    pub fields_to_composer: FieldsOwnedSequenceComposerLink<SPEC, Self>,
    pub field_composers: CommaArgComposers<SPEC>,

    pub ffi_object_composer: Option<SeqKindComposerLink<SPEC, Self>>,
    pub ffi_conversions_composer: MaybeFFIComposerLink<SPEC, Self>,
    pub bindings_composer: MaybeFFIBindingsComposerLink<SPEC, Self, ArgKindPairs<SPEC>>,
}


impl<SPEC, I> ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized {

    pub(crate) fn new<C>(
        ty_context: SPEC::TYC,
        attrs: AttrsModel,
        lifetimes: Vec<Lifetime>,
        generics: Option<Generics>,
        fields: &CommaPunctuatedFields,
        context: &ScopeContextLink
    ) -> ComposerLink<Self>
        where C: FFIFieldsSpec<SPEC, ComposerLink<Self>>
        + FFIObjectSpec<SPEC, ComposerLink<Self>>
        + FFIBindingsSpec<SPEC, ComposerLink<Self>, ArgKindPairs<SPEC>>
        + FFIConversionsSpec<SPEC, ComposerLink<Self>>
        + ItemComposerSpec<SPEC> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
            fields_from_composer: <C as FFIFieldsSpec<SPEC, ComposerLink<Self>>>::FROM,
            fields_to_composer: <C as FFIFieldsSpec<SPEC, ComposerLink<Self>>>::TO,
            bindings_composer: <C as FFIBindingsSpec<SPEC, ComposerLink<Self>, ArgKindPairs<SPEC>>>::COMPOSER,
            ffi_conversions_composer: <C as FFIConversionsSpec<SPEC, ComposerLink<Self>>>::COMPOSER,
            ffi_object_composer: <C as FFIObjectSpec<SPEC, ComposerLink<Self>>>::COMPOSER,
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

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> SeqKind<SPEC> {
        self.ffi_conversions_composer
            .as_ref()
            .expect("Composer doesn't exist")
            .compose(&aspect)
    }
}

impl<SPEC, I> FieldsContext<SPEC> for ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized {
    fn field_composers_ref(&self) -> &CommaArgComposers<SPEC> {
        &self.field_composers
    }
}

impl<SPEC, I> FieldsConversionComposable<SPEC> for ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposerLink<SPEC, Self> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposerLink<SPEC, Self> {
        &self.fields_to_composer
    }
}

impl<SPEC> NameKindComposable for ItemComposer<SPEC, Paren>
    where SPEC: Specification {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Unnamed
    }
}
impl<SPEC> NameKindComposable for ItemComposer<SPEC, Brace>
    where SPEC: Specification {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Named
    }
}
impl<SPEC> NameKindComposable for ItemComposer<SPEC, Void>
    where SPEC: Specification {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Unit
    }
}

impl<SPEC, I> DocsComposable for ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<SPEC, I> FFIObjectComposable for ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized,
          SeqKind<SPEC>: ScopeContextPresentable {
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

impl<SPEC, I> BindingComposable<SPEC> for ItemComposer<SPEC, I>
    where SPEC: Specification,
          I: DelimiterTrait + ?Sized {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<SPEC>> {
        let source = self.source_ref();
        self.bindings_composer
            .as_ref()
            .map(|c| c.compose(&source))
            .unwrap_or_else(|| Depunctuated::new())
    }
}







