use std::clone::Clone;
use syn::token::Comma;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{Field, parse_quote};
use crate::composer::{BindingDtorComposer, ComposerPresenter, ComposerPresenterByRef, ConstructorComposer, Depunctuated, DropSequenceMixer, FFIComposer, FFIConversionMixer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectIteratorLocalContext, OwnerIteratorPostProcessingComposer, SharedComposer, EnumComposer, LocalConversionContext, VariantIteratorLocalContext, ConstructorFieldsContext, ParentComposer, OwnedItemPresentationPair, CommaPunctuatedTokens, CommaPunctuatedOwnedItems, CommaPunctuatedFields, FunctionContext, ConstructorArgComposer, FieldsComposer, TypeContextComposer, DestructorContext, ParentComposerPresenterByRef, ParentSharedComposer, ParentComposerRef, constants, ItemParentComposer, FieldTypeLocalContext};
use crate::composer::r#abstract::{Composer, ContextComposer, SequenceComposer, SequenceMixer};
use crate::composer::composable::{NameContext, BasicComposable, SourceAccessible, FieldsContext, FieldsConversionComposable};
use crate::composition::CfgAttributes;
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind, GenericTypeConversion, TypeConversion};
use crate::ext::Conversion;
use crate::naming::{DictionaryName, Name};
use crate::presentation::BindingPresentation;
use crate::presentation::context::{ConstructorPresentableContext, FieldContext, OwnedItemPresentableContext, SequenceOutput};
use crate::presentation::context::binding::{BindingPresentableContext, ConstructorBindingPresentableContext};
use crate::presentation::context::name::Aspect;
use crate::shared::SharedAccess;
use crate::wrapped::{DelimiterTrait, Wrapped};

pub const FFI_FROM_ROOT_PRESENTER: ComposerPresenterByRef<OwnedItemPresentationPair, SequenceOutput> = |(field_path, conversions)|
    SequenceOutput::FromRoot(Box::new(field_path.clone()), Box::new(conversions.clone()));
pub const FFI_TO_ROOT_PRESENTER: ComposerPresenterByRef<OwnedItemPresentationPair, SequenceOutput> = |(_, conversions)|
    SequenceOutput::Boxed(conversions.clone().into());
pub const CURLY_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    SequenceOutput::CurlyBracesFields(local_context);
pub const ROUND_BRACES_FIELDS_PRESENTER: OwnerIteratorConversionComposer<Comma> = |local_context|
    SequenceOutput::RoundBracesFields(local_context.clone());
pub const ROOT_DESTROY_CONTEXT_COMPOSER: ComposerPresenter<SequenceOutput, SequenceOutput> =
    |_| SequenceOutput::UnboxedRoot;
pub const fn empty_context_presenter<'a, C>() -> ParentComposerPresenterByRef<'a, C, SequenceOutput>
    where C: FieldsConversionComposable + 'static {
    |_| SequenceOutput::Empty
}
pub const fn fields_from_presenter<'a, C>() -> ParentComposerPresenterByRef<'a, C, SequenceOutput>
    where C: FieldsConversionComposable + 'static {
    |composer: &ParentComposerRef<C>| composer.fields_from().compose(&())
}
pub const fn fields_to_presenter<'a, C>() -> ParentComposerPresenterByRef<'a, C, SequenceOutput>
    where C: FieldsConversionComposable + 'static {
    |composer: &ParentComposerRef<C>| composer.fields_to().compose(&())
}
pub const fn field_types_composer<'a, C>() -> ParentComposerPresenterByRef<'a, C, FieldTypesContext>
    where C: FieldsContext {
    |composer| composer.field_types()
}

pub const fn bypass_field_context() -> FieldTypePresentationContextPassRef {
    |(_, context)| context.clone()
}
pub const fn ffi_aspect_seq_context<C>() -> ParentSharedComposer<C, LocalConversionContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    |composer: &ParentComposerRef<C>| ((composer.ffi_name_aspect(), composer.field_types()), composer.compose_generics())
}

pub const fn target_aspect_seq_context<C>() -> ParentSharedComposer<C, LocalConversionContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    |composer: &ParentComposerRef<C>| ((composer.target_name_aspect(), composer.field_types()), composer.compose_generics())
}
pub const fn raw_target_aspect_seq_context<C>() -> ParentSharedComposer<C, LocalConversionContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    |composer: &ParentComposerRef<C>| ((composer.raw_target_name_aspect(), composer.field_types()), composer.compose_generics())
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

const fn owner_iterator_lambda_composer() -> ComposerPresenterByRef<OwnedItemPresentationPair, SequenceOutput> {
    |(left, right)|
        SequenceOutput::Lambda(Box::new(left.clone()), right.clone().into())
}

pub const fn fields_from_composer<C>(
    root_presenter: ComposerPresenter<VariantIteratorLocalContext, SequenceOutput>,
    field_presenter: OwnedFieldTypeComposerRef
) -> FieldsOwnedComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsConversionComposable + FieldsContext + 'static {
    fields_composer(root_presenter, ffi_aspect_seq_context(), field_presenter)
}
pub const fn fields_to_composer<C>(
    root_presenter: ComposerPresenter<VariantIteratorLocalContext, SequenceOutput>,
    field_presenter: OwnedFieldTypeComposerRef
) -> FieldsOwnedComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsConversionComposable + FieldsContext + 'static {
    fields_composer(root_presenter, target_aspect_seq_context(), field_presenter)
}

pub const fn fields_composer<Parent: SharedAccess>(
    root: ComposerPresenter<VariantIteratorLocalContext, SequenceOutput>,
    context: SharedComposer<Parent, LocalConversionContext>,
    iterator_item: OwnedFieldTypeComposerRef,
) -> FieldsOwnedComposer<Parent> {
    FieldsOwnedComposer::with_iterator_setup(
        root,
        context,
        |(((aspect, field_types), _generics), presenter)|
            (aspect, field_types.iter().map(presenter).collect()),
        iterator_item)
}



pub const fn default_opaque_ctor_context_composer<C>() -> ParentSharedComposer<C, ConstructorFieldsContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    move |composer|
        (ConstructorPresentableContext::Default(composer_target_binding::<C>()(composer)), composer.field_types())
}

/// Type Alias Composers
pub const fn type_alias_composer_ffi_conversions<C>() -> FFIComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + FieldsConversionComposable + 'static {
    FFIComposer::new(
        type_alias_composer_from(),
        type_alias_composer_to(),
        struct_destroy_composer(),
        struct_drop_sequence_mixer())
}
pub const fn type_alias_composer_from<C>() -> FFIConversionMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    SequenceMixer::with_sequence(
        FFI_FROM_ROOT_PRESENTER,
        |_| SequenceOutput::AddrDeref(DictionaryName::Ffi.to_token_stream()),
        SequenceComposer::with_iterator_setup(
            |(_, fields)|
                SequenceOutput::TypeAliasFromConversion(Depunctuated::from_iter(fields.into_iter())),
            target_aspect_seq_context(),
            |(((aspect, field_types), _generics), presenter)| {
                (aspect, compose_fields_conversions(field_types, |field_type|
                        OwnedItemPresentableContext::FieldType(
                            presenter(&(
                                field_type.name(),
                                field_type.conversion_from(FieldContext::FfiRefWithFieldName(FieldContext::Simple(field_type.name()).into())))),
                            field_type.attrs())))
            },
            bypass_field_context()
        )
    )
}

pub const fn type_alias_composer_to<C>() -> FFIConversionMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    SequenceMixer::with_sequence(
        FFI_TO_ROOT_PRESENTER,
        |_| SequenceOutput::Obj,
        SequenceComposer::with_iterator_setup(
            ROUND_BRACES_FIELDS_PRESENTER,
            ffi_aspect_seq_context(),
            |(((aspect, field_types), _generics), presenter)|
                (aspect, compose_fields_conversions(field_types, |field_type|
                        OwnedItemPresentableContext::FieldType(
                            presenter(&(
                                quote!(),
                                field_type.conversion_to(FieldContext::Obj))),
                            field_type.attrs()))),
            bypass_field_context()
        )
    )
}

/// Struct Composers
pub const fn struct_ffi_composer<C>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsConversionComposable + FieldsContext + 'static {
    FFIComposer::new(
        struct_from_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_to_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_destroy_composer(),
        struct_drop_sequence_mixer(),
    )
}

pub const fn struct_from_ffi_conversion_mixer<C>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    SequenceMixer::with_sequence(
        FFI_FROM_ROOT_PRESENTER,
        |_| SequenceOutput::AddrDeref(DictionaryName::Ffi.to_token_stream()),
        SequenceComposer::with_iterator_setup(
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
        )
    )
}


fn compose_fields_conversions<F>(fields: FieldTypesContext, mapper: F) -> CommaPunctuatedOwnedItems
    where F: Fn(&FieldTypeConversion) -> OwnedItemPresentableContext {
    fields.iter().map(mapper).collect()
}

// pub const fn struct_to_ffi_iterator_post<Root, Map>() -> Root
//     where Root: Fn((LocalConversionContext, Map)) -> (Aspect, Vec<OwnedItemPresentableContext>),
//           Map: Fn(&FieldTypeLocalContext) -> FieldContext {
//     |(((name, fields), _generics), presenter): (LocalConversionContext, Map)| (
//         name.clone(),
//         fields.iter()
//             .map(|field_type|
//                 OwnedItemPresentableContext::FieldType(
//                     presenter(
//                         &(
//                             field_type.name(),
//                             field_type.conversion_to(FieldContext::ObjFieldName(field_type.name())))),
//                     field_type.attrs()))
//             .collect())
// }

pub const fn struct_to_ffi_conversion_mixer<C>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    SequenceMixer::with_sequence(
        FFI_TO_ROOT_PRESENTER,
        |_| SequenceOutput::Empty,
        SequenceComposer::with_iterator_setup(
            seq_root,
            ffi_aspect_seq_context(),
            |(((name, fields), _generics), presenter)|
                (
                    name.clone(),
                    compose_fields_conversions(
                        fields,
                        |field_type|
                            OwnedItemPresentableContext::FieldType(
                                presenter(&(field_type.name(), field_type.conversion_to(FieldContext::ObjFieldName(field_type.name())))), field_type.attrs()))),
            seq_iterator_item
        ))

}
pub const fn struct_destroy_composer<C>() -> OwnerIteratorPostProcessingComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsConversionComposable + 'static {
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, empty_context_presenter())
}
pub const fn struct_drop_sequence_mixer<C>() -> DropSequenceMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static {
    SequenceMixer::new(
        |(_, conversion)| conversion.clone(),
        |_| SequenceOutput::Empty,
        |fields| SequenceOutput::StructDropBody(fields.clone()),
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

pub const fn struct_composer_ctor_root<I>() -> ComposerPresenter<FunctionContext, ConstructorBindingPresentableContext<I>>
    where I: DelimiterTrait + ?Sized {
    |(context, field_pairs)| {
        let (args, names): (CommaPunctuatedOwnedItems, CommaPunctuatedOwnedItems) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(context, args, Wrapped::<_, _, I>::new(names))
    }
}

pub const fn opaque_struct_ctor_post_processor() -> ComposerPresenter<((ConstructorPresentableContext, FieldTypesContext), ConstructorArgComposer), FunctionContext> {
    |((context, fields), field_composer)|
        (context, fields.iter().map(field_composer).collect())
}
pub const fn opaque_struct_composer_ctor_unnamed<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized + 'static {
    ConstructorComposer::with_iterator_setup(
        struct_composer_ctor_root(),
        default_opaque_ctor_context_composer::<C>(),
        opaque_struct_ctor_post_processor(),
        struct_composer_ctor_unnamed_item()
    )
}
pub const fn opaque_struct_composer_ctor_named<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized + 'static {
    ConstructorComposer::with_iterator_setup(
        struct_composer_ctor_root(),
        default_opaque_ctor_context_composer::<C>(),
        opaque_struct_ctor_post_processor(),
        struct_composer_ctor_named_opaque_item()
    )
}

const fn struct_composer_ctor_unnamed_item() -> ConstructorArgComposer {
    |field_type| (
        OwnedItemPresentableContext::BindingArg(field_type.clone()),
        OwnedItemPresentableContext::BindingFieldName(field_type.clone())
    )
}
const fn struct_composer_ctor_named_item() -> ConstructorArgComposer {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultField(field_type.clone())
    )
}
const fn struct_composer_ctor_named_opaque_item() -> ConstructorArgComposer {
    |field_type| (
        OwnedItemPresentableContext::Named(field_type.clone(), false),
        OwnedItemPresentableContext::DefaultFieldConversion(
            field_type.clone(),
            match TypeConversion::from(field_type.ty()) {
                TypeConversion::Primitive(_) =>
                    FieldContext::Simple(field_type.name()),
                TypeConversion::Complex(_) => FieldContext::From(FieldContext::Simple(field_type.name()).into()),
                TypeConversion::Generic(generic_ty) => match generic_ty {
                    GenericTypeConversion::Optional(_) => FieldContext::FromOpt(FieldContext::Simple(field_type.name()).into()),
                    _ => FieldContext::From(FieldContext::Simple(field_type.name()).into())
                }
            },
            field_type.attrs())
    )
}
pub const fn struct_composer_object<C>() -> OwnerIteratorPostProcessingComposer<ParentComposer<C>>
    where C: FieldsConversionComposable + 'static {
    ContextComposer::new(|name| name, fields_from_presenter::<C>())
}
pub const fn struct_composer_conversion_named() -> FieldTypePresentationContextPassRef {
    |(field_path, field_context)|
        FieldContext::Named((field_path.clone(), Box::new(field_context.clone())))
}

pub const fn struct_composer_root_presenter_unnamed() -> OwnerIteratorConversionComposer<Comma> {
    |local_context| SequenceOutput::UnnamedStruct(local_context)
}

pub const fn struct_composer_root_presenter_named() -> OwnerIteratorConversionComposer<Comma> {
    |local_context| SequenceOutput::NamedStruct(local_context)
}
pub const fn unnamed_struct_field_composer() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.ty().clone(), field_type.attrs())
}

pub const fn named_struct_field_composer() -> OwnedFieldTypeComposerRef {
    |field_type| OwnedItemPresentableContext::Named(field_type.clone(), true)
}


/// Enum Variant Composers
pub const fn enum_variant_composer_ffi_composer<C>(
    conversion_mixer_seq_root: OwnerIteratorConversionComposer<Comma>,
    conversion_seq_iterator_item: FieldTypePresentationContextPassRef,
    destroy_context_root: ComposerPresenter<SequenceOutput, SequenceOutput>,
    destroy_seq_iterator_item: FieldTypePresentationContextPassRef,
) -> FFIComposer<ParentComposer<C>>
    where C: FieldsConversionComposable + FieldsContext + BasicComposable<ParentComposer<C>> + 'static {
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
pub const fn enum_variant_from_ffi_conversion_mixer<C>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ParentComposer<C>>
    where C: FieldsContext + FieldsConversionComposable + BasicComposable<ParentComposer<C>> + 'static {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        fields_from_presenter(),
        seq_root,
        raw_target_aspect_seq_context(),
        seq_iterator_item,
        enum_variant_composer_from_sequence_iterator_root())
}
pub const fn enum_variant_to_ffi_conversion_mixer<C>(
    seq_root: OwnerIteratorConversionComposer<Comma>,
    seq_iterator_item: FieldTypePresentationContextPassRef
) -> FFIConversionMixer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + FieldsConversionComposable + 'static {
    SequenceMixer::new(
        owner_iterator_lambda_composer(),
        fields_to_presenter::<C>(),
        seq_root,
        ffi_aspect_seq_context::<C>(),
        seq_iterator_item,
        enum_variant_composer_to_sequence_iterator_root())
}
pub const fn enum_variant_destroy_composer<C>(
    root: ComposerPresenter<SequenceOutput, SequenceOutput>,
) -> OwnerIteratorPostProcessingComposer<ParentComposer<C>>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + FieldsConversionComposable + 'static {
    ContextComposer::new(root, fields_from_presenter::<C>())
}
pub const fn fields_from_presenter_composer<C>(
    root: ComposerPresenter<SequenceOutput, SequenceOutput>
) -> OwnerIteratorPostProcessingComposer<ParentComposer<C>>
    where C: FieldsConversionComposable + 'static {
    ContextComposer::new(root, fields_from_presenter::<C>())
}

pub const fn enum_variant_drop_sequence_mixer<C>(
    seq_iterator_item: FieldTypePresentationContextPassRef,
) -> DropSequenceMixer<ParentComposer<C>>
    where C: FieldsConversionComposable + FieldsContext + 'static {
    SequenceMixer::new(
        |(field_path_context, context)|
            SequenceOutput::Lambda(Box::new(field_path_context.clone()), Box::new(context.clone())),
        fields_from_presenter::<C>(),
        |fields| SequenceOutput::DropCode(fields),
        field_types_composer(),
        seq_iterator_item,
        enum_variant_composer_drop_sequence_iterator_root())
}
pub const fn enum_variant_composer_object<C>() -> OwnerIteratorPostProcessingComposer<ParentComposer<C>>
    where C: FieldsConversionComposable + 'static {
    ContextComposer::new(|_owner_iter| SequenceOutput::Empty, empty_context_presenter::<C>())
}

pub const fn enum_variant_composer_field_presenter() -> OwnedFieldTypeComposerRef {
    |field_type|
        OwnedItemPresentableContext::DefaultField(field_type.clone())
}

pub const fn enum_variant_composer_conversion_unit() -> OwnerIteratorConversionComposer<Comma> {
    |(aspect, _)|
        SequenceOutput::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            Aspect::FFI(_) => aspect.clone(),
            Aspect::RawTarget(_) => aspect.clone(),
        })
}

pub const ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: FieldsComposer = |fields|
    compose_fields(fields, |index, Field { ty, attrs, .. }| FieldTypeConversion::Unnamed(
        Name::UnnamedArg(index),
        FieldTypeConversionKind::Type(ty.clone()),
        attrs.cfg_attributes_expanded()));

pub const STRUCT_UNNAMED_FIELDS_COMPOSER: FieldsComposer = |fields|
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
pub const EMPTY_FIELDS_COMPOSER: FieldsComposer = |_| Punctuated::new();
fn compose_fields<F>(fields: &CommaPunctuatedFields, mapper: F) -> FieldTypesContext
    where F: Fn(usize, &Field) -> FieldTypeConversion {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| mapper(index, field))
        .collect()
}

/// Enum composers
pub const fn enum_composer_object<I>() -> OwnerIteratorPostProcessingComposer<ParentComposer<EnumComposer<I>>>
    where I: DelimiterTrait + ?Sized + 'static {
    ContextComposer::new(
        |context| SequenceOutput::Enum(Box::new(context)),
        |composer: &ParentComposerRef<EnumComposer<I>>|
            SequenceOutput::Variants(
                composer.target_name_aspect(),
                composer.compose_attributes(),
                composer.variant_presenters
                    .iter()
                    .map(|(variant_composer, variant_context)| variant_composer(variant_context))
                    .collect()))
}
pub const fn composer_doc<C>() -> TypeContextComposer<ParentComposer<C>>
    where C: NameContext + SourceAccessible + 'static {
    ContextComposer::new(
        |target_name| {
            let comment = format!("FFI-representation of the [`{}`]", target_name.to_token_stream());
            // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
            parse_quote! { #[doc = #comment] }
        },
        |composer: &ParentComposerRef<C>| composer.compose_target_name()
    )
}

pub const fn composer_ctor<C, I>(
    context: ParentSharedComposer<C, ConstructorFieldsContext>,
    iterator_item: ConstructorArgComposer,
) -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where I: DelimiterTrait + ?Sized  {
    ConstructorComposer::with_iterator_setup(
        struct_composer_ctor_root(),
        context,
        |((constructor_context, fields), presenter)|
            (constructor_context, fields.iter().map(presenter).collect()),
        iterator_item
    )
}

pub const fn struct_composer_ctor_named<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized {
    composer_ctor(
        default_ctor_context_composer(),
        struct_composer_ctor_named_item())
}
pub const fn struct_composer_ctor_unnamed<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized {
    composer_ctor(
        default_ctor_context_composer(),
        struct_composer_ctor_unnamed_item())
}

pub const fn enum_variant_composer_ctor_unit<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_unnamed<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_composer_ctor_unnamed_item())
}

pub const fn enum_variant_composer_ctor_named<C, I>() -> ConstructorComposer<ParentComposer<C>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext + 'static, I: DelimiterTrait + ?Sized {
    composer_ctor(
        enum_variant_ctor_context_composer(),
        struct_composer_ctor_named_item()
    )
}

pub const fn default_ctor_context_composer<C>() -> ParentSharedComposer<C, ConstructorFieldsContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext {
    |composer| (ConstructorPresentableContext::Default(composer_ffi_binding::<C>()(composer)), composer.field_types())
}

pub const fn enum_variant_ctor_context_composer<C>() -> ParentSharedComposer<C, ConstructorFieldsContext>
    where C: BasicComposable<ParentComposer<C>> + FieldsContext {
    |composer| (ConstructorPresentableContext::EnumVariant(composer_ffi_binding::<C>()(composer)), composer.field_types())
}

pub const fn composer_ffi_binding<C>() -> ParentSharedComposer<C, DestructorContext>
    where C: BasicComposable<ParentComposer<C>> + 'static {
    |composer: &ParentComposerRef<C>| (composer.compose_ffi_name(), composer.compose_attributes(), composer.compose_generics())
}
pub const fn composer_target_binding<C>() -> ParentSharedComposer<C, DestructorContext>
    where C: BasicComposable<ParentComposer<C>> + 'static {
    |composer: &ParentComposerRef<C>| (composer.compose_target_name(), composer.compose_attributes(), composer.compose_generics())
}
