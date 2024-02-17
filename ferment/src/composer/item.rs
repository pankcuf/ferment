use std::rc::Rc;
use std::cell::RefCell;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::composer::{AttrsComposer, Composer, ComposerAspect, ComposerPresenter, ConversionComposer, ConversionsComposer, DropComposer, FFIBindingsComposer, FFIContextComposer, FFIConversionComposer, FieldsComposer, NameComposer};
use crate::composer::method::MethodComposer;
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::interface::{DEFAULT_DOC_PRESENTER, DEREF_FIELD_PATH, FFI_DEREF_FIELD_NAME, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, IteratorPresenter, LAMBDA_CONVERSION_PRESENTER, MapPairPresenter, MapPresenter, obj, OBJ_FIELD_NAME, OwnerIteratorPresenter, ROOT_DESTROY_CONTEXT_COMPOSER, ScopeTreeFieldTypedPresenter, SIMPLE_COMPOSER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER};
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation};

pub const FFI_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    SIMPLE_COMPOSER,
    ItemComposer::fields_from);
pub const DESTROY_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    ROOT_DESTROY_CONTEXT_COMPOSER,
    EMPTY_CONTEXT_PRESENTER);
pub const DROP_STRUCT_COMPOSER: DropComposer = DropComposer::new(
    SIMPLE_CONVERSION_PRESENTER,
    EMPTY_CONTEXT_PRESENTER,
    |fields|
        IteratorPresentationContext::StructDestroy(fields),
    SIMPLE_PRESENTER,
    vec![]);
pub const DEFAULT_DOC_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    DEFAULT_DOC_PRESENTER,
    |composer| composer.target_name_composer.compose(&composer.context.borrow()));

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!(&*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!(obj);
pub const EMPTY_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!();

pub const FFI_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |composer|
    composer.ffi_name_composer.compose(&composer.context.borrow());
pub const TARGET_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> = |composer|
    composer.target_name_composer.compose(&composer.context.borrow());


pub struct ItemComposer {
    pub context: Rc<RefCell<ScopeContext>>,
    pub need_drop_presentation: bool,
    pub ffi_name_composer: NameComposer,
    pub target_name_composer: NameComposer,
    pub attrs_composer: AttrsComposer,
    pub ffi_conversions_composer: FFIConversionComposer,
    pub fields_from_composer: FieldsComposer,
    pub fields_to_composer: FieldsComposer,
    pub fields_get_composer: MethodComposer,
    pub fields_set_composer: MethodComposer,
    pub ffi_object_composer: FFIContextComposer,
    pub doc_composer: FFIContextComposer,
}


impl ItemComposer {

    pub(crate) fn type_alias_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        conversions_composer: ConversionsComposer
    ) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            |(name, fields)|
                OwnerIteratorPresentationContext::TypeAlias(name, fields),
            DEFAULT_DOC_COMPOSER,
            |field_type|
                OwnedItemPresenterContext::DefaultFieldType(field_type),
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    |(_, fields)|
                        OwnerIteratorPresentationContext::TypeAliasFromConversion(fields),
                    SIMPLE_CONVERSION_PRESENTER,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    TO_OBJ_CONTEXT_PRESENTER,
                    |(name, fields)|
                        OwnerIteratorPresentationContext::TypeAliasToConversion(name, fields),
                    SIMPLE_CONVERSION_PRESENTER,
                    ffi_name.to_token_stream(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                |_| quote!(ffi_ref.0),
                |_| obj(),
                FFIBindingsComposer::new(
                    |fields|
                        IteratorPresentationContext::Round(fields),
                    |field_type|
                        OwnedItemPresenterContext::BindingArg(field_type),
                    |field_type|
                        OwnedItemPresenterContext::BindingField(field_type)),
                FFI_DEREF_FIELD_NAME),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn struct_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: ScopeTreeFieldTypedPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        bindings_presenter: IteratorPresenter,
        bindings_arg_presenter: ScopeTreeFieldTypedPresenter,
        bindings_field_names_presenter: ScopeTreeFieldTypedPresenter,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
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
                    root_conversion_presenter,
                    conversion_presenter,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    EMPTY_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.to_token_stream(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                FFI_DEREF_FIELD_NAME,
                OBJ_FIELD_NAME,
                FFIBindingsComposer::new(
                    bindings_presenter,
                    bindings_arg_presenter,
                    bindings_field_names_presenter),
                FFI_DEREF_FIELD_NAME
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
        root_presenter: OwnerIteratorPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: ComposerPresenter<TokenStream2, TokenStream2>,
        destroy_presenter: MapPresenter,
        bindings_iterator_presenter: IteratorPresenter,
        bindings_arg_presenter: ScopeTreeFieldTypedPresenter,
        bindings_field_names_presenter: ScopeTreeFieldTypedPresenter,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            |field_type|
                OwnedItemPresenterContext::DefaultField(field_type),
            FFIContextComposer::new(
                |_| quote!(),
                EMPTY_CONTEXT_PRESENTER),
            FFIConversionComposer::new(
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    ItemComposer::fields_from,
                    root_conversion_presenter,
                    conversion_presenter,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    ItemComposer::fields_to,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.to_token_stream(),
                    vec![]),
                FFIContextComposer::new(
                    destroy_code_context_presenter,
                    ItemComposer::fields_from),
                DropComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    ItemComposer::fields_from,
                    |fields|
                        IteratorPresentationContext::DefaultDestroyFields(fields),
                    destroy_presenter,
                    vec![]),
                DEREF_FIELD_PATH,
                SIMPLE_PRESENTER,
                FFIBindingsComposer::new(
                    bindings_iterator_presenter,
                    bindings_arg_presenter,
                    bindings_field_names_presenter),
                |f| quote!(*#f)),
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorPresenter,
        doc_composer: FFIContextComposer,
        field_presenter: ScopeTreeFieldTypedPresenter,
        ffi_object_composer: FFIContextComposer,
        ffi_conversions_composer: FFIConversionComposer,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<ItemComposer>> where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: true,
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: FieldsComposer::new(
                root_presenter,
                FFI_NAME_CONTEXT_PRESENTER,
                field_presenter,
                vec![]),
            fields_to_composer: FieldsComposer::new(
                root_presenter,
                TARGET_NAME_CONTEXT_PRESENTER,
                field_presenter,
                vec![]),
            fields_get_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Getter {
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                |composer| composer.ffi_name_composer.compose(&composer.context.borrow())),
            fields_set_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Setter {
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                |composer| composer.ffi_name_composer.compose(&composer.context.borrow())),
            ffi_conversions_composer,
            ffi_object_composer,
            doc_composer,
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
            root_borrowed.setup_conversion(conversions_composer);
        }
        root
    }

    fn setup_composers(&mut self, root: &Rc<RefCell<ItemComposer>>) {
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

    fn setup_conversion(&mut self, conversions_composer: ConversionsComposer) {
        conversions_composer
            .compose(&self.context)
            .into_iter()
            .for_each(|field_type|
                self.add_conversion(field_type));
    }

    fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.ffi_conversions_composer.add_conversion(field_type.clone(), &self.context);
        self.fields_from_composer.add_conversion(field_type.clone());
        self.fields_to_composer.add_conversion(field_type.clone());
        self.fields_get_composer.add_conversion(field_type.clone());
        self.fields_set_composer.add_conversion(field_type);
    }

    // pub(crate) fn compose_attrs(&self) -> TokenStream2 {
    //     self.attrs_composer.compose(&self.context.borrow())
    // }

    pub(crate) fn fields_from(&self) -> TokenStream2 {
        self.fields_from_composer.compose(&self.context.borrow())
    }

    pub(crate) fn fields_to(&self) -> TokenStream2 {
        self.fields_to_composer.compose(&self.context.borrow())
    }

    pub(crate) fn fields_get(&self) -> Vec<BindingPresentation> {
        self.fields_get_composer.compose(&self.context.borrow())
    }
    pub(crate) fn fields_set(&self) -> Vec<BindingPresentation> {
        self.fields_set_composer.compose(&self.context.borrow())
    }

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
        println!("make_expansion: constructor: {}", quote!(#constructor_presentation));
        println!("make_expansion: destructor: {}", quote!(#destructor_presentation));
        let mut bindings = vec![constructor_presentation, destructor_presentation];
        bindings.extend(self.fields_get());
        bindings.extend(self.fields_set());
            // .iter()
            // .map(|field| BindingPresentation::Getter {
            //     field_name: field.present(&ctx), //::DefaultField
            //     obj_type: ffi_name.clone(),
            //     field_type: field.present(&ctx) //::DefaultFieldType
            // }));
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
