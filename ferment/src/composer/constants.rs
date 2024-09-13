use std::clone::Clone;
use std::marker::PhantomData;
use syn::token::Comma;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Expr, ExprPath, Field, parse_quote, Visibility, VisPublic};
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{CfgAttributes, FieldComposer, FieldTypeKind};
use crate::composer::{Composer, ComposerPresenter, ComposerPresenterByRef, ContextComposer, CtorSequenceComposer, DropSequenceMixer, FieldsContext, FieldsConversionComposable, FFIComposer, FFIConversionMixer, FieldsOwnedSequenceComposer, FieldTypePresentationContextPassRef, FieldComposers, NameContext, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectWithItems, OwnerIteratorPostProcessingComposer, SharedComposer, LocalConversionContext, OwnerAspectWithCommaPunctuatedItems, ConstructorFieldsContext, ComposerLink, SequenceOutputPair, CommaPunctuatedFields, ConstructorArgComposerRef, FieldsComposerRef, TypeContextComposer, DestructorContext, ComposerLinkDelegateByRef, SharedComposerLink, ComposerRef, OwnedItemsPunctuated, LocallyOwnedFieldComposers, SequenceComposer, SequenceMixer, SourceAccessible, OwnedStatement, FieldTypeLocalContext, ToConversionComposer, DestroyConversionComposer, VariantComposable, NameComposable, ArgComposers, BindingDtorComposer, BindingAccessorComposer, AttrComposable, GenericsComposable, BindingCtorComposer};
use crate::ext::{ConversionType, ToPath};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Aspect, BindingPresentableContext, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryName, Name};
use crate::shared::SharedAccess;

pub type FieldTypeIterator<T, RES, LANG, SPEC> = ComposerPresenter<(T, FieldTypePresentationContextPassRef<LANG, SPEC>), RES>;
pub type FieldPathResolver<LANG, SPEC> = ComposerPresenterByRef<FieldComposer<LANG, SPEC>, FieldTypeLocalContext<LANG, SPEC>>;
pub type DropFieldsIterator<SEP, LANG, SPEC> = FieldTypeIterator<FieldComposers<LANG, SPEC>, OwnedItemsPunctuated<SEP, LANG, SPEC>, LANG, SPEC>;
pub type OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen> = FieldTypeIterator<LocalConversionContext<LANG, SPEC, Gen>, OwnerAspectWithItems<SEP, LANG, SPEC>, LANG, SPEC>;

pub const fn ffi_from_root_presenter<LANG, SPEC>()
    -> ComposerPresenterByRef<SequenceOutputPair<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(field_path, conversions)|
        SequenceOutput::FromRoot(Box::new(field_path.clone()), Box::new(conversions.clone()))
}
pub const fn ffi_to_root_presenter<LANG, SPEC>()
    -> ComposerPresenterByRef<SequenceOutputPair<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(_, conversions)|
        SequenceOutput::Boxed(conversions.clone().into())
}
pub const fn curly_braces_fields_presenter<LANG, SPEC>()
    -> OwnerIteratorConversionComposer<Comma, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |local_context|
        SequenceOutput::CurlyBracesFields(local_context)
}
pub const fn round_braces_fields_presenter<LANG, SPEC>()
    -> OwnerIteratorConversionComposer<Comma, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |local_context|
        SequenceOutput::RoundBracesFields(local_context)
}
pub const fn root_destroy_context_composer<LANG, SPEC>()
    -> ComposerPresenter<SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |_|
        SequenceOutput::UnboxedRoot
}

pub const fn empty_context_presenter<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, SequenceOutput<LANG, SPEC>>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |_|
        SequenceOutput::Empty
}
pub const fn deref_context_presenter<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, SequenceOutput<LANG, SPEC>>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |_|
        SequenceOutput::AddrDeref(DictionaryName::Ffi.to_token_stream())
}
pub const fn obj_context_presenter<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, SequenceOutput<LANG, SPEC>>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |_|
        SequenceOutput::Obj
}
pub const fn fields_from_presenter<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, SequenceOutput<LANG, SPEC>>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer: &ComposerRef<C>|
        composer.fields_from().compose(&())
}
pub const fn fields_to_presenter<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, SequenceOutput<LANG, SPEC>>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer: &ComposerRef<C>|
        composer.fields_to().compose(&())
}
pub const fn field_types_composer<'a, C, LANG, SPEC, Gen>()
    -> ComposerLinkDelegateByRef<'a, C, FieldComposers<LANG, SPEC>>
    where C: FieldsContext<LANG, SPEC>,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer|
        composer.field_types()
}

pub const fn bypass_field_context<LANG, SPEC, Gen>()
    -> FieldTypePresentationContextPassRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |(_, context)|
        Expression::ConversionType(context.clone().into())
}
pub const fn empty_field_context<LANG, SPEC, Gen>()
    -> FieldTypePresentationContextPassRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |_| Expression::Empty
}
pub const fn terminated_field_context<LANG, SPEC>()
    -> FieldTypePresentationContextPassRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(_, context)|
        Expression::Terminated(context.clone().into())
}
pub const fn ffi_aspect_seq_context<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: NameContext<Context>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer: &ComposerRef<C>|
        ((composer.ffi_name_aspect(), composer.field_types()), C::compose_generics(composer))
}

pub const fn target_aspect_seq_context<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: NameContext<Context>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer: &ComposerRef<C>|
        ((composer.target_name_aspect(), composer.field_types()), composer.compose_generics())
}
pub const fn raw_target_aspect_seq_context<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, LocalConversionContext<LANG, SPEC, Gen>>
    where C: NameContext<Context>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer: &ComposerRef<C>|
        ((composer.raw_target_name_aspect(), composer.field_types()), composer.compose_generics())
}


/// Bindings
pub const fn struct_composer_ctor_root<LANG, SPEC, Gen>()
    -> BindingCtorComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    // |(context, field_pairs)| {
    |context| {
        // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        // BindingPresentableContext::ctor(context, args, Wrapped::<_, _, I>::new(names))
        BindingPresentableContext::ctor(context)
    }
}
pub const fn enum_variant_composer_ctor_root<LANG, SPEC, Gen>()
    -> BindingCtorComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    // |(context, field_pairs)| {
    |context| {
        // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        // BindingPresentableContext::ctor(context, args, Wrapped::<_, _, I>::new(names))
        BindingPresentableContext::variant_ctor(context)
    }
}

// pub const fn binding_ctor_composer<LANG, SPEC>() -> BindingCtorComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG>,
//           // I: DelimiterTrait + ?Sized,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
//     |context| BindingPresentableContext::ctor(context)
// }
// pub const fn binding_ctor_variant_composer<I, LANG, SPEC>() -> BindingCtorComposer<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG>,
//           // I: DelimiterTrait + ?Sized,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
//     |context| BindingPresentableContext::variant_ctor(context)
// }
pub const fn binding_dtor_composer<LANG, SPEC, Gen>() -> BindingDtorComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          // I: DelimiterTrait + ?Sized,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    |context| BindingPresentableContext::dtor(context)
}
pub const fn binding_getter_composer<LANG, SPEC, Gen>() -> BindingAccessorComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          // I: DelimiterTrait + ?Sized,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    |context| BindingPresentableContext::get(context)
}
pub const fn binding_setter_composer<LANG, SPEC, Gen>() -> BindingAccessorComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          // I: DelimiterTrait +?Sized,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    |context| BindingPresentableContext::set(context)
}

// pub const BINDING_DTOR_COMPOSER: RustBindingDtorComposer<RustFermentate, Vec<Attribute>> =
//     |(ty, attrs, generics, ..)|
//         BindingPresentation::Destructor { attrs, generics, ty };
// pub const BINDING_GETTER_COMPOSER: RustBindingComposer<BindingAccessorContext<RustFermentate, Vec<Attribute>>> =
//     |(obj_type, field_name, field_type, attrs, generics, ..)|
//         BindingPresentation::Getter {
//             attrs,
//             name: Name::getter(obj_type.to_path(), &field_name),
//             field_name,
//             obj_type,
//             field_type,
//             generics
//         };
// pub const BINDING_SETTER_COMPOSER: RustBindingComposer<BindingAccessorContext<RustFermentate, Vec<Attribute>>> =
//     |(obj_type, field_name, field_type, attrs, generics, ..)|
//         BindingPresentation::Setter {
//             attrs,
//             name: Name::setter(obj_type.to_path(), &field_name),
//             field_name,
//             obj_type,
//             field_type,
//             generics
//         };
// pub const BINDING_OPAQUE_GETTER_COMPOSER: RustBindingComposer<BindingAccessorContext<RustFermentate, Vec<Attribute>>> =
//     |(obj_type, field_name, field_type, attrs, generics, ..)|
//         BindingPresentation::GetterOpaque {
//             attrs,
//             name: Name::getter(obj_type.to_path(), &field_name),
//             field_name,
//             obj_type,
//             field_type,
//             generics
//         };
// pub const BINDING_OPAQUE_SETTER_COMPOSER: RustBindingComposer<BindingAccessorContext<RustFermentate, Vec<Attribute>>> =
//     |(obj_type, field_name, field_type, attrs, generics, ..)|
//         BindingPresentation::SetterOpaque {
//             attrs,
//             name: Name::setter(obj_type.to_path(), &field_name),
//             field_name,
//             obj_type,
//             field_type,
//             generics
//         };

const fn owner_iterator_lambda_composer<LANG, SPEC, Gen>()
    -> ComposerPresenterByRef<SequenceOutputPair<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |(left, right)|
        SequenceOutput::Lambda(Box::new(left.clone()), right.clone().into())
}

pub const fn fields_from_composer<C, LANG, SPEC, Gen>(
    root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
    field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>
) -> FieldsOwnedSequenceComposer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fields_composer(root_presenter, ffi_aspect_seq_context(), field_presenter)
}
pub const fn fields_to_composer<C, LANG, SPEC, Gen>(
    root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
    field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>
) -> FieldsOwnedSequenceComposer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fields_composer(root_presenter, target_aspect_seq_context(), field_presenter)
}

pub const fn fields_composer<Link, LANG, SPEC, Gen>(
    root: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
    context: SharedComposer<Link, LocalConversionContext<LANG, SPEC, Gen>>,
    iterator_item: OwnedFieldTypeComposerRef<LANG, SPEC>,
) -> FieldsOwnedSequenceComposer<Link, LANG, SPEC, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    FieldsOwnedSequenceComposer::with_iterator_setup(
        root,
        context,
        fields_composer_iterator_root(),
        iterator_item)
}

pub const fn named_opaque_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
    // BindingPresentableContext::ctor(context, args, Wrapped::<_, _, I>::new(names))

    move |composer| {

        // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        (((composer_target_binding()(composer), false), composer.field_types()), composer.compose_generics())
        // ((ConstructorPresentableContext::Default(composer_target_binding()(composer)), composer.field_types()), composer.compose_generics())
    }

}
pub const fn unnamed_opaque_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
    // BindingPresentableContext::ctor(context, args, Wrapped::<_, _, I>::new(names))

    move |composer| {

        // let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        (((composer_target_binding()(composer), true), composer.field_types()), composer.compose_generics())
        // ((ConstructorPresentableContext::Default(composer_target_binding()(composer)), composer.field_types()), composer.compose_generics())
    }

}
pub const fn named_enum_variant_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer|

        (((composer_ffi_binding()(composer), false), composer.field_types()), composer.compose_generics())
        // ((ConstructorPresentableContext::EnumVariant(composer_ffi_binding()(composer)), composer.field_types()), composer.compose_generics())
}
pub const fn unnamed_enum_variant_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer|

        (((composer_ffi_binding()(composer), true), composer.field_types()), composer.compose_generics())
        // ((ConstructorPresentableContext::EnumVariant(composer_ffi_binding()(composer)), composer.field_types()), composer.compose_generics())
}
pub const fn unnamed_struct_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer|
        (((composer_ffi_binding()(composer), true), composer.field_types()), composer.compose_generics())
}
pub const fn named_struct_ctor_context_composer<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer|
        (((composer_ffi_binding()(composer), false), composer.field_types()), composer.compose_generics())
}
/// Type Alias Composers
pub const fn type_alias_composer_ffi_conversions<C, LANG, SPEC, Gen>()
    -> FFIComposer<ComposerLink<C>, LANG, SPEC, Gen>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    FFIComposer::new(
        type_alias_composer_from(),
        type_alias_composer_to(),
        struct_destroy_composer(),
        struct_drop_sequence_mixer())
}

pub const fn type_alias_composer_from<C, LANG, SPEC, Gen>()
    -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::with_sequence(
        ffi_from_root_presenter(),
        deref_context_presenter(),
        SequenceComposer::with_iterator_setup(
            |(_, fields)| SequenceOutput::TypeAliasFromConversion(Depunctuated::from_iter(fields)),
            target_aspect_seq_context(),
            struct_composer_from_iterator_post_processor(),
            bypass_field_context::<LANG, SPEC, Gen>()
        )
    )
}

pub const fn type_alias_composer_to<C, LANG, SPEC, Gen>()
    -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::with_sequence(
        ffi_to_root_presenter(),
        obj_context_presenter(),
        SequenceComposer::with_iterator_setup(
            round_braces_fields_presenter(),
            ffi_aspect_seq_context(),
            type_alias_composer_to_iterator_post_processor(),
            bypass_field_context::<LANG, SPEC, Gen>()
        )
    )
}

/// Struct Composers
pub const fn struct_ffi_composer<C, LANG, SPEC, Gen>(
    seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>,
) -> FFIComposer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable  {
    FFIComposer::new(
        struct_from_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_to_ffi_conversion_mixer(seq_root, seq_iterator_item),
        struct_destroy_composer(),
        struct_drop_sequence_mixer(),
    )
}

pub const fn struct_from_ffi_conversion_mixer<C, LANG, SPEC, Gen>(
    seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>
) -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::with_sequence(
        ffi_from_root_presenter(),
        deref_context_presenter(),
        SequenceComposer::with_iterator_setup(
            seq_root,
            target_aspect_seq_context(),
            struct_composer_from_iterator_post_processor(),
            seq_iterator_item
        )
    )
}


pub const fn struct_to_ffi_conversion_mixer<C, LANG, SPEC, Gen>(
    seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>
) -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::with_sequence(
        ffi_to_root_presenter(),
        empty_context_presenter(),
        SequenceComposer::with_iterator_setup(
            seq_root,
            ffi_aspect_seq_context(),
            struct_composer_to_iterator_post_processor(),
            seq_iterator_item
        )
    )
}
pub const fn struct_destroy_composer<C, LANG, SPEC, Gen>()
    -> OwnerIteratorPostProcessingComposer<ComposerLink<C>, LANG, SPEC>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    ContextComposer::new(root_destroy_context_composer(), empty_context_presenter())
}


pub(crate) const fn struct_ctor_sequence_composer<C, LANG, SPEC, Gen>(
    ctor_root: BindingCtorComposer<LANG, SPEC, Gen>,
    context: SharedComposerLink<C, ConstructorFieldsContext<LANG, SPEC, Gen>>,
    field_item_iterator: ConstructorArgComposerRef<LANG, SPEC>,
) -> CtorSequenceComposer<ComposerLink<C>, LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    CtorSequenceComposer::with_iterator_setup(
        ctor_root,
        context,
        fields_composer_iterator_root(),
        field_item_iterator
    )
}

impl<LANG, SPEC> From<&FieldComposer<LANG, SPEC>> for Expr
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn from(value: &FieldComposer<LANG, SPEC>) -> Self {
        Expr::Path(ExprPath { attrs: vec![], qself: None, path: value.tokenized_name().to_path() })
    }
}

pub const fn struct_composer_ctor_callback_item<LANG, SPEC>()
    -> ConstructorArgComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| (
        OwnedItemPresentableContext::CallbackArg(composer.clone()),
        OwnedItemPresentableContext::BindingFieldName(composer.clone())
    )
}
pub const fn struct_composer_ctor_unnamed_item<LANG, SPEC>()
    -> ConstructorArgComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| (
        OwnedItemPresentableContext::BindingArg(composer.clone()),
        OwnedItemPresentableContext::BindingFieldName(composer.clone())
    )
}
pub const fn struct_composer_ctor_named_item<LANG, SPEC>()
    -> ConstructorArgComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| (
        OwnedItemPresentableContext::Named(composer.clone(), Visibility::Inherited),
        OwnedItemPresentableContext::Expression(Expression::Expr(Expr::from(composer)), composer.attrs.clone())
    )
}
pub const fn struct_composer_ctor_named_opaque_item<LANG, SPEC>()
    -> ConstructorArgComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| (
        OwnedItemPresentableContext::Named(composer.clone(), Visibility::Inherited),
        OwnedItemPresentableContext::DefaultFieldConversion(composer.clone())
    )
}
pub const fn struct_composer_object<C, LANG, SPEC, Gen>()
    -> OwnerIteratorPostProcessingComposer<ComposerLink<C>, LANG, SPEC>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    ContextComposer::new(|name| name, fields_from_presenter::<C, LANG, SPEC, Gen>())
}
pub const fn struct_composer_conversion_named<LANG, SPEC>()
    -> FieldTypePresentationContextPassRef<LANG, SPEC> where LANG: Clone,
                                                             SPEC: LangAttrSpecification<LANG> {
    |(name, composer)|
        Expression::NamedComposer((name.to_token_stream(), Box::new(composer.clone())))
}
pub const fn struct_composer_root_presenter_unnamed<LANG, SPEC>()
    -> OwnerIteratorConversionComposer<Comma, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |local_context| SequenceOutput::UnnamedStruct(local_context)
}
pub const fn struct_composer_root_presenter_named<LANG, SPEC>()
    -> OwnerIteratorConversionComposer<Comma, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |local_context| SequenceOutput::NamedStruct(local_context)
}
pub const fn unnamed_struct_field_composer<LANG, SPEC>() -> OwnedFieldTypeComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| OwnedItemPresentableContext::DefaultFieldType(composer.ty().clone(), composer.attrs.clone())
}

pub const fn named_struct_field_composer<LANG, SPEC>() -> OwnedFieldTypeComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| OwnedItemPresentableContext::Named(composer.clone(), Visibility::Public(VisPublic { pub_token: Default::default() }))
}
pub const fn callback_field_composer<LANG, SPEC>() -> OwnedFieldTypeComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer| OwnedItemPresentableContext::CallbackArg(composer.clone())
}


/// Enum Variant Composers
pub const fn enum_variant_composer_ffi_composer<C, LANG, SPEC, Gen>(
    conversion_mixer_seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    conversion_seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>,
    destroy_context_root: ComposerPresenter<SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
    destroy_seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>,
) -> FFIComposer<ComposerLink<C>, LANG, SPEC, Gen>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + FieldsContext<LANG, SPEC>
            + SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            // + BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    FFIComposer::new(
        enum_variant_from_ffi_conversion_mixer(conversion_mixer_seq_root, conversion_seq_iterator_item),
        enum_variant_to_ffi_conversion_mixer(conversion_mixer_seq_root, conversion_seq_iterator_item),
        fields_from_presenter_composer(destroy_context_root),
        enum_variant_drop_sequence_mixer(destroy_seq_iterator_item)
    )
}
pub const fn enum_variant_from_ffi_conversion_mixer<C, LANG, SPEC, Gen>(
    seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>
) -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    where C: FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            // + BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
            + SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::new(
        owner_iterator_lambda_composer::<LANG, SPEC, Gen>(),
        fields_from_presenter(),
        seq_root,
        raw_target_aspect_seq_context(),
        seq_iterator_item,
        enum_variant_composer_from_sequence_iterator_root())
}
pub const fn enum_variant_to_ffi_conversion_mixer<C, LANG, SPEC, Gen>(
    seq_root: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>
) -> FFIConversionMixer<ComposerLink<C>, LANG, SPEC, Gen>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    SequenceMixer::new(
        owner_iterator_lambda_composer::<LANG, SPEC, Gen>(),
        fields_to_presenter::<C, LANG, SPEC, Gen>(),
        seq_root,
        ffi_aspect_seq_context::<C, LANG, SPEC, Gen>(),
        seq_iterator_item,
        enum_variant_composer_to_sequence_iterator_root())
}
pub const fn fields_from_presenter_composer<C, LANG, SPEC, Gen>(
    root: ComposerPresenter<SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
) -> OwnerIteratorPostProcessingComposer<ComposerLink<C>, LANG, SPEC>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    ContextComposer::new(root, fields_from_presenter::<C, LANG, SPEC, Gen>())
}

pub const fn struct_drop_sequence_post_processor<LANG, SPEC>()
    -> ComposerPresenterByRef<(SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>), SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(_, conversion)| conversion.clone()
}
pub const fn struct_drop_sequence_root_presenter<LANG, SPEC>()
    -> ComposerPresenter<OwnedStatement<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |fields| SequenceOutput::StructDropBody(fields.clone())
}
pub const fn enum_variant_drop_sequence_post_processor<LANG, SPEC>()
    -> ComposerPresenterByRef<(SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>), SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(field_path_context, context)| SequenceOutput::Lambda(Box::new(field_path_context.clone()), Box::new(context.clone()))
}
pub const fn enum_variant_drop_sequence_root_presenter<LANG, SPEC>()
    -> ComposerPresenter<OwnedStatement<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |fields| SequenceOutput::DropCode(fields)
}
pub const fn struct_drop_sequence_mixer<C, CTX, LANG, SPEC, Gen>() -> DropSequenceMixer<ComposerLink<C>, LANG, SPEC>
    // where C: BasicComposable<ComposerLink<C>, CTX, LANG, SPEC, Option<Generics>>
    where C: SourceAccessible
            + NameContext<Context>
            + AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + FieldsContext<LANG, SPEC>
            + FieldsConversionComposable<LANG, SPEC, Gen>
            + 'static,
          CTX: Clone,
          Aspect<CTX>: ScopeContextPresentable,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    DropSequenceMixer::new(
        struct_drop_sequence_post_processor(),
        empty_context_presenter(),
        struct_drop_sequence_root_presenter(),
        field_types_composer::<C, LANG, SPEC, Gen>(),
        bypass_field_context::<LANG, SPEC, Gen>(),
        struct_composer_drop_fields_iterator::<_, LANG, SPEC, Gen>()
    )
}

pub const fn enum_variant_drop_sequence_mixer<C, LANG, SPEC, Gen>(
    seq_iterator_item: FieldTypePresentationContextPassRef<LANG, SPEC>,
) -> DropSequenceMixer<ComposerLink<C>, LANG, SPEC>
    where C: FieldsConversionComposable<LANG, SPEC, Gen>
            + FieldsContext<LANG, SPEC>
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable {
    DropSequenceMixer::new(
        enum_variant_drop_sequence_post_processor(),
        fields_from_presenter::<C, LANG, SPEC, Gen>(),
        enum_variant_drop_sequence_root_presenter(),
        field_types_composer::<C, LANG, SPEC, Gen>(),
        seq_iterator_item,
        enum_variant_composer_drop_fields_iterator())
}
pub const fn enum_variant_composer_object<C, LANG, SPEC, Gen>()
    -> OwnerIteratorPostProcessingComposer<ComposerLink<C>, LANG, SPEC>
    where C: FieldsConversionComposable<LANG, SPEC, Gen> + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    ContextComposer::new(|_owner_iter| SequenceOutput::Empty, empty_context_presenter::<C, LANG, SPEC, Gen>())
}

pub const fn enum_variant_composer_field_presenter<LANG, SPEC>()
    -> OwnedFieldTypeComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Expr: for<'a> From<&'a FieldComposer<LANG, SPEC>> {
    |composer|
        OwnedItemPresentableContext::Expression(Expression::Expr(Expr::from(composer)), composer.attrs.clone())
}

pub const fn enum_variant_composer_conversion_unit<LANG, SPEC>()
    -> OwnerIteratorConversionComposer<Comma, LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(aspect, _)|
        SequenceOutput::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            _ => aspect.clone(),
        })
}

pub const fn enum_variant_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |fields| field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::new(Name::UnnamedArg(index), FieldTypeKind::r#type(ty), false, SPEC::from_attrs(attrs.cfg_attributes())))
}

pub const fn struct_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |fields| field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::new(Name::UnnamedStructFieldsComp(ty.clone(), index), FieldTypeKind::r#type(ty), false, SPEC::from_attrs(attrs.cfg_attributes())))
}
pub const fn struct_named_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |fields| field_composers_iterator(
        fields,
        |_index, Field { ident, ty, attrs, .. }|
            FieldComposer::new(Name::Optional(ident.clone()), FieldTypeKind::r#type(ty), true, SPEC::from_attrs(attrs.cfg_attributes())))
}

pub const fn empty_fields_composer<LANG, SPEC>() -> FieldsComposerRef<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |_| Punctuated::new()
}

/// Enum composers
pub const fn enum_composer_object_output<LANG, SPEC>() -> ComposerPresenter<SequenceOutput<LANG, SPEC>, SequenceOutput<LANG, SPEC>>
    where
        LANG: Clone,
        SPEC: LangAttrSpecification<LANG> {
    |context| SequenceOutput::Enum(Box::new(context))
}
pub const fn enum_composer_object_context<C, LANG, SPEC>()
    -> SharedComposerLink<C, SequenceOutput<LANG, SPEC>>
    where
        // C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
        C: AttrComposable<SPEC>
            + NameContext<Context>
            + VariantComposable<LANG, SPEC>
            + 'static,
        // I: DelimiterTrait
        //     + ?Sized,
        LANG: Clone,
        SPEC: LangAttrSpecification<LANG> {
    |composer| SequenceOutput::Variants(C::target_name_aspect(composer), C::compose_attributes(composer), C::compose_variants(composer))
}
pub const fn enum_composer_object<C, LANG, SPEC>()
    -> OwnerIteratorPostProcessingComposer<ComposerLink<C>, LANG, SPEC>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: AttrComposable<SPEC> + NameContext<Context>
            + VariantComposable<LANG, SPEC>
            + NameContext<Context>
            + 'static,
          // I: DelimiterTrait
          //   + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    ContextComposer::new(
        enum_composer_object_output(),
        enum_composer_object_context::<C, LANG, SPEC>())
}
pub const fn composer_doc<C>() -> TypeContextComposer<ComposerLink<C>>
    where C: NameComposable<Context>
            + SourceAccessible
            + 'static {
    ContextComposer::new(
        |target_name| {
            let comment = format!("FFI-representation of the [`{}`]", target_name.to_token_stream());
            // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
            parse_quote! { #[doc = #comment] }
        },
        |composer: &ComposerRef<C>| composer.compose_target_name()
    )
}

pub const fn composer_ffi_binding<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, DestructorContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + NameContext<Context>
            + NameComposable<Context>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer| (
        composer.compose_ffi_name(),
        composer.compose_attributes(),
        composer.compose_generics(),
        PhantomData::default())
}
pub const fn composer_target_binding<C, LANG, SPEC, Gen>()
    -> SharedComposerLink<C, DestructorContext<LANG, SPEC, Gen>>
    // where C: BasicComposable<ComposerLink<C>, Context, LANG, SPEC, Option<Generics>>
    where C: AttrComposable<SPEC>
            + GenericsComposable<Gen>
            + NameContext<Context>
            + NameComposable<Context>
            + SourceAccessible
            + 'static,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |composer| (
        composer.compose_target_name(),
        composer.compose_attributes(),
        composer.compose_generics(),
        PhantomData::default())
}

pub const fn resolver_from_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        composer.name.clone(),
        ConversionType::From(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::FfiRefWithName(composer.name.clone()))))
}
pub const fn resolver_from_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        composer.name.clone(),
        ConversionType::From(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::DerefName(composer.name.clone()))))
}
pub const fn resolver_to_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        composer.name.clone(),
        ConversionType::To(ToConversionComposer::new(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::Name(composer.name.clone())))))
}

pub const fn resolver_to_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        composer.name.clone(),
        ConversionType::To(ToConversionComposer::new(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::ObjName(composer.name.clone())))))
}
pub const fn resolver_to_type_alias_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        Name::Empty,
        ConversionType::To(ToConversionComposer::new(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::Name(Name::Dictionary(DictionaryName::Obj))))))
}
pub const fn resolver_drop_enum_variant_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        composer.name.clone(),
        ConversionType::Destroy(DestroyConversionComposer::new(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::DerefName(composer.name.clone())))))
}
pub const fn resolver_drop_struct_field_statement<LANG, SPEC>()
    -> FieldPathResolver<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |composer | (
        Name::Empty,
        ConversionType::Destroy(DestroyConversionComposer::new(
            composer.name.clone(),
            composer.ty().clone(),
            Some(Expression::FfiRefWithName(composer.name.clone())))))
}

const fn struct_composer_from_iterator_post_processor<SEP, LANG, SPEC, Gen>()
    -> OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(((aspect, field_types), _generics), presenter)|
        (aspect, field_conversion_expressions_iterator((field_types, presenter), resolver_from_struct_field_statement()))
}
const fn struct_composer_to_iterator_post_processor<SEP, LANG, SPEC, Gen>()
    -> OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(((aspect, field_types), _generics), presenter)|
        (aspect, field_conversion_expressions_iterator((field_types, presenter), resolver_to_struct_field_statement()))
}
const fn type_alias_composer_to_iterator_post_processor<SEP, LANG, SPEC, Gen>()
    -> OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(((aspect, field_types), _generics), presenter)|
        (aspect, field_conversion_expressions_iterator((field_types, presenter), resolver_to_type_alias_statement()))
}
const fn enum_variant_composer_from_sequence_iterator_root<SEP, LANG, SPEC, Gen>()
    -> OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(((aspect, field_types), _generics), presenter)|
        (aspect, field_conversion_expressions_iterator((field_types, presenter), resolver_from_enum_variant_statement()))
}

const fn enum_variant_composer_to_sequence_iterator_root<SEP, LANG, SPEC, Gen>()
    -> OwnedIteratorPostProcessor<SEP, LANG, SPEC, Gen>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(((aspect, field_types), _generics), presenter)|
        (aspect, field_conversion_expressions_iterator((field_types, presenter), resolver_to_enum_variant_statement()))
}

const fn struct_composer_drop_fields_iterator<SEP, LANG, SPEC, Gen>()
    -> DropFieldsIterator<SEP, LANG, SPEC>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(field_types, presenter)|
        field_conversion_expressions_iterator((field_types, presenter), resolver_drop_struct_field_statement())
}
const fn enum_variant_composer_drop_fields_iterator<SEP, LANG, SPEC>()
    -> DropFieldsIterator<SEP, LANG, SPEC>
    where SEP: Default,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    |(field_types, presenter)|
        field_conversion_expressions_iterator((field_types, presenter), resolver_drop_enum_variant_statement())
}

const fn fields_composer_iterator_root<CTX, Item, OUT, LANG, SPEC, Gen>()
    -> ComposerPresenter<(LocallyOwnedFieldComposers<CTX, LANG, SPEC, Gen>, ComposerPresenterByRef<FieldComposer<LANG, SPEC>, Item>), (CTX, OUT)>
    where OUT: FromIterator<Item>,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    |(((aspect, field_composers), _generics), composer)|
        (aspect, field_conversions_iterator(field_composers, composer))
}



pub fn field_composers_iterator<MAP, LANG, SPEC>(
    fields: &CommaPunctuatedFields,
    mapper: MAP
) -> FieldComposers<LANG, SPEC>
    where MAP: Fn(usize, &Field) -> FieldComposer<LANG, SPEC>,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    CommaPunctuated::from_iter(fields.iter().enumerate().map(|(index, field)| mapper(index, field)))
}

pub fn field_conversions_iterator<MAP, Out, It, LANG, SPEC, SEP>(
    composers: ArgComposers<SEP, LANG, SPEC>,
    mapper: MAP
) -> It
    where MAP: Fn(&FieldComposer<LANG, SPEC>) -> Out,
          It: FromIterator<Out>,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    It::from_iter(composers.iter().map(mapper))
}

fn field_conversion_expressions_iterator<It, LANG, SPEC>(
    (composers, presenter): (FieldComposers<LANG, SPEC>, FieldTypePresentationContextPassRef<LANG, SPEC>),
    resolver: FieldPathResolver<LANG, SPEC>
) -> It
    where It: FromIterator<OwnedItemPresentableContext<LANG, SPEC>>,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    field_conversions_iterator(
        composers,
        |composer| {
            let template = resolver(composer);
            let expr = presenter(&template);
            OwnedItemPresentableContext::Expression(expr, composer.attrs.clone())
        })
}
