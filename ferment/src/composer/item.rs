use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use std::iter::{IntoIterator, Iterator};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Generics, parse_quote, Path};
use syn::token::Comma;
use syn::punctuated::Punctuated;
use crate::composer::{AttrsComposer, BindingAccessorContext, Composer, ComposerPresenter, constants, ConversionsComposer, CtorOwnedComposer, Depunctuated, DestructorContext, FFIAspect, FFIComposer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, HasParent, ItemParentComposer, LocalConversionContext, MethodComposer, NameComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerIteratorLocalContext, OwnerIteratorPostProcessingComposer, ParentComposer, SimpleItemParentContextComposer};
use crate::composer::constants::{BINDING_DTOR_COMPOSER, DEFAULT_DOC_COMPOSER, FFI_NAME_DTOR_COMPOSER, FFI_NAME_LOCAL_CONVERSION_COMPOSER, TARGET_NAME_LOCAL_CONVERSION_COMPOSER};
use crate::composer::parent_composer::IParentComposer;
use crate::composition::AttrsComposition;
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};

pub struct ItemComposer {
    pub context: ParentComposer<ScopeContext>,
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

    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer>,
    pub doc_composer: SimpleItemParentContextComposer,

    pub generics: Option<Generics>,
    pub field_types: FieldTypesContext,
}

impl ItemComposer {

    pub(crate) fn type_alias_composer(
        ffi_name: Path,
        target_name: Path,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        context: &ParentComposer<ScopeContext>,
        conversions_composer: ConversionsComposer
    ) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            generics,
            attrs,
            context,
            constants::type_alias_composer_root_presenter(),
            DEFAULT_DOC_COMPOSER,
            constants::type_alias_composer_field_presenter(),
            constants::struct_composer_object(),
            constants::type_alias_composer_ffi_conversions(),
            constants::struct_composer_ctor_unnamed(),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn struct_composer(
        ffi_name: Path,
        target_name: Path,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        field_presenter: OwnedFieldTypeComposerRef,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            generics,
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            field_presenter,
            constants::struct_composer_object(),
            constants::struct_composer_ffi_conversions(root_conversion_presenter, conversion_presenter),
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
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        destroy_code_context_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
        destroy_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        Self::new(
            ffi_name,
            target_name,
            None,
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            constants::enum_variant_composer_field_presenter(),
            constants::enum_variant_composer_object(),
            constants::enum_variant_composer_ffi_conversions(root_conversion_presenter, conversion_presenter, destroy_code_context_presenter, destroy_presenter),
            ctor_composer,
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: ComposerPresenter<OwnerIteratorLocalContext<Comma>, OwnerIteratorPresentationContext>,
        doc_composer: SimpleItemParentContextComposer,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer>,
        ffi_conversions_composer: FFIComposer<ItemParentComposer>,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        conversions_composer: ConversionsComposer) -> ItemParentComposer {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: constants::fields_composer(root_presenter, FFI_NAME_LOCAL_CONVERSION_COMPOSER, field_presenter),
            fields_to_composer: constants::fields_composer(root_presenter, TARGET_NAME_LOCAL_CONVERSION_COMPOSER, field_presenter),
            getter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Getter {
                        name: Name::Getter(parse_quote!(#root_obj_type), parse_quote!(#field_name)),
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                FFI_NAME_LOCAL_CONVERSION_COMPOSER),
            setter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Setter {
                        name: Name::Setter(parse_quote!(#root_obj_type), parse_quote!(#field_name)),
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                FFI_NAME_LOCAL_CONVERSION_COMPOSER),
            dtor_composer: MethodComposer::new(BINDING_DTOR_COMPOSER, FFI_NAME_DTOR_COMPOSER, ),
            ctor_composer,
            ffi_conversions_composer,
            ffi_object_composer,
            doc_composer,
            generics,
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
}

impl IParentComposer for ItemComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        &self.context
    }

    fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation> {
        self.attrs_composer.compose(&self.context().borrow())
    }

    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.context().borrow();
        let mut bindings = Punctuated::new();
        bindings.push(self.ctor_composer.compose(&()).present(&source));
        bindings.push(self.dtor_composer.compose(&source));
        bindings.extend(self.getter_composer.compose(&source));
        bindings.extend(self.setter_composer.compose(&source));
        bindings
    }

    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.doc_composer.compose(&()))
    }

    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.context().borrow()))
    }

    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            name: self.ffi_name_composer.compose(&()),
            body: self.compose_aspect(FFIAspect::Drop)
        }
    }

    fn compose_names(&self) -> (TokenStream2, TokenStream2) {
        (self.ffi_name_composer.compose(&()), self.target_name_composer.compose(&()))
    }

    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>) {
        (FromConversionPresentation::Struct(self.compose_aspect(FFIAspect::From)),
         ToConversionPresentation::Struct(self.compose_aspect(FFIAspect::To)),
         self.compose_aspect(FFIAspect::Destroy),
         self.generics.clone())
    }
}
