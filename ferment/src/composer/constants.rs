use std::clone::Clone;
use syn::token::Comma;
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Path};
use crate::composer::{BindingDtorComposer, Composer, ComposerPresenter, ComposerPresenterByRef, ConstructorPresentableContext, ContextComposer, ConversionComposer, CtorOwnedComposer, Depunctuated, DestructorContext, DropConversionComposer, EnumComposerPresenterRef, EnumParentComposer, FFIComposer, FFIConversionComposer, FieldsOwnedComposer, FieldTypeComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemComposerFieldTypesContextPresenter, ItemComposerLocalConversionContextPresenter, ItemComposerPresenterRef, ItemParentComposer, LocalConversionContext, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerIteratorLocalContext, OwnerIteratorPostProcessingComposer, SharedComposer, r#type};
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, Mangle, Pop};
use crate::interface::{DEFAULT_DOC_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, ROOT_DESTROY_CONTEXT_COMPOSER};
use crate::naming::Name;
use crate::presentation::BindingPresentation;
use crate::presentation::context::{FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::binding::BindingPresentableContext;

pub const DEFAULT_DOC_COMPOSER: ContextComposer<Name, TokenStream2, ItemParentComposer> =
    ContextComposer::new(DEFAULT_DOC_PRESENTER, TARGET_NAME_PRESENTER);
pub const FIELDS_FROM_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer| composer.fields_from_composer.compose(&());
pub const FIELDS_TO_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer| composer.fields_to_composer.compose(&());
pub const TARGET_NAME_PRESENTER: ItemComposerPresenterRef<Name> =
    |composer|
        composer.type_composer.compose_aspect(r#type::FFIAspect::Target, &composer.context.borrow());

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi));
pub const EMPTY_CONTEXT_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
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
    |composer| (composer.type_composer.compose_aspect(r#type::FFIAspect::FFI, &composer.context.borrow()), composer.field_types.clone());
pub const FFI_NAME_DTOR_COMPOSER: ItemComposerPresenterRef<DestructorContext> =
    |composer| {
        let ffi_name = composer.type_composer.compose_aspect(r#type::FFIAspect::FFI, &composer.context.borrow());
        (ffi_name.clone(), ffi_name)
    };

pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter =
    |composer| composer.field_types.clone();

pub const BYPASS_FIELD_CONTEXT: FieldTypePresentationContextPassRef =
    |(_, context)| context.clone();


/// Bindings
pub const BINDING_DTOR_COMPOSER: BindingDtorComposer =
    |(ffi_ident, ffi_name)|
        BindingPresentation::Destructor {
            name: Name::Destructor(Box::new(ffi_ident)),
            ffi_name: ffi_name.to_token_stream()
        };
const fn owner_iterator_lambda_composer() -> ComposerPresenter<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> {
    |(field_path_context, context)|
        OwnerIteratorPresentationContext::Lambda(field_path_context.into(), context.into())
}

const fn iterator_lambda_composer() -> ComposerPresenter<(OwnerIteratorPresentationContext, IteratorPresentationContext), IteratorPresentationContext> {
    |(field_path_context, context)|
        IteratorPresentationContext::Lambda(field_path_context.into(), context.into())
}
pub const fn fields_composer(
    root_composer: ComposerPresenter<OwnerIteratorLocalContext<Comma>, OwnerIteratorPresentationContext>,
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
        |((constructor_context, fields), presenter)|
            (constructor_context, fields.iter().map(presenter).collect()),
        iterator_item_composer
    )
}


pub const fn default_ctor_context_composer() -> SharedComposer<ItemParentComposer, (ConstructorPresentableContext, FieldTypesContext)> {
    |composer| {
        let ffi_name = composer.type_composer.compose_aspect(r#type::FFIAspect::FFI, &composer.context.borrow());
        (ConstructorPresentableContext::Default(Name::Constructor(Box::new(ffi_name.clone())), ffi_name), FIELD_TYPES_COMPOSER(composer))
    }
}

/// Type Alias Composers
pub const fn type_alias_composer_ffi_conversions() -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        type_alias_composer_from(),
        type_alias_composer_to(),
        struct_composer_destroy(),
        type_alias_composer_drop())
}
pub const fn type_alias_composer_from()
    -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        FFI_FROM_ROOT_PRESENTER,
        FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
        |(_, fields)|
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields.into_iter().collect::<Depunctuated<OwnedItemPresentableContext>>()),
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

pub const fn type_alias_composer_to() -> FFIConversionComposer<ItemParentComposer>  {
    ConversionComposer::new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Obj,
        |local_context|
            OwnerIteratorPresentationContext::TypeAliasToConversion((local_context.0, local_context.1.into_iter().collect())),
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        |(_, conversion)|
            conversion.clone(),
        |(context, presenter)|
            (context.0, context.1.iter().map(|field_type| {
                let conversion_context = (quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}

pub const fn type_alias_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        |_| OwnerIteratorPresentationContext::Empty,
        |fields| IteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
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
pub const fn type_alias_composer_root_presenter() -> ComposerPresenter<OwnerIteratorLocalContext<Comma>, OwnerIteratorPresentationContext> {
    |local_context| OwnerIteratorPresentationContext::TypeAlias(local_context)
}

pub const fn type_alias_composer_field_presenter() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.ty().clone())
}


/// Struct Composers
pub const fn struct_composer_ffi_conversions(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
    conversion_presenter: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        struct_composer_from(root_conversion_presenter, conversion_presenter),
        struct_composer_to(root_conversion_presenter, conversion_presenter),
        struct_composer_destroy(),
        struct_composer_drop(),
    )
}
pub const fn struct_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
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
pub const fn struct_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
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
pub const fn struct_composer_destroy() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, EMPTY_CONTEXT_PRESENTER)
}
pub const fn struct_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion,
        |_| OwnerIteratorPresentationContext::Empty,
        |fields| IteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
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
pub const fn struct_composer_ctor_named() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Curly(names))
        },
        struct_composer_ctor_named_item())
}
pub const fn struct_composer_ctor_unnamed() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Round(names))
        },
        struct_composer_ctor_unnamed_item())
}
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
pub const fn struct_composer_object() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(|name| name, FIELDS_FROM_PRESENTER)
}


/// Enum Variant Composers
pub const fn enum_variant_composer_ffi_conversions(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
    conversion_presenter: FieldTypePresentationContextPassRef,
    destroy_code_context_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
    destroy_presenter: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        enum_variant_composer_from(root_conversion_presenter, conversion_presenter),
        enum_variant_composer_to(root_conversion_presenter, conversion_presenter),
        enum_variant_composer_destroy(destroy_code_context_presenter),
        enum_variant_composer_drop(destroy_presenter)
    )
}
const fn enum_variant_composer_from_local_context_iterator_root_composer<T: Default + ToTokens>() -> ComposerPresenter<
    (LocalConversionContext, ComposerPresenterByRef<(TokenStream2, FieldTypePresentableContext), FieldTypePresentableContext>),
    OwnerIteratorLocalContext<T>> {
    |((name, fields), presenter)|
        (name, fields.iter().map(|field_type| {
            let conversion_context = (field_type.name(), DEREF_FIELD_PATH_FROM_PRESENTER(field_type));
            OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
        }).collect())
}
const fn enum_variant_composer_to_local_context_iterator_root_composer<T: Default + ToTokens>() -> ComposerPresenter<(LocalConversionContext, ComposerPresenterByRef<(TokenStream2, FieldTypePresentableContext), FieldTypePresentableContext>), OwnerIteratorLocalContext<T>> {
    |((name, fields), presenter)|
        (name, fields.iter().map(|field_type| {
            let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER(field_type));
            OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
        }).collect())
}
const fn enum_variant_composer_drop_local_context_iterator_root_composer<SEP: Default + ToTokens>() -> ComposerPresenter<(FieldTypesContext, ComposerPresenterByRef<(TokenStream2, FieldTypePresentableContext), FieldTypePresentableContext>), Punctuated<OwnedItemPresentableContext, SEP>> {
    |(fields, presenter)|
        fields.iter()
            .map(|field_type| {
                let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            })
            .collect()
}
pub const fn enum_variant_composer_from(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        owner_iterator_lambda_composer(),
        FIELDS_FROM_PRESENTER,
        root_conversion_presenter,
        TARGET_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        enum_variant_composer_from_local_context_iterator_root_composer())
}
pub const fn enum_variant_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        owner_iterator_lambda_composer(),
        FIELDS_TO_PRESENTER,
        root_conversion_presenter,
        FFI_NAME_LOCAL_CONVERSION_COMPOSER,
        conversion_presenter,
        enum_variant_composer_to_local_context_iterator_root_composer())
}
pub const fn enum_variant_composer_destroy(
    root_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
) -> ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, ItemParentComposer> {
    ContextComposer::new(root_presenter, FIELDS_FROM_PRESENTER)
}
pub const fn enum_variant_composer_drop(
    conversion_presenter: FieldTypePresentationContextPassRef,
) -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        iterator_lambda_composer(),
        FIELDS_FROM_PRESENTER,
        |fields| IteratorPresentationContext::DropCode(fields),
        FIELD_TYPES_COMPOSER,
        conversion_presenter,
        enum_variant_composer_drop_local_context_iterator_root_composer())
}
pub const fn enum_variant_composer_object() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(|_| OwnerIteratorPresentationContext::Empty, EMPTY_CONTEXT_PRESENTER)
}

pub const fn enum_variant_composer_field_presenter() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultField(field_type.clone())
}
pub const fn enum_variant_ctor_context_composer() -> SharedComposer<ItemParentComposer, (ConstructorPresentableContext, FieldTypesContext)> {
    |composer| {
        let ffi = composer.type_composer.compose_aspect(r#type::FFIAspect::FFI, &composer.context.borrow());
        let tt = ffi.to_token_stream();
        let ffi_path: Path = parse_quote!(#tt);
        println!("enum_variant_ctor_context_composer::: {} \n\t\t ---- {} \n\t\t ---- {}", ffi, ffi_path.popped().to_token_stream(), ffi_path.to_mangled_ident_default());
        (ConstructorPresentableContext::EnumVariant(
            Name::Constructor(Box::new(ffi.clone())),
            ffi_path.popped().to_token_stream(),
            ffi_path.to_token_stream()
        ), FIELD_TYPES_COMPOSER(composer))
    }
}
pub const fn enum_variant_composer_ctor_unit() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, _): (Punctuated<OwnedItemPresentableContext, Comma>, Punctuated<OwnedItemPresentableContext, Comma>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Empty)
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_unnamed() -> CtorOwnedComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
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
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, if names.is_empty() {
                IteratorPresentationContext::Empty
            } else {
                IteratorPresentationContext::Curly(names)
            })
        },
        struct_composer_ctor_named_item()
    )
}


/// Enum composers
pub const VARIANTS_FROM_PRESENTER: EnumComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer|
        OwnerIteratorPresentationContext::Variants((
            composer.type_composer.compose_aspect(r#type::FFIAspect::Target, &composer.context.borrow()),
            composer.variant_presenters.iter()
                .map(|(presenter, variant_name, fields_context)|
                    presenter((variant_name.clone(), fields_context.clone())))
                .collect()));



pub const fn enum_composer_object() -> OwnerIteratorPostProcessingComposer<EnumParentComposer> {
    ContextComposer::new(|context| OwnerIteratorPresentationContext::Enum(Box::new(context)), VARIANTS_FROM_PRESENTER)
}


