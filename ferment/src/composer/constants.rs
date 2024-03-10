use std::cell::Ref;
use std::clone::Clone;
use syn::token::Comma;
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::{Field, parse_quote, Path, Type};
use crate::composer::{BindingDtorComposer, Composer, ComposerPresenter, ComposerPresenterByRef, ConstructorPresentableContext, ContextComposer, ConversionComposer, CtorOwnedComposer, Depunctuated, DropConversionComposer, EnumComposerPresenterRef, EnumParentComposer, FFIComposer, FFIConversionComposer, FieldsOwnedComposer, FieldTypeComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ItemComposerFieldTypesContextPresenter, ItemComposerPresenterRef, ItemParentComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectIteratorLocalContext, OwnerIteratorPostProcessingComposer, SharedComposer, Composable, ItemComposer, EnumComposer, LocalConversionContext};
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, Pop};
use crate::formatter::format_token_stream;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ScopeContextPresentable};
use crate::presentation::context::{FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::binding::BindingPresentableContext;
use crate::presentation::context::name::Aspect;

pub const FFI_FROM_ROOT_PRESENTER: ComposerPresenterByRef<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(field_path, conversions)|
    OwnerIteratorPresentationContext::FromRoot(Box::new(field_path.clone()), Box::new(conversions.clone()));
pub const FFI_TO_ROOT_PRESENTER: ComposerPresenterByRef<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> = |(_, conversions)|
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
// pub const FIELDS_TO_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
//     |composer| composer.fields_to_composer.compose(&());

// pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
//     |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi));
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
    |field_type| field_type.destroy(FieldTypePresentableContext::FfiRefWithConversion(field_type.clone()));

// pub const TARGET_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenterRef =
//     |composer| (Aspect::Target(composer.name_context()), composer.field_types.clone());
// pub const FFI_NAME_LOCAL_CONVERSION_COMPOSER: ItemComposerLocalConversionContextPresenterRef =
//     |composer| (Aspect::FFI(composer.name_context()), composer.field_types.clone());
// pub const FFI_NAME_DTOR_COMPOSER: ItemComposerPresenterRef<DestructorContext> =
//     |composer|
//         Aspect::FFI(composer.name_context())
//             .present(&composer.as_source_ref());

pub const FIELD_TYPES_COMPOSER: ItemComposerFieldTypesContextPresenter =
    |composer| composer.field_types.clone();

pub const BYPASS_FIELD_CONTEXT: FieldTypePresentationContextPassRef =
    |(_, context)| context.clone();


/// Bindings
pub const BINDING_DTOR_COMPOSER: BindingDtorComposer =
    |context|
        BindingPresentation::Destructor {
            ffi_name: context.to_token_stream(),
            name: Name::Destructor(context)
        };
const fn owner_iterator_lambda_composer() -> ComposerPresenterByRef<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> {
    |(left, right)|
        OwnerIteratorPresentationContext::Lambda(Box::new(left.clone()), right.clone().into())
}

// const fn iterator_lambda_composer() -> ComposerPresenterByRef<(OwnerIteratorPresentationContext, OwnerIteratorPresentationContext), OwnerIteratorPresentationContext> {
//     |(left, right)|
//         OwnerIteratorPresentationContext::Lambda(Box::new(left.clone()), Box::new(right.clone()))
// }
pub const fn fields_composer(
    root_composer: ComposerPresenter<OwnerAspectIteratorLocalContext<Comma>, OwnerIteratorPresentationContext>,
    context_composer: SharedComposer<ItemParentComposer, LocalConversionContext>,
    iterator_item_composer: OwnedFieldTypeComposerRef,
) -> FieldsOwnedComposer<ItemParentComposer> {
    FieldsOwnedComposer::new(
        root_composer,
        context_composer,
        |((aspect, field_types), presenter)|
            (aspect, field_types.iter().map(presenter).collect()),
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
    move |composer| {
        let source = composer.as_source_ref();
        let ffi_name = Aspect::FFI(composer.name_context()).present(&source);
        (ConstructorPresentableContext::Default(Name::Constructor(ffi_name.clone()), ffi_name), FIELD_TYPES_COMPOSER(composer))
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
        |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi)),
        |(_, fields)|
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields.into_iter().collect::<Depunctuated<OwnedItemPresentableContext>>()),
        |composer: &Ref<ItemComposer>| (composer.target_name_aspect(), composer.field_types.clone()),
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            (context.0.clone(), context.1.iter()
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
        |composer: &Ref<ItemComposer>| (Aspect::FFI(composer.name_context()), composer.field_types.clone()),
        |(_, conversion)|
            conversion.clone(),
        |(context, presenter)|
            (context.0.clone(), context.1.iter().map(|field_type| {
                let conversion_context = (quote!(), TYPE_ALIAS_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}

pub const fn type_alias_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion.clone(),
        |_| OwnerIteratorPresentationContext::Empty,
        |fields|
            OwnerIteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
        |composer: &Ref<ItemComposer>| composer.field_types.clone(),
        |(_, conversion)| conversion.clone(),
        |(context, presenter)|
            context.iter()
                .map(|field_type| {
                    let conversion_context = (quote!(), STRUCT_FIELD_TYPE_DESTROY_PRESENTER(field_type));
                    OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
                })
                .collect())
}
pub const fn type_alias_composer_root_presenter() -> ComposerPresenter<OwnerAspectIteratorLocalContext<Comma>, OwnerIteratorPresentationContext> {
    |local_context| OwnerIteratorPresentationContext::TypeAlias(local_context)
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
        |_| OwnerIteratorPresentationContext::AddrDeref(quote!(ffi)),
        root_conversion_presenter,
        |composer: &Ref<ItemComposer>| (Aspect::Target(composer.name_context()), composer.field_types.clone()),
        conversion_presenter,
        |((name, fields), presenter)|
            (name.clone(), fields.iter().map(|field_type| {
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
        |_| OwnerIteratorPresentationContext::Empty,
        root_conversion_presenter,
        |composer: &Ref<ItemComposer>| (Aspect::FFI(composer.name_context()), composer.field_types.clone()),
        conversion_presenter,
        |((name, fields), presenter)|
            (name.clone(), fields.iter().map(|field_type| {
                let conversion_context = (field_type.name(), STRUCT_FIELD_TYPE_TO_PRESENTER(field_type));
                OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
            }).collect()))
}
pub const fn struct_composer_destroy() -> OwnerIteratorPostProcessingComposer<ItemParentComposer> {
    ContextComposer::new(ROOT_DESTROY_CONTEXT_COMPOSER, EMPTY_CONTEXT_PRESENTER)
}
pub const fn struct_composer_drop() -> DropConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        |(_, conversion)| conversion.clone(),
        |_| OwnerIteratorPresentationContext::Empty,
        |fields|
            OwnerIteratorPresentationContext::StructDropBody(fields.into_iter().collect()),
        |composer: &Ref<ItemComposer>| composer.field_types.clone(),
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
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Curly(names))
        },
        struct_composer_ctor_named_item())
}
pub const fn struct_composer_ctor_unnamed() -> CtorOwnedComposer<ItemParentComposer> {
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
    OwnerAspectIteratorLocalContext<T>> {
    |((name, fields), presenter)|
        (name.clone(), fields.iter().map(|field_type| {
            let conversion_context = (field_type.name(), DEREF_FIELD_PATH_FROM_PRESENTER(field_type));
            OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
        }).collect())
}
const fn enum_variant_composer_to_local_context_iterator_root_composer<T: Default + ToTokens>() -> ComposerPresenter<(LocalConversionContext, ComposerPresenterByRef<(TokenStream2, FieldTypePresentableContext), FieldTypePresentableContext>), OwnerAspectIteratorLocalContext<T>> {
    |((name, fields), presenter)| {
        (name.clone(), fields.iter().map(|field_type| {
            let conversion_context = (field_type.name(), ENUM_VARIANT_FIELD_TYPE_TO_PRESENTER(field_type));
            OwnedItemPresentableContext::FieldType(presenter(&conversion_context))
        }).collect())
    }
}
const fn enum_variant_composer_drop_local_context_iterator_root_composer<'a, SEP: Default + ToTokens>() -> ComposerPresenter<(FieldTypesContext, ComposerPresenterByRef<(TokenStream2, FieldTypePresentableContext), FieldTypePresentableContext>), Punctuated<OwnedItemPresentableContext, SEP>> {
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
        |composer: &Ref<ItemComposer>| composer.fields_from_composer.compose(&()),
        root_conversion_presenter,
        |composer: &Ref<ItemComposer>|
            (Aspect::RawTarget(composer.name_context()), composer.field_types.clone()),
        conversion_presenter,
        enum_variant_composer_from_local_context_iterator_root_composer())
}
pub const fn enum_variant_composer_to(
    root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
    conversion_presenter: FieldTypePresentationContextPassRef
) -> FFIConversionComposer<ItemParentComposer> {
    ConversionComposer::new(
        owner_iterator_lambda_composer(),
        |composer: &Ref<ItemComposer>| composer.fields_to_composer.compose(&()),
        root_conversion_presenter,
        |composer: &Ref<ItemComposer>|
            (Aspect::FFI(composer.name_context()), composer.field_types.clone()),
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
        |(field_path_context, context)|
            OwnerIteratorPresentationContext::Lambda(Box::new(field_path_context.clone()), Box::new(context.clone())),
        |composer: &Ref<ItemComposer>| composer.fields_from_composer.compose(&()),
        |fields|
            OwnerIteratorPresentationContext::DropCode(fields),
        |composer: &Ref<ItemComposer>| composer.field_types.clone(),
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
        let ffi = Aspect::FFI(composer.name_context()).present(&composer.as_source_ref());
        // let ffi = composer.compose_ffi_name();
        // println!("enum_variant_ctor_context_composer.1::: {} ", ffi.to_token_stream());
        let tt = ffi.to_token_stream();
        let ffi_path: Path = parse_quote!(#tt);
        // println!("enum_variant_ctor_context_composer.2::: {} \n\t\t ---- {} \n\t\t ", ffi.to_token_stream(), ffi_path.popped().to_token_stream());
        (ConstructorPresentableContext::EnumVariant(
            Name::Constructor(ffi),
            // ffi_path.to_token_stream(),
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
            BindingPresentableContext::Constructor(
                context,
                args,
                IteratorPresentationContext::Round(names))
        },
        struct_composer_ctor_named_item())
}

pub const fn enum_variant_composer_ctor_named() -> CtorOwnedComposer<ItemParentComposer> {
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
pub const fn item_composer_doc() -> ContextComposer<Type, TokenStream2, ItemParentComposer> {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<ItemComposer>|
        Aspect::Target(composer.name_context()).present(&composer.as_source_ref())
    )
}


pub const ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    Punctuated<FieldTypeConversion, Comma>> = |fields|
    fields.iter()
        .enumerate()
        .map(|(index, Field { ty, .. })|
            FieldTypeConversion::Unnamed(Name::UnnamedArg(index), ty.clone()))
        .collect();

pub const STRUCT_UNNAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    Punctuated<FieldTypeConversion, Comma>> = |fields|
    fields
        .iter()
        .enumerate()
        .map(|(index, Field { ty, .. })|
            FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty.clone()))
        .collect();

pub const STRUCT_NAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
    Punctuated<Field, Comma>,
    Punctuated<FieldTypeConversion, Comma>> = |fields|
    fields
        .iter()
        .map(|Field { ident, ty, .. }| {
            FieldTypeConversion::Named(Name::Optional(ident.clone()), ty.clone())
        })
        .collect();
pub const EMPTY_FIELDS_COMPOSER: ComposerPresenterByRef<Punctuated<Field, Comma>,
    Punctuated<FieldTypeConversion, Comma>> = |_| Punctuated::new();

/// Enum composers
pub const VARIANTS_FROM_PRESENTER: EnumComposerPresenterRef<OwnerIteratorPresentationContext> =
    |composer| {
        let source = composer.as_source_ref();
        OwnerIteratorPresentationContext::Variants((
            {
                let target_aspect = composer.target_name_aspect();
                let ty = target_aspect.present(&source);
                // parse_quote!(#ty::var)
                // Aspect::Target(composer.name_context())
                //     .present(&source)
                ty
            },
            // composer.compose_target_name(),
            composer.variant_presenters.iter()
                .map(|(variant_composer, variant_context)| {
                    // println!("variant_presenters.1 -> {:?} ---- {}", variant_context.0, variant_context.1.present(&source).to_token_stream());
                    // let aspect = variant_context.0;
                    // aspect
                    let result = variant_composer(variant_context);
                    // println!("variant_presenters.2 -> {} ", result.present(&source).to_token_stream());
                    result
                })
                .collect()))
    };



pub const fn enum_composer_object() -> OwnerIteratorPostProcessingComposer<EnumParentComposer> {
    ContextComposer::new(
        |context| OwnerIteratorPresentationContext::Enum(Box::new(context)),
        VARIANTS_FROM_PRESENTER)
}
pub const fn enum_composer_doc() -> ContextComposer<Type, TokenStream2, EnumParentComposer> {
    ContextComposer::new(DEFAULT_DOC_PRESENTER, |composer: &Ref<EnumComposer>|
        composer.target_name_aspect().present(&composer.as_source_ref()))
}

// pub const ENUM_VARIANT_PRESENTER_UNNAMED: IteratorComposer<OwnerIteratorLocalContext<Comma>, Field, OwnedItemPresentableContext, OwnerIteratorPresentationContext> = IteratorComposer::new(
// |(local_context, presenter)| presenter(local_context),
// |field| OwnedItemPresentableContext::DefaultFieldType(field.ty.clone())
// );