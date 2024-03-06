use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::clone::Clone;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Field, Generics, parse_quote, Type, Visibility, VisPublic};
use syn::token::{Comma, Pub};
use syn::punctuated::Punctuated;
use crate::composer::{AttrsComposer, BindingAccessorContext, BYPASS_FIELD_CONTEXT, Composer, ComposerPresenter, constants, CtorOwnedComposer, Depunctuated, DestructorContext, FFIAspect, FFIComposer, FieldsComposer, FieldsOwnedComposer, FieldTypePresentationContextPassRef, FieldTypesContext, HasParent, ItemParentComposer, LocalConversionContext, MethodComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, OwnerAspectIteratorLocalContext, OwnerIteratorPostProcessingComposer, ParentComposer, TypeContextComposer};
use crate::composer::constants::{BINDING_DTOR_COMPOSER, EMPTY_FIELDS_COMPOSER, ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::composer::composable::Composable;
use crate::composer::r#type::TypeComposer;
use crate::composition::AttrsComposition;
use crate::context::{ScopeChain, ScopeContext};
use crate::naming::Name;
use crate::presentation::context::{FieldTypePresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::presentation::context::name::{Aspect, Context};

pub struct ItemComposer {
    pub context: ParentComposer<ScopeContext>,
    pub attrs_composer: AttrsComposer<ItemParentComposer>,
    pub doc_composer: TypeContextComposer<ItemParentComposer>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ParentComposer<ItemComposer>>,
    pub type_composer: TypeComposer<ParentComposer<ItemComposer>>,
    pub generics: Option<Generics>,

    pub ffi_conversions_composer: FFIComposer<ItemParentComposer>,
    pub fields_from_composer: FieldsOwnedComposer<ItemParentComposer>,
    pub fields_to_composer: FieldsOwnedComposer<ItemParentComposer>,
    pub getter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    pub setter_composer: MethodComposer<ItemParentComposer, BindingAccessorContext, LocalConversionContext>,
    pub ctor_composer: CtorOwnedComposer<ItemParentComposer>,
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
            |(field_path, field_context)|
                FieldTypePresentableContext::Named((field_path.clone(), Box::new(field_context.clone()))),
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
        Self::new(
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
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer {
        Self::new(
            Context::Struct { ident: target_name.clone() },
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            fields,
            context,
            root_presenter,
            constants::item_composer_doc(),
            field_presenter,
            constants::struct_composer_object(),
            constants::struct_composer_ffi_conversions(root_conversion_presenter, conversion_presenter),
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
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer {
        Self::new(
            name_context,
            None,
            attrs,
            fields,
            context,
            root_presenter,
            constants::item_composer_doc(),
            constants::enum_variant_composer_field_presenter(),
            constants::enum_variant_composer_object(),
            constants::enum_variant_composer_ffi_conversions(root_conversion_presenter, conversion_presenter, destroy_code_context_presenter, destroy_presenter),
            ctor_composer,
            fields_composer)
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new(
        name_context: Context,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        fields: &Punctuated<Field, Comma>,
        context: &ParentComposer<ScopeContext>,
        root_presenter: ComposerPresenter<OwnerAspectIteratorLocalContext<Comma>, OwnerIteratorPresentationContext>,
        doc_composer: TypeContextComposer<ItemParentComposer>,
        field_presenter: OwnedFieldTypeComposerRef,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ItemParentComposer>,
        ffi_conversions_composer: FFIComposer<ItemParentComposer>,
        ctor_composer: CtorOwnedComposer<ItemParentComposer>,
        fields_composer: FieldsComposer) -> ItemParentComposer {
        let root = Rc::new(RefCell::new(Self {
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            type_composer: TypeComposer::new(name_context),
            fields_from_composer: constants::fields_composer(
                root_presenter,
                |composer| (Aspect::FFI(composer.name_context()), composer.field_types.clone()),
                field_presenter),
            fields_to_composer: constants::fields_composer(
                root_presenter,
                |composer| (Aspect::Target(composer.name_context()), composer.field_types.clone()),
                field_presenter),
            getter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Getter {
                        name: Name::Getter(parse_quote!(#root_obj_type), parse_quote!(#field_name)),
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                |composer: &Ref<ItemComposer>| (Aspect::FFI(composer.name_context()), composer.field_types.clone())),
            setter_composer: MethodComposer::new(
                |(root_obj_type, field_name, field_type)|
                    BindingPresentation::Setter {
                        name: Name::Setter(parse_quote!(#root_obj_type), parse_quote!(#field_name)),
                        field_name: field_name.to_token_stream(),
                        obj_type: root_obj_type.to_token_stream(),
                        field_type: field_type.to_token_stream()
                    },
                |composer: &Ref<ItemComposer>| (Aspect::FFI(composer.name_context()), composer.field_types.clone())),
            dtor_composer: MethodComposer::new(BINDING_DTOR_COMPOSER, |composer: &Ref<ItemComposer>|
                Aspect::FFI(composer.name_context())
                    .present(&composer.as_source_ref())),
            ctor_composer,
            ffi_conversions_composer,
            ffi_object_composer,
            doc_composer,
            generics,
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
        self.attrs_composer.set_parent(root);
        self.type_composer.set_parent(root);
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
        self.ffi_conversions_composer.compose_aspect(aspect, &self.as_source_ref())
    }
}

impl Composable for ItemComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        &self.context
    }

    fn name_context_ref(&self) -> &Context {
        &self.type_composer.context
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
        let source = self.as_source_ref();
        let ty = self.ffi_name_aspect().present(&source);
        DropInterfacePresentation::Full {
            ty,
            body: self.compose_aspect(FFIAspect::Drop)
        }
    }

    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>) {
        (FromConversionPresentation::Just(self.compose_aspect(FFIAspect::From)),
         ToConversionPresentation::Struct(self.compose_aspect(FFIAspect::To)),
         self.compose_aspect(FFIAspect::Destroy),
         self.generics.clone())
    }
}

