use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Field, Fields, Generics, Type, Visibility, VisPublic};
use syn::token::{Brace, Comma, Paren, Pub};
use syn::punctuated::Punctuated;
use crate::composer::{ComposerPresenter, constants, ConstructorComposer, Depunctuated, FFIAspect, FFIComposer, FieldsComposer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemParentComposer, MethodComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerIteratorPostProcessingComposer, ParentComposer, VariantIteratorLocalContext, CommaPunctuatedTokens, CommaPunctuatedOwnedItems, CommaPunctuatedFields};
use crate::composer::r#abstract::Composer;
use crate::composer::basic::BasicComposer;
use crate::composer::ffi_bindings::FFIBindingsComposer;
use crate::composer::constants::{BINDING_DTOR_COMPOSER, EMPTY_FIELDS_COMPOSER, ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::composer::composable::{BasicComposable, BindingComposable, ConversionComposable, DropComposable, SourceExpandable, FFIObjectComposable, NameContext, SourceAccessible, FieldsContext, FieldsConversionComposable};
use crate::composer::r#abstract::ParentLinker;
use crate::composition::{AttrsComposition, CfgAttributes};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentation::context::{FieldContext, name, name::Context, SequenceOutput};
use crate::presentation::{BindingPresentation, DestroyPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::shared::SharedAccess;
use crate::wrapped::DelimiterTrait;


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

    pub fn compose_aspect(&self, aspect: FFIAspect) -> TokenStream2 {
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


pub struct ItemComposer<I>
    where
        I: DelimiterTrait + 'static + ?Sized {
    pub base: BasicComposer<ItemParentComposer<I>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>,
    pub ffi_conversions_composer: FFIComposer<ItemParentComposer<I>>,
    pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer<I>>,
    pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer<I>>,
    pub bindings_composer: FFIBindingsComposer<ItemParentComposer<I>, I>,
    pub fields_composer: FieldsComposer,
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
    ) -> ItemParentComposer<I> {
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
    ) -> ItemParentComposer<I> {
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
    ) -> ItemParentComposer<I> {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::enum_variant_composer_conversion_unit(),
            constants::enum_variant_composer_conversion_unit(),
            constants::bypass_field_context(),
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            |_| FieldContext::Empty,
            constants::enum_variant_composer_ctor_unit(),
            EMPTY_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unnamed(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer<I> {
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
    ) -> ItemParentComposer<I> {
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
    ) -> ItemParentComposer<I> {
        Self::new::<ItemParentComposer<I>>(
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
        ctor_composer: ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> ItemParentComposer<I> {
        Self::new::<ItemParentComposer<I>>(
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
            constants::struct_ffi_composer(root_conversion_presenter, conversion_presenter),
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
        ctor_composer: ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> ItemParentComposer<I> {
        Self::new::<ItemParentComposer<I>>(
            name_context,
            None,
            attrs,
            fields,
            context,
            root_presenter,
            constants::enum_variant_composer_field_presenter(),
            constants::enum_variant_composer_object(),
            constants::enum_variant_composer_ffi_composer(root_conversion_presenter, conversion_presenter, destroy_code_context_presenter, destroy_presenter),
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
        root_presenter: ComposerPresenter<VariantIteratorLocalContext, SequenceOutput>,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>,
        ffi_conversions_composer: FFIComposer<ItemParentComposer<I>>,
        ctor_composer: ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> ItemParentComposer<I> {
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
            fields_composer,
            field_types: fields_composer(fields),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &ItemParentComposer<I>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
        self.ffi_object_composer.link(root);
        self.ffi_conversions_composer.link(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> TokenStream2 {
        self.ffi_conversions_composer.compose_aspect(aspect, &self.source_ref())
    }
}

impl<I> SourceAccessible for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
}

impl<I> SourceExpandable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.base.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionComposable::<ItemParentComposer<I>>::compose_conversion(self),
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            // traits: BasicComposable::<ItemParentComposer>::compose_attributes(self)
            traits: Depunctuated::new()
        }
    }
}

impl<I> DropComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            attrs: self.compose_attributes().to_token_stream(),
            ty: self.base.ffi_name_aspect().present(&self.source_ref()),
            body: self.compose_aspect(FFIAspect::Drop)
        }
    }
}

impl<I> FieldsContext for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn field_types_ref(&self) -> &FieldTypesContext {
        &self.field_types
    }
}

impl<I> FieldsConversionComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn fields_from(&self) -> &FieldsOwnedComposer<ParentComposer<Self>> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedComposer<ParentComposer<Self>> {
        &self.fields_to_composer
    }
}

impl<I> BasicComposable<ItemParentComposer<I>> for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }
    fn compose_generics(&self) -> Option<Generics> {
        self.base.generics.compose(self.context())
    }
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}
impl<I> NameContext for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}


impl<Parent, I> ConversionComposable<Parent> for ItemComposer<I>
    where
        Parent: SharedAccess,
        I: DelimiterTrait + ?Sized {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        (
            FromConversionPresentation::Just(self.compose_aspect(FFIAspect::From)),
            ToConversionPresentation::Simple(self.compose_aspect(FFIAspect::To)),
            DestroyPresentation::Custom(self.compose_aspect(FFIAspect::Destroy)),
            self.compose_generics()
        )
    }
}

impl<I> FFIObjectComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(
            self.ffi_object_composer
                .compose(&())
                .present(&self.context().borrow()))
    }
}

impl<I> BindingComposable for ItemComposer<I> where I: DelimiterTrait + ?Sized {

    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        self.bindings_composer.compose_bindings(&self.context().borrow(), true)
    }
}

