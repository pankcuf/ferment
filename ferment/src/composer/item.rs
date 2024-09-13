use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;
use std::vec;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Field, Generics, Type, Visibility, VisPublic};
use syn::token::{Comma, Pub};
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, GenModel};
use crate::composer::{BasicComposer, BindingComposable, CommaPunctuatedFields, Composer, ComposerPresenter, constants, ConversionComposable, CtorSequenceComposer, DocsComposable, FFIAspect, FFIBindingsComposer, FFIComposer, FFIObjectComposable, FieldsComposerRef, FieldsContext, FieldsConversionComposable, FieldsOwnedSequenceComposer, FieldTypePresentationContextPassRef, FieldComposers, Linkable, MethodComposer, OwnedFieldTypeComposerRef, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorConversionComposer, OwnerIteratorPostProcessingComposer, ComposerLink, SourceAccessible, NameComposable, ConstructorArgComposerRef, AttrComposable, GenericsComposable, SourceFermentable2, BasicComposerOwner, NameContext, ConstructorFieldsContext, SharedComposerLink};
use crate::context::ScopeContext;
use crate::presentable::{ScopeContextPresentable, Context, SequenceOutput, OwnedItemPresentableContext, BindingPresentableContext};
use crate::presentation::{DocPresentation, RustFermentate, FFIObjectPresentation, InterfacePresentation};
use crate::shared::SharedAccess;
use crate::ast::DelimiterTrait;
use crate::lang::{LangAttrSpecification, LangGenSpecification};


pub struct ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + 'static + ?Sized,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ComposerLink<Self>, LANG, SPEC>,
    pub ffi_conversions_composer: FFIComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub fields_from_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub fields_to_composer: FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub bindings_composer: FFIBindingsComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub field_types: FieldComposers<LANG, SPEC>,
    #[cfg(feature = "objc")]
    pub objc_composer: crate::lang::objc::composer::ItemComposer<ComposerLink<Self>>
}
impl<I, LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<I, LANG, SPEC, Gen> ItemComposer<I, LANG, SPEC, Gen>
    // where Self: BasicComposable<ComposerLink<Self>, Context, LANG, SPEC, Option<Generics>>,
    where Self: GenericsComposable<Gen>
            + AttrComposable<SPEC>
            + NameContext<Context>,
          I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn struct_composer_unnamed(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::round_braces_fields_presenter(),
            constants::bypass_field_context::<LANG, SPEC, Gen>(),
            constants::unnamed_struct_ctor_context_composer(),
            constants::struct_composer_ctor_unnamed_item(),
            constants::struct_unnamed_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::constants::OBJC_STRUCT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn struct_composer_named(
        target_name: &Ident,
        attrs: &Vec<Attribute>,
        generics: &Generics,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::struct_composer(
            target_name,
            generics,
            attrs,
            fields,
            context,
            constants::struct_composer_root_presenter_named(),
            constants::named_struct_field_composer(),
            constants::curly_braces_fields_presenter(),
            constants::struct_composer_conversion_named(),
            constants::named_struct_ctor_context_composer(),
            constants::struct_composer_ctor_named_item(),
            constants::struct_named_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::constants::OBJC_STRUCT_NAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unit(
        target_name: &Ident,
        variant_name: &Ident,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::enum_variant_composer(
            target_name,
            variant_name,
            attrs,
            fields,
            context,
            constants::enum_variant_composer_conversion_unit(),
            constants::enum_variant_composer_conversion_unit(),
            constants::bypass_field_context::<LANG, SPEC, Gen>(),
            constants::empty_field_context::<LANG, SPEC, Gen>(),
            constants::named_enum_variant_ctor_context_composer(),
            constants::struct_composer_ctor_named_item(),
            constants::empty_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::constants::OBJC_EMPTY_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_unnamed(
        target_name: &Ident,
        variant_name: &Ident,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::enum_variant_composer(
            target_name,
            variant_name,
            attrs,
            fields,
            context,
            constants::round_braces_fields_presenter(),
            constants::round_braces_fields_presenter(),
            constants::bypass_field_context::<LANG, SPEC, Gen>(),
            constants::terminated_field_context(),
            constants::unnamed_enum_variant_ctor_context_composer(),
            constants::struct_composer_ctor_unnamed_item(),
            constants::enum_variant_unnamed_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::constants::OBJC_ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER
        )
    }
    pub fn enum_variant_composer_named(
        target_name: &Ident,
        variant_name: &Ident,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::enum_variant_composer(
            target_name,
            variant_name,
            attrs,
            fields,
            context,
            constants::curly_braces_fields_presenter(),
            constants::curly_braces_fields_presenter(),
            constants::struct_composer_conversion_named(),
            constants::terminated_field_context(),
            constants::named_enum_variant_ctor_context_composer(),
            constants::struct_composer_ctor_named_item(),
            constants::struct_named_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::constants::OBJC_STRUCT_NAMED_FIELDS_COMPOSER
        )
    }
    pub(crate) fn type_alias_composer(
        target_name: &Ident,
        ty: &Type,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        context: &ComposerLink<ScopeContext>,
    ) -> ComposerLink<Self> {
        let fields = CommaPunctuated::from_iter([Field {
            vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
            ty: (*ty).clone(),
            attrs: vec![],
            ident: None,
            colon_token: None,
        }]);
        let name_context = Context::r#struct(target_name, attrs.cfg_attributes());
        Self::new::<ComposerLink<Self>>(
            name_context.clone(),
            Some(generics.clone()),
            AttrsModel::from(attrs),
            &fields,
            context,
            constants::struct_composer_root_presenter_unnamed(),
            constants::unnamed_struct_field_composer(),
            constants::struct_composer_object(),
            constants::type_alias_composer_ffi_conversions(),
            constants::struct_ctor_sequence_composer(
                constants::struct_composer_ctor_root(),
                constants::unnamed_struct_ctor_context_composer(),
                constants::struct_composer_ctor_unnamed_item()
            ),
            constants::struct_unnamed_fields_composer(),
            #[cfg(feature = "objc")]
            crate::lang::objc::composer::ItemComposer::new(
                name_context,
                &fields,
                crate::lang::objc::constants::OBJC_STRUCT_UNNAMED_FIELDS_COMPOSER
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn struct_composer(
        target_name: &Ident,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
        field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
        conversion_presenter: FieldTypePresentationContextPassRef<LANG, SPEC>,
        ctor_context_composer: SharedComposerLink<Self, ConstructorFieldsContext<LANG, SPEC, Gen>>,
        ctor_arg_composer: ConstructorArgComposerRef<LANG, SPEC>,
        fields_composer: FieldsComposerRef<LANG, SPEC>,
        #[cfg(feature = "objc")]
        objc_fields_composer: FieldsComposerRef<crate::lang::objc::ObjCFermentate, crate::lang::objc::composers::AttrWrapper>) -> ComposerLink<Self> {
        let presentation_context = Context::r#struct(target_name, attrs.cfg_attributes());
        Self::new::<ComposerLink<Self>>(
            presentation_context.clone(),
            Some(generics.clone()),
            AttrsModel::from(attrs),
            fields,
            context,
            root_presenter,
            field_presenter,
            constants::struct_composer_object(),
            constants::struct_ffi_composer(root_conversion_presenter, conversion_presenter),
            constants::struct_ctor_sequence_composer(
                constants::struct_composer_ctor_root(),
                ctor_context_composer,
                ctor_arg_composer
            ),
            fields_composer,
            #[cfg(feature = "objc")]
            crate::lang::objc::composer::ItemComposer::new(presentation_context, fields, objc_fields_composer),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn enum_variant_composer(
        target_name: &Ident,
        variant_name: &Ident,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
        root_presenter: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
        root_conversion_presenter: OwnerIteratorConversionComposer<Comma, LANG, SPEC>,
        conversion_presenter: FieldTypePresentationContextPassRef<LANG, SPEC>,
        destroy_presenter: FieldTypePresentationContextPassRef<LANG, SPEC>,
        ctor_context_composer: SharedComposerLink<Self, ConstructorFieldsContext<LANG, SPEC, Gen>>,
        ctor_arg_composer: ConstructorArgComposerRef<LANG, SPEC>,
        fields_composer: FieldsComposerRef<LANG, SPEC>,
        #[cfg(feature = "objc")]
        objc_fields_composer: FieldsComposerRef<crate::lang::objc::ObjCFermentate, crate::lang::objc::composers::AttrWrapper>,
    ) -> ComposerLink<Self> {
        let name_context = Context::EnumVariant { ident: target_name.clone(), variant_ident: variant_name.clone(), attrs: attrs.attrs.cfg_attributes() };
        Self::new::<ComposerLink<Self>>(
            name_context.clone(),
            None,
            attrs,
            fields,
            context,
            root_presenter,
            constants::enum_variant_composer_field_presenter(),
            constants::enum_variant_composer_object(),
            constants::enum_variant_composer_ffi_composer(
                root_conversion_presenter,
                conversion_presenter,
                constants::root_destroy_context_composer(),
                destroy_presenter),
            constants::struct_ctor_sequence_composer(
                constants::enum_variant_composer_ctor_root(),
                ctor_context_composer,
                ctor_arg_composer
            ),
            fields_composer,
            #[cfg(feature = "objc")]
            crate::lang::objc::composer::ItemComposer::new(name_context, fields, objc_fields_composer),
        )
    }

    #[allow(clippy::too_many_arguments, non_camel_case_types)]
    fn new<T: SharedAccess + 'static>(
        name_context: Context,
        generics: Option<Generics>,
        attrs: AttrsModel,
        fields: &CommaPunctuatedFields,
        context: &ComposerLink<ScopeContext>,
        root_presenter: ComposerPresenter<OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>, SequenceOutput<LANG, SPEC>>,
        field_presenter: OwnedFieldTypeComposerRef<LANG, SPEC>,
        ffi_object_composer: OwnerIteratorPostProcessingComposer<ComposerLink<Self>, LANG, SPEC>,
        ffi_conversions_composer: FFIComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
        ctor_composer: CtorSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
        fields_composer: FieldsComposerRef<LANG, SPEC>,
        #[cfg(feature = "objc")]
        objc_composer: crate::lang::objc::composer::ItemComposer<ComposerLink<Self>>
    ) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(attrs, name_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            fields_from_composer: constants::fields_from_composer(root_presenter, field_presenter),
            fields_to_composer: constants::fields_to_composer(root_presenter, field_presenter),
            bindings_composer: FFIBindingsComposer::new(
                ctor_composer,
                MethodComposer::new(constants::binding_dtor_composer(), constants::composer_ffi_binding::<Self, LANG, SPEC, Gen>()),
                MethodComposer::new(constants::binding_getter_composer(), constants::ffi_aspect_seq_context()),
                MethodComposer::new(constants::binding_setter_composer(), constants::ffi_aspect_seq_context()),
                true
            ),
            ffi_conversions_composer,
            ffi_object_composer,
            field_types: fields_composer(fields),
            #[cfg(feature = "objc")]
            objc_composer,
        }));
        {
            root.borrow_mut().setup_composers(&root);
        }
        root
    }
    fn setup_composers(&mut self, root: &ComposerLink<Self>) {
        self.base.link(root);
        self.fields_from_composer.link(root);
        self.fields_to_composer.link(root);
        self.bindings_composer.link(root);
        self.ffi_object_composer.link(root);
        self.ffi_conversions_composer.link(root);
        #[cfg(feature = "objc")]
        self.objc_composer.link(root);
    }

    pub(crate) fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput<LANG, SPEC> {
        self.ffi_conversions_composer.compose_aspect(aspect)
    }
    pub(crate) fn present_aspect(&self, aspect: FFIAspect) -> <SequenceOutput<LANG, SPEC> as ScopeContextPresentable>::Presentation {
        self.compose_aspect(aspect)
            .present(&self.source_ref())
    }
}

impl<I, LANG, SPEC, Gen> AttrComposable<SPEC> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}
impl<I, LANG, SPEC, Gen> GenericsComposable<Gen> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_generics(&self) -> Gen {
        self.base().compose_generics()
    }
}

impl<I, LANG, SPEC, Gen> FieldsContext<LANG, SPEC> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable{
    fn field_types_ref(&self) -> &FieldComposers<LANG, SPEC> {
        &self.field_types
    }
}

impl<I, LANG, SPEC, Gen> FieldsConversionComposable<LANG, SPEC, Gen> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn fields_from(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.fields_from_composer
    }

    fn fields_to(&self) -> &FieldsOwnedSequenceComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.fields_to_composer
    }
}

impl<I, LANG, SPEC, Gen> DocsComposable for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait
            + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        self.base.compose_docs()
    }
}

impl<I> ConversionComposable for ItemComposer<I, RustFermentate, Vec<Attribute>, Option<Generics>>
    // where Self: BasicComposable<ComposerLink<Self>, Context, RustFermentate, Vec<Attribute>, Option<Generics>>,
    where Self: GenericsComposable<Option<Generics>> + AttrComposable<Vec<Attribute>> + NameContext<Context>,
          I: DelimiterTrait + ?Sized {
    fn compose_conversions(&self) -> Depunctuated<InterfacePresentation> {
        let generics = self.compose_generics();
        let attrs = self.compose_attributes();
        let ffi_type = self.compose_ffi_name();
        let types = (ffi_type.clone(), self.compose_target_name());
        let from  = self.present_aspect(FFIAspect::From);
        Depunctuated::from_iter([
            InterfacePresentation::conversion_from(&attrs, &types, from, &generics),
            InterfacePresentation::conversion_to(&attrs, &types, self.present_aspect(FFIAspect::To), &generics),
            InterfacePresentation::conversion_destroy(&attrs, &types, self.present_aspect(FFIAspect::Destroy), &generics),
            InterfacePresentation::drop(&attrs, ffi_type, self.present_aspect(FFIAspect::Drop))
        ])
    }
}

impl<I, LANG, SPEC, Gen> FFIObjectComposable for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.source_ref())
            .to_token_stream())
    }
}

impl<I, LANG, SPEC, Gen> BindingComposable<LANG, SPEC, Gen> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC, Gen>> {
        self.bindings_composer.compose(&self.source_ref())
    }
}



// impl<I, LANG, SPEC> SourceFermentable for ItemComposer<I, LANG, SPEC>
//     where I: DelimiterTrait + ?Sized,
//           LANG: Clone,
//           SPEC: LangAttrSpecification<LANG>,
//           SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable,
// {
//     fn ferment(&self) -> Depunctuated<Fermentate> {
//         let mut fermentate = Depunctuated::new();
//         fermentate.push(Fermentate::Rust(RustFermentate::Item {
//             attrs: self.compose_attributes(),
//             comment: self.compose_docs(),
//             ffi_presentation: self.compose_object(),
//             conversions: self.compose_conversions(),
//             bindings: self.compose_bindings(),
//             traits: Depunctuated::new()
//         }));
//         #[cfg(feature = "objc")]
//         fermentate.extend(self.objc_composer.ferment(&self.context()));
//         fermentate
//     }
// }
impl<I, LANG, SPEC, Gen> SourceAccessible for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}
impl<I, LANG, SPEC, Gen> NameContext<Context> for ItemComposer<I, LANG, SPEC, Gen>
    where I: DelimiterTrait +?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}



impl<I> SourceFermentable2<RustFermentate> for ItemComposer<I, RustFermentate, Vec<Attribute>, Option<Generics>>
    where I: DelimiterTrait + ?Sized,
          // SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          // OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable,
          // Self: BindingComposable<I, LANG, SPEC>

{
    fn ferment(&self) -> Depunctuated<RustFermentate> {
        let mut fermentate = Depunctuated::new();
        fermentate.push(RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversions: self.compose_conversions(),
            bindings: self.compose_bindings().present(&self.source_ref()),
            traits: Depunctuated::new()
        });

        // fermentate.push(Fermentate::Rust(RustFermentate::Item {
        //     attrs: self.compose_attributes(),
        //     comment: self.compose_docs(),
        //     ffi_presentation: self.compose_object(),
        //     conversions: self.compose_conversions(),
        //     bindings: self.compose_bindings(),
        //     traits: Depunctuated::new()
        // }));
        // #[cfg(feature = "objc")]
        // fermentate.extend(self.objc_composer.ferment(&self.context()));
        fermentate
    }
}

