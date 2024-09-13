use syn::{Attribute, Field, Fields, FieldsNamed, FieldsUnnamed, Generics, ItemEnum, Variant, Visibility};
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use quote::quote;
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedOwnedItems, Composer, constants, ConversionComposable, DocsComposable, FFIAspect, FFIObjectComposable, GenericsComposable, ItemComposerWrapper, Linkable, NameComposable, OwnerAspectWithCommaPunctuatedItems, OwnerIteratorPostProcessingComposer, ComposerLink, SourceAccessible, SourceFermentable2, VariantComposable, VariantComposerRef, NameContext};
use crate::context::ScopeContext;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Aspect, BindingPresentableContext, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, DocPresentation, RustFermentate, FFIObjectPresentation, InterfacePresentation, Name};

// #[derive(BasicComposerOwner)]
pub struct EnumComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    pub ffi_object_composer: OwnerIteratorPostProcessingComposer<ComposerLink<Self>, LANG, SPEC>,
    pub variant_composers: Vec<ItemComposerWrapper<LANG, SPEC, Gen>>,
    pub variant_presenters: Vec<(VariantComposerRef<LANG, SPEC>, OwnerAspectWithCommaPunctuatedItems<LANG, SPEC>)>,
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}


impl<LANG, SPEC, Gen> EnumComposer<LANG, SPEC, Gen>
    where //Self: BasicComposable<ComposerLink<Self>, Context, LANG, SPEC, Option<Generics>>,
          // I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable,
          Self: GenericsComposable<Gen> {
    pub fn new(item_enum: &ItemEnum, context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
        let ItemEnum { attrs, ident: target_name, variants, generics, .. } = item_enum;
        let variant_composers = variants.iter()
            .map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
                let (variant_composer, fields_context): (VariantComposerRef<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = match discriminant {
                    Some((_, expr)) => (
                        |local_context|
                            SequenceOutput::EnumUnitFields(local_context.clone()),
                        CommaPunctuated::from_iter([OwnedItemPresentableContext::Expression(Expression::Expr(expr.clone()), SPEC::from_attrs(attrs.cfg_attributes()))])
                    ),
                    None => match fields {
                        Fields::Unit => (
                            |(aspect, _)|
                                SequenceOutput::NoFields(aspect.clone()),
                            CommaPunctuated::new()
                        ),
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                            |local_context|
                                SequenceOutput::RoundVariantFields(local_context.clone()),
                            CommaPunctuated::from_iter(unnamed
                                .iter()
                                .map(|Field { attrs, ty, .. }|
                                    OwnedItemPresentableContext::DefaultFieldType(ty.clone(), SPEC::from_attrs(attrs.cfg_attributes())))),
                        ),
                        Fields::Named(FieldsNamed { named, .. }) => (
                            |local_context|
                                SequenceOutput::CurlyVariantFields(local_context.clone()),
                            CommaPunctuated::from_iter(named
                                .iter()
                                .map(|Field { ident, attrs, ty, .. }|
                                    OwnedItemPresentableContext::Named(FieldComposer::new(Name::Optional(ident.clone()), FieldTypeKind::r#type(ty), true, SPEC::from_attrs(attrs.cfg_attributes())), Visibility::Inherited))),
                        ),
                    },
                };
                (
                    ItemComposerWrapper::variant(fields, target_name, variant_name, attrs, context),
                    (
                        variant_composer,
                        (
                            Aspect::FFI(Context::variant(target_name, variant_name, attrs.cfg_attributes())),
                            fields_context
                        )
                    )
                )
            }).unzip();
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(
                AttrsModel::from(attrs),
                Context::r#enum(target_name, attrs.cfg_attributes()),
                GenModel::new(Some(generics.clone())),
                constants::composer_doc(),
                Rc::clone(context)
            ),
            variant_composers: variant_composers.0,
            variant_presenters: variant_composers.1,
            ffi_object_composer: constants::enum_composer_object::<Self, LANG, SPEC>(),
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
        }
        root
    }

    fn setup_composers(&mut self, root: &ComposerLink<Self>) {
        self.base.link(root);
        self.ffi_object_composer.link(root);
    }
}
impl<LANG, SPEC, Gen> AttrComposable<SPEC> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized + 'static,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}
impl<LANG, SPEC, Gen> GenericsComposable<Gen> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized + 'static,
          LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_generics(&self) -> Gen {
        self.base().compose_generics()
    }
}


impl<LANG, SPEC, Gen> DocsComposable for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(&()))
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl SourceFermentable2<RustFermentate> for EnumComposer<RustFermentate, Vec<Attribute>, Option<Generics>>
    // where
    //       I: DelimiterTrait + ?Sized,
          // SequenceOutput<LANG, SPEC>: ScopeContextPresentable<Presentation = TokenStream2>,
          // OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable,
        // Self: BindingComposable<I, RustFermentate, Vec<Attribute>>
{
    fn ferment(&self) -> Depunctuated<RustFermentate> {

        let bindings = self.compose_bindings();
        Depunctuated::from_iter([
            RustFermentate::Item {
                attrs: self.compose_attributes(),
                comment: self.compose_docs(),
                ffi_presentation: self.compose_object(),
                conversions: self.compose_conversions(),
                bindings: bindings.present(&self.source_ref()),
                traits: Depunctuated::new()
            }
        ])
    }
}

impl<LANG, SPEC, Gen> FFIObjectComposable for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable<Presentation = TokenStream2>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.source_ref()))
    }
}

impl<LANG, SPEC, Gen> BindingComposable<LANG, SPEC, Gen> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC, Gen>> {
        let source = self.source_ref();
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers
            .iter()
            .filter_map(|composer| composer.compose_ctor(&source)));
        bindings.push(BindingPresentableContext::<LANG, SPEC, Gen>::dtor((self.compose_ffi_name(), self.compose_attributes(), self.compose_generics(), PhantomData)));
        bindings
    }
}

impl ConversionComposable for EnumComposer<RustFermentate, Vec<Attribute>, Option<Generics>>
    // where Self: BasicComposable<ComposerLink<Self>, Context, RustFermentate, Vec<Attribute>, Option<Generics>>,
    where Self: SourceAccessible
            + NameContext<Context>
            + AttrComposable<Vec<Attribute>>
            + GenericsComposable<Option<Generics>>,
          // I: DelimiterTrait
          //   + ?Sized
{
    fn compose_conversions(&self) -> Depunctuated<InterfacePresentation> {
        let source = self.source_ref();
        let generics = self.compose_generics();
        let attrs = self.compose_attributes();
        let ffi_type = self.compose_ffi_name();
        let types = (ffi_type.clone(), self.compose_target_name());
        let from_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>>| {
            let attrs = composer.compose_attributes();
            let conversion = composer.compose_aspect(FFIAspect::From).present(&source);
            quote!(#(#attrs)* #conversion)
        };
        let to_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>> | {
            let attrs = composer.compose_attributes();
            let conversion = composer.compose_aspect(FFIAspect::To).present(&source);
            quote!(#(#attrs)* #conversion)
        };

        let from_conversions = CommaPunctuated::from_iter(self.variant_composers.iter().map(from_variant_composer));
        let from_body = DictionaryExpr::Match(quote!(ffi_ref { #from_conversions }));
        let mut to_conversions = CommaPunctuated::from_iter(self.variant_composers.iter().map(to_variant_composer));
        to_conversions.push(quote!(_ => unreachable!("Enum Variant unreachable")));
        let to_body = DictionaryExpr::Match(quote!(obj { #to_conversions }));
        Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &generics),
            InterfacePresentation::conversion_to_boxed(&attrs, &types, to_body, &generics),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &generics),
            InterfacePresentation::drop(&attrs, ffi_type, SequenceOutput::MatchFields((
                Expression::Simple(quote!(self)).into(),
                {
                    let composer = |composer: &ItemComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>>|
                        OwnedItemPresentableContext::SequenceOutput(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes());
                    let mut result =
                        CommaPunctuated::from_iter(self.variant_composers.iter().map(composer));
                    // TODO: make only if fields contain any conditional compilation flags (need composer for determining conditions)
                    result.push(OwnedItemPresentableContext::Exhaustive(Vec::new()));
                    result
                }))
                .present(&source))
        ])
    }
}


impl<LANG, SPEC, Gen> VariantComposable<LANG, SPEC> for EnumComposer<LANG, SPEC, Gen>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_variants(&self) -> CommaPunctuated<SequenceOutput<LANG, SPEC>> {
        CommaPunctuated::from_iter(
            self.variant_presenters
                .iter()
                .map(|(composer, context)| composer(context)))
    }
}
