use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Type, TypeTuple};
use crate::ast::Depunctuated;
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{AnyOtherComposer, ArrayComposer, BoundsComposer, CallbackComposer, SourceComposable, ComposerLink, GroupComposer, MapComposer, PresentableArgKindComposerRef, ResultComposer, SliceComposer, SmartPointerComposer, TupleComposer, NameKind};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::kind::{CallbackKind, GenericTypeKind, MixinKind, SmartPointerKind, TypeKind};
use crate::ext::{AsType, GenericNestedArg};
use crate::lang::Specification;
use crate::presentable::{Aspect, BindingPresentableContext, ArgKind};

pub struct GenericComposerInfo<SPEC>
    where SPEC: Specification + 'static {
    pub field_composer: PresentableArgKindComposerRef<SPEC>,
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
        let ctor_context = (dtor_context.clone(), Vec::from_iter(field_composers.iter().map(ArgKind::named_ready_struct_ctor_pair)));
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
            field_composer: ArgKind::public_named_ready,
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
            field_composer: ArgKind::public_named_ready,
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
    pub fn smart_ptr(root_kind: &SmartPointerKind, smart_pointer_kind: SmartPointerKind, type_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> GenericComposerWrapper<SPEC> {
        Self::SmartPointer(SmartPointerComposer::new(root_kind, smart_pointer_kind, type_context, attrs, scope_context))
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
            MixinKind::Generic(GenericTypeKind::Callback(kind)) =>
                GenericComposerWrapper::callback(kind, ty_context, attrs, scope_context),
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
            MixinKind::Generic(GenericTypeKind::SmartPointer(root_kind)) => match root_kind {
                SmartPointerKind::Cell(..) |
                SmartPointerKind::RefCell(..) |
                SmartPointerKind::UnsafeCell(..) |
                SmartPointerKind::Mutex(..) |
                SmartPointerKind::OnceLock(..) |
                SmartPointerKind::RwLock(..) =>
                    GenericComposerWrapper::smart_ptr(root_kind, root_kind.clone(), ty_context, attrs, scope_context),
                SmartPointerKind::Rc(ty) |
                SmartPointerKind::Arc(ty) => match TypeKind::from(ty.maybe_first_nested_type_ref()?) {
                    TypeKind::Generic(GenericTypeKind::SmartPointer(smart_pointer_kind)) =>
                        GenericComposerWrapper::smart_ptr(root_kind, smart_pointer_kind, ty_context, attrs, scope_context),
                    _ =>
                        GenericComposerWrapper::any_other(root_kind.as_type(), ty_context, attrs, scope_context),
                },
                _ =>
                    GenericComposerWrapper::any_other(root_kind.as_type(), ty_context, attrs, scope_context),
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

