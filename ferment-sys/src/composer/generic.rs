use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Attribute, Type};
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AnyOtherComposer, BoundsComposer, CallbackComposer, Composer, ComposerLink, constants, ConstructorArgComposerRef, GroupComposer, MapComposer, PresentableArgumentComposerRef, ResultComposer, SliceComposer, TupleComposer};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::ToType;
use crate::lang::{PresentableSpecification, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{DocPresentation, FFIObjectPresentation, present_struct, RustFermentate};

pub struct GenericComposerInfo<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub binding_composer: ConstructorArgComposerRef<LANG, SPEC>,
    pub field_composer: PresentableArgumentComposerRef<LANG, SPEC>,

    pub ffi_name: Ident,
    pub attrs: Vec<Attribute>,
    pub field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
    pub interfaces: Depunctuated<SPEC::Interface>,
}

impl<LANG, SPEC> GenericComposerInfo<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub const fn callback(
        ffi_name: Ident,
        attrs: Vec<Attribute>,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>,
        ) -> Self {
        Self {
            ffi_name,
            attrs,
            field_composers,
            interfaces,
            binding_composer: PresentableArgument::callback_ctor_pair,
            field_composer: PresentableArgument::callback_arg,
        }
    }
    pub fn default(
        ffi_name: Ident,
        attrs: &Vec<Attribute>,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>,
        ) -> Self {
        Self {
            ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            binding_composer: PresentableArgument::named_struct_ctor_pair,
            field_composer: PresentableArgument::public_named,
        }
    }
}

#[allow(unused)]
pub enum GenericComposerWrapper<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: PresentableSpecification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    Bounds(BoundsComposer<LANG, SPEC>),
    Callback(CallbackComposer<LANG, SPEC>),
    Group(GroupComposer<LANG, SPEC>),
    Result(ResultComposer<LANG, SPEC>),
    Slice(SliceComposer<LANG, SPEC>),
    Tuple(TupleComposer<LANG, SPEC>),
    AnyOther(AnyOtherComposer<LANG, SPEC>),
    Map(MapComposer<LANG, SPEC>),
}

impl<'a, SPEC> Composer<'a> for GenericComposerWrapper<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
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
#[cfg(feature = "objc")]
impl<'a, SPEC> Composer<'a> for GenericComposerWrapper<crate::lang::objc::ObjCFermentate, SPEC>
    where SPEC: crate::lang::objc::ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<crate::lang::objc::ObjCFermentate, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
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
// #[derive(BasicComposerOwner)]
pub struct GenericComposer<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: PresentableSpecification<LANG, Expr=Expression<LANG, SPEC>> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable
{
    pub wrapper: GenericComposerWrapper<LANG, SPEC>,
}

impl<LANG, SPEC> GenericComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: PresentableSpecification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(kind: &MixinKind, attrs: Vec<Attribute>, ty_context: SPEC::TYC, scope_context: &ComposerLink<ScopeContext>) -> Option<ComposerLink<Self>> {
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

impl<'a, SPEC> Composer<'a> for GenericComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<RustFermentate>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        self.wrapper
            .compose(source)
            .map(|GenericComposerInfo {
                      field_composers,
                      field_composer,
                      ffi_name,
                      attrs,
                      binding_composer,
                      interfaces }| {
                let fields = CommaPunctuated::from_iter(field_composers.iter().map(field_composer));
                let implementation = BraceWrapped::new(fields).present(source);
                let ffi_presentation = FFIObjectPresentation::Full(present_struct(&ffi_name, &attrs, implementation));
                let ffi_type = ffi_name.to_type();
                let bindings = Depunctuated::from_iter([
                    BindingPresentableContext::ctor((
                        ((ffi_type.clone(), attrs.clone(), SPEC::Gen::default()) , false),
                        constants::field_conversions_iterator(field_composers, binding_composer)
                    )),
                    BindingPresentableContext::dtor((ffi_type, attrs.clone(), SPEC::Gen::default()))
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
