use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Attribute, Generics};
use syn::token::Comma;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, GenModel};
use crate::composer::{BasicComposer, BindingComposable, CommaPunctuatedFields, ComposerPresenter, constants, CtorSequenceComposer, DocsComposable, FFIBindingsComposer, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldComposers, MethodComposer, OwnedFieldTypeComposerRef, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorConversionComposer, ComposerLink, Linkable, SourceAccessible, Composer, SourceFermentable2, BasicComposerOwner, GenericsComposable, AttrComposable, NameContext};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{BindingPresentableContext, Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::DocPresentation;
use crate::shared::SharedAccess;

// #[derive(BasicComposerOwner)]
pub struct OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized + 'static,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub fields_from_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub fields_to_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub bindings_composer: FFIBindingsComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub field_types: FieldComposers<LANG, SPEC>,
}

impl<I, LANG, SPEC, Gen> OpaqueItemComposer<I, LANG, SPEC, Gen>
    where Self: GenericsComposable<Gen>
            + AttrComposable<SPEC>
            + NameContext<Context>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {

    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::struct_ctor_sequence_composer(
                constants::struct_composer_ctor_root(),
                constants::unnamed_opaque_ctor_context_composer(),
                constants::struct_composer_ctor_unnamed_item()
            ),
            constants::struct_unnamed_fields_composer()
        )
    }
    pub fn struct_composer_named(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            context,
            constants::struct_composer_root_presenter_named(),
            constants::named_struct_field_composer(),
            constants::struct_ctor_sequence_composer(
                constants::struct_composer_ctor_root(),
                constants::named_opaque_ctor_context_composer(),
                constants::struct_composer_ctor_named_opaque_item()
            ),
            constants::struct_named_fields_composer()
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn struct_composer(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
        field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>,
        ctor_composer: CtorSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
        fields_composer: FieldsComposerRef<LANG, SPEC>) -> ComposerLink<Self> {
        Self::new::<ComposerLink<Self>>(
            Context::Struct { ident: target_name.clone(), attrs: attrs.cfg_attributes() },
            Some(generics.clone()),
            AttrsModel::from(attrs),
            fields,
            context,
            root_presenter,
            field_presenter,
            ctor_composer,
            fields_composer
        )
    }
    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new<T: SharedAccess + 'static>(
        name_context: Context,
        generics: Option<Generics>,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
        root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
        field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>,
        ctor_composer: CtorSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
        fields_composer: FieldsComposerRef<LANG, SPEC>) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(attrs, name_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: constants::fields_from_composer(root_presenter, field_presenter),
            fields_to_composer: constants::fields_to_composer(root_presenter, field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(constants::binding_dtor_composer(), constants::composer_target_binding::<Self, LANG, SPEC, Gen>()),
                MethodComposer::new(constants::binding_getter_composer(), constants::target_aspect_seq_context()),
                MethodComposer::new(constants::binding_setter_composer(), constants::target_aspect_seq_context()),
                false
            ),
            field_types: fields_composer(fields),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &ComposerLink<Self>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
    }
}
impl<I, LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<I, LANG, SPEC, Gen> AttrComposable<SPEC> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}
impl<I, LANG, SPEC, Gen> GenericsComposable<Gen> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_generics(&self) -> Gen {
        self.base().compose_generics()
    }
}

impl<I, LANG, SPEC, Gen> NameContext<Context> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait +?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}

impl<I, LANG, SPEC, Gen> FieldsContext<LANG, SPEC> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn field_types_ref(&self) -> &FieldComposers<LANG, SPEC> {
        &self.field_types
    }
}
impl<I, LANG, SPEC, Gen> FieldsConversionComposable<LANG, SPEC, Gen> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.fields_to_composer
    }
}
impl<I, LANG, SPEC, Gen> DocsComposable for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I, LANG, SPEC, Gen> SourceAccessible for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl<I, LANG, SPEC, Gen> SourceFermentable2<LANG> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn ferment(&self) -> Depunctuated<LANG> {
        Depunctuated::new()
    }
}
impl<I, LANG, SPEC, Gen> BindingComposable<LANG, SPEC, Gen> for OpaqueItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC, Gen>> {
        self.bindings_composer.compose(&self.source_ref())
    }
}
