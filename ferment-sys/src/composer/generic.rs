use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use syn::{Attribute, Type, TypeTuple};
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated};
use crate::composable::{CfgAttributes, FieldComposer, GenericBoundsModel};
use crate::composer::{AnyOtherComposer, BoundsComposer, CallbackComposer, SourceComposable, ComposerLink, GroupComposer, MapComposer, PresentableArgumentComposerRef, ResultComposer, SliceComposer, TupleComposer, NameKind};
use crate::composer::arc_composer::ArcComposer;
use crate::composer::array::ArrayComposer;
use crate::composer::rc::RcComposer;
use crate::composer::smart_pointer::SmartPointerComposer;
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{expand_attributes, CallbackKind, GenericTypeKind, MixinKind, SmartPointerKind, TypeKind};
use crate::ext::{AsType, GenericNestedArg};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind, ScopeContextPresentable, TypeContext};
use crate::presentation::{DocPresentation, FFIObjectPresentation, present_struct, RustFermentate};

pub struct GenericComposerInfo<SPEC>
    where SPEC: Specification + 'static {
    pub field_composer: PresentableArgumentComposerRef<SPEC>,
    pub ffi_aspect: Aspect<SPEC::TYC>,
    pub attrs: SPEC::Attr,
    pub field_composers: Depunctuated<FieldComposer<SPEC>>,
    pub interfaces: Depunctuated<SPEC::Interface>,
    pub bindings: Depunctuated<BindingPresentableContext<SPEC>>,
}

impl<SPEC> GenericComposerInfo<SPEC>
    where SPEC: Specification {

    pub fn callback(
        ffi_name: Aspect<SPEC::TYC>,
        attrs: &SPEC::Attr,
        field_composers: Depunctuated<FieldComposer<SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>
    ) -> Self {
        let dtor_context = (ffi_name.clone(), (attrs.clone(), SPEC::Lt::default(), SPEC::Gen::default()), NameKind::Named);
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::callback_ctor_pair)));
        let bindings = Depunctuated::from_iter([
            BindingPresentableContext::<SPEC>::ctor::<Vec<_>>(ctor_context),
            BindingPresentableContext::<SPEC>::dtor((dtor_context, Default::default()))
        ]);
        Self {
            ffi_aspect: ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            bindings,
            field_composer: ArgKind::callback_arg,
        }
    }
    pub fn default(
        ffi_name: Aspect<SPEC::TYC>,
        attrs: &SPEC::Attr,
        field_composers: Depunctuated<FieldComposer<SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>,
        ) -> Self {
        let dtor_context = (ffi_name.clone(), (attrs.clone(), SPEC::Lt::default(), SPEC::Gen::default()), NameKind::Named);
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::named_struct_ctor_pair)));
        let bindings = Depunctuated::from_iter([
            BindingPresentableContext::<SPEC>::ctor::<Vec<_>>(ctor_context),
            BindingPresentableContext::<SPEC>::dtor((dtor_context, Default::default()))
        ]);

        Self {
            ffi_aspect: ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            bindings,
            field_composer: ArgKind::public_named,
        }
    }

    pub fn default_with_bindings(
        ffi_name: Aspect<SPEC::TYC>,
        attrs: &SPEC::Attr,
        field_composers: Depunctuated<FieldComposer<SPEC>>,
        interfaces: Depunctuated<SPEC::Interface>,
        bindings: Depunctuated<BindingPresentableContext<SPEC>>,
    ) -> Self {
        Self {
            ffi_aspect: ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            bindings,
            field_composer: ArgKind::public_named,
        }
    }
}

#[allow(unused)]
pub enum GenericComposerWrapper<SPEC>
    where SPEC: Specification + 'static {
    Bounds(BoundsComposer<SPEC>),
    Callback(CallbackComposer<SPEC>),
    Array(ArrayComposer<SPEC>),
    Group(GroupComposer<SPEC>),
    Result(ResultComposer<SPEC>),
    Slice(SliceComposer<SPEC>),
    Tuple(TupleComposer<SPEC>),
    AnyOther(AnyOtherComposer<SPEC>),
    Arc(ArcComposer<SPEC>),
    Rc(RcComposer<SPEC>),
    SmartPointer(SmartPointerComposer<SPEC>),
    Map(MapComposer<SPEC>),
}

impl<SPEC> GenericComposerWrapper<SPEC>
where SPEC: Specification {
    pub fn bounds(model: &GenericBoundsModel, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Bounds(BoundsComposer::new(model, type_context, attrs, scope_context))
    }
    pub fn callback(kind: &CallbackKind, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Callback(CallbackComposer::new(kind, type_context, attrs, scope_context))
    }
    pub fn group(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Group(GroupComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn array(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Array(ArrayComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn result(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Result(ResultComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn slice(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Slice(SliceComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn tuple(ty: &TypeTuple, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Tuple(TupleComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn map(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Map(MapComposer::new(ty, type_context, attrs, scope_context))
    }
    pub fn arc(ty: &Type, smart_pointer_kind: SmartPointerKind, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Arc(ArcComposer::new(ty, smart_pointer_kind, type_context, attrs, scope_context))
    }
    pub fn rc(ty: &Type, smart_pointer_kind: SmartPointerKind, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::Rc(RcComposer::new(ty, smart_pointer_kind, type_context, attrs, scope_context))
    }
    pub fn smart_ptr(ty: &Type, smart_pointer_kind: SmartPointerKind, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::SmartPointer(SmartPointerComposer::new(ty, smart_pointer_kind, type_context, attrs, scope_context))
    }
    pub fn any_other(ty: &Type, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::AnyOther(AnyOtherComposer::new(ty, type_context, attrs, scope_context))
    }
}

impl<SPEC> SourceComposable for GenericComposerWrapper<SPEC>
    where SPEC: Specification,
          BoundsComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          CallbackComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          ArrayComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          GroupComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          ResultComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          SliceComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          TupleComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          AnyOtherComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          ArcComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          RcComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          SmartPointerComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
          MapComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=Option<GenericComposerInfo<SPEC>>>,
{
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        match self {
            GenericComposerWrapper::Bounds(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Callback(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Array(composer) =>
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
            GenericComposerWrapper::Arc(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Rc(composer) =>
                composer.compose(source),
            GenericComposerWrapper::SmartPointer(composer) =>
                composer.compose(source),
            GenericComposerWrapper::Map(composer) =>
                composer.compose(source),
        }
    }
}

#[allow(unused)]
pub struct GenericComposer<SPEC>
    where SPEC: Specification + 'static {
    pub wrapper: GenericComposerWrapper<SPEC>,
}

impl<SPEC> GenericComposer<SPEC>
    where SPEC: Specification {

    pub fn new(kind: &MixinKind, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Option<ComposerLink<Self>> {
        let wrapper = match kind {
            MixinKind::Bounds(model) =>
                GenericComposerWrapper::bounds(model, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Callback(ty)) =>
                GenericComposerWrapper::callback(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Group(ty)) =>
                GenericComposerWrapper::group(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Array(ty)) =>
                GenericComposerWrapper::array(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Result(ty)) =>
                GenericComposerWrapper::result(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Slice(ty)) =>
                GenericComposerWrapper::slice(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Tuple(Type::Tuple(type_tuple))) =>
                GenericComposerWrapper::tuple(type_tuple, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::Map(ty)) =>
                GenericComposerWrapper::map(ty, ty_context, attrs, scope_context),
            // MixinKind::Generic(GenericTypeKind::Cow(ty)) =>
            //     GenericComposerWrapper::cow(ty, ty_context, attrs, scope_context),
            MixinKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Arc(ty))) => match TypeKind::from(ty.maybe_first_nested_type_ref()?) {
                TypeKind::Generic(GenericTypeKind::SmartPointer(smart_pointer_kind)) => GenericComposerWrapper::arc(ty, smart_pointer_kind, ty_context, attrs, scope_context),
                _ => GenericComposerWrapper::any_other(ty, ty_context, attrs, scope_context),
            },
            MixinKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Rc(ty))) => match TypeKind::from(ty.maybe_first_nested_type_ref()?) {
                TypeKind::Generic(GenericTypeKind::SmartPointer(smart_pointer_kind)) => GenericComposerWrapper::rc(ty, smart_pointer_kind, ty_context, attrs, scope_context),
                _ => GenericComposerWrapper::any_other(ty, ty_context, attrs, scope_context),
            },
            MixinKind::Generic(GenericTypeKind::SmartPointer(kind)) => match kind {
                SmartPointerKind::Mutex(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::OnceLock(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::RwLock(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::Cell(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::RefCell(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::UnsafeCell(ty) => GenericComposerWrapper::smart_ptr(ty, kind.clone(), ty_context, attrs, scope_context),
                _ => GenericComposerWrapper::any_other(kind.as_type(), ty_context, attrs, scope_context),
            }
            MixinKind::Generic(GenericTypeKind::AnyOther(ty)) =>
                GenericComposerWrapper::any_other(ty, ty_context, attrs, scope_context),
            _ => {
                return None;
            }
        };
        Some(Rc::new(RefCell::new(Self { wrapper })))
    }
}

impl GenericComposer<RustSpecification> {
    pub fn mixin(context: (&MixinKind, &HashSet<Option<Attribute>>), scope_link: &ScopeContextLink) -> Option<ComposerLink<Self>> {
        let (mixin, attrs) = context;
        let attrs = expand_attributes(attrs);
        Self::new(mixin, TypeContext::mixin(mixin, attrs.cfg_attributes()), attrs, scope_link)
    }
}

impl SourceComposable for GenericComposer<RustSpecification> {
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
                      interfaces,
                      bindings
                  }| {
                let struct_body = BraceWrapped::new(CommaPunctuated::from_iter(field_composers.iter().map(field_composer)));
                let ffi_presentation = FFIObjectPresentation::Full(present_struct(ffi_aspect.present(source), &attrs, struct_body.present(source)));

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
