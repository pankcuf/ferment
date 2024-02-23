use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use std::iter::Iterator;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::composer::{AttrsComposer, BindingComposer, Composer, FFIConversionAspect, ContextComposer, ConversionsComposer, FFIBindingsComposer, FFIConversionComposer, FieldsComposer, FieldTypeComposer, FieldTypesContext, HasParent, ItemComposerFieldTypesContextPresenter, ItemComposerLocalConversionContextPresenter, ItemComposerTokenStreamPresenter, ItemParentComposer, IteratorConversionComposer, MethodComposer, NameComposer, OwnedFieldTypeComposer, OwnerIteratorConversionComposer, SimpleComposerPresenter, SimpleItemParentContextComposer, ComposerPresenterByRef, ConversionComposer, LocalConversionContext, FieldTypePresentationContextPassRef};
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::ext::Conversion;
use crate::interface::{DEFAULT_DOC_PRESENTER, EMPTY_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, LAMBDA_CONVERSION_PRESENTER2, LAMBDA_CONVERSION_PRESENTER3, obj, ROOT_DESTROY_CONTEXT_COMPOSER, SIMPLE_PRESENTER};
use crate::naming::Name;
use crate::presentation::context::{FieldTypePresentationContext, IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation};

pub const EMPTY_CONTEXT_COMPOSER: SimpleItemParentContextComposer = ContextComposer::new(
    EMPTY_PRESENTER,
    EMPTY_CONTEXT_PRESENTER);

pub const STRUCT_DROP_CONVERSIONS_COMPOSER: IteratorConversionComposer =
    |fields|
        IteratorPresentationContext::StructDropBody(fields.clone());
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
const fn type_alias_composer_from()
    -> StructConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_FROM_ROOT_PRESENTER,
        FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
        TYPE_ALIAS_FROM_CONVERSIONS_COMPOSER,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            (context.0, context.1.iter()
                .map(|field_type| {
                    let conversion_context = (field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type));
                    OwnedItemPresenterContext::FieldType(presenter(&conversion_context))
                })
                .collect())
    )
}
const fn type_alias_composer_to() -> StructConversionComposer<ItemParentComposer>  {
    ConversionComposer::new(
        FFI_TO_ROOT_PRESENTER,
        TO_OBJ_CONTEXT_PRESENTER,
        TYPE_ALIAS_TO_CONVERSIONS_COMPOSER,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type));
                let conversion = presenter(&conversion_context);
                OwnedItemPresenterContext::FieldType(conversion)
            }).collect()))
}
const fn type_alias_composer_drop() -> StructDropComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        EMPTY_CONTEXT_PRESENTER,
        STRUCT_DROP_CONVERSIONS_COMPOSER,
        FIELD_TYPES_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            context.iter()
                .map(|field_type| {
                    let conversion_context = (quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    let conversion = presenter(&conversion_context);
                    OwnedItemPresenterContext::FieldType(conversion)
                })
                .collect())
}
pub type StructConversionComposer<Parent> = ConversionComposer<
    Parent,
    LocalConversionContext,
    (TokenStream2, FieldTypePresentationContext),
    FieldTypePresentationContext,
    (TokenStream2, Vec<OwnedItemPresenterContext>),
    OwnerIteratorPresentationContext,
    TokenStream2,
    OwnerIteratorPresentationContext>;
pub type StructDropComposer<Parent> = ConversionComposer<
    Parent,
    FieldTypesContext,
    (TokenStream2, FieldTypePresentationContext),
    FieldTypePresentationContext,
    Vec<OwnedItemPresenterContext>,
    IteratorPresentationContext,
    TokenStream2,
    IteratorPresentationContext>;

const fn struct_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>
) -> StructConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_FROM_ROOT_PRESENTER,
        FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
        root_conversion_presenter,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type));
                let conversion = presenter(&conversion_context);
                OwnedItemPresenterContext::FieldType(conversion)
            }).collect()))
}

const fn struct_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>
) -> StructConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_TO_ROOT_PRESENTER,
        EMPTY_CONTEXT_PRESENTER,
        root_conversion_presenter,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (field_type.name(), STRUCT_FIELD_TYPE_TO_PRESENTER(field_type));
                let conversion = presenter(&conversion_context);
                OwnedItemPresenterContext::FieldType(conversion)
            }).collect()))
}
const fn struct_composer_drop() -> StructDropComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        EMPTY_CONTEXT_PRESENTER,
        STRUCT_DROP_CONVERSIONS_COMPOSER,
        FIELD_TYPES_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            context.iter()
                .map(|field_type| {
                    let conversion_context = (quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    let conversion = presenter(&conversion_context);
                    OwnedItemPresenterContext::FieldType(conversion)
                })
                .collect())
}
const fn enum_variant_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>
) -> StructConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(l_value, r_value)|
            OwnerIteratorPresentationContext::Lambda(l_value.clone(), Box::new(r_value)),
        FIELDS_FROM_PRESENTER,
        root_conversion_presenter,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (field_type.name(), DEREF_FIELD_PATH_FROM_PRESENTER(field_type));
                let conversion = presenter(&conversion_context);
                OwnedItemPresenterContext::FieldType(conversion)
            }).collect()))
}

const fn enum_variant_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>
) -> StructConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        LAMBDA_CONVERSION_PRESENTER2,
        FIELDS_TO_PRESENTER,
        root_conversion_presenter,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER(field_type));
                let conversion = presenter(&conversion_context);
                OwnedItemPresenterContext::FieldType(conversion)
            }).collect()))
}
const fn enum_variant_composer_drop(
    conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>,
) -> StructDropComposer<ItemParentComposer> {
    ConversionComposer::new(
        LAMBDA_CONVERSION_PRESENTER3,
        FIELDS_FROM_PRESENTER,
        ENUM_VARIANT_DROP_CONVERSIONS_PRESENTER,
        FIELD_TYPES_COMPOSER,
        conversion_presenter,
        |(context, presenter)|
            context.iter()
                .map(|field_type| {
                    let conversion_context = ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER(field_type);
                    let conversion = presenter(&(field_type.name(), conversion_context));
                    OwnedItemPresenterContext::FieldType(conversion)
                })
                .collect())
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
                type_alias_composer_from(),
                type_alias_composer_to(),
                DESTROY_STRUCT_COMPOSER,
                type_alias_composer_drop(),
                FFIBindingsComposer::new(
                    TYPE_ALIAS_BINDING_ROOT_PRESENTER,
                    FIELD_TYPES_COMPOSER,
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
        conversion_presenter: FieldTypePresentationContextPassRef,
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
                struct_composer_from(root_conversion_presenter, conversion_presenter),
                struct_composer_to(root_conversion_presenter, conversion_presenter),
                DESTROY_STRUCT_COMPOSER,
                struct_composer_drop(),
                FFIBindingsComposer::new(
                    bindings_presenter,
                    FIELD_TYPES_COMPOSER,
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
        conversion_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>,
        destroy_code_context_presenter: SimpleComposerPresenter,
        destroy_presenter: ComposerPresenterByRef<(TokenStream2, FieldTypePresentationContext), FieldTypePresentationContext>,
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
                enum_variant_composer_from(root_conversion_presenter, conversion_presenter),
                enum_variant_composer_to(root_conversion_presenter, conversion_presenter),
                ContextComposer::new(destroy_code_context_presenter, FIELDS_FROM_PRESENTER),
                enum_variant_composer_drop(destroy_presenter),
                FFIBindingsComposer::new(
                    bindings_iterator_presenter,
                    FIELD_TYPES_COMPOSER,
                    bindings_arg_presenter,
                    bindings_field_names_presenter)),
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
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
        conversions_composer: ConversionsComposer) -> ItemParentComposer
        where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: FieldsComposer::new(
                root_presenter,
                FFI_NAME_LOCAL_CONVERSION_COMPOSER,
                field_presenter),
            fields_to_composer: FieldsComposer::new(
                root_presenter,
                TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
                field_presenter),
            fields_get_composer: MethodComposer::new(
                BINDING_GETTER_COMPOSER,
                FFI_NAME_LOCAL_CONVERSION_COMPOSER),
            fields_set_composer: MethodComposer::new(
                BINDING_SETTER_COMPOSER,
                FFI_NAME_LOCAL_CONVERSION_COMPOSER),
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
                .for_each(|field_type| root_borrowed.field_types.push(field_type));
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


    // pub(crate) fn compose_attrs(&self) -> TokenStream2 {
    //     self.attrs_composer.compose(&self.context.borrow())
    // }

    pub(crate) fn compose_aspect(&self, aspect: FFIConversionAspect) -> TokenStream2 {
        self.ffi_conversions_composer.compose_aspect(aspect, &self.context.borrow())
    }
    pub(crate) fn make_expansion(&self) -> Expansion {
        let ctx = self.context.borrow();
        let ffi_name = self.ffi_name_composer.compose(&ctx);
        // println!("make_expansion: {}: [{}]", format_token_stream(&ffi_name), quote!(#(#traits), *));
        // TODO: avoid this
        let ffi_ident = format_ident!("{}", ffi_name.to_string());
        let target_name = self.target_name_composer.compose(&ctx);
        let ffi_presentation = FFIObjectPresentation::Full(self.ffi_object_composer.compose(&ctx));
        let conversion = ConversionInterfacePresentation::Interface {
            ffi_type: ffi_name.clone(),
            target_type: target_name.clone(),
            from_presentation: FromConversionPresentation::Struct(self.compose_aspect(FFIConversionAspect::From)),
            to_presentation: ToConversionPresentation::Struct(self.compose_aspect(FFIConversionAspect::To)),
            destroy_presentation: self.compose_aspect(FFIConversionAspect::Destroy),
            generics: None
        };
        let constructor_presentation = BindingPresentation::Constructor {
            ffi_ident: ffi_ident.clone(),
            ctor_arguments: self.ffi_conversions_composer.bindings_composer.compose_arguments(),
            body_presentation: self.ffi_conversions_composer.bindings_composer.compose_field_names(),
            context: self.context.clone(),
        };
        let destructor_presentation = BindingPresentation::Destructor {
            ffi_name: ffi_name.clone(),
            destructor_ident: Name::Destructor(ffi_ident)
        };

        let mut bindings = vec![constructor_presentation, destructor_presentation];
        bindings.extend(self.fields_get_composer.compose(&ctx));
        bindings.extend(self.fields_set_composer.compose(&ctx));
        let traits = self.attrs_composer.compose(&ctx);
        let comment = DocPresentation::Direct(self.doc_composer.compose(&ctx));
        let drop = DropInterfacePresentation::Full(self.ffi_name_composer.compose(&ctx), self.compose_aspect(FFIConversionAspect::Drop));
        Expansion::Full {
            comment,
            ffi_presentation,
            conversion,
            bindings,
            drop,
            traits
        }
    }
}

