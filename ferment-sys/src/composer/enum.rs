use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Variant};
use std::rc::Rc;
use std::cell::RefCell;
use quote::ToTokens;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, GenModel, LifetimesModel};
use crate::composer::{r#abstract::LinkedContextComposer, AttrComposable, BasicComposer, BasicComposerOwner, BindingComposable, CommaPunctuatedArgKinds, DocComposer, SourceComposable, ComposerLink, DocsComposable, FFIObjectComposable, GenericsComposable, ItemComposerWrapper, Linkable, AspectCommaPunctuatedArgKinds, SourceAccessible, TypeAspect, VariantComposable, VariantComposerRef, SeqKindComposerLink, BasicComposerLink, NameKindComposable, NameKind, LifetimesComposable};
use crate::context::ScopeContextLink;
use crate::lang::{LangAttrSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{DocPresentation, FFIObjectPresentation, Name};

#[derive(ComposerBase)]
pub struct EnumComposer<SPEC>
    where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
    pub ffi_object_composer: SeqKindComposerLink<SPEC, Self>,
    pub variant_composers: Vec<ItemComposerWrapper<SPEC>>,
    pub variant_presenters: Vec<(VariantComposerRef<SPEC>, AspectCommaPunctuatedArgKinds<SPEC>)>,
}

impl<SPEC> NameKindComposable for EnumComposer<SPEC>
where SPEC: Specification {
    fn compose_name_kind(&self) -> NameKind {
        NameKind::Named
    }
}

impl<SPEC> EnumComposer<SPEC>
    where SPEC: Specification<
            Expr=Expression<SPEC>,
            Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens,
          Self: AttrComposable<SPEC::Attr> + GenericsComposable<SPEC::Gen> + LifetimesComposable<SPEC::Lt> + TypeAspect<SPEC::TYC> + NameKindComposable {
    pub fn new(item_enum: &ItemEnum, ty_context: SPEC::TYC, context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemEnum { attrs, ident: target_name, variants, generics, .. } = item_enum;
        let variant_composers = variants
            .iter()
            .map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
                let ty_context = ty_context.join_variant(target_name.clone(), variant_name.clone(), attrs.cfg_attributes());
                let ffi_aspect = Aspect::FFI(ty_context.clone());
                let (variant_composer, fields_context): (VariantComposerRef<SPEC>, CommaPunctuatedArgKinds<SPEC>) = match discriminant {
                    Some((_, expr)) => (
                        SeqKind::unit_fields,
                        CommaPunctuated::from_iter([
                            ArgKind::AttrName(expr.to_token_stream(), SPEC::Attr::from_cfg_attrs(attrs)) ])
                    ),
                    None => match fields {
                        Fields::Unit => (SeqKind::unit, CommaPunctuated::new()),
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                            SeqKind::paren_variants,
                            CommaPunctuated::from_iter(unnamed
                                .iter()
                                .map(|Field { attrs, ty, .. }|
                                    ArgKind::Unnamed(FieldComposer::unnamed_typed(Name::default(), ty, attrs)))),
                        ),
                        Fields::Named(FieldsNamed { named, .. }) => (
                            SeqKind::brace_variants,
                            CommaPunctuated::from_iter(named
                                .iter()
                                .map(|Field { ident, attrs, ty, .. }|
                                    ArgKind::inherited_named_type(Name::Optional(ident.clone()), ty, SPEC::Attr::from_cfg_attrs(attrs)))),
                        ),
                    },
                };
                let aspect_presentable_args = ((ffi_aspect, (SPEC::Attr::from_cfg_attrs(attrs), SPEC::Lt::default(), SPEC::Gen::default()), NameKind::Named), fields_context);
                let variant_composer_wrapper = ItemComposerWrapper::variant(fields, ty_context, attrs, context);
                (variant_composer_wrapper, (variant_composer, aspect_presentable_args))
            }).unzip();
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                DocComposer::from(&ty_context),
                AttrsModel::from(attrs),
                ty_context,
                GenModel::from(generics),
                LifetimesModel::default(),
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


impl<SPEC> DocsComposable for EnumComposer<SPEC>
    where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::DefaultT(self.base.doc.compose(self.context()))
    }
}
impl<SPEC> FFIObjectComposable for EnumComposer<SPEC>
    where SPEC: Specification,
          SeqKind<SPEC>: ScopeContextPresentable {
    fn compose_object(&self) -> FFIObjectPresentation {
        FFIObjectPresentation::Full(self.ffi_object_composer.compose(&())
            .present(&self.source_ref())
            .to_token_stream())
    }
}

impl<SPEC> BindingComposable<SPEC> for EnumComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Name<SPEC>: ToTokens {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<SPEC>> {
        let mut bindings = Depunctuated::new();
        bindings.extend(self.variant_composers.iter().filter_map(ItemComposerWrapper::compose_ctor));
        bindings.push(BindingPresentableContext::<SPEC>::dtor(((self.ffi_type_aspect(), (self.compose_attributes(), self.compose_lifetimes(), self.compose_generics()), NameKind::Named), Default::default())));
        bindings
    }
}
impl<SPEC> VariantComposable<SPEC> for EnumComposer<SPEC>
    where SPEC: Specification {
    fn compose_variants(&self) -> CommaPunctuated<SeqKind<SPEC>> {
        CommaPunctuated::from_iter(
            self.variant_presenters
                .iter()
                .map(|(composer, context)| composer(context)))
    }
}
