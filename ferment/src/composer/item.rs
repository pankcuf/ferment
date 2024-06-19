use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::clone::Clone;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Field, Fields, Generics, Type, Visibility, VisPublic};
use syn::token::{Brace, Comma, Paren, Pub};
use syn::punctuated::Punctuated;
use crate::composer::{AttrsComposer, Composer, ComposerPresenter, constants, ConstructorComposer, Depunctuated, FFIAspect, FFIComposer, FieldsComposer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ParentLinker, ItemParentComposer, MethodComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerIteratorPostProcessingComposer, ParentComposer, TypeContextComposer, VariantIteratorLocalContext, CommaPunctuatedTokens, CommaPunctuatedOwnedItems, CommaPunctuatedFields};
use crate::composer::basic::BasicComposer;
use crate::composer::ffi_bindings::FFIBindingsComposer;
use crate::composer::constants::{BINDING_DTOR_COMPOSER, composer_ctor, default_ctor_context_composer, EMPTY_FIELDS_COMPOSER, ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, struct_named_root, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::composer::composable::{BasicComposable, BindingComposable, ConversionComposable, DropComposable, SourceExpandable, FFIObjectComposable, NameContext, SourceAccessible};
use crate::composer::generics_composer::GenericsComposer;
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentation::context::{FieldContext, name, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::destroy_presentation::DestroyPresentation;
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

    // pub fn struct_composer() -> ItemComposerWrapper {
    //     ma
    // }

    // pub fn base(&self) -> BasicComposer<ItemParentComposer<S, SP, I>> {
    //     match self {
    //         ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow()
    //         ItemComposerWrapper::EnumVariantUnnamed(_) => {}
    //         ItemComposerWrapper::EnumVariantUnit(_) => {}
    //         ItemComposerWrapper::StructNamed(_) => {}
    //         ItemComposerWrapper::StructUnnamed(_) => {}
    //     }
    // }

    // pub base: BasicComposer<ItemParentComposer<S, SP, I>>,
    // pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer<S, SP, I>>,
    // pub ffi_conversions_composer: FFIComposer<ItemParentComposer<S, SP, I>>,
    // pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer<S, SP, I>>,
    // pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer<S, SP, I>>,
    // pub bindings_composer: FFIBindingsComposer<ItemParentComposer<S, SP, I>, S, SP, I>,

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
//
// impl SourceExpandable for ItemComposerWrapper {
//     fn expand(&self) -> Expansion {
//         match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().expand(),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().expand(),
//             ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().expand(),
//             ItemComposerWrapper::StructNamed(composer) => composer.borrow().expand(),
//             ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().expand(),
//         }
//     }
// }
//
// impl SourceAccessible for ItemComposerWrapper {
//     fn context(&self) -> &ParentComposer<ScopeContext> {
//         match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().context(),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().context(),
//             ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().context(),
//             ItemComposerWrapper::StructNamed(composer) => composer.borrow().context(),
//             ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().context(),
//         }
//     }
// }
//
// impl NameContext for ItemComposerWrapper {
//     fn name_context_ref(&self) -> &Context {
//         match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().name_context_ref(),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().name_context_ref(),
//             ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().name_context_ref(),
//             ItemComposerWrapper::StructNamed(composer) => composer.borrow().name_context_ref(),
//             ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().name_context_ref(),
//         }
//     }
// }
//
// impl<Parent> BasicComposable<Parent> for ItemComposerWrapper where Parent: SharedAccess {
//     fn compose_attributes(&self) -> Depunctuated<Expansion> {
//         match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => {
//                 let composer = composer.borrow();
//                 composer.base.attr.compose(composer.context())
//             },
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => {
//                 let composer = composer.borrow();
//                 composer.base.attr.compose(composer.context())
//             },
//             ItemComposerWrapper::EnumVariantUnit(composer) => {
//                 let composer = composer.borrow();
//                 composer.base.attr.compose(composer.context())
//             },
//             ItemComposerWrapper::StructNamed(composer) => {
//                 let composer = composer.borrow();
//                 composer.base.attr.compose(composer.context())
//             },
//             ItemComposerWrapper::StructUnnamed(composer) => {
//                 let composer = composer.borrow();
//                 composer.base.attr.compose(composer.context())
//             },
//         }
//     }
//     fn compose_docs(&self) -> DocPresentation {
//         DocPresentation::Direct(match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().base.doc.compose(&()),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().base.doc.compose(&()),
//             ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().base.doc.compose(&()),
//             ItemComposerWrapper::StructNamed(composer) => composer.borrow().base.doc.compose(&()),
//             ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().base.doc.compose(&()),
//         })
//
//     }
// }


// impl<S, SP, I> Borrow<ItemComposerWrapper> for ItemComposerWrapper
//     where S: ScopeContextPresentable<Presentation = SP>, SP: ToTokens, I: DelimiterTrait {
//     fn borrow(&self) -> &ItemComposer<S, SP, I> {
//         match self {
//             ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow(),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow(),
//             ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow(),
//             ItemComposerWrapper::StructNamed(composer) => composer.borrow(),
//             ItemComposerWrapper::StructUnnamed(composer) => composer.borrow(),
//         }
//     }
// }

pub struct ItemComposer<I>
    where
        I: DelimiterTrait + 'static + ?Sized {
    pub base: BasicComposer<ItemParentComposer<I>>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>,
    pub ffi_conversions_composer: FFIComposer<ItemParentComposer<I>>,
    pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer<I>>,
    pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer<I>>,
    pub bindings_composer: FFIBindingsComposer<ItemParentComposer<I>, I>,

    // pub getter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    // pub setter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    // pub ctor_composer: ConstructorComposer<ItemParentComposer>,
    // pub dtor_composer: MethodComposer<ItemParentComposer, DestructorContext, DestructorContext>,

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
            composer_ctor(
                default_ctor_context_composer(),
                struct_named_root(),
                constants::struct_composer_ctor_named_item()),
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
            constants::enum_variant_composer_ctor_unit::<I>(),
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
            constants::type_alias_composer_root_presenter(),
            constants::item_composer_doc(),
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
            constants::item_composer_doc(),
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
        destroy_code_context_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
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
            constants::item_composer_doc(),
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
        root_presenter: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
        doc_composer: TypeContextComposer<ItemParentComposer<I>>,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer<I>>,
        ffi_conversions_composer: FFIComposer<ItemParentComposer<I>>,
        ctor_composer: ConstructorComposer<ItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> ItemParentComposer<I> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(name_context),
                GenericsComposer::new(generics),
                Rc::clone(context)
            ),
            fields_from_composer: constants::fields_composer::<ItemParentComposer<I>>(
                root_presenter,
                |composer| ((Aspect::FFI(composer.base.name_context()), composer.field_types.clone()), composer.base.generics.compose(composer.context())),
                field_presenter),
            fields_to_composer: constants::fields_composer::<ItemParentComposer<I>>(
                root_presenter,
                |composer: &Ref<Self>| ((Aspect::Target(composer.base.name_context()), composer.field_types.clone()), composer.base.generics.compose(composer.context())),
                field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(
                    BINDING_DTOR_COMPOSER,
                    |composer: &Ref<ItemComposer<I>>| (
                        Aspect::FFI(composer.base.name_context()).present(&composer.source_ref()),
                        composer.compose_attributes().to_token_stream(),
                        composer.base.generics.compose(composer.context())
                    )
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
impl<I> BasicComposable<ItemParentComposer<I>> for ItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
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
            self.base.generics.compose(self.context())
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

