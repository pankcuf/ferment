use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Variant, Visibility};
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use quote::{quote, ToTokens};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedPresentableArguments, SourceComposable, ComposerLink, constants, DocsComposable, FFIAspect, FFIObjectComposable, GenericsComposable, InterfaceComposable, ItemComposerWrapper, Linkable, AspectCommaPunctuatedArguments, SourceAccessible, SourceFermentable, TypeAspect, VariantComposable, VariantComposerRef, SequenceOutputComposerLink, BasicComposerLink};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::ext::ToType;
use crate::lang::{LangAttrSpecification, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, TypeContext, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{DictionaryExpr, DictionaryName, DocPresentation, FFIObjectPresentation, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct EnumComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    pub ffi_object_composer: SequenceOutputComposerLink<Self, LANG, SPEC>,
    pub variant_composers: Vec<ItemComposerWrapper<LANG, SPEC>>,
    pub variant_presenters: Vec<(VariantComposerRef<LANG, SPEC>, AspectCommaPunctuatedArguments<LANG, SPEC>)>,
}

impl<LANG, SPEC> EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: GenericsComposable<SPEC::Gen> + AspectPresentable<SPEC::TYC> {
    pub fn new(item_enum: &ItemEnum, ty_context: SPEC::TYC, context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemEnum { attrs, ident: target_name, variants, generics, .. } = item_enum;
        let variant_composers = variants
            .iter()
            .map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
                let (variant_composer, fields_context): (VariantComposerRef<LANG, SPEC>, CommaPunctuatedPresentableArguments<LANG, SPEC>) = match discriminant {
                    Some((_, expr)) => (
                        |local_context| PresentableSequence::EnumUnitFields(local_context.clone()),
                        CommaPunctuated::from_iter([
                            PresentableArgument::AttrName(expr.to_token_stream(), SPEC::Attr::from_attrs(attrs.cfg_attributes())) ])
                    ),
                    None => match fields {
                        Fields::Unit => (
                            |((aspect, _), _)| PresentableSequence::NoFields(aspect.clone()),
                            CommaPunctuated::new()
                        ),
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                            |local_context| PresentableSequence::RoundVariantFields(local_context.clone()),
                            CommaPunctuated::from_iter(unnamed
                                .iter()
                                .map(|Field { attrs, ty, .. }|
                                    PresentableArgument::DefaultFieldType(FieldComposer::typed(Name::Empty, ty, false, attrs)))),
                        ),
                        Fields::Named(FieldsNamed { named, .. }) => (
                            |local_context| PresentableSequence::CurlyVariantFields(local_context.clone()),
                            CommaPunctuated::from_iter(named
                                .iter()
                                .map(|Field { ident, attrs, ty, .. }|
                                    PresentableArgument::Named(FieldComposer::typed(Name::Optional(ident.clone()), ty, true, attrs), Visibility::Inherited))),
                        ),
                    },
                };
                let ty_context = ty_context.join_variant(target_name.clone(), variant_name.clone(), attrs.cfg_attributes());
                (ItemComposerWrapper::variant(fields, ty_context.clone(), attrs, context), (variant_composer, ((Aspect::FFI(ty_context), SPEC::Gen::default()), fields_context)))
            }).unzip();
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                AttrsModel::from(attrs),
                ty_context,
                GenModel::new(Some(generics.clone())),
                constants::composer_doc(),
                Rc::clone(context)
            ),
            variant_composers: variant_composers.0,
            variant_presenters: variant_composers.1,
            ffi_object_composer: LinkedContextComposer::new(PresentableSequence::r#enum, PresentableSequence::variants),
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


impl<LANG, SPEC> DocsComposable for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(&()))
    }
}
impl<LANG, SPEC> FFIObjectComposable for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.source_ref())
            .to_token_stream())
    }
}

impl<LANG, SPEC> BindingComposable<LANG, SPEC> for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>> {
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers.iter().filter_map(ItemComposerWrapper::compose_ctor));
        bindings.push(BindingPresentableContext::<LANG, SPEC>::dtor((self.present_ffi_aspect(), self.compose_attributes(), self.compose_generics())));
        bindings
    }
}
impl<LANG, SPEC> VariantComposable<LANG, SPEC> for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_variants(&self) -> CommaPunctuated<PresentableSequence<LANG, SPEC>> {
        CommaPunctuated::from_iter(
            self.variant_presenters
                .iter()
                .map(|(composer, context)| composer(context)))
    }
}

impl<SPEC> InterfaceComposable<SPEC::Interface> for EnumComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification,
          Self: SourceAccessible
            + TypeAspect<TypeContext>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen> {
    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let source = self.source_ref();
        let generics = self.compose_generics();
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let from_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC>|
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::From), composer.compose_attributes());
        let to_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC> |
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::To), composer.compose_attributes());
        let drop_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC>|
            PresentableArgument::AttrSequence(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes());

        let from_conversions = CommaPunctuated::from_iter(self.variant_composers.iter().map(from_variant_composer));
        let mut to_conversions = CommaPunctuated::from_iter(self.variant_composers.iter().map(to_variant_composer));
        to_conversions.push(PresentableArgument::AttrExhaustive(vec![]));
        let mut destroy_conversions = CommaPunctuated::from_iter(self.variant_composers.iter().map(drop_variant_composer));
        destroy_conversions.push(PresentableArgument::AttrExhaustive(vec![]));
        let from_body = DictionaryExpr::MatchFields(quote!(ffi_ref), from_conversions.present(&source));
        let to_body = DictionaryExpr::MatchFields(quote!(obj), to_conversions.present(&source));
        let drop_body = DictionaryExpr::MatchFields(quote!(self), destroy_conversions.present(&source));

        Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &generics),
            InterfacePresentation::conversion_to_boxed(&attrs, &types, to_body, &generics),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &generics),
            InterfacePresentation::drop(&attrs, ffi_type, drop_body)
        ])
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for EnumComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let bindings = self.compose_bindings();
        RustFermentate::Item {
            attrs: self.compose_attributes(),
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversions: self.compose_interfaces(),
            bindings: bindings.present(&self.source_ref()),
            traits: Depunctuated::new()
        }
    }
}

