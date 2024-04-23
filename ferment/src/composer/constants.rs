use std::cell::Ref;
use std::clone::Clone;
use syn::token::Comma;
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::{Field, parse_quote, Type};
use crate::composer::{Composer, BindingDtorComposer, ComposerPresenter, ComposerPresenterByRef, ConstructorPresentableContext, ContextComposer, SequenceMixer, ConstructorComposer, Depunctuated, DropSequenceMixer, EnumComposerPresenterRef, EnumParentComposer, FFIComposer, FFIConversionMixer, FieldsOwnedComposer, FieldTypeComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemComposerFieldTypesContextPresenter, ItemComposerPresenterRef, ItemParentComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectIteratorLocalContext, OwnerIteratorPostProcessingComposer, SharedComposer, ItemComposer, EnumComposer, LocalConversionContext, VariantIteratorLocalContext, ConstructorFieldsContext, OwnedItemPresentablePair, SequenceComposer, ParentComposer, OwnedItemPresentationPair};
use crate::composer::composable::{SourceExpandable, NameContext, BasicComposable};
use crate::conversion::FieldTypeConversion;
use crate::ext::Conversion;
use crate::formatter::format_token_stream;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::Aspect;

pub const FFI_FROM_ROOT_PRESENTER: ComposerPresenterByRef<OwnedItemPresentationPair, OwnerIteratorPresentationContext> = |(field_path, conversions)|
    OwnerIteratorPresentationContext::FromRoot(Box::new(field_path.clone()), Box::new(conversions.clone()));
pub const FFI_TO_ROOT_PRESENTER: ComposerPresenterByRef<OwnedItemPresentationPair, OwnerIteratorPresentationContext> = |(_, conversions)|
    OwnerIteratorPresentationContext::Boxed(conversions.clone().into());
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    OwnerIteratorPresentationContext::CurlyBracesFields(local_context);
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    OwnerIteratorPresentationContext::RoundBracesFields(local_context.clone());
pub const ROOT_DESTROY_CONTEXT_COMPOSER: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::UnboxedRoot;
pub const DEFAULT_DOC_PRESENTER: ComposerPresenter<Type, TokenStream2> = |target_name| {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(&target_name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }
};

pub const FIELDS_FROM_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer| composer.fields_from_composer.compose(&());

pub const EMPTY_CONTEXT_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
    |_| OwnerIteratorPresentationContext::Empty;


/// FieldTypeComposers
pub const FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_from(FieldTypePresentableContext::FfiRefWithFieldName(FieldTypePresentableContext::Simple(field_type.name()).into()));
pub const DEREF_FIELD_PATH_FROM_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_from(FieldTypePresentableContext::Deref(field_type.name()));
pub const TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_to(FieldTypePresentableContext::Obj);
pub const STRUCT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_to(FieldTypePresentableContext::ObjFieldName(field_type.name()));
pub const ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_to(FieldTypePresentableContext::FieldTypeConversionName(field_type.clone()));
pub const ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_destroy(FieldTypePresentableContext::Deref(field_type.name()));
pub const STRUCT_FIELD_TYPE_DESTROY_PRESENTER: FieldTypeComposer =
    |field_type| field_type.conversion_destroy(FieldTypePresentableContext::FfiRefWithConversion(field_type.clone()));

pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter =
    |composer| composer.field_types.clone();

pub const BYPASS_FIELD_CONTEXT: FieldTypePresentationContextPassRef =
    |(_, context)| context.clone();

pub const FFI_ASPECT_SEQ_CONTEXT: SharedComposer<ItemParentComposer, LocalConversionContext> =
    |composer: &Ref<ItemComposer>| (composer.base.ffi_name_aspect(), FIELD_TYPES_COMPOSER(composer));

pub const TARGET_ASPECT_SEQ_CONTEXT: SharedComposer<ItemParentComposer, LocalConversionContext> =
    |composer: &Ref<ItemComposer>| (composer.base.target_name_aspect(), FIELD_TYPES_COMPOSER(composer));

// pub const fn field_type_composer(
//     post_processor: FieldTypePresentationContextPassRef,
//     field_type_presenter: FieldTypeComposer
// ) -> OwnedFieldTypeComposerRef {
//     |field_type|
//         OwnedItemPresentableContext::FieldType(post_processor(&(field_type.name(), field_type_presenter(field_type))))
// }

/// Bindings
pub const BINDING_DTOR_COMPOSER: BindingDtorComposer =
    |context|
        BindingPresentation::Destructor {
            ffi_name: context.to_token_stream(),
            name: Name::Destructor(context)
        };

const fn owner_iterator_lambda_composer() -> ComposerPresenterByRef<OwnedItemPresentationPair, OwnerIteratorPresentationContext> {
    |(left, right)|
        OwnerIteratorPresentationContext::Lambda(Box::new(left.clone()), right.clone().into())
}
pub const fn fields_composer(
    root: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
    context: SharedComposer<ItemParentComposer, LocalConversionContext>,
    iterator_item: OwnedFieldTypeComposerRef,
) -> FieldsOwnedComposer<ItemParentComposer> {
    FieldsOwnedComposer::new(
        root,
        context,
        |((aspect, field_types), presenter)|
            (aspect, field_types.iter().map(presenter).collect()),
        iterator_item)
}

pub const fn composer_ctor(
    context: SharedComposer<ItemParentComposer, ConstructorFieldsContext>,
    root: ComposerPresenter<
        (ConstructorPresentableContext, Vec<OwnedItemPresentablePair>),
        BindingPresentableContext>,
    iterator_item: ComposerPresenterByRef<
        FieldTypeConversion,
        OwnedItemPresentablePair>
) -> ConstructorComposer<ItemParentComposer> {
    ConstructorComposer::new(
        root,
        context,
        |((constructor_context, fields), presenter)|
            (constructor_context, fields.iter().map(presenter).collect()),
        iterator_item
    )
}


pub const fn default_ctor_context_composer() -> SharedComposer<ItemParentComposer, ConstructorFieldsContext> {
    move |composer| (
        ConstructorPresentableContext::Default(
            composer.base
                .ffi_name_aspect()
                .present(&composer.source_ref())),
        FIELD_TYPES_COMPOSER(composer))
}

/// Type Alias Composers
pub const fn type_alias_composer_ffi_conversions() -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        type_alias_composer_from(),
        type_alias_composer_to(),
        struct_destroy_composer(),
        struct_drop_sequence_mixer())
}
pub const fn type_alias_composer_from() -> FFIConversionMixer<ItemParentComposer> {
    SequenceMixer::new_new(
        FFI_FROM_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi)),
        SequenceComposer::new(
            |(_, fields)|
                OwnerIteratorPresentationContext::TypeAliasFromConversion(Depunctuated::from_iter(fields.into_iter())),
            TARGET_ASPECT_SEQ_CONTEXT,
            |((aspect, field_types), presenter)| {
                (aspect, field_types.iter().map(|field_type|
                    OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type))))).collect())
            },
            BYPASS_FIELD_CONTEXT
        )
    )
}

// pub const fn type_alias_to_seq_composer(
//     context: SharedComposer<ItemParentComposer, LocalConversionContext>
// ) -> SequenceComposer<ItemParentComposer, LocalConversionContext, FieldTypeLocalContext, FieldTypePresentableContext, VariantIteratorLocalContext, OwnerIteratorPresentationContext> {
//     SequenceComposer::new(
//         |(aspect, context)|
//             OwnerIteratorPresentationContext::TypeAliasToConversion((aspect, context)),
//         context,
//         |((aspect, field_types), presenter)|
//             (aspect, field_types.iter().map(|field_type|
//                 OwnedItemPresentableContext::FieldType(presenter(&(quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type))))).collect()),
//         BYPASS_FIELD_CONTEXT
//     )
// }

pub const fn type_alias_composer_to() -> FFIConversionMixer<ItemParentComposer>  {
    SequenceMixer::new_new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Obj,
        SequenceComposer::new(
            |(aspect, context)|
                OwnerIteratorPresentationContext::TypeAliasToConversion((aspect, context)),
            FFI_ASPECT_SEQ_CONTEXT,
            |((aspect, field_types), presenter)|
                (aspect, field_types.iter().map(|field_type|
                    OwnedItemPresentableContext::FieldType(presenter(&(quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type))))).collect()),
            BYPASS_FIELD_CONTEXT
        )
    )
}

pub const fn type_alias_composer_root_presenter() -> ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext> {
    |local_context| OwnerIteratorPresentationContext::TypeAlias(local_context)
}

/// Struct Composers
pub const fn struct_ffi_composer(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        struct_from_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_to_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_destroy_composer(),
        struct_drop_sequence_mixer(),
    )
}
pub const fn struct_from_ffi_conversion_mixer(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer> {
    SequenceMixer::new_new(
        FFI_FROM_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi)),
        SequenceComposer::new(
            seq_root,
        TARGET_ASPECT_SEQ_CONTEXT,
        |((aspect, fields), presenter)|
            (aspect, fields.iter().map(|field_type|
                OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), FIELD_PATH_FROM_PRESENTER(field_type)))))
                .collect()),
            seq_iterator_item
        ))
}
pub const fn struct_to_ffi_conversion_mixer(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer> {
    SequenceMixer::new_new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Empty,
        SequenceComposer::new(
            seq_root,
            FFI_ASPECT_SEQ_CONTEXT,
            |((name, fields), presenter)|
                (name.clone(), fields.iter().map(|field_type|
                    OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), STRUCT_FIELD_TYPE_TO_PRESENTER(field_type)))))
                    .collect()),
            seq_iterator_item
        ))

}
pub const fn struct_destroy_composer() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, EMPTY_CONTEXT_PRESENTER)
}
pub const fn struct_drop_sequence_mixer() -> DropSequenceMixer<ItemParentComposer> {
    SequenceMixer::new(
        |(_, conversion)| conversion.clone(),
        |_| OwnerIteratorPresentationContext::Empty,
        |fields|
            OwnerIteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
        FIELD_TYPES_COMPOSER,
        BYPASS_FIELD_CONTEXT,
        |(fields, presenter)|
            fields.iter()
                .map(|field_type|
                    OwnedItemPresentableContext::FieldType(presenter(&(quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type)))))
                .collect())
}
pub const fn struct_composer_ctor_named() -> ConstructorComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Curly(names))
        },
        struct_composer_ctor_named_item())
}
pub const fn struct_composer_ctor_unnamed() -> ConstructorComposer<ItemParentComposer> {
    composer_ctor(
        default_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Round(names))
        },
        struct_composer_ctor_unnamed_item())
}
const fn struct_composer_ctor_unnamed_item() -> ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair> {
    |field_type| (
        OwnedItemPresentableContext::BindingArg(field_type.clone()),
        OwnedItemPresentableContext::BindingFieldName(field_type.clone())
    )
}
const fn struct_composer_ctor_named_item() -> ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair> {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultField(field_type.clone())
    )
}
pub const fn struct_composer_object() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(|name| name, FIELDS_FROM_PRESENTER)
}
pub const fn struct_composer_conversion_named() -> FieldTypePresentationContextPassRef {
    |(field_path, field_context)|
        FieldTypePresentableContext::Named((field_path.clone(), Box::new(field_context.clone())))
}

pub const fn struct_composer_root_presenter_unnamed() -> OwnerIteratorConversionComposer<Comma> {
    |local_context|
        OwnerIteratorPresentationContext::UnnamedStruct(local_context)
}

pub const fn struct_composer_root_presenter_named() -> OwnerIteratorConversionComposer<Comma> {
    |local_context|
        OwnerIteratorPresentationContext::NamedStruct(local_context)
}
pub const fn struct_composer_field_presenter_unnamed() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.ty().clone())
}

pub const fn struct_composer_field_presenter_named() -> OwnedFieldTypeComposerRef {
    |field_type|
        OwnedItemPresentableContext::Named(field_type.clone(), true)}


/// Enum Variant Composers
pub const fn enum_variant_composer_ffi_composer(
    conversion_mixer_seq_root: OwnerIteratorConversionComposer<Comma>,
    conversion_seq_iterator_item: FieldTypePresentationContextPassRef,
    destroy_context_root: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
    destroy_seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer> {
    FFIComposer::new(
        enum_variant_from_ffi_conversion_mixer(conversion_mixer_seq_root, conversion_seq_iterator_item),
        enum_variant_to_ffi_conversion_mixer(conversion_mixer_seq_root, conversion_seq_iterator_item),
        enum_variant_destroy_composer(destroy_context_root),
        enum_variant_drop_sequence_mixer(destroy_seq_iterator_item)
    )
}
const fn enum_variant_composer_from_sequence_iterator_root<T: Default + ToTokens>() -> ComposerPresenter<
    (LocalConversionContext, FieldTypePresentationContextPassRef),
    OwnerAspectIteratorLocalContext<T>> {
    |((name, fields), presenter)|
        (name.clone(), fields.iter().map(|field_type|
            OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), DEREF_FIELD_PATH_FROM_PRESENTER(field_type))))
        ).collect())
}
const fn enum_variant_composer_to_sequence_iterator_root<T: Default + ToTokens>() -> ComposerPresenter<(LocalConversionContext, FieldTypePresentationContextPassRef), OwnerAspectIteratorLocalContext<T>> {
    |((name, fields), presenter)| {
        (name.clone(), fields.iter().map(|field_type|
            OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER(field_type))))).collect())
    }
}
const fn enum_variant_composer_drop_sequence_iterator_root<'a, SEP: Default + ToTokens>() -> ComposerPresenter<(FieldTypesContext, FieldTypePresentationContextPassRef), Punctuated<OwnedItemPresentableContext, SEP>> {
    |(fields, presenter)|
        fields.iter()
            .map(|field_type|
                OwnedItemPresentableContext::FieldType(presenter(&(field_type.name(), ENUM_VARIANT_FIELD_TYPE_DESTROY_PRESENTER(field_type))))
            )
            .collect()
}
pub const fn enum_variant_from_ffi_conversion_mixer(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer> {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        FIELDS_FROM_PRESENTER,
        seq_root,
        |composer: &Ref<ItemComposer>| (Aspect::RawTarget(composer.base.name_context()), composer.field_types.clone()),
        seq_iterator_item,
        enum_variant_composer_from_sequence_iterator_root())
}
pub const fn enum_variant_to_ffi_conversion_mixer(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer> {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        |composer: &Ref<ItemComposer>| composer.fields_to_composer.compose(&()),
        seq_root,
        FFI_ASPECT_SEQ_CONTEXT,
        seq_iterator_item,
        enum_variant_composer_to_sequence_iterator_root())
}
pub const fn enum_variant_destroy_composer(
    root: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
) -> ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, ItemParentComposer> {
    ContextComposer::new(root, FIELDS_FROM_PRESENTER)
}
pub const fn enum_variant_drop_sequence_mixer(
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> DropSequenceMixer<ItemParentComposer> {
    SequenceMixer::new(
        |(field_path_context, context)|
            OwnerIteratorPresentationContext::Lambda(Box::new(field_path_context.clone()), Box::new(context.clone())),
        FIELDS_FROM_PRESENTER,
        |fields|
            OwnerIteratorPresentationContext::DropCode(fields),
        FIELD_TYPES_COMPOSER,
        seq_iterator_item,
        enum_variant_composer_drop_sequence_iterator_root())
}
pub const fn enum_variant_composer_object() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(|_| OwnerIteratorPresentationContext::Empty, EMPTY_CONTEXT_PRESENTER)
}

pub const fn enum_variant_composer_field_presenter() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultField(field_type.clone())
}
pub const fn enum_variant_ctor_context_composer() -> SharedComposer<ItemParentComposer, ConstructorFieldsContext> {
    |composer|
        (ConstructorPresentableContext::EnumVariant(Aspect::FFI(composer.base.name_context()).present(&composer.source_ref())), FIELD_TYPES_COMPOSER(composer))

}
pub const fn enum_variant_composer_ctor_unit() -> ConstructorComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, _): (Punctuated<OwnedItemPresentableContext, Comma>, Punctuated<OwnedItemPresentableContext, Comma>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(context, args, IteratorPresentationContext::Empty)
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_unnamed() -> ConstructorComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Round(names))
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_named() -> ConstructorComposer<ItemParentComposer> {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        |(context, field_pairs)| {
            let (args, names): (Punctuated<OwnedItemPresentableContext, _>, Punctuated<OwnedItemPresentableContext, _>) = field_pairs.into_iter().unzip();
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Curly(names)
            )
        },
        struct_composer_ctor_named_item()
    )
}

pub const fn enum_variant_composer_conversion_unit() -> OwnerIteratorConversionComposer<Comma> {
    |(aspect, _)| {
        OwnerIteratorPresentationContext::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            Aspect::FFI(_) => aspect.clone(),
            Aspect::RawTarget(_) => aspect.clone(),
            // Aspect::RawFFI(_) => aspect.clone(),
        })
    }
}

pub const fn composer_doc_default<T: 'static + BasicComposable<ParentComposer<T>>>() -> ContextComposer<Type, TokenStream2, ParentComposer<T>> {
    ContextComposer::new(
        DEFAULT_DOC_PRESENTER,
        |composer: &Ref<T>|
            composer.target_name_aspect()
                .present(&composer.source_ref()))
}

pub const fn item_composer_doc() -> ContextComposer<Type, TokenStream2, ItemParentComposer> {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<ItemComposer>| Aspect::Target(composer.base.name_context()).present(&composer.source_ref()))
}


pub const ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    FieldTypesContext> = |fields|
    fields.iter()
        .enumerate()
        .map(|(index, Field { ty, .. })|
            FieldTypeConversion::Unnamed(Name::UnnamedArg(index), ty.clone()))
        .collect();

pub const STRUCT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    FieldTypesContext> = |fields|
    fields
        .iter()
        .enumerate()
        .map(|(index, Field { ty, .. })|
            FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty.clone()))
        .collect();

pub const STRUCT_NAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    FieldTypesContext> = |fields|
    fields
        .iter()
        .map(|Field { ident, ty, .. }| {
            FieldTypeConversion::Named(Name::Optional(ident.clone()), ty.clone())
        })
        .collect();
pub const EMPTY_FIELDS_COMPOSER: ComposerPresenterByRef<Punctuated<Field, Comma>,
    FieldTypesContext> = |_| Punctuated::new();

/// Enum composers
pub const VARIANTS_FROM_PRESENTER: EnumComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer| {
        let source = composer.source_ref();
        OwnerIteratorPresentationContext::Variants((
            composer.base.target_name_aspect().present(&source),
            composer.variant_presenters.iter()
                .map(|(variant_composer, variant_context)|
                    variant_composer(variant_context))
                .collect()))
    };
pub const fn enum_composer_object() -> OwnerIteratorPostProcessingComposer<EnumParentComposer> {
    ContextComposer::new(
        |context| OwnerIteratorPresentationContext::Enum(Box::new(context)),
        VARIANTS_FROM_PRESENTER)
}
pub const fn enum_composer_doc() -> ContextComposer<Type, TokenStream2, EnumParentComposer> {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<EnumComposer>|
        composer.base.target_name_aspect().present(&composer.source_ref()))
}
