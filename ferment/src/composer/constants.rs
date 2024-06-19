use std::cell::Ref;
use std::clone::Clone;
use syn::token::Comma;
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::{Field, parse_quote, Type};
use crate::composer::{Composer, BindingDtorComposer, ComposerPresenter, ComposerPresenterByRef, ConstructorPresentableContext, ContextComposer, SequenceMixer, ConstructorComposer, Depunctuated, DropSequenceMixer, EnumComposerPresenterRef, EnumParentComposer, FFIComposer, FFIConversionMixer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemComposerFieldTypesContextPresenter, ItemComposerPresenterRef, ItemParentComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectIteratorLocalContext, OwnerIteratorPostProcessingComposer, SharedComposer, ItemComposer, EnumComposer, LocalConversionContext, VariantIteratorLocalContext, ConstructorFieldsContext, OwnedItemPresentablePair, SequenceComposer, ParentComposer, OwnedItemPresentationPair, CommaPunctuated, OpaqueItemParentComposer, CommaPunctuatedTokens, CommaPunctuatedOwnedItems, CommaPunctuatedFields, FunctionContext};
use crate::composer::composable::{NameContext, BasicComposable, SourceAccessible};
use crate::composer::opaque_item::OpaqueItemComposer;
use crate::composition::CfgAttributes;
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind, GenericTypeConversion, TypeConversion};
use crate::ext::Conversion;
use crate::formatter::format_token_stream;
use crate::naming::{DictionaryName, Name};
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{FieldContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::Aspect;
use crate::shared::SharedAccess;
use crate::wrapped::{DelimiterTrait, Wrapped};

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

pub const fn fields_from_presenter<'a, I>() -> ItemComposerPresenterRef<'a, OwnerIteratorPresentationContext, I>
    where I: DelimiterTrait + ?Sized {
    |composer: &Ref<ItemComposer<I>>| composer.fields_from_composer.compose(&())
}
pub const fn empty_context_presenter<'a, I>() -> ItemComposerPresenterRef<'a, OwnerIteratorPresentationContext, I>
    where I: DelimiterTrait + ?Sized {
    |_| OwnerIteratorPresentationContext::Empty
}

// pub const FIELDS_FROM_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext, dyn ScopeContextPresentable, dyn DelimiterTrait> =
//     |composer| composer.fields_from_composer.compose(&());
//
// pub const EMPTY_CONTEXT_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext, dyn DelimiterTrait> =
//     |_| OwnerIteratorPresentationContext::Empty;

pub const fn field_types_composer<'a, I>() -> ItemComposerFieldTypesContextPresenter<'a, I>
    where I: DelimiterTrait + ?Sized {
    |composer| composer.field_types.clone()
}

/// FieldTypeComposers
// pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter<dyn DelimiterTrait> =
//     |composer| composer.field_types.clone();
//

pub const fn bypass_field_context() -> FieldTypePresentationContextPassRef {
    |(_, context)| context.clone()
}
// pub const BYPASS_FIELD_CONTEXT: FieldTypePresentationContextPassRef =
//     |(_, context)| context.clone();
//
// pub const FFI_ASPECT_SEQ_CONTEXT: SharedComposer<ItemParentComposer<dyn DelimiterTrait>, LocalConversionContext> =
//     |composer| ((composer.base.ffi_name_aspect(), composer.field_types.clone()), composer.base.generics.clone());
//
// pub const TARGET_ASPECT_SEQ_CONTEXT: SharedComposer<ItemParentComposer<dyn DelimiterTrait>, LocalConversionContext> =
//     |composer| ((composer.base.target_name_aspect(), composer.field_types.clone()), composer.base.generics.clone());

pub const fn ffi_aspect_seq_context<I>() -> SharedComposer<ItemParentComposer<I>, LocalConversionContext>
    where I: DelimiterTrait + ?Sized {
    |composer: &Ref<ItemComposer<I>>| ((composer.base.ffi_name_aspect(), composer.field_types.clone()), composer.base.generics.compose(composer.context()))
}
pub const fn target_aspect_seq_context<I>() -> SharedComposer<ItemParentComposer<I>, LocalConversionContext>
    where I: DelimiterTrait + ?Sized {
    |composer: &Ref<ItemComposer<I>>| ((composer.base.target_name_aspect(), composer.field_types.clone()), composer.base.generics.compose(composer.context()))
}


/// Bindings
pub const BINDING_DTOR_COMPOSER: BindingDtorComposer =
    |context|
        BindingPresentation::Destructor {
            attrs: context.1.to_token_stream(),
            ffi_name: context.0.clone(),
            name: Name::Destructor(context.0),
            generics: context.2
        };

const fn owner_iterator_lambda_composer() -> ComposerPresenterByRef<OwnedItemPresentationPair, OwnerIteratorPresentationContext> {
    |(left, right)|
        OwnerIteratorPresentationContext::Lambda(Box::new(left.clone()), right.clone().into())
}
pub const fn fields_composer<Parent: SharedAccess>(
    root: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
    context: SharedComposer<Parent, LocalConversionContext>,
    iterator_item: OwnedFieldTypeComposerRef,
) -> FieldsOwnedComposer<Parent> {
    FieldsOwnedComposer::new(
        root,
        context,
        |(((aspect, field_types), _generics), presenter)|
            (aspect, field_types.iter().map(presenter).collect()),
        iterator_item)
}
// pub const fn fields_composer(
//     root: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
//     context: SharedComposer<ItemParentComposer, LocalConversionContext>,
//     iterator_item: OwnedFieldTypeComposerRef,
// ) -> FieldsOwnedComposer<ItemParentComposer> {
//     FieldsOwnedComposer::new(
//         root,
//         context,
//         |((aspect, field_types), presenter)|
//             (aspect, field_types.iter().map(presenter).collect()),
//         iterator_item)
// }

pub const fn composer_ctor<S, SP, I>(
    context: SharedComposer<ItemParentComposer<I>, ConstructorFieldsContext>,
    root: ComposerPresenter<FunctionContext, BindingPresentableContext<S, SP, I>>,
    iterator_item: ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair>
) -> ConstructorComposer<ItemParentComposer<I>, S, SP, I>
    where S: ScopeContextPresentable<Presentation = SP>,
          SP: ToTokens,
          I: DelimiterTrait + ?Sized  {
    ConstructorComposer::new(
        root,
        context,
        |((constructor_context, fields), presenter)|
            (constructor_context, fields.iter().map(presenter).collect()),
        iterator_item
    )
}


pub const fn default_ctor_context_composer<I>() -> SharedComposer<ItemParentComposer<I>, ConstructorFieldsContext>
    where I: DelimiterTrait + ?Sized {
    move |composer| (
        ConstructorPresentableContext::Default(
            composer.base
                .ffi_name_aspect()
                .present(&composer.source_ref()),
            composer.compose_attributes().to_token_stream(),
            composer.base.generics.compose(composer.context())),
        composer.field_types.clone())
}
pub const fn default_opaque_ctor_context_composer<I>() -> SharedComposer<OpaqueItemParentComposer<I>, ConstructorFieldsContext>
    where I: DelimiterTrait + ?Sized {
    move |composer| (
        ConstructorPresentableContext::Default(
            composer.base.target_name_aspect().present(&composer.source_ref()),
            composer.compose_attributes().to_token_stream(),
            composer.base.generics.compose(composer.context())),
        composer.field_types.clone())
}

/// Type Alias Composers
pub const fn type_alias_composer_ffi_conversions<I>() -> FFIComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    FFIComposer::new(
        type_alias_composer_from(),
        type_alias_composer_to(),
        struct_destroy_composer(),
        struct_drop_sequence_mixer())
}
pub const fn type_alias_composer_from<I>() -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new_new(
        FFI_FROM_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::AddrDeref(DictionaryName::Ffi.to_token_stream()),
        SequenceComposer::new(
            |(_, fields)|
                OwnerIteratorPresentationContext::TypeAliasFromConversion(Depunctuated::from_iter(fields.into_iter())),
            target_aspect_seq_context(),
            |(((aspect, field_types), _generics), presenter)| {
                (aspect, field_types.iter()
                    .map(|field_type|
                        OwnedItemPresentableContext::FieldType(
                            presenter(&(
                                field_type.name(),
                                field_type.conversion_from(FieldContext::FfiRefWithFieldName(FieldContext::Simple(field_type.name()).into())))),
                            field_type.attrs()))
                    .collect())
            },
            bypass_field_context()
        )
    )
}

pub const fn type_alias_composer_to<I>() -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new_new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Obj,
        SequenceComposer::new(
            |(aspect, context)|
                OwnerIteratorPresentationContext::TypeAliasToConversion((aspect, context)),
            ffi_aspect_seq_context(),
            |(((aspect, field_types), _generics), presenter)|
                (aspect, field_types.iter()
                    .map(|field_type|
                        OwnedItemPresentableContext::FieldType(
                            presenter(&(
                                quote!(),
                                field_type.conversion_to(FieldContext::Obj))),
                            field_type.attrs()))
                    .collect()),
            bypass_field_context()
        )
    )
}

pub const fn type_alias_composer_root_presenter() -> ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext> {
    |local_context| OwnerIteratorPresentationContext::TypeAlias(local_context)
}

/// Struct Composers
pub const fn struct_ffi_composer<I>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    FFIComposer::new(
        struct_from_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_to_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_destroy_composer(),
        struct_drop_sequence_mixer(),
    )
}
pub const fn struct_from_ffi_conversion_mixer<I>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new_new(
        FFI_FROM_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::AddrDeref(DictionaryName::Ffi.to_token_stream()),
        SequenceComposer::new(
            seq_root,
        target_aspect_seq_context(),
        |(((aspect, fields), _generics), presenter)|
            (aspect, fields.iter().map(|field_type|
                OwnedItemPresentableContext::FieldType(
                    presenter(&(
                        field_type.name(),
                        field_type.conversion_from(FieldContext::FfiRefWithFieldName(FieldContext::Simple(field_type.name()).into())))),
                    field_type.attrs()))
                .collect()),
            seq_iterator_item
        ))
}
pub const fn struct_to_ffi_conversion_mixer<I>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new_new(
        FFI_TO_ROOT_PRESENTER,
        |_| OwnerIteratorPresentationContext::Empty,
        SequenceComposer::new(
            seq_root,
            ffi_aspect_seq_context(),
            |(((name, fields), _generics), presenter)|
                (name.clone(), fields.iter().map(|field_type|
                    OwnedItemPresentableContext::FieldType(
                        presenter(&(
                            field_type.name(),
                            field_type.conversion_to(FieldContext::ObjFieldName(field_type.name())))),
                        field_type.attrs()))
                    .collect()),
            seq_iterator_item
        ))

}
pub const fn struct_destroy_composer<I>() -> OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, empty_context_presenter())
}
pub const fn struct_drop_sequence_mixer<I>() -> DropSequenceMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new(
        |(_, conversion)| conversion.clone(),
        |_| OwnerIteratorPresentationContext::Empty,
        |fields|
            OwnerIteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
        field_types_composer(),
        bypass_field_context(),
        |(fields, presenter)|
            fields.iter()
                .map(|field_type|
                    OwnedItemPresentableContext::FieldType(
                        presenter(&(
                            quote!(),
                            field_type.conversion_destroy(FieldContext::FfiRefWithConversion(field_type.clone())))),
                        field_type.attrs()))
                .collect())
}
pub const fn struct_composer_ctor_unnamed<I>() -> ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized {
    composer_ctor(
        default_ctor_context_composer(),
        struct_unnamed_root(),
        struct_composer_ctor_unnamed_item())
}

pub const fn struct_unit_root<I>() -> ComposerPresenter<FunctionContext, BindingPresentableContext<CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>>
    where I: DelimiterTrait + ?Sized {
    |(context, field_pairs)| {
        let (args, _): (CommaPunctuatedOwnedItems, CommaPunctuatedOwnedItems) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(context, args, Wrapped::<_, _, I>::new(CommaPunctuated::new()))
    }
}
pub const fn struct_named_root<I>() -> ComposerPresenter<FunctionContext, BindingPresentableContext<CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>>
    where I: DelimiterTrait + ?Sized {
    |(context, field_pairs)| {
        let (args, names): (CommaPunctuatedOwnedItems, CommaPunctuatedOwnedItems) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(context, args, Wrapped::<_, _, I>::new(names))
    }
}
pub const fn struct_unnamed_root<I>() -> ComposerPresenter<FunctionContext, BindingPresentableContext<CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>>
    where I: DelimiterTrait + ?Sized {
    |(context, field_pairs)| {
        let (args, names): (CommaPunctuatedOwnedItems, CommaPunctuatedOwnedItems) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(context, args, Wrapped::<_, _, I>::new(names))
    }
}

pub const fn opaque_struct_ctor_post_processor() -> ComposerPresenter<((ConstructorPresentableContext, FieldTypesContext), ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair>), FunctionContext> {
    |((context, fields), field_composer)|
        (context, fields.iter().map(field_composer).collect())
}
pub const fn opaque_struct_composer_ctor_unnamed<I>() -> ConstructorComposer<OpaqueItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized + 'static {
    ConstructorComposer::new(
        struct_unnamed_root(),
        default_opaque_ctor_context_composer(),
        opaque_struct_ctor_post_processor(),
        struct_composer_ctor_unnamed_item()
    )
}
pub const fn opaque_struct_composer_ctor_named<I>() -> ConstructorComposer<OpaqueItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized + 'static {
    ConstructorComposer::new(
        struct_named_root(),
        default_opaque_ctor_context_composer(),
        opaque_struct_ctor_post_processor(),
        struct_composer_ctor_named_opaque_item()
    )
}

const fn struct_composer_ctor_unnamed_item() -> ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair> {
    |field_type| (
        OwnedItemPresentableContext::BindingArg(field_type.clone()),
        OwnedItemPresentableContext::BindingFieldName(field_type.clone())
    )
}
pub(crate) const fn struct_composer_ctor_named_item() -> ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair> {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultField(field_type.clone())
    )
}
const fn struct_composer_ctor_named_opaque_item() -> ComposerPresenterByRef<FieldTypeConversion, OwnedItemPresentablePair> {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultFieldConversion(
            field_type.clone(),
            // FieldContext::Simple(field_type.name()),
            match TypeConversion::from(field_type.ty()) {
                TypeConversion::Primitive(_) =>
                    FieldContext::Simple(field_type.name()),
                TypeConversion::Complex(_) => FieldContext::From(FieldContext::Simple(field_type.name()).into()),
                TypeConversion::Generic(generic_ty) => match generic_ty {
                    GenericTypeConversion::Optional(_) => FieldContext::FromOpt(FieldContext::Simple(field_type.name()).into()),
                    _ => FieldContext::From(FieldContext::Simple(field_type.name()).into())
                }
            },
            // FieldContext::From(FieldContext::Simple(field_type.name()).into()),
            field_type.attrs())
    )
}
pub const fn struct_composer_object<I>() -> OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    ContextComposer::new(|name| name, fields_from_presenter::<I>())
}
pub const fn struct_composer_conversion_named() -> FieldTypePresentationContextPassRef {
    |(field_path, field_context)|
        FieldContext::Named((field_path.clone(), Box::new(field_context.clone())))
}

pub const fn struct_composer_root_presenter_unnamed() -> OwnerIteratorConversionComposer<Comma> {
    |local_context| OwnerIteratorPresentationContext::UnnamedStruct(local_context)
}

pub const fn struct_composer_root_presenter_named() -> OwnerIteratorConversionComposer<Comma> {
    |local_context| OwnerIteratorPresentationContext::NamedStruct(local_context)
}
pub const fn unnamed_struct_field_composer() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.ty().clone(), field_type.attrs())
}

pub const fn named_struct_field_composer() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::Named(field_type.clone(), true)
}


/// Enum Variant Composers
pub const fn enum_variant_composer_ffi_composer<I>(
    conversion_mixer_seq_root: OwnerIteratorConversionComposer<Comma>,
    conversion_seq_iterator_item: FieldTypePresentationContextPassRef,
    destroy_context_root: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
    destroy_seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
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
    |(((name, fields), _generics), presenter)|
        (name, fields.iter().map(|field_type|
            OwnedItemPresentableContext::FieldType(
                presenter(&(
                    field_type.name(),
                    field_type.conversion_from(FieldContext::Deref(field_type.name())))),
                field_type.attrs())).collect()
        )
}
const fn enum_variant_composer_to_sequence_iterator_root<T: Default + ToTokens>() -> ComposerPresenter<(LocalConversionContext, FieldTypePresentationContextPassRef), OwnerAspectIteratorLocalContext<T>> {
    |(((name, fields), _generics), presenter)| {
        (name, fields.iter().map(|field_type|
            OwnedItemPresentableContext::FieldType(
                presenter(&(
                    field_type.name(),
                    field_type.conversion_to(FieldContext::FieldTypeConversionName(field_type.clone())))),
                field_type.attrs())).collect())
    }
}
const fn enum_variant_composer_drop_sequence_iterator_root<'a, SEP: Default + ToTokens>() -> ComposerPresenter<(FieldTypesContext, FieldTypePresentationContextPassRef), Punctuated<OwnedItemPresentableContext, SEP>> {
    |(fields, presenter)|
        fields.iter()
            .map(|field_type|
                OwnedItemPresentableContext::FieldType(
                    presenter(&(
                        field_type.name(),
                        field_type.conversion_destroy(FieldContext::Deref(field_type.name())))),
                    field_type.attrs())
            )
            .collect()
}
pub const fn enum_variant_from_ffi_conversion_mixer<I>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        fields_from_presenter(),
        seq_root,
        |composer: &Ref<ItemComposer<I>>| ((Aspect::RawTarget(composer.base.name_context()), composer.field_types.clone()), composer.base.generics.compose(composer.context())),
        seq_iterator_item,
        enum_variant_composer_from_sequence_iterator_root())
}
pub const fn enum_variant_to_ffi_conversion_mixer<I>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        |composer: &Ref<ItemComposer<I>>| composer.fields_to_composer.compose(&()),
        seq_root,
        ffi_aspect_seq_context::<I>(),
        seq_iterator_item,
        enum_variant_composer_to_sequence_iterator_root())
}
pub const fn enum_variant_destroy_composer<I>(
    root: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
) -> ContextComposer<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext, ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    ContextComposer::new(root, fields_from_presenter::<I>())
}
pub const fn enum_variant_drop_sequence_mixer<I>(
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> DropSequenceMixer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    SequenceMixer::new(
        |(field_path_context, context)|
            OwnerIteratorPresentationContext::Lambda(Box::new(field_path_context.clone()), Box::new(context.clone())),
        fields_from_presenter::<I>(),
        |fields|
            OwnerIteratorPresentationContext::DropCode(fields),
        field_types_composer(),
        seq_iterator_item,
        enum_variant_composer_drop_sequence_iterator_root())
}
pub const fn enum_variant_composer_object<I>() -> OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>
    where I: DelimiterTrait + ?Sized {
    ContextComposer::new(|_owner_iter| OwnerIteratorPresentationContext::Empty, empty_context_presenter::<I>())
}

pub const fn enum_variant_composer_field_presenter() -> OwnedFieldTypeComposerRef {
    |field_type|
        OwnedItemPresentableContext::DefaultField(field_type.clone())
}
pub const fn enum_variant_ctor_context_composer<I>() -> SharedComposer<ItemParentComposer<I>, ConstructorFieldsContext>
    where I: DelimiterTrait + ?Sized {
    |composer|
        (ConstructorPresentableContext::EnumVariant(
            Aspect::FFI(composer.base.name_context())
                .present(&composer.source_ref()),
            composer.compose_attributes().to_token_stream(),
            composer.base.generics.compose(composer.context())
        ),
         field_types_composer()(composer))

}
pub const fn enum_variant_composer_ctor_unit<I>() -> ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_unit_root(),
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_unnamed<I>() -> ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_unnamed_root(),
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_named<I>() -> ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_named_root(),
        struct_composer_ctor_named_item()
    )
}

pub const fn enum_variant_composer_conversion_unit() -> OwnerIteratorConversionComposer<Comma> {
    |(aspect, _)|
        OwnerIteratorPresentationContext::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            Aspect::FFI(_) => aspect.clone(),
            Aspect::RawTarget(_) => aspect.clone(),
        })
}

pub const fn composer_doc_default<T: 'static + BasicComposable<ParentComposer<T>>>() -> ContextComposer<Type, TokenStream2, ParentComposer<T>> {
    ContextComposer::new(
        DEFAULT_DOC_PRESENTER,
        |composer: &Ref<T>|
            composer.target_name_aspect()
                .present(&composer.source_ref()))
}

pub const fn item_composer_doc<I>() -> ContextComposer<Type, TokenStream2, ParentComposer<ItemComposer<I>>>
    where I: DelimiterTrait + ?Sized {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<ItemComposer<I>>| Aspect::Target(composer.base.name_context()).present(&composer.source_ref()))
}
pub const fn opaque_item_composer_doc<I>() -> ContextComposer<Type, TokenStream2, ParentComposer<OpaqueItemComposer<I>>>
    where I: DelimiterTrait + ?Sized + 'static {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<OpaqueItemComposer<I>>| Aspect::Target(composer.base.name_context()).present(&composer.source_ref()))
}


pub const ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<CommaPunctuatedFields, FieldTypesContext> = |fields|
    compose_fields(fields, |index, Field { ty, attrs, .. }| FieldTypeConversion::Unnamed(
        Name::UnnamedArg(index),
        FieldTypeConversionKind::Type(ty.clone()),
        attrs.cfg_attributes_expanded()));

fn compose_fields<F>(fields: &CommaPunctuatedFields, field_mapper: F) -> FieldTypesContext
    where F: Fn(usize, &Field) -> FieldTypeConversion {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| field_mapper(index, field))
        .collect()
}
pub const STRUCT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<CommaPunctuatedFields, FieldTypesContext> = |fields|
    compose_fields(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldTypeConversion::Unnamed(
                Name::UnnamedStructFieldsComp(ty.clone(), index),
                FieldTypeConversionKind::Type(ty.clone()),
                attrs.cfg_attributes_expanded()));

pub const STRUCT_NAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    CommaPunctuatedFields,
    FieldTypesContext> = |fields|
    compose_fields(fields, |_index, Field { ident, ty, attrs, .. }|
        FieldTypeConversion::Named(
            Name::Optional(ident.clone()),
            FieldTypeConversionKind::Type(ty.clone()),
            attrs.cfg_attributes_expanded(),
        ));
pub const EMPTY_FIELDS_COMPOSER: ComposerPresenterByRef<CommaPunctuatedFields,
    FieldTypesContext> = |_| Punctuated::new();

/// Enum composers
// pub const VARIANTS_FROM_PRESENTER: EnumComposerPresenterRef<OwnerIteratorPresentationContext, dyn ScopeContextPresentable<Presentation = TokenStream2>, dyn DelimiterTrait> =
//     |composer|
//         OwnerIteratorPresentationContext::Variants((
//             (composer.base.target_name_aspect().present(&composer.source_ref()), composer.compose_attributes().to_token_stream()),
//             composer.variant_presenters.iter()
//                 .map(|(variant_composer, variant_context)| variant_composer(variant_context))
//                 .collect()));

pub const fn variants_from_presenter<'a, I>() -> EnumComposerPresenterRef<'a, OwnerIteratorPresentationContext, I>
    where I: DelimiterTrait + ?Sized {
    |composer: &Ref<EnumComposer<I>>|
        OwnerIteratorPresentationContext::Variants((
            (composer.base.target_name_aspect().present(&composer.source_ref()), composer.compose_attributes().to_token_stream()),
            composer.variant_presenters.iter()
                .map(|(variant_composer, variant_context)| variant_composer(variant_context))
                .collect()))
}
pub const fn enum_composer_object<I>() -> OwnerIteratorPostProcessingComposer<EnumParentComposer<I>>
    where I: DelimiterTrait + ?Sized + 'static {
    ContextComposer::new(|context| OwnerIteratorPresentationContext::Enum(Box::new(context)), variants_from_presenter())
}
pub const fn enum_composer_doc<I>() -> ContextComposer<Type, TokenStream2, EnumParentComposer<I>>
    where I: DelimiterTrait + ?Sized + 'static {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<EnumComposer<I>>|
        composer.base.target_name_aspect().present(&composer.source_ref()))
}
