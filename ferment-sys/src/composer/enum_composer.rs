use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Variant, Visibility};
use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedArgKinds, SourceComposable, ComposerLink, DocsComposable, FFIAspect, FFIObjectComposable, GenericsComposable, InterfaceComposable, ItemComposerWrapper, Linkable, AspectCommaPunctuatedArguments, SourceAccessible, SourceFermentable, TypeAspect, VariantComposable, VariantComposerRef, SeqKindComposerLink, BasicComposerLink, NameKindComposable, NameKind, LifetimesComposable};
use crate::composer::r#abstract::LinkedContextComposer;
use crate::context::ScopeContextLink;
use crate::lang::{LangAttrSpecification, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, TypeContext, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{DictionaryExpr, DocComposer, DocPresentation, FFIObjectPresentation, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct EnumComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
    pub ffi_object_composer: SeqKindComposerLink<LANG, SPEC, Self>,
    pub variant_composers: Vec<ItemComposerWrapper<LANG, SPEC>>,
    pub variant_presenters: Vec<(VariantComposerRef<LANG, SPEC>, AspectCommaPunctuatedArguments<LANG, SPEC>)>,
}

impl<LANG, SPEC> NameKindComposable for EnumComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Named
    }
}

impl<LANG, SPEC> EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Self: AttrComposable<SPEC::Attr> + GenericsComposable<SPEC::Gen> + LifetimesComposable<SPEC::Lt> + TypeAspect<SPEC::TYC> + NameKindComposable {
    pub fn new(item_enum: &ItemEnum, ty_context: SPEC::TYC, context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemEnum { attrs, ident: target_name, variants, generics, .. } = item_enum;
        let variant_composers = variants
            .iter()
            .map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
                let ty_context = ty_context.join_variant(target_name.clone(), variant_name.clone(), attrs.cfg_attributes());
                let ffi_aspect = Aspect::FFI(ty_context.clone());
                let (variant_composer, fields_context): (VariantComposerRef<LANG, SPEC>, CommaPunctuatedArgKinds<LANG, SPEC>) = match discriminant {
                    Some((_, expr)) => (
                        SeqKind::unit_fields,
                        CommaPunctuated::from_iter([
                            ArgKind::AttrName(expr.to_token_stream(), SPEC::Attr::from_attrs(attrs.cfg_attributes())) ])
                    ),
                    None => match fields {
                        Fields::Unit => (SeqKind::unit, CommaPunctuated::new()),
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                            SeqKind::paren_variants,
                            CommaPunctuated::from_iter(unnamed
                                .iter()
                                .map(|Field { attrs, ty, .. }|
                                    ArgKind::Unnamed(FieldComposer::typed(Name::default(), ty, false, attrs)))),
                        ),
                        Fields::Named(FieldsNamed { named, .. }) => (
                            SeqKind::brace_variants,
                            CommaPunctuated::from_iter(named
                                .iter()
                                .map(|Field { ident, attrs, ty, .. }|
                                    ArgKind::Named(FieldComposer::typed(Name::Optional(ident.clone()), ty, true, attrs), Visibility::Inherited))),
                        ),
                    },
                };
                let aspect_presentable_args = ((ffi_aspect, SPEC::Attr::from_attrs(attrs.cfg_attributes()), SPEC::Gen::default(), NameKind::Named), fields_context);
                let variant_composer_wrapper = ItemComposerWrapper::variant(fields, ty_context, attrs, context);
                (variant_composer_wrapper, (variant_composer, aspect_presentable_args))
            }).unzip();
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                DocComposer::new(ty_context.to_token_stream()),
                AttrsModel::from(attrs),
                ty_context,
                GenModel::new(Some(generics.clone())),
                LifetimesModel::new(vec![]),
                Rc::clone(context)
            ),
            variant_composers: variant_composers.0,
            variant_presenters: variant_composers.1,
            ffi_object_composer: LinkedContextComposer::new(SeqKind::r#enum, SeqKind::variants),
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
          SPEC: Specification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(self.context()))
    }
}
impl<LANG, SPEC> FFIObjectComposable for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.source_ref())
            .to_token_stream())
    }
}

impl<LANG, SPEC> BindingComposable<LANG, SPEC> for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC>> {
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers.iter().filter_map(ItemComposerWrapper::compose_ctor));
        bindings.push(BindingPresentableContext::<LANG, SPEC>::dtor(((self.ffi_type_aspect(), self.compose_attributes(), self.compose_generics(), NameKind::Named), Default::default())));
        bindings
    }
}
impl<LANG, SPEC> VariantComposable<LANG, SPEC> for EnumComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_variants(&self) -> CommaPunctuated<SeqKind<LANG, SPEC>> {
        CommaPunctuated::from_iter(
            self.variant_presenters
                .iter()
                .map(|(composer, context)| composer(context)))
    }
}

impl<SPEC> InterfaceComposable<SPEC::Interface> for EnumComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification,
          Self: SourceAccessible
            + NameKindComposable
            + TypeAspect<TypeContext>
            + AttrComposable<SPEC::Attr>
            + GenericsComposable<SPEC::Gen>
            + LifetimesComposable<SPEC::Lt> {
    fn compose_interfaces(&self) -> Depunctuated<SPEC::Interface> {
        let source = self.source_ref();
        let generics = self.compose_generics();
        let lifetimes = self.compose_lifetimes();
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let from_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC>|
            ArgKind::AttrSequence(composer.compose_aspect(FFIAspect::From), composer.compose_attributes());
        let to_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC> |
            ArgKind::AttrSequence(composer.compose_aspect(FFIAspect::To), composer.compose_attributes());
        let drop_variant_composer = |composer: &ItemComposerWrapper<RustFermentate, SPEC>|
            ArgKind::AttrSequence(composer.compose_aspect(FFIAspect::Drop), composer.compose_attributes());

        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = CommaPunctuated::new();

        self.variant_composers.iter()
            .for_each(|variant_composer| {
                let from = from_variant_composer(variant_composer);
                let to = to_variant_composer(variant_composer);
                let destroy = drop_variant_composer(variant_composer);
                from_conversions.push(from);
                to_conversions.push(to);
                destroy_conversions.push(destroy);
            });
        to_conversions.push(ArgKind::AttrExhaustive(vec![]));
        destroy_conversions.push(ArgKind::AttrExhaustive(vec![]));

        let from_body = DictionaryExpr::MatchFields(quote!(ffi_ref), from_conversions.present(&source));
        let to_body = DictionaryExpr::MatchFields(quote!(obj), to_conversions.present(&source));
        let drop_body = DictionaryExpr::MatchFields(quote!(self), destroy_conversions.present(&source));

        Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &generics, &lifetimes),
            InterfacePresentation::conversion_to_boxed(&attrs, &types, to_body, &generics, &lifetimes),
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

