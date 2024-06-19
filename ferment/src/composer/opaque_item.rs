use std::cell::{Ref, RefCell};
use std::rc::Rc;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics};
use syn::token::Comma;
use crate::composer::basic::BasicComposer;
use crate::composer::{AttrsComposer, CommaPunctuatedFields, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, Composer, ComposerPresenter, constants, ConstructorComposer, Depunctuated, FieldsComposer, FieldsOwnedComposer, FieldTypesContext, MethodComposer, OpaqueItemParentComposer, OwnedFieldTypeComposerRef, OwnerIteratorConversionComposer, ParentComposer, TypeContextComposer, VariantIteratorLocalContext};
use crate::composer::composable::{BasicComposable, BindingComposable, NameContext, SourceAccessible, SourceExpandable};
use crate::composer::constants::{BINDING_DTOR_COMPOSER, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::composer::ffi_bindings::FFIBindingsComposer;
use crate::composer::generics_composer::GenericsComposer;
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, InterfacePresentation, ScopeContextPresentable};
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::context::{name, OwnerIteratorPresentationContext};
use crate::shared::{ParentLinker, SharedAccess};
use crate::wrapped::DelimiterTrait;

pub struct OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized + 'static {
    pub base: BasicComposer<OpaqueItemParentComposer<I>>,
    pub fields_from_composer: FieldsOwnedComposer<OpaqueItemParentComposer<I>>,
    pub fields_to_composer: FieldsOwnedComposer<OpaqueItemParentComposer<I>>,
    pub bindings_composer: FFIBindingsComposer<OpaqueItemParentComposer<I>, I>,
    pub fields_composer: FieldsComposer,
    pub field_types: FieldTypesContext,
}
impl<I> OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> OpaqueItemParentComposer<I> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::opaque_struct_composer_ctor_unnamed(),
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
    ) -> OpaqueItemParentComposer<I> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            scope,
            context,
            constants::struct_composer_root_presenter_named(),
            constants::named_struct_field_composer(),
            constants::opaque_struct_composer_ctor_named(),
            STRUCT_NAMED_FIELDS_COMPOSER
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
        ctor_composer: ConstructorComposer<OpaqueItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> OpaqueItemParentComposer<I> {
        Self::new::<OpaqueItemParentComposer<I>>(
            Context::Struct { ident: target_name.clone(), attrs: attrs.cfg_attributes_expanded() },
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            fields,
            context,
            root_presenter,
            constants::opaque_item_composer_doc(),
            field_presenter,
            ctor_composer,
            fields_composer
        )
    }
    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new<T: SharedAccess + 'static>(
        name_context: Context,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        fields: &CommaPunctuatedFields,
        context: &ParentComposer<ScopeContext>,
        root_presenter: ComposerPresenter<VariantIteratorLocalContext, OwnerIteratorPresentationContext>,
        doc_composer: TypeContextComposer<OpaqueItemParentComposer<I>>,
        field_presenter: OwnedFieldTypeComposerRef,
        ctor_composer: ConstructorComposer<OpaqueItemParentComposer<I>, CommaPunctuatedOwnedItems, CommaPunctuatedTokens, I>,
        fields_composer: FieldsComposer) -> OpaqueItemParentComposer<I> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(name_context),
                GenericsComposer::new(generics),
                Rc::clone(context)
            ),
            fields_from_composer: constants::fields_composer::<OpaqueItemParentComposer<I>>(
                root_presenter,
                |composer| ((Aspect::FFI(composer.base.name_context()), composer.field_types.clone()), composer.base.generics.compose(composer.context())),
                field_presenter),
            fields_to_composer: constants::fields_composer::<OpaqueItemParentComposer<I>>(
                root_presenter,
                |composer| ((Aspect::Target(composer.base.name_context()), composer.field_types.clone()), composer.base.generics.compose(composer.context())),
                field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(
                    BINDING_DTOR_COMPOSER,
                    |composer: &Ref<OpaqueItemComposer<I>>| (
                        Aspect::Target(composer.base.name_context()).present(&composer.source_ref()),
                        composer.compose_attributes().to_token_stream(),
                        composer.base.generics.compose(composer.context()),
                    )
                ),
                MethodComposer::new(
                    |(root_obj_type, field_name, field_type, attrs, generics)|
                        BindingPresentation::GetterOpaque {
                            attrs,
                            name: Name::Getter(root_obj_type.to_path(), field_name.clone()),
                            field_name,
                            obj_type: root_obj_type,
                            field_type,
                            generics,
                        },
                    |composer: &Ref<OpaqueItemComposer<I>>| (
                        (composer.base.target_name_aspect(), composer.field_types.iter().map(|field_type| field_type.clone()).collect()),
                        composer.base.generics.compose(composer.context()))),
                MethodComposer::new(
                    |(root_obj_type, field_name, field_type, attrs, generics)|
                        BindingPresentation::SetterOpaque {
                            attrs,
                            name: Name::Setter(root_obj_type.to_path(), field_name.clone()),
                            field_name,
                            obj_type: root_obj_type,
                            field_type,
                            generics
                        },
                    |composer: &Ref<OpaqueItemComposer<I>>| ((composer.base.target_name_aspect(), composer.field_types.clone()), composer.base.generics.compose(composer.context())))
            ),
            fields_composer,
            field_types: fields_composer(fields),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &OpaqueItemParentComposer<I>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
    }

}
impl<I> NameContext for OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}

impl<I> BasicComposable<OpaqueItemParentComposer<I>> for OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }

    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I> SourceAccessible for OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
}

impl<I> SourceExpandable for OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.base.compose_docs(),
            ffi_presentation: FFIObjectPresentation::Empty,
            conversion: InterfacePresentation::Empty,
            drop: DropInterfacePresentation::Empty,
            bindings: self.compose_bindings(),
            traits: Depunctuated::new()
        }
    }
}
impl<I> BindingComposable for OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation> {
        self.bindings_composer.compose_bindings(&self.source_ref(), false)
    }
}
