use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::composer::{AttrsComposer, BindingComposer, Composer, ComposerAspect, ContextComposer, ConversionComposer, ConversionsComposer, FFIBindingsComposer, FFIConversionComposer, FieldsComposer, FieldTypeComposer, FieldTypesContext, HasParent, ItemComposerFieldTypesContextPresenter, ItemComposerLocalConversionContextPresenter, ItemComposerTokenStreamPresenter, ItemParentComposer, IteratorConversionComposer, MethodComposer, NameComposer, OwnedComposer, OwnedFieldTypeComposer, OwnerIteratorConversionComposer, SimplePairConversionComposer, SimpleComposerPresenter, SimpleItemParentContextComposer};
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::Conversion;
use crate::interface::{DEFAULT_DOC_PRESENTER, EMPTY_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, LAMBDA_CONVERSION_PRESENTER, obj, ROOT_DESTROY_CONTEXT_COMPOSER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER};
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation};

pub const EMPTY_CONTEXT_COMPOSER: SimpleItemParentContextComposer = ContextComposer::new(
    EMPTY_PRESENTER,
    EMPTY_CONTEXT_PRESENTER);

pub const STRUCT_DESTROY_CONVERSIONS_COMPOSER: IteratorConversionComposer =
    |fields|
        IteratorPresentationContext::StructDestroy(fields.clone());
pub const TYPE_ALIAS_FROM_CONVERSIONS_COMPOSER: OwnerIteratorConversionComposer =
    |(_, fields)|
        OwnerIteratorPresentationContext::TypeAliasFromConversion(fields);
pub const TYPE_ALIAS_TO_CONVERSIONS_COMPOSER: OwnerIteratorConversionComposer =
    |local_context|
        OwnerIteratorPresentationContext::TypeAliasToConversion(local_context);
pub const TYPE_ALIAS_ROOT_COMPOSER: OwnerIteratorConversionComposer =
    |local_context|
        OwnerIteratorPresentationContext::TypeAlias(local_context);

pub const FFI_STRUCT_COMPOSER: SimpleItemParentContextComposer = ContextComposer::new(
    SIMPLE_PRESENTER,
    FIELDS_FROM_PRESENTER);
pub const DESTROY_STRUCT_COMPOSER: SimpleItemParentContextComposer = ContextComposer::new(
    ROOT_DESTROY_CONTEXT_COMPOSER,
    EMPTY_CONTEXT_PRESENTER);
pub const DEFAULT_DOC_COMPOSER: SimpleItemParentContextComposer = ContextComposer::new(
    DEFAULT_DOC_PRESENTER,
    TARGET_NAME_PRESENTER);

pub const DROP_STRUCT_COMPOSER: ConversionComposer<ItemParentComposer, FieldTypesContext, TokenStream2, Vec<OwnedItemPresenterContext>, IteratorPresentationContext> = ConversionComposer::new(
    SIMPLE_CONVERSION_PRESENTER,
    EMPTY_CONTEXT_PRESENTER,
    OwnedComposer::new(
        STRUCT_DESTROY_CONVERSIONS_COMPOSER,
        FIELD_TYPES_COMPOSER,
        SIMPLE_PRESENTER,
        STRUCT_FIELD_TYPE_DESTROY_PRESENTER));

pub const FIELDS_FROM_PRESENTER: ItemComposerTokenStreamPresenter =
    |composer| composer.fields_from_composer.compose(&composer.context.borrow());
pub const FIELDS_TO_PRESENTER: ItemComposerTokenStreamPresenter =
    |composer| composer.fields_to_composer.compose(&composer.context.borrow());
pub const TARGET_NAME_PRESENTER: ItemComposerTokenStreamPresenter =
    |composer| composer.target_name_composer.compose(&composer.context.borrow());
pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ItemComposerTokenStreamPresenter =
    |_| quote!(&*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ItemComposerTokenStreamPresenter =
    |_| obj();
pub const EMPTY_CONTEXT_PRESENTER: ItemComposerTokenStreamPresenter =
    |_| quote!();

pub const FFI_NAME_CONTEXT_PRESENTER: ItemComposerTokenStreamPresenter =
    |composer| composer.ffi_name_composer.compose(&composer.context.borrow());

pub const FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| {
        let field_name = field_type.name();
        field_type.from(quote!(ffi_ref.#field_name))
    };
pub const DEREF_FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| {
        let field_path = field_type.name();
        field_type.from(quote!(*#field_path))
    };

pub const TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| {
        field_type.to(obj())
    };

pub const STRUCT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| {
        let field_name = field_type.name();
        field_type.to(quote!(obj.#field_name))
    };

pub const ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| {
        field_type.to(field_type.name())
    };
pub const ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| {
        let field_path = field_type.name();
        field_type.destroy(quote!(*#field_path))
    };
pub const STRUCT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| {
        let field_name = field_type.name();
        field_type.destroy(quote!(ffi_ref.#field_name))
    };

pub const TYPE_ALIAS_FIELD_PRESENTER: OwnedFieldTypeComposer =
    |field_type| OwnedItemPresenterContext::DefaultFieldType(field_type);

pub const ENUM_VARIANT_FIELD_PRESENTER: OwnedFieldTypeComposer =
    |field_type|
        OwnedItemPresenterContext::DefaultField(field_type);

pub const TYPE_ALIAS_BINDING_ARG_PRESENTER: OwnedFieldTypeComposer =
    |field_type|
        OwnedItemPresenterContext::BindingArg(field_type);
pub const TYPE_ALIAS_BINDING_FIELD_PRESENTER: OwnedFieldTypeComposer =
    |field_type|
        OwnedItemPresenterContext::BindingField(field_type);

pub const ENUM_VARIANT_DROP_CONVERSIONS_PRESENTER: IteratorConversionComposer =
    |fields|
        IteratorPresentationContext::DefaultDestroyFields(fields);


pub const TYPE_ALIAS_BINDING_ROOT_PRESENTER: IteratorConversionComposer =
    |fields|
        IteratorPresentationContext::Round(fields);


pub const BINDING_GETTER_COMPOSER: BindingComposer = |(root_obj_type, field_name, field_type)|
    BindingPresentation::Getter {
        field_name: field_name.to_token_stream(),
        obj_type: root_obj_type.to_token_stream(),
        field_type: field_type.to_token_stream()
    };

pub const BINDING_SETTER_COMPOSER: BindingComposer = |(root_obj_type, field_name, field_type)|
    BindingPresentation::Setter {
        field_name: field_name.to_token_stream(),
        obj_type: root_obj_type.to_token_stream(),
        field_type: field_type.to_token_stream()
    };
pub const TARGET_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenter =
    |composer|
        (TARGET_NAME_PRESENTER(composer), composer.field_types.clone());
pub const FFI_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenter =
    |composer|
        (FFI_NAME_CONTEXT_PRESENTER(composer), composer.field_types.clone());

pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter =
    |composer| composer.field_types.clone();

pub struct ItemComposer {
    pub context: Rc<RefCell<ScopeContext>>,
    pub need_drop_presentation: bool,
    pub ffi_name_composer: NameComposer<ItemParentComposer>,
    pub target_name_composer: NameComposer<ItemParentComposer>,
    pub attrs_composer: AttrsComposer<ItemParentComposer>,
    pub ffi_conversions_composer: FFIConversionComposer<ItemParentComposer>,
    pub fields_from_composer: FieldsComposer<ItemParentComposer>,
    pub fields_to_composer: FieldsComposer<ItemParentComposer>,
    pub fields_get_composer: MethodComposer<ItemParentComposer>,
    pub fields_set_composer: MethodComposer<ItemParentComposer>,
    pub ffi_object_composer: SimpleItemParentContextComposer,
    pub doc_composer: SimpleItemParentContextComposer,

    pub field_types: FieldTypesContext,
}


impl ItemComposer {

    pub(crate) fn type_alias_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        conversions_composer: ConversionsComposer
    ) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            attrs,
            context,
            TYPE_ALIAS_ROOT_COMPOSER,
            DEFAULT_DOC_COMPOSER,
            TYPE_ALIAS_FIELD_PRESENTER,
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    OwnedComposer::new(TYPE_ALIAS_FROM_CONVERSIONS_COMPOSER, TARGET_NAME_LOCAL_CONVERSION_COMPOSER, SIMPLE_CONVERSION_PRESENTER, FIELD_PATH_FROM_PRESENTER)),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    TO_OBJ_CONTEXT_PRESENTER,
                    OwnedComposer::new(TYPE_ALIAS_TO_CONVERSIONS_COMPOSER, FFI_NAME_LOCAL_CONVERSION_COMPOSER, SIMPLE_CONVERSION_PRESENTER, TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER)),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                FFIBindingsComposer::new(
                    TYPE_ALIAS_BINDING_ROOT_PRESENTER,
                    TYPE_ALIAS_BINDING_ARG_PRESENTER,
                    TYPE_ALIAS_BINDING_FIELD_PRESENTER),
                ),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn struct_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorConversionComposer,
        field_presenter: OwnedFieldTypeComposer,
        root_conversion_presenter: OwnerIteratorConversionComposer,
        conversion_presenter: SimplePairConversionComposer,
        bindings_presenter: IteratorConversionComposer,
        bindings_arg_presenter: OwnedFieldTypeComposer,
        bindings_field_names_presenter: OwnedFieldTypeComposer,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            field_presenter,
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    OwnedComposer::new(
                        root_conversion_presenter,
                        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
                        conversion_presenter,
                        FIELD_PATH_FROM_PRESENTER)),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    EMPTY_CONTEXT_PRESENTER,
                    OwnedComposer::new(
                        root_conversion_presenter,
                        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
                        conversion_presenter,
                        STRUCT_FIELD_TYPE_TO_PRESENTER)),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                FFIBindingsComposer::new(
                    bindings_presenter,
                    bindings_arg_presenter,
                    bindings_field_names_presenter),
            ),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enum_variant_default_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorConversionComposer,
        root_conversion_presenter: OwnerIteratorConversionComposer,
        conversion_presenter: SimplePairConversionComposer,
        destroy_code_context_presenter: SimpleComposerPresenter,
        destroy_presenter: SimpleComposerPresenter,
        bindings_iterator_presenter: IteratorConversionComposer,
        bindings_arg_presenter: OwnedFieldTypeComposer,
        bindings_field_names_presenter: OwnedFieldTypeComposer,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            ENUM_VARIANT_FIELD_PRESENTER,
            EMPTY_CONTEXT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    FIELDS_FROM_PRESENTER,
                    OwnedComposer::new(
                        root_conversion_presenter,
                        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
                        conversion_presenter,
                        DEREF_FIELD_PATH_FROM_PRESENTER)),
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    FIELDS_TO_PRESENTER,
                    OwnedComposer::new(
                        root_conversion_presenter,
                        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
                        conversion_presenter,
                        ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER)),
                ContextComposer::new(
                    destroy_code_context_presenter,
                    FIELDS_FROM_PRESENTER),
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    FIELDS_FROM_PRESENTER,
                    OwnedComposer::new(ENUM_VARIANT_DROP_CONVERSIONS_PRESENTER, FIELD_TYPES_COMPOSER, destroy_presenter, ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER)),
                FFIBindingsComposer::new(
                    bindings_iterator_presenter,
                    bindings_arg_presenter,
                    bindings_field_names_presenter)),
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorConversionComposer,
        doc_composer: SimpleItemParentContextComposer,
        field_presenter: OwnedFieldTypeComposer,
        ffi_object_composer: SimpleItemParentContextComposer,
        ffi_conversions_composer: FFIConversionComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: true,
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: FieldsComposer::new(
                root_presenter,
                FFI_NAME_CONTEXT_PRESENTER,
                field_presenter),
            fields_to_composer: FieldsComposer::new(
                root_presenter,
                TARGET_NAME_PRESENTER,
                field_presenter),
            fields_get_composer: MethodComposer::new(
                BINDING_GETTER_COMPOSER,
                FFI_NAME_CONTEXT_PRESENTER),
            fields_set_composer: MethodComposer::new(
                BINDING_SETTER_COMPOSER,
                FFI_NAME_CONTEXT_PRESENTER),
            ffi_conversions_composer,
            ffi_object_composer,
            doc_composer,
            field_types: vec![],
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
            conversions_composer
                .compose(&root_borrowed.context)
                .into_iter()
                .for_each(|field_type|
                    root_borrowed.add_conversion(field_type));

        }
        root
    }

    fn setup_composers(&mut self, root: &ItemParentComposer) {
        self.attrs_composer.set_parent(root);
        self.ffi_name_composer.set_parent(root);
        self.target_name_composer.set_parent(root);
        self.fields_from_composer.set_parent(root);
        self.fields_to_composer.set_parent(root);
        self.fields_get_composer.set_parent(root);
        self.fields_set_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
        self.ffi_conversions_composer.set_parent(root);
        self.doc_composer.set_parent(root);
    }

    fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.field_types.push(field_type.clone());
        self.ffi_conversions_composer.add_conversion(field_type.clone());
        self.fields_from_composer.add_conversion(field_type.clone());
        self.fields_to_composer.add_conversion(field_type.clone());
        self.fields_get_composer.add_conversion(field_type.clone());
        self.fields_set_composer.add_conversion(field_type);
    }

    // pub(crate) fn compose_attrs(&self) -> TokenStream2 {
    //     self.attrs_composer.compose(&self.context.borrow())
    // }

    pub(crate) fn compose_aspect(&self, aspect: ComposerAspect) -> TokenStream2 {
        self.ffi_conversions_composer.compose_aspect(aspect, &self.context.borrow())
    }
    pub(crate) fn make_expansion(&self) -> Expansion {
        let ctx = self.context.borrow();
        let ffi_name = self.ffi_name_composer.compose(&ctx);
        // println!("make_expansion: {}: [{}]", format_token_stream(&ffi_name), quote!(#(#traits), *));
        // TODO: avoid this
        let ffi_ident = format_ident!("{}", ffi_name.to_string());
        let target_name = self.target_name_composer.compose(&ctx);
        let conversion_presentation = ConversionInterfacePresentation::Interface {
            ffi_type: ffi_name.clone(),
            target_type: target_name.clone(),
            from_presentation: FromConversionPresentation::Struct(self.compose_aspect(ComposerAspect::From)),
            to_presentation: ToConversionPresentation::Struct(self.compose_aspect(ComposerAspect::To)),
            destroy_presentation: self.compose_aspect(ComposerAspect::Destroy),
            generics: None
        };
        let constructor_presentation = BindingPresentation::Constructor {
            ffi_ident: ffi_ident.clone(),
            ctor_arguments: self.ffi_conversions_composer.bindings_composer.compose_arguments(),
            body_presentation: self.ffi_conversions_composer.bindings_composer.present_field_names(&ctx),
            context: self.context.clone(),
        };
        let destructor_presentation = BindingPresentation::Destructor {
            ffi_name: ffi_name.clone(),
            destructor_ident: Name::Destructor(ffi_ident)
        };

        let mut bindings = vec![constructor_presentation, destructor_presentation];
        bindings.extend(self.fields_get_composer.compose(&self.context.borrow()));
        bindings.extend(self.fields_set_composer.compose(&self.context.borrow()));
        Expansion::Full {
            comment: DocPresentation::Direct(self.doc_composer.compose(&ctx)),
            ffi_presentation: FFIObjectPresentation::Full(self.ffi_object_composer.compose(&ctx)),
            conversion: conversion_presentation,
            bindings,
            drop: if self.need_drop_presentation {
                DropInterfacePresentation::Full(self.ffi_name_composer.compose(&ctx), self.compose_aspect(ComposerAspect::Drop))
            } else {
                DropInterfacePresentation::Empty
            },
            traits: self.attrs_composer.compose(&ctx)
        }
    }
}
