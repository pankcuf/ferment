use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Attribute, Generics, Type};
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated};
use crate::composable::{CfgAttributes, FieldComposer};
use crate::composer::{AnyOtherComposer, BoundsComposer, CallbackComposer, Composer, constants, ConstructorArgComposerRef, DocsComposable, GroupComposer, MapComposer, OwnedFieldTypeComposerRef, ComposerLink, ResultComposer, SliceComposer, TupleComposer, BindingComposable};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::ToType;
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{BindingPresentableContext, Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DocPresentation, FFIObjectPresentation, InterfacePresentation, present_struct, RustFermentate};

pub struct GenericComposerInfo<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static {
    pub binding_composer: ConstructorArgComposerRef<LANG, SPEC>,
    pub field_composer: OwnedFieldTypeComposerRef<LANG, SPEC>,

    pub ffi_name: Ident,
    pub attrs: Vec<Attribute>,
    pub field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
    pub interfaces: Depunctuated<InterfacePresentation>,
    phantom_data: PhantomData<Gen>
}

impl<LANG, SPEC, Gen> GenericComposerInfo<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    pub const fn callback(
        ffi_name: Ident,
        attrs: Vec<Attribute>,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<InterfacePresentation>,
        ) -> Self {
        Self {
            ffi_name,
            attrs,
            field_composers,
            interfaces,
            binding_composer: constants::struct_composer_ctor_callback_item(),
            field_composer: constants::callback_field_composer(),
            phantom_data: PhantomData
        }
    }
    pub fn default(
        ffi_name: Ident,
        attrs: &Vec<Attribute>,
        field_composers: Depunctuated<FieldComposer<LANG, SPEC>>,
        interfaces: Depunctuated<InterfacePresentation>,
        ) -> Self {
        Self {
            ffi_name,
            attrs: attrs.clone(),
            field_composers,
            interfaces,
            binding_composer: constants::struct_composer_ctor_named_item(),
            field_composer: constants::named_struct_field_composer(),
            phantom_data: PhantomData
        }
    }
}

impl<LANG, SPEC, Gen> DocsComposable for GenericComposerInfo<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Empty
    }
}

impl<LANG, SPEC> BindingComposable<LANG, SPEC, Option<Generics>> for GenericComposerInfo<LANG, SPEC, Option<Generics>>
    where //I: DelimiterTrait + ?Sized,
          LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<LANG, SPEC, Option<Generics>>> {
        let ffi_type = self.ffi_name.to_type();
        Depunctuated::<BindingPresentableContext<LANG, SPEC, Option<Generics>>>::from_iter([
            constants::struct_composer_ctor_root()((

                // ConstructorPresentableContext::default(ffi_type.clone(), SPEC::from_attrs(self.attrs.clone()), None),
                ((ffi_type.clone(), SPEC::from_attrs(self.attrs.clone()), None, PhantomData) , false),
                constants::field_conversions_iterator(self.field_composers.clone(), self.binding_composer)
            )),
            BindingPresentableContext::dtor((ffi_type, SPEC::from_attrs(self.attrs.clone()), None, PhantomData))
        ])
    }
}

impl ScopeContextPresentable for GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Presentation = RustFermentate;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let fields = CommaPunctuated::from_iter(self.field_composers.iter().map(self.field_composer));
        let implementation = BraceWrapped::new(fields).present(source);
        let ffi_presentation = FFIObjectPresentation::Full(present_struct(&self.ffi_name, &self.attrs, implementation));
        let bindings = <Self as BindingComposable<RustFermentate, Vec<Attribute>, Option<Generics>>>::compose_bindings(self);
        RustFermentate::Item {
            attrs: self.attrs.clone(),
            comment: self.compose_docs(),
            ffi_presentation,
            conversions: self.interfaces.clone(),
            bindings: bindings.present(source),
            traits: Default::default(),
        }
    }
}

#[allow(unused)]
pub enum GenericComposerWrapper<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    Bounds(BoundsComposer<LANG, SPEC, Gen>),
    Callback(CallbackComposer<LANG, SPEC, Gen>),
    Group(GroupComposer<LANG, SPEC, Gen>),
    Result(ResultComposer<LANG, SPEC, Gen>),
    Slice(SliceComposer<LANG, SPEC, Gen>),
    Tuple(TupleComposer<LANG, SPEC, Gen>),
    AnyOther(AnyOtherComposer<LANG, SPEC, Gen>),
    Map(MapComposer<LANG, SPEC, Gen>),
}

impl<'a> Composer<'a> for GenericComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

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
pub struct GenericComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub wrapper: GenericComposerWrapper<LANG, SPEC, Gen>,
    _phantom: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> GenericComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(kind: &MixinKind, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Option<ComposerLink<Self>> {
        let context = Context::mixin(kind, attrs.cfg_attributes());
        let wrapper = match kind {
            MixinKind::Bounds(model) =>
                GenericComposerWrapper::Bounds(BoundsComposer::new(model, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Callback(ty)) =>
                GenericComposerWrapper::Callback(CallbackComposer::new(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Group(ty)) =>
                GenericComposerWrapper::Group(GroupComposer::default(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Array(ty)) =>
                GenericComposerWrapper::Group(GroupComposer::array(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Result(ty)) =>
                GenericComposerWrapper::Result(ResultComposer::new(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Slice(ty)) =>
                GenericComposerWrapper::Slice(SliceComposer::new(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Tuple(Type::Tuple(type_tuple))) =>
                GenericComposerWrapper::Tuple(TupleComposer::new(type_tuple, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::Map(ty)) =>
                GenericComposerWrapper::Map(MapComposer::new(ty, context, attrs, scope_context)),
            MixinKind::Generic(GenericTypeKind::AnyOther(ty)) =>
                GenericComposerWrapper::AnyOther(AnyOtherComposer::new(ty, context, attrs, scope_context)),
            _ => {
                return None;
            }
        };
        let root = Rc::new(RefCell::new(Self { _phantom: PhantomData, wrapper, }));
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
//
// impl SourceFermentable2<RustFermentate> for GenericComposer<RustFermentate, Vec<Attribute>> {
//     fn ferment(&self) -> Depunctuated<RustFermentate> {
//         let mut fermentate = Depunctuated::new();
//         let source = self.source_ref();
//         if let Some(result) = self.wrapper.compose(&source) {
//             fermentate.push(result.present(&source));
//         }
//
//         fermentate
//     }
//
// }

impl<'a> Composer<'a> for GenericComposer<RustFermentate, Vec<Attribute>, Option<Generics>>  {
    type Source = ScopeContext;
    type Output = Option<RustFermentate>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        self.wrapper.compose(source)
            .map(|r| r.present(source))
        // let Self { kind, attrs, .. } = self;
        // match kind {
        //     MixinKind::Bounds(model) =>
        //         BoundsComposer::new(model, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Callback(ty)) =>
        //         CallbackComposer::new(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Group(ty)) =>
        //         GroupComposer::default(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Array(ty)) =>
        //         GroupComposer::array(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Result(ty)) =>
        //         ResultComposer::new(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Slice(ty)) =>
        //         SliceComposer::new(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Tuple(Type::Tuple(type_tuple))) =>
        //         TupleComposer::new(type_tuple, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::Map(ty)) =>
        //         MapComposer::new(ty, attrs)
        //             .compose(source),
        //     MixinKind::Generic(GenericTypeKind::AnyOther(ty)) =>
        //         AnyOtherComposer::new(ty, attrs)
        //             .compose(source),
        //     _ => None
        // }
    }
}
