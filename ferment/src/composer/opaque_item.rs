use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Attribute, Generics};
use syn::token::Comma;
use ferment_macro::BasicComposerOwner;
use crate::ast::{DelimiterTrait, Depunctuated};
use crate::composable::{AttrsComposition, CfgAttributes};
use crate::composer::{BasicComposable, BasicComposer, BindingComposable, CommaPunctuatedFields, ComposerPresenter, constants, CtorSequenceComposer, DocsComposable, FFIBindingsComposer, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldTypesContext, MethodComposer, OwnedFieldTypeComposerRef, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorConversionComposer, ParentComposer, Linkable, SourceAccessible, SourceExpandable};
use crate::composer::constants::{BINDING_DTOR_COMPOSER, composer_target_binding, STRUCT_NAMED_FIELDS_COMPOSER, STRUCT_UNNAMED_FIELDS_COMPOSER};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::ToPath;
use crate::naming::Name;
use crate::presentable::{Context, SequenceOutput};
use crate::presentation::{BindingPresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, InterfacePresentation};
use crate::shared::SharedAccess;

#[derive(BasicComposerOwner)]
pub struct OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized + 'static {
    pub base: BasicComposer<ParentComposer<Self>>,
    pub fields_from_composer: FieldsOwnedSequenceComposer<ParentComposer<Self>>,
    pub fields_to_composer: FieldsOwnedSequenceComposer<ParentComposer<Self>>,
    pub bindings_composer: FFIBindingsComposer<ParentComposer<Self>, I>,
    pub field_types: FieldTypesContext,
}
impl<I> OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized {
    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> ParentComposer<Self> {
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
    ) -> ParentComposer<Self> {
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
        ctor_composer: CtorSequenceComposer<ParentComposer<Self>, I>,
        fields_composer: FieldsComposerRef) -> ParentComposer<Self> {
        Self::new::<ParentComposer<Self>>(
            Context::Struct { ident: target_name.clone(), attrs: attrs.cfg_attributes_expanded() },
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            fields,
            context,
            root_presenter,
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
        root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems, SequenceOutput>,
        field_presenter: OwnedFieldTypeComposerRef,
        ctor_composer: CtorSequenceComposer<ParentComposer<Self>, I>,
        fields_composer: FieldsComposerRef) -> ParentComposer<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, name_context, generics, constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: constants::fields_from_composer(root_presenter, field_presenter),
            fields_to_composer: constants::fields_to_composer(root_presenter, field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(
                    BINDING_DTOR_COMPOSER,
                    composer_target_binding()
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
                    constants::target_aspect_seq_context()),
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
                    constants::target_aspect_seq_context())
            ),
            field_types: fields_composer(fields),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &ParentComposer<Self>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
    }
}

impl<I> FieldsContext for OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn field_types_ref(&self) -> &FieldTypesContext {
        &self.field_types
    }
}
impl<I> FieldsConversionComposable for OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ParentComposer<Self>> {
        &self.fields_to_composer
    }
}
impl<I> DocsComposable for OpaqueItemComposer<I> where I: DelimiterTrait + ?Sized {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I> SourceExpandable for OpaqueItemComposer<I>
    where I: DelimiterTrait + ?Sized {
    fn expand(&self) -> Expansion {
        Expansion::Full {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
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
