use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use std::convert::Into;
use std::iter::Iterator;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Path};
use crate::composer::{AttrsComposer, Composer, FFIAspect, ContextComposer, ConversionsComposer, FFIComposer, FieldTypeComposer, FieldTypesContext, HasParent, ItemComposerFieldTypesContextPresenter, ItemComposerLocalConversionContextPresenter, ItemComposerTokenStreamPresenter, ItemParentComposer, IteratorConversionComposer, MethodComposer, NameComposer, OwnerIteratorConversionComposer, SimpleItemParentContextComposer, ConversionComposer, FieldTypePresentationContextPassRef, ItemComposerPresenter, ComposerPresenter, OwnerIteratorLocalContext, OwnedFieldTypeComposerRef, OwnerIteratorPostProcessingComposer, FieldsOwnedComposer, FFIConversionComposer, DropConversionComposer, BindingAccessorComposer, BindingAccessorContext, DestructorContext, BindingDtorComposer, LocalConversionContext, ComposerPresenterByRef, CtorOwnedComposer, ConstructorPresentableContext, SharedComposer};
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, Mangle, Pop};
use crate::interface::{DEFAULT_DOC_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, ROOT_DESTROY_CONTEXT_COMPOSER};
use crate::naming::Name;
use crate::presentation::context::{FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::binding::BindingPresentableContext;

pub const STRUCT_DROP_CONVERSIONS_COMPOSER: IteratorConversionComposer =
    |fields| IteratorPresentationContext::StructDropBody(fields.clone());
pub const FFI_STRUCT_COMPOSER: OwnerIteratorPostProcessingComposer =
    ContextComposer::new(|name| name, FIELDS_FROM_PRESENTER);
pub const DESTROY_STRUCT_COMPOSER: OwnerIteratorPostProcessingComposer =
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, EMPTY_CONTEXT_PRESENTER);
pub const DEFAULT_DOC_COMPOSER: SimpleItemParentContextComposer =
    ContextComposer::new(DEFAULT_DOC_PRESENTER, TARGET_NAME_PRESENTER);

pub const FIELDS_FROM_PRESENTER: ItemComposerPresenter<OwnerIteratorPresentationContext> =
    |composer| composer.fields_from_composer.compose(&());
pub const FIELDS_TO_PRESENTER: ItemComposerPresenter<OwnerIteratorPresentationContext> =
    |composer| composer.fields_to_composer.compose(&());
pub const TARGET_NAME_PRESENTER: ItemComposerTokenStreamPresenter =
    |composer| composer.target_name_composer.compose(&());

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ItemComposerPresenter<OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi));
pub const EMPTY_CONTEXT_PRESENTER: ItemComposerPresenter<OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::Empty;

/// FieldTypeComposers
pub const FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| field_type.from(FieldTypePresentableContext::FfiRefWithFieldName(FieldTypePresentableContext::Simple(field_type.name()).into()));
pub const DEREF_FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| field_type.from(FieldTypePresentableContext::Deref(field_type.name()));
pub const TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.to(FieldTypePresentableContext::Obj);
pub const STRUCT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.to(FieldTypePresentableContext::ObjFieldName(field_type.name()));
pub const ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.to(FieldTypePresentableContext::FieldTypeConversionName(field_type.clone()));
pub const ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| field_type.destroy(FieldTypePresentableContext::Deref(field_type.name()));
pub const STRUCT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| field_type.destroy(FieldTypePresentableContext::FfiRefWithFieldName(FieldTypePresentableContext::FieldTypeConversionName(field_type.clone()).into()));

pub const TARGET_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenter =
    |composer| (TARGET_NAME_PRESENTER(composer), composer.field_types.clone());
pub const FFI_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenter =
    |composer| (composer.ffi_name_composer.compose(&()), composer.field_types.clone());
pub const FFI_NAME_DTOR_COMPOSER: ItemComposerPresenter<DestructorContext> =
    |composer| {
        let ffi_name = composer.ffi_name_composer.compose(&());
        (parse_quote!(#ffi_name), ffi_name)
    };

pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter =
    |composer| composer.field_types.clone();

pub const BYPASS_FIELD_CONTEXT: FieldTypePresentationContextPassRef =
    |(_, context)| context.clone();


/// Bindings
pub const BINDING_DTOR_COMPOSER: BindingDtorComposer =
    |(ffi_ident, ffi_name)|
        BindingPresentation::Destructor {
            ffi_name,
            destructor_ident: Name::Destructor(ffi_ident)
        };
pub const BINDING_GETTER_COMPOSER: BindingAccessorComposer =
    |(root_obj_type, field_name, field_type)|
        BindingPresentation::Getter {
            field_name: field_name.to_token_stream(),
            obj_type: root_obj_type.to_token_stream(),
            field_type: field_type.to_token_stream()
        };

pub const BINDING_SETTER_COMPOSER: BindingAccessorComposer =
    |(root_obj_type, field_name, field_type)|
        BindingPresentation::Setter {
            field_name: field_name.to_token_stream(),
            obj_type: root_obj_type.to_token_stream(),
            field_type: field_type.to_token_stream()
        };

const fn type_alias_composer_from()
    -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_FROM_ROOT_PRESENTER,
        FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
        |(_, fields)|
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields),
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            (context.0, context.1.iter()
                .map(|field_type| {
                    let conversion_context = (field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type));
                    OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
                })
                .collect())
    )
}
const fn type_alias_composer_to() -> FFIConversionComposer<ItemParentComposer>  {
    ConversionComposer::new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Obj,
        |local_context|
            OwnerIteratorPresentationContext::TypeAliasToConversion(local_context),
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        |(_, conversion)|
            conversion.clone(),
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}
const fn type_alias_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        |_| OwnerIteratorPresentationContext::Empty,
        STRUCT_DROP_CONVERSIONS_COMPOSER,
        FIELD_TYPES_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            context.iter()
                .map(|field_type| {
                    let conversion_context = (quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
                })
                .collect())
}


const fn struct_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_FROM_ROOT_PRESENTER,
        FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
        root_conversion_presenter,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |((name, fields), presenter)|
            (name, fields.iter().map(|field_type| {
                let conversion_context = (field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}

const fn struct_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_TO_ROOT_PRESENTER,
        EMPTY_CONTEXT_PRESENTER,
        root_conversion_presenter,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |((name, fields), presenter)|
            (name, fields.iter().map(|field_type| {
                let conversion_context = (field_type.name(), STRUCT_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}
const fn struct_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        |_| OwnerIteratorPresentationContext::Empty,
        STRUCT_DROP_CONVERSIONS_COMPOSER,
        FIELD_TYPES_COMPOSER,
        |(_, conversion)| conversion.clone(),
        |(fields, presenter)|
            fields.iter()
                .map(|field_type| {
                    let conversion_context = (quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
                })
                .collect())
}
const fn enum_variant_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(field_path, context)|
            OwnerIteratorPresentationContext::Lambda(field_path.into(), context.into()),
        FIELDS_FROM_PRESENTER,
        root_conversion_presenter,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |((name, fields), presenter)|
            (name, fields.iter().map(|field_type| {
                let conversion_context = (field_type.name(), DEREF_FIELD_PATH_FROM_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}

const fn enum_variant_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(field_path, context)|
            OwnerIteratorPresentationContext::Lambda(field_path.into(), context.into()),
        FIELDS_TO_PRESENTER,
        root_conversion_presenter,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        |((name, fields), presenter)|
            (name, fields.iter().map(|field_type| {
                let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}
const fn enum_variant_composer_drop(
    conversion_presenter: FieldTypePresentationContextPassRef,
) -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(field_path, context)|
            IteratorPresentationContext::Lambda(field_path.into(), context.into()),
        FIELDS_FROM_PRESENTER,
        |fields| IteratorPresentationContext::DefaultDestroyFields(fields),
        FIELD_TYPES_COMPOSER,
        conversion_presenter,
        |(fields, presenter)|
            fields.iter()
                .map(|field_type| {
                    let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
                })
                .collect())
}

pub const fn fields_composer(
    root_composer: ComposerPresenter<(TokenStream2, Vec<OwnedItemPresentableContext>), OwnerIteratorPresentationContext>,
    context_composer: SharedComposer<ItemParentComposer, LocalConversionContext>,
    iterator_item_composer: OwnedFieldTypeComposerRef,
) -> FieldsOwnedComposer<ItemParentComposer> {
    FieldsOwnedComposer::new(
        root_composer,
        context_composer,
        |((name, field_types), presenter)|
            (name, field_types.iter().map(presenter).collect()),
        iterator_item_composer)
}

// const fn enum_variant_ctor_context() ->

pub const fn composer_ctor(
    context_composer: SharedComposer<ItemParentComposer, (ConstructorPresentableContext, FieldTypesContext)>,
    root_composer: ComposerPresenter<
        (ConstructorPresentableContext, Vec<(OwnedItemPresentableContext, OwnedItemPresentableContext)>),
       BindingPresentableContext>,
    iterator_item_composer: ComposerPresenterByRef<
        FieldTypeConversion,
        (OwnedItemPresentableContext, OwnedItemPresentableContext)>
) -> CtorOwnedComposer<ItemParentComposer> {
    CtorOwnedComposer::new(
        root_composer,
        context_composer,
        // |composer| {
        //     // let context = composer.ffi_name_composer.compose()
        //     let ffi_name = composer.ffi_name_composer.compose(&());
        //
        //     (ffi_name, FIELD_TYPES_COMPOSER(composer))
        // },
        |((constructor_context, fields), presenter)|
            (constructor_context, fields.iter().map(presenter).collect()),
        iterator_item_composer
    )
}
// const fn composer_ctor2(
//     root_composer: ComposerPresenter<
//         Vec<(OwnedItemPresentableContext, OwnedItemPresentableContext)>,
//         BindingPresentation>,
//     iterator_item_composer: ComposerPresenterByRef<
//         FieldTypeConversion,
//         (OwnedItemPresentableContext, OwnedItemPresentableContext)>
// ) -> CtorOwnedComposer<ItemParentComposer> {
//     CtorOwnedComposer::new(
//         root_composer,
//         FIELD_TYPES_COMPOSER,
//         |(fields, presenter)|
//             fields.iter().map(presenter).collect(),
//         iterator_item_composer
//     )
// }

const fn struct_composer_ctor_unnamed_item() -> ComposerPresenterByRef<FieldTypeConversion, (OwnedItemPresentableContext, OwnedItemPresentableContext)> {
    |field_type| (
        OwnedItemPresentableContext::BindingArg(field_type.clone()),
        OwnedItemPresentableContext::BindingFieldName(field_type.clone())
    )
}
const fn struct_composer_ctor_named_item() -> ComposerPresenterByRef<FieldTypeConversion, (OwnedItemPresentableContext, OwnedItemPresentableContext)> {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultField(field_type.clone())
    )
}


pub const fn struct_composer_ctor_unnamed() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Round(names))
        },
        struct_composer_ctor_unnamed_item())
}

pub const fn default_ctor_context_composer() -> SharedComposer<ItemParentComposer, (ConstructorPresentableContext, FieldTypesContext)> {
    |composer| {
        let ffi_name = composer.ffi_name_composer.compose(&());
        (ConstructorPresentableContext::Default(parse_quote!(#ffi_name)), FIELD_TYPES_COMPOSER(composer))
    }
}
pub const fn enum_variant_ctor_context_composer() -> SharedComposer<ItemParentComposer, (ConstructorPresentableContext, FieldTypesContext)> {
    |composer| {
        let ffi = composer.ffi_name_composer.compose(&());
        let ffi_path: Path = parse_quote!(#ffi);
        (ConstructorPresentableContext::EnumVariant(
            ffi_path.popped().to_token_stream(),
            ffi_path.to_mangled_ident_default(),
            ffi
        ), FIELD_TYPES_COMPOSER(composer))
    }
}

pub const fn struct_composer_ctor_named() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Curly(names))
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_unit() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, _): (Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Empty)
        },
        struct_composer_ctor_named_item())
}
pub const fn enum_variant_composer_ctor_unnamed() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, if names.is_empty() {
                IteratorPresentationContext::Empty
            } else {
                IteratorPresentationContext::Round(names)
            })
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_named() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Vec<OwnedItemPresentableContext>, Vec<OwnedItemPresentableContext>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, if names.is_empty() {
                IteratorPresentationContext::Empty
            } else {
                IteratorPresentationContext::Curly(names)
            })
        },
        struct_composer_ctor_named_item()
    )
}


pub struct ItemComposer {
    pub context: Rc<RefCell<ScopeContext>>,
    pub ffi_name_composer: NameComposer<ItemParentComposer>,
    pub target_name_composer: NameComposer<ItemParentComposer>,
    pub attrs_composer: AttrsComposer<ItemParentComposer>,
    pub ffi_conversions_composer: FFIComposer<ItemParentComposer>,
    pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer>,
    pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer>,

    pub getter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    pub setter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,

    pub ctor_composer: CtorOwnedComposer<ItemParentComposer>,
    pub dtor_composer: MethodComposer<ItemParentComposer, DestructorContext, DestructorContext>,

    pub ffi_object_composer: OwnerIteratorPostProcessingComposer,
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
            |local_context|
                OwnerIteratorPresentationContext::TypeAlias(local_context),
            DEFAULT_DOC_COMPOSER,
            |field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.clone()),
            FFI_STRUCT_COMPOSER,
            FFIComposer::new(
                type_alias_composer_from(),
                type_alias_composer_to(),
                DESTROY_STRUCT_COMPOSER,
                type_alias_composer_drop()),
            struct_composer_ctor_unnamed(),
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
        field_presenter: OwnedFieldTypeComposerRef,
        root_conversion_presenter: OwnerIteratorConversionComposer,
        conversion_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
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
            FFIComposer::new(
                struct_composer_from(root_conversion_presenter, conversion_presenter),
                struct_composer_to(root_conversion_presenter, conversion_presenter),
                DESTROY_STRUCT_COMPOSER,
                struct_composer_drop(),
            ),
            ctor_composer,
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enum_variant_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorConversionComposer,
        root_conversion_presenter: OwnerIteratorConversionComposer,
        conversion_presenter: FieldTypePresentationContextPassRef,
        destroy_code_context_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
        destroy_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            |field_type| OwnedItemPresentableContext::DefaultField(field_type.clone()),
            ContextComposer::new(|_| OwnerIteratorPresentationContext::Empty, EMPTY_CONTEXT_PRESENTER),
            FFIComposer::new(
                enum_variant_composer_from(root_conversion_presenter, conversion_presenter),
                enum_variant_composer_to(root_conversion_presenter, conversion_presenter),
                ContextComposer::new(destroy_code_context_presenter, FIELDS_FROM_PRESENTER),
                enum_variant_composer_drop(destroy_presenter)),
            ctor_composer,
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: ComposerPresenter<OwnerIteratorLocalContext, OwnerIteratorPresentationContext>,
        doc_composer: SimpleItemParentContextComposer,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer,
        ffi_conversions_composer: FFIComposer<ItemParentComposer>,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer
        where Self: Sized {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: fields_composer(root_presenter, FFI_NAME_LOCAL_CONVERSION_COMPOSER, field_presenter),
            fields_to_composer: fields_composer(root_presenter, TARGET_NAME_LOCAL_CONVERSION_COMPOSER, field_presenter),
            getter_composer: MethodComposer::new(BINDING_GETTER_COMPOSER, FFI_NAME_LOCAL_CONVERSION_COMPOSER),
            setter_composer: MethodComposer::new(BINDING_SETTER_COMPOSER, FFI_NAME_LOCAL_CONVERSION_COMPOSER),
            dtor_composer: MethodComposer::new(BINDING_DTOR_COMPOSER, FFI_NAME_DTOR_COMPOSER, ),
            ctor_composer,
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
        self.ctor_composer.set_parent(root);
        self.dtor_composer.set_parent(root);
        self.getter_composer.set_parent(root);
        self.setter_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
        self.ffi_conversions_composer.set_parent(root);
        self.doc_composer.set_parent(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> TokenStream2 {
        self.ffi_conversions_composer.compose_aspect(aspect, &self.context.borrow())
    }
    pub(crate) fn make_expansion(&self) -> Expansion {
        let source = self.context.borrow();
        let ffi_name = self.ffi_name_composer.compose(&());

        // let ffi_ident: Ident = parse_quote!(#ffi_name);
        let target_name = self.target_name_composer.compose(&());
        let comment = DocPresentation::Direct(self.doc_composer.compose(&()));
        let ffi_presentation = FFIObjectPresentation::Full(self.ffi_object_composer.compose(&()).present(&source));
        let conversion = ConversionInterfacePresentation::Interface {
            ffi_type: ffi_name.clone(),
            target_type: target_name.clone(),
            from_presentation: FromConversionPresentation::Struct(self.compose_aspect(FFIAspect::From)),
            to_presentation: ToConversionPresentation::Struct(self.compose_aspect(FFIAspect::To)),
            destroy_presentation: self.compose_aspect(FFIAspect::Destroy),
            generics: None
        };
        let mut bindings = vec![self.ctor_composer.compose(&()).present(&source)];
        bindings.push(self.dtor_composer.compose(&source));
        bindings.extend(self.getter_composer.compose(&source));
        bindings.extend(self.setter_composer.compose(&source));
        let traits = self.attrs_composer.compose(&source);
        let drop = DropInterfacePresentation::Full {
            name: ffi_name,
            body: self.compose_aspect(FFIAspect::Drop)
        };
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

