use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Field, Fields, Generics, Type, Visibility, VisPublic};
use syn::token::{Brace, Comma, Paren, Pub};
use syn::punctuated::Punctuated;
use ferment_macro::BasicComposerOwner;
use crate::ast::Depunctuated;
use crate::composable::{AttrsComposition, CfgAttributes};
use crate::composer::{BasicComposable, BasicComposer, BindingComposable, CommaPunctuatedFields, Composer, ComposerPresenter, constants, ConversionComposable, CtorSequenceComposer, DocsComposable, DropComposable, FFIAspect, FFIBindingsComposer, FFIComposer, FFIObjectComposable, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemParentComposer, Linkable, MethodComposer, NameContext, OwnedFieldTypeComposerRef, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorConversionComposer, OwnerIteratorPostProcessingComposer, ParentComposer, SourceAccessible, SourceExpandable};
use crate::composer::constants::{BINDING_DTOR_COMPOSER, EMPTY_FIELDS_COMPOSER, ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentable::{ScopeContextPresentable, Expression, Context, SequenceOutput};
use crate::presentation::{BindingPresentation, DestroyPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation};
use crate::shared::SharedAccess;
use crate::ast::DelimiterTrait;


#[allow(unused)]
pub enum ItemComposerWrapper {
    EnumVariantNamed(ItemParentComposer<Brace>),
    EnumVariantUnnamed(ItemParentComposer<Paren>),
    EnumVariantUnit(ItemParentComposer<Brace>),
    StructNamed(ItemParentComposer<Brace>),
    StructUnnamed(ItemParentComposer<Paren>),
}


impl ItemComposerWrapper {
    pub fn enum_variant(fields: &Fields, name_context: Context, attrs: AttrsComposition, context: &ParentComposer<ScopeContext>) -> ItemComposerWrapper {
        match fields {
            Fields::Unit =>
                ItemComposerWrapper::EnumVariantUnit(ItemComposer::enum_variant_composer_unit(name_context, attrs, &Punctuated::new(), context)),
            Fields::Unnamed(fields) =>
                ItemComposerWrapper::EnumVariantUnnamed(ItemComposer::enum_variant_composer_unnamed(name_context, attrs, &fields.unnamed, context)),
            Fields::Named(fields) =>
                ItemComposerWrapper::EnumVariantUnit(ItemComposer::enum_variant_composer_named(name_context, attrs, &fields.named, context)),
        }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().compose_aspect(aspect),
        }
    }
    pub fn compose_ctor(&self, source: &ScopeContext) -> BindingPresentation {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().bindings_composer.ctor_composer.compose(&()).present(&source),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().bindings_composer.ctor_composer.compose(&()).present(&source),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().bindings_composer.ctor_composer.compose(&()).present(&source),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().bindings_composer.ctor_composer.compose(&()).present(&source),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().bindings_composer.ctor_composer.compose(&()).present(&source),
        }
    }
    pub fn compose_attributes(&self) -> Depunctuated<Expansion> {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().compose_attributes(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().compose_attributes(),
            ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().compose_attributes(),
            ItemComposerWrapper::StructNamed(composer) => composer.borrow().compose_attributes(),
            ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().compose_attributes(),
        }
    }
}


#[derive(BasicComposerOwner)]
pub struct ItemComposer<I>
    where
        I: DelimiterTrait + 'static + ?Sized {
    pub base: BasicComposer<ParentComposer<Self>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<Self>>,
    pub ffi_conversions_composer: FFIComposer<ParentComposer<Self>>,
    pub fields_from_composer: FieldsOwnedSequenceComposer<ParentComposer<Self>>,
    pub fields_to_composer: FieldsOwnedSequenceComposer<ParentComposer<Self>>,
    pub bindings_composer: FFIBindingsComposer<ParentComposer<Self>, I>,
    // pub fields_composer: FieldsComposerRef,
    pub field_types: FieldTypesContext,
}

impl<I> ItemComposer<I> where I: DelimiterTrait + ?Sized {
    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            constants::bypass_field_context(),
            constants::struct_composer_ctor_unnamed(),
            STRUCT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn struct_composer_named(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_named(),
            constants::named_struct_field_composer(),
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::struct_composer_conversion_named(),
            constants::struct_composer_ctor_named(),
            STRUCT_NAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unit(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::enum_variant_composer_conversion_unit(),
            constants::enum_variant_composer_conversion_unit(),
            constants::bypass_field_context(),
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            |_| Expression::Empty,
            constants::enum_variant_composer_ctor_unit(),
            EMPTY_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unnamed(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            constants::bypass_field_context(),
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            constants::bypass_field_context(),
            constants::enum_variant_composer_ctor_unnamed(),
            ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_named(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::struct_composer_conversion_named(),
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            constants::bypass_field_context(),
            constants::enum_variant_composer_ctor_named(),
            STRUCT_NAMED_FIELDS_COMPOSER
        )
    }
    pub(crate) fn type_alias_composer(
        target_name: &Ident,
        ty: &Type,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
    ) -> ParentComposer<Self> {
        Self::new::<ParentComposer<Self>>(
            Context::Struct {
                ident: target_name.clone(),
                attrs: attrs.cfg_attributes_expanded(),
            },
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            &Punctuated::from_iter([Field {
                vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
                ty: (*ty).clone(),
                attrs: vec![],
                ident: None,
                colon_token: None,
            }]),
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::struct_composer_object(),
            constants::type_alias_composer_ffi_conversions(),
            constants::struct_composer_ctor_unnamed(),
            STRUCT_UNNAMED_FIELDS_COMPOSER
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn struct_composer(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        field_presenter: OwnedFieldTypeComposerRef,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorSequenceComposer<ParentComposer<Self>, I>,
        fields_composer: FieldsComposerRef) -> ParentComposer<Self> {
        Self::new::<ParentComposer<Self>>(
            Context::Struct {
                ident: target_name.clone(),
                attrs: attrs.cfg_attributes_expanded(),
            },
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            fields,
            context,
            root_presenter,
            field_presenter,
            constants::struct_composer_object(),
            constants::struct_ffi_composer(
                root_conversion_presenter,
                conversion_presenter),
            ctor_composer,
            fields_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn enum_variant_composer(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        destroy_code_context_presenter: ComposerPresenter<SequenceOutput, SequenceOutput>,
        destroy_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorSequenceComposer<ParentComposer<Self>, I>,
        fields_composer: FieldsComposerRef) -> ParentComposer<Self> {
        Self::new::<ParentComposer<Self>>(
            name_context,
            None,
            attrs,
            fields,
            context,
            root_presenter,
            constants::enum_variant_composer_field_presenter(),
            constants::enum_variant_composer_object(),
            constants::enum_variant_composer_ffi_composer(
                root_conversion_presenter,
                conversion_presenter,
                destroy_code_context_presenter,
                destroy_presenter),
            ctor_composer,
            fields_composer)
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new<T: SharedAccess + 'static>(
        name_context: Context,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>,
        root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems, SequenceOutput>,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<Self>>,
        ffi_conversions_composer: FFIComposer<ParentComposer<Self>>,
        ctor_composer: CtorSequenceComposer<ParentComposer<Self>, I>,
        fields_composer: FieldsComposerRef) -> ParentComposer<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, name_context, generics, constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: constants::fields_from_composer(root_presenter, field_presenter),
            fields_to_composer: constants::fields_to_composer(root_presenter, field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(
                    BINDING_DTOR_COMPOSER,
                    constants::composer_ffi_binding()
                ),
                MethodComposer::new(
                    |(root_obj_type, field_name, field_type, attrs, generics)|
                        BindingPresentation::Getter {
                            attrs,
                            name: Name::Getter(root_obj_type.to_path(), field_name.clone()),
                            field_name,
                            obj_type: root_obj_type,
                            field_type,
                            generics
                        },
                    constants::ffi_aspect_seq_context()),
                MethodComposer::new(
                    |(root_obj_type, field_name, field_type, attrs, generics)|
                        BindingPresentation::Setter {
                            attrs,
                            name: Name::Setter(root_obj_type.to_path(), field_name.clone()),
                            field_name,
                            obj_type: root_obj_type,
                            field_type,
                            generics
                        },
                    constants::ffi_aspect_seq_context())
            ),
            ffi_conversions_composer,
            ffi_object_composer,
            field_types: fields_composer(fields),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &ParentComposer<Self>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
        self.ffi_object_composer.link(root);
        self.ffi_conversions_composer.link(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput {
        self.ffi_conversions_composer.compose_aspect(aspect)
    }
}

impl<I> SourceExpandable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.base.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionComposable::<ParentComposer<Self>>::compose_conversion(self),
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            traits: Depunctuated::new()
        }
    }
}

impl<I> DropComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            attrs: self.compose_attributes().to_token_stream(),
            ty: self.compose_ffi_name(),
            body: self.compose_aspect(FFIAspect::Drop).present(&self.source_ref())
        }
    }
}

impl<I> FieldsContext for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn field_types_ref(&self) -> &FieldTypesContext {
        &self.field_types
    }
}

impl<I> FieldsConversionComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> {
        &self.fields_to_composer
    }
}

impl<I> DocsComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<Parent, I> ConversionComposable<Parent> for ItemComposer<I>
    where
        Parent: SharedAccess,
        I: DelimiterTrait + ?Sized {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        let source = self.source_ref();
        (
            FromConversionPresentation::Just(self.compose_aspect(FFIAspect::From).present(&source)),
            ToConversionPresentation::Simple(self.compose_aspect(FFIAspect::To).present(&source)),
            DestroyPresentation::Custom(self.compose_aspect(FFIAspect::Destroy).present(&source)),
            self.compose_generics()
        )
    }
}

impl<I> FFIObjectComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(
            self.ffi_object_composer
                .compose(&())
                .present(&self.source_ref()))
    }
}

impl<I> BindingComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        self.bindings_composer.compose_bindings(&self.source_ref(), true)
    }
}

