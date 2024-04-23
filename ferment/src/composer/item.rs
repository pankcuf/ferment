use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::clone::Clone;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Field, Generics, Type, Visibility, VisPublic};
use syn::token::{Comma, Pub};
use syn::punctuated::Punctuated;
use crate::composer::{AttrsComposer, Composer, BindingAccessorContext, BYPASS_FIELD_CONTEXT, ComposerPresenter, constants, ConstructorComposer, Depunctuated, DestructorContext, FFIAspect, FFIComposer, FieldsComposer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, ParentLinker, ItemParentComposer, LocalConversionContext, MethodComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerIteratorPostProcessingComposer, ParentComposer, TypeContextComposer, VariantIteratorLocalContext};
use crate::composer::basic::BasicComposer;
use crate::composer::constants::{BINDING_DTOR_COMPOSER, EMPTY_FIELDS_COMPOSER, ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER, FFI_ASPECT_SEQ_CONTEXT};
use crate::composer::composable::{BasicComposable, BindingComposable, ConversionComposable, DropComposable, SourceExpandable, FFIObjectComposable, NameContext};
use crate::composer::r#type::TypeComposer;
use crate::composition::AttrsComposition;
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentation::context::{FieldTypePresentableContext, name, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::shared::SharedAccess;

pub struct ItemComposer {
    pub base: BasicComposer<ItemParentComposer>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer>,
    pub ffi_conversions_composer: FFIComposer<ItemParentComposer>,
    pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer>,
    pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer>,
    pub getter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    pub setter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    pub ctor_composer: ConstructorComposer<ItemParentComposer>,
    pub dtor_composer: MethodComposer<ItemParentComposer, DestructorContext, DestructorContext>,
    pub fields_composer: FieldsComposer,
    pub field_types: FieldTypesContext,
}

impl ItemComposer {

    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &Punctuated<Field, Comma>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::struct_composer_field_presenter_unnamed(),
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            BYPASS_FIELD_CONTEXT,
            constants::struct_composer_ctor_unnamed(),
            STRUCT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn struct_composer_named(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &Punctuated<Field, Comma>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_named(),
            constants::struct_composer_field_presenter_named(),
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::struct_composer_conversion_named(),
            constants::struct_composer_ctor_named(),
            STRUCT_NAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unit(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::enum_variant_composer_conversion_unit(),
            constants::enum_variant_composer_conversion_unit(),
            BYPASS_FIELD_CONTEXT,
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            |_| FieldTypePresentableContext::Empty,
            constants::enum_variant_composer_ctor_unit(),
            EMPTY_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unnamed(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            constants::ROUND_BRACES_FIELDS_PRESENTER,
            BYPASS_FIELD_CONTEXT,
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            BYPASS_FIELD_CONTEXT,
            constants::enum_variant_composer_ctor_unnamed(),
            ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_named(
        name_context: Context,
        attrs: AttrsComposition,
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>
    ) -> ItemParentComposer {
        Self::enum_variant_composer(
            name_context,
            attrs,
            fields,
            context,
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::CURLY_BRACES_FIELDS_PRESENTER,
            constants::struct_composer_conversion_named(),
            constants::ROOT_DESTROY_CONTEXT_COMPOSER,
            BYPASS_FIELD_CONTEXT,
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
    ) -> ItemParentComposer {
        Self::new::<ItemParentComposer>(
            Context::Struct { ident: target_name.clone() },
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
            constants::struct_composer_field_presenter_unnamed(),
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
        fields: &Punctuated<Field, Comma>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        field_presenter: OwnedFieldTypeComposerRef,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: ConstructorComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer {
        Self::new::<ItemParentComposer>(
            Context::Struct { ident: target_name.clone() },
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
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma>,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma>,
        conversion_presenter: FieldTypePresentationContextPassRef,
        destroy_code_context_presenter: ComposerPresenter<OwnerIteratorPresentationContext, OwnerIteratorPresentationContext>,
        destroy_presenter: FieldTypePresentationContextPassRef,
        ctor_composer: ConstructorComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer {
        Self::new::<ItemParentComposer>(
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
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>,
        root_presenter: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
        doc_composer: TypeContextComposer<ItemParentComposer>,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer>,
        ffi_conversions_composer: FFIComposer<ItemParentComposer>,
        ctor_composer: ConstructorComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer where  {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(name_context),
                generics,
                Rc::clone(context)
            ),
            fields_from_composer: constants::fields_composer(
                root_presenter,
                |composer| (Aspect::FFI(composer.base.name_context()), composer.field_types.clone()),
                field_presenter),
            fields_to_composer: constants::fields_composer(
                root_presenter,
                |composer| (Aspect::Target(composer.base.name_context()), composer.field_types.clone()),
                field_presenter),
            getter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Getter {
                        name: Name::Getter(root_obj_type.to_path(), field_name.clone()),
                        field_name,
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                FFI_ASPECT_SEQ_CONTEXT),
            setter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Setter {
                        name: Name::Setter(root_obj_type.to_path(), field_name.clone()),
                        field_name,
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                FFI_ASPECT_SEQ_CONTEXT),
            dtor_composer: MethodComposer::new(BINDING_DTOR_COMPOSER, |composer: &Ref<ItemComposer>| Aspect::FFI(composer.base.name_context()).present(&composer.source_ref())),
            ctor_composer,
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
    fn setup_composers(&mut self, root: &ItemParentComposer) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.ctor_composer.link(root);
        self.dtor_composer.link(root);
        self.getter_composer.link(root);
        self.setter_composer.link(root);
        self.ffi_object_composer.link(root);
        self.ffi_conversions_composer.link(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> TokenStream2 {
        self.ffi_conversions_composer.compose_aspect(aspect, &self.source_ref())
    }
}

impl SourceExpandable for ItemComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }

    fn expand(&self) -> Expansion {
        Expansion::Full {
            comment: self.base.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionComposable::<ItemParentComposer>::compose_conversion(self),
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            traits: BasicComposable::<ItemParentComposer>::compose_attributes(self)
        }
    }
}

impl DropComposable for ItemComposer {
    fn compose_drop(&self) -> DropInterfacePresentation {
        DropInterfacePresentation::Full {
            ty: self.base.ffi_name_aspect().present(&self.source_ref()),
            body: self.compose_aspect(FFIAspect::Drop)
        }
    }
}
impl BasicComposable<ItemParentComposer> for ItemComposer {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }

    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}
impl NameContext for ItemComposer {
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}


impl<Parent> ConversionComposable<Parent> for ItemComposer where Parent: SharedAccess {
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, DestroyPresentation, Option<Generics>) {
        (
            FromConversionPresentation::Just(self.compose_aspect(FFIAspect::From)),
            ToConversionPresentation::Struct(self.compose_aspect(FFIAspect::To)),
            DestroyPresentation::Custom(self.compose_aspect(FFIAspect::Destroy)),
            self.base.generics.clone()
        )
    }
}

impl FFIObjectComposable for ItemComposer {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(
            self.ffi_object_composer
                .compose(&())
                .present(&self.context().borrow()))
    }
}

impl BindingComposable for ItemComposer {

    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        let source = self.context().borrow();
        let mut bindings = Punctuated::new();
        bindings.push(self.ctor_composer.compose(&()).present(&source));
        bindings.push(self.dtor_composer.compose(&source));
        bindings.extend(self.getter_composer.compose(&source));
        bindings.extend(self.setter_composer.compose(&source));
        bindings
    }
}

