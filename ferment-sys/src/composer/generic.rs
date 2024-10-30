use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Type};
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AnyOtherComposer, BoundsComposer, CallbackComposer, SourceComposable, ComposerLink, ConstructorArgComposerRef, GroupComposer, MapComposer, PresentableArgumentComposerRef, ResultComposer, SliceComposer, TupleComposer, arg_conversions_iterator, NameKind};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::ToType;
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{DocPresentation, FFIObjectPresentation, present_struct, RustFermentate};

pub struct GenericComposerInfo<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub binding_composer: ConstructorArgComposerRef<LANG, SPEC>,
    pub field_composer: PresentableArgumentComposerRef<LANG, SPEC>,

    pub ffi_aspect: Aspect<SPEC::TYC>,
    pub attrs: SPEC::Attr,
    pub field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
    pub interfaces: Depunctuated<SPEC::Interface>,
}

impl<LANG, SPEC> GenericComposerInfo<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn callback(
        ffi_name: Aspect<SPEC::TYC>,
        attrs: &SPEC::Attr,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>
    ) -> Self {
        Self {
            ffi_aspect: ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            binding_composer: ArgKind::callback_ctor_pair,
            field_composer: ArgKind::callback_arg,
        }
    }
    pub fn default(
        ffi_name: Aspect<SPEC::TYC>,
        attrs: &SPEC::Attr,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>,
        ) -> Self {
        Self {
            ffi_aspect: ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            binding_composer: ArgKind::named_struct_ctor_pair,
            field_composer: ArgKind::public_named,
        }
    }
}

#[allow(unused)]
pub enum GenericComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    Bounds(BoundsComposer<LANG, SPEC>),
    Callback(CallbackComposer<LANG, SPEC>),
    Group(GroupComposer<LANG, SPEC>),
    Result(ResultComposer<LANG, SPEC>),
    Slice(SliceComposer<LANG, SPEC>),
    Tuple(TupleComposer<LANG, SPEC>),
    AnyOther(AnyOtherComposer<LANG, SPEC>),
    Map(MapComposer<LANG, SPEC>),
}

impl<LANG, SPEC> SourceComposable for GenericComposerWrapper<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Expression<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          BoundsComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          CallbackComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          GroupComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          ResultComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          SliceComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          TupleComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          AnyOtherComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
          MapComposer<LANG, SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<LANG, SPEC>>>,
{
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<LANG, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            GenericComposerWrapper::Bounds(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Callback(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Group(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Result(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Slice(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Tuple(composer) =>
                composer.compose(source),
            GenericComposerWrapper::AnyOther(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Map(composer) =>
                composer.compose(source),
        }
    }
}

#[allow(unused)]
pub struct GenericComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable
{
    pub wrapper: GenericComposerWrapper<LANG, SPEC>,
}

impl<LANG, SPEC> GenericComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(kind: &MixinKind, attrs: Vec<Attribute>, ty_context: SPEC::TYC, scope_context: &ScopeContextLink) -> Option<ComposerLink<Self>> {
        let wrapper = match kind {
            MixinKind::Bounds(model) =>
                GenericComposerWrapper::Bounds(BoundsComposer::new(model, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Callback(ty)) =>
                GenericComposerWrapper::Callback(CallbackComposer::new(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Group(ty)) =>
                GenericComposerWrapper::Group(GroupComposer::default(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Array(ty)) =>
                GenericComposerWrapper::Group(GroupComposer::array(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Result(ty)) =>
                GenericComposerWrapper::Result(ResultComposer::new(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Slice(ty)) =>
                GenericComposerWrapper::Slice(SliceComposer::new(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Tuple(Type::Tuple(type_tuple))) =>
                GenericComposerWrapper::Tuple(TupleComposer::new(type_tuple, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Map(ty)) =>
                GenericComposerWrapper::Map(MapComposer::new(ty, ty_context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::AnyOther(ty)) =>
                GenericComposerWrapper::AnyOther(AnyOtherComposer::new(ty, ty_context, attrs, scope_context)),
            _ => {
                return None;
            }
        };
        let root = Rc::new(RefCell::new(Self { wrapper, }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
        }
        Some(root)
    }
    fn setup_composers(&mut self, _root: &ComposerLink<Self>) {
        // self.base.link(root);
    }
}

impl<SPEC> SourceComposable for GenericComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<RustFermentate>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        self.wrapper
            .compose(source)
            .map(|GenericComposerInfo {
                      field_composers,
                      field_composer,
                      ffi_aspect,
                      attrs,
                      binding_composer,
                      interfaces }| {
                let fields = CommaPunctuated::from_iter(field_composers.iter().map(field_composer));
                let ffi_name_tokens = ffi_aspect.present(source).to_token_stream();
                let ffi_presentation = FFIObjectPresentation::Full(present_struct(&ffi_name_tokens, &attrs, BraceWrapped::new(fields).present(source)));
                let dtor_context = (ffi_aspect, attrs.clone(), SPEC::Gen::default(), NameKind::Named);
                let ctor_context = (dtor_context.clone(), arg_conversions_iterator(&field_composers, binding_composer));
                let bindings = Depunctuated::from_iter([
                    BindingPresentableContext::ctor(ctor_context),
                    BindingPresentableContext::dtor((dtor_context, Default::default()))
                ]);
                RustFermentate::Item {
                    attrs,
                    comment: DocPresentation::Empty,
                    ffi_presentation,
                    conversions: interfaces,
                    bindings: bindings.present(source),
                    traits: Default::default(),
                }
            })
    }
}
