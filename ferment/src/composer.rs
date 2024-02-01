use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Field, FieldsNamed, FieldsUnnamed, ItemTrait, parse_quote, Path, Type, TypePath};
use crate::composition::{AttrsComposition, FnReturnTypeComposition, TraitDecompositionPart2, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, PathConversion, TypeConversion};
use crate::interface::{DEFAULT_DOC_PRESENTER, DEREF_FIELD_PATH, FFI_DEREF_FIELD_NAME, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, IteratorPresenter, LAMBDA_CONVERSION_PRESENTER, MapPairPresenter, MapPresenter, OBJ_FIELD_NAME, OwnerIteratorPresenter, package_unbox_any_expression, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, ScopeTreeFieldTypedPresenter, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER};
use crate::interface::obj;
use crate::presentation::{BindingPresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation, TraitVTablePresentation};
use crate::helper::{destroy_path, destroy_ptr, destroy_reference, ffi_destructor_name, ffi_trait_obj_name, ffi_unnamed_arg_name, ffi_vtable_name, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference, usize_to_tokenstream};
use crate::holder::{EMPTY, PathHolder};
use crate::presentation::context::{OwnedItemPresenterContext, IteratorPresentationContext, OwnerIteratorPresentationContext};
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;
use crate::presentation::doc_presentation::DocPresentation;
use crate::presentation::expansion::Expansion;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;

/// Composer Context Presenters
pub type ComposerPresenter<C, P> = fn(context: &C) -> P;

pub enum ConversionsComposer<'a> {
    Empty,
    NamedStruct(&'a FieldsNamed),
    UnnamedStruct(&'a FieldsUnnamed),
    UnnamedEnumVariant(&'a FieldsUnnamed),
    TypeAlias(&'a Type),
}

fn unnamed_struct_fields_comp(ty: &Type, index: usize) -> TokenStream2 {
    match ty {
        Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
            PathConversion::Primitive(..) => usize_to_tokenstream(index),
            _ => usize_to_tokenstream(index),
        },
        Type::Array(_type_array) => usize_to_tokenstream(index),
        Type::Ptr(_type_ptr) => obj(),
        _ => unimplemented!("from_unnamed_struct: not supported {}", quote!(#ty))
    }
}

impl<'a> ConversionsComposer<'a> {
    pub fn compose(&self, context: &Rc<RefCell<ScopeContext>>) -> Vec<FieldTypeConversion> {
        let ctx = context.borrow();
        match self {
            Self::Empty => vec![],
            Self::NamedStruct(fields) =>
                fields
                    .named
                    .iter()
                    .map(|Field { ident, ty, .. }|
                        FieldTypeConversion::Named(quote!(#ident), ctx.full_type_for(ty)))
                    .collect(),
            Self::UnnamedEnumVariant(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(ffi_unnamed_arg_name(index).to_token_stream(), ctx.full_type_for(ty)))
                        // (context.full_type_for(ty), ffi_unnamed_arg_name(index).to_token_stream()))
                    .collect(),
            Self::UnnamedStruct(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(unnamed_struct_fields_comp(ty, index), ctx.full_type_for(ty)))
                        // (context.full_type_for(ty), unnamed_struct_fields_comp(ty, index)))
                    .collect(),
            Self::TypeAlias(ty) =>
                vec![FieldTypeConversion::Unnamed(unnamed_struct_fields_comp(ty, 0), ctx.full_type_for(ty))],
        }
    }
}

pub const FFI_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    SIMPLE_PRESENTER,
    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER);
pub const DESTROY_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    ROOT_DESTROY_CONTEXT_PRESENTER,
    |_composer| /*composer.drop_composer.as_ref().unwrap().destructors.clone()*/ quote!());
pub const DROP_STRUCT_COMPOSER: DropComposer = DropComposer::new(
    SIMPLE_CONVERSION_PRESENTER,
    EMPTY_CONTEXT_PRESENTER,
    |fields|
        IteratorPresentationContext::StructDestroy(fields),
    SIMPLE_PRESENTER,
    vec![]);
pub const DEFAULT_DOC_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    DEFAULT_DOC_PRESENTER,
    |composer| composer.target_name_composer.compose(&composer.context.borrow()));

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!(&*ffi);
// pub const FROM_DEREF_FFI_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!(obj);
pub const EMPTY_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |_| quote!();
pub const CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> =
    |composer| composer.fields_from();
pub const CONVERSION_FIELDS_TO_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> = |composer|
    composer.fields_to();
pub const FFI_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> = |composer|
    composer.ffi_name_composer.compose(&composer.context.borrow());
pub const TARGET_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer, TokenStream2> = |composer|
    composer.target_name_composer.compose(&composer.context.borrow());

pub trait Composer where Self: Sized {
    // type Item: ToTokens;
    type Item;
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>);
    fn compose(&self, context: &ScopeContext) -> Self::Item;
}

pub struct ItemComposer {
    pub context: Rc<RefCell<ScopeContext>>,
    pub need_drop_presentation: bool,
    pub ffi_name_composer: NameComposer,
    pub target_name_composer: NameComposer,
    pub attrs_composer: AttrsComposer,
    pub ffi_conversions_composer: FFIConversionComposer,
    pub fields_from_composer: FieldsComposer,
    pub fields_to_composer: FieldsComposer,
    pub ffi_object_composer: FFIContextComposer,
    pub doc_composer: FFIContextComposer,
}



impl ItemComposer {

    pub(crate) fn type_alias_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        conversions_composer: ConversionsComposer
    ) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            |(name, fields)|
                OwnerIteratorPresentationContext::TypeAlias(name, fields),
            DEFAULT_DOC_COMPOSER,
            |field_type|
                OwnedItemPresenterContext::DefaultFieldType(field_type),
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    |(_, fields)|
                        OwnerIteratorPresentationContext::TypeAliasFromConversion(fields),
                    SIMPLE_CONVERSION_PRESENTER,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    TO_OBJ_CONTEXT_PRESENTER,
                    |(name, fields)|
                        OwnerIteratorPresentationContext::TypeAliasToConversion(name, fields),
                    SIMPLE_CONVERSION_PRESENTER,
                    ffi_name.to_token_stream(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                |_| quote!(ffi_ref.0),
                |_| obj(),
                FFIBindingsComposer::new(|fields|
                    IteratorPresentationContext::Round(fields)),
                FFI_DEREF_FIELD_NAME),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn struct_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: ScopeTreeFieldTypedPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        bindings_presenter: IteratorPresenter,
        // bindings_arg_presenter: MapPresenter,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            field_presenter,
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    EMPTY_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.to_token_stream(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                FFI_DEREF_FIELD_NAME,
                OBJ_FIELD_NAME,
                FFIBindingsComposer::new(bindings_presenter),
                FFI_DEREF_FIELD_NAME
            ),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enum_variant_default_composer(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        bindings_iterator_presenter: IteratorPresenter,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            attrs,
            context,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            |field_type|
                OwnedItemPresenterContext::DefaultField(field_type),
            FFIContextComposer::new(
                |_| quote!(),
                EMPTY_CONTEXT_PRESENTER),
            FFIConversionComposer::new(
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_TO_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.to_token_stream(),
                    vec![]),
                FFIContextComposer::new(
                    destroy_code_context_presenter,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER),
                DropComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER,
                    |fields|
                        IteratorPresentationContext::DefaultDestroyFields(fields),
                    destroy_presenter,
                    vec![]),
                DEREF_FIELD_PATH,
                SIMPLE_PRESENTER,
                FFIBindingsComposer::new(bindings_iterator_presenter),
                |f| quote!(*#f)),
            conversions_composer)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        attrs: AttrsComposition,
        context: &Rc<RefCell<ScopeContext>>,
        root_presenter: OwnerIteratorPresenter,
        doc_composer: FFIContextComposer,
        field_presenter: ScopeTreeFieldTypedPresenter,
        ffi_object_composer: FFIContextComposer,
        ffi_conversions_composer: FFIConversionComposer,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<ItemComposer>> where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: true,
            context: Rc::clone(context),
            attrs_composer: AttrsComposer::new(attrs),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: FieldsComposer::new(
                root_presenter,
                FFI_NAME_CONTEXT_PRESENTER,
                field_presenter,
                vec![]),
            fields_to_composer: FieldsComposer::new(
                root_presenter,
                TARGET_NAME_CONTEXT_PRESENTER,
                field_presenter,
                vec![]),
            ffi_conversions_composer,
            ffi_object_composer,
            doc_composer,
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
            root_borrowed.setup_conversion(conversions_composer);
        }
        root
    }

    fn setup_composers(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.attrs_composer.set_parent(root);
        self.ffi_name_composer.set_parent(root);
        self.target_name_composer.set_parent(root);
        self.fields_from_composer.set_parent(root);
        self.fields_to_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
        self.ffi_conversions_composer.set_parent(root);
        self.doc_composer.set_parent(root);
    }

    fn setup_conversion(&mut self, conversions_composer: ConversionsComposer) {
        conversions_composer
            .compose(&self.context)
            .into_iter()
            .for_each(|field_type|
                self.add_conversion(field_type));
    }

    fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.ffi_conversions_composer.add_conversion(field_type.clone(), &self.context);
        self.fields_from_composer.add_conversion(field_type.clone());
        self.fields_to_composer.add_conversion(field_type.clone());
    }

    // pub(crate) fn compose_attrs(&self) -> TokenStream2 {
    //     self.attrs_composer.compose(&self.context.borrow())
    // }

    pub(crate) fn fields_from(&self) -> TokenStream2 {
        self.fields_from_composer.compose(&self.context.borrow())
    }

    pub(crate) fn fields_to(&self) -> TokenStream2 {
        self.fields_to_composer.compose(&self.context.borrow())
    }

    pub(crate) fn compose_from(&self) -> TokenStream2 {
        self.ffi_conversions_composer.from_conversion_composer.compose(&self.context.borrow())
    }


    pub(crate) fn compose_to(&self) -> TokenStream2 {
        self.ffi_conversions_composer.to_conversion_composer.compose(&self.context.borrow())
    }

    pub(crate) fn compose_destroy(&self) -> TokenStream2 {
        self.ffi_conversions_composer.destroy_composer.compose(&self.context.borrow())
    }

    pub(crate) fn compose_drop(&self) -> TokenStream2 {
        self.ffi_conversions_composer.drop_composer.compose(&self.context.borrow())
    }
    // pub(crate) fn make_expansion(&self, traits: Vec<TraitVTablePresentation>) -> Expansion {
    pub(crate) fn make_expansion(&self) -> Expansion {
        let ctx = self.context.borrow();
        let ffi_name = self.ffi_name_composer.compose(&ctx);
        // println!("make_expansion: {}: [{}]", format_token_stream(&ffi_name), quote!(#(#traits), *));
        // TODO: avoid this
        let ffi_ident = format_ident!("{}", ffi_name.to_string());
        let target_name = self.target_name_composer.compose(&ctx);
        let conversion_presentation = ConversionInterfacePresentation::Interface {
            ffi_type: ffi_name.clone(),
            target_type: target_name.clone(),
            from_presentation: FromConversionPresentation::Struct(self.compose_from()),
            to_presentation: ToConversionPresentation::Struct(self.compose_to()),
            destroy_presentation: self.compose_destroy(),
            generics: None
        };
        Expansion::Full {
            comment: DocPresentation::Default(self.doc_composer.compose(&ctx)),
            ffi_presentation: FFIObjectPresentation::Full(self.ffi_object_composer.compose(&ctx)),
            conversion: conversion_presentation,
            bindings: vec![
                BindingPresentation::Constructor {
                    ffi_ident: ffi_ident.clone(),
                    ctor_arguments: self.ffi_conversions_composer.bindings_composer.compose_arguments(|field_type|
                        OwnedItemPresenterContext::Named(field_type, false)),
                    body_presentation: self.ffi_conversions_composer.bindings_composer.compose_field_names(&ctx, |field_type|
                        OwnedItemPresenterContext::DefaultField(field_type)),
                    context: self.context.clone(),
                },
                BindingPresentation::Destructor {
                    ffi_name: ffi_name.clone(),
                    destructor_ident: ffi_destructor_name(&ffi_ident).to_token_stream()
                }
            ],
            drop: if self.need_drop_presentation {
                DropInterfacePresentation::Full(self.ffi_name_composer.compose(&ctx), self.compose_drop())
            } else {
                DropInterfacePresentation::Empty
            },
            traits: self.attrs_composer.compose(&ctx)
        }
    }
}

pub struct FFIConversionComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub from_conversion_composer: ConversionComposer,
    pub to_conversion_composer: ConversionComposer,
    pub destroy_composer: FFIContextComposer,
    pub drop_composer: DropComposer,
    pub bindings_composer: FFIBindingsComposer,

    from_presenter: MapPresenter,
    to_presenter: MapPresenter,
    destructor_presenter: MapPresenter,
}

impl FFIConversionComposer {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        from_conversion_composer: ConversionComposer,
        to_conversion_composer: ConversionComposer,
        destroy_composer: FFIContextComposer,
        drop_composer: DropComposer,
        from_presenter: MapPresenter,
        to_presenter: MapPresenter,
        bindings_composer: FFIBindingsComposer,
        destructor_presenter: MapPresenter) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, from_presenter, to_presenter, bindings_composer, destructor_presenter, parent: None }
    }
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.bindings_composer.set_parent(root);
        self.from_conversion_composer.set_parent(root);
        self.to_conversion_composer.set_parent(root);
        self.destroy_composer.set_parent(root);
        self.drop_composer.set_parent(root);
        self.parent = Some(Rc::clone(root));
    }
    pub fn add_conversion(&mut self, field_type: FieldTypeConversion, context: &Rc<RefCell<ScopeContext>>) {
        let field_path_to = (self.to_presenter)(field_type.name());
        let field_path_from = (self.from_presenter)(field_type.name());
        let field_path_destroy = (self.destructor_presenter)(field_type.name());
        let context = context.borrow();
        let (converted_field_to, converted_field_from, destructor) = match field_type.ty() {
            Type::Ptr(type_ptr) => (
                to_ptr(field_path_to, type_ptr, &context),
                from_ptr(field_path_from, type_ptr),
                destroy_ptr(field_path_destroy, type_ptr)
            ),
            Type::Path(TypePath { path, .. }) => (
                to_path(field_path_to, path, &context),
                from_path(field_path_from, path),
                destroy_path(field_path_destroy, path),
            ),
            Type::Reference(type_reference) => (
                to_reference(field_path_to, type_reference, &context),
                from_reference(field_path_from, type_reference),
                destroy_reference(field_path_destroy, type_reference)
            ),
            Type::Array(type_array) => (
                to_array(field_path_to, type_array, &context),
                from_array(field_path_from, type_array),
                package_unbox_any_expression(field_path_destroy),
            ),
            _ => panic!("add_conversion: Unknown field {}", quote!(#field_type)),
        };
        self.to_conversion_composer.add_conversion(field_type.name(), converted_field_to);
        self.from_conversion_composer.add_conversion(field_type.name(), converted_field_from);
        self.bindings_composer.add_conversion(field_type);
        self.drop_composer.add_conversion(destructor);
    }

}

pub struct FFIBindingsComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    root_presenter: IteratorPresenter,
    field_types: Vec<FieldTypeConversion>,
}

impl FFIBindingsComposer {
    pub const fn new(root_presenter: IteratorPresenter) -> Self {
        Self { parent: None, root_presenter, field_types: vec![] }
    }
    pub(crate) fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        self.field_types.push(field_type);
    }

    pub fn compose_field_names(&self, context: &ScopeContext, presenter: ScopeTreeFieldTypedPresenter) -> TokenStream2 {
        (self.root_presenter)(self.compose_arguments(presenter)).present(context)
    }

    pub fn compose_arguments(&self, presenter: ScopeTreeFieldTypedPresenter) -> Vec<OwnedItemPresenterContext> {
        self.field_types.iter()
            .map(|field_type| presenter(field_type.clone()))
            .collect()
    }
}

impl Composer for FFIBindingsComposer {
    type Item = TokenStream2;

    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.parent = Some(Rc::clone(root));
    }

    fn compose(&self, context: &ScopeContext) -> Self::Item {
        (self.root_presenter)(self.field_types.iter().map(|ff| OwnedItemPresenterContext::DefaultField(ff.clone())).collect::<Vec<_>>()).present(context)
    }
}



pub struct FFIContextComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPresenter,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
}
impl FFIContextComposer {
    pub const fn new(composer: MapPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter }
    }
}

pub struct ConversionComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    conversions_presenter: OwnerIteratorPresenter,
    conversion_presenter: MapPairPresenter,
    path: TokenStream2,
    conversions: Vec<TokenStream2>,
}
impl ConversionComposer {
    pub fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, conversions_presenter: OwnerIteratorPresenter, conversion_presenter: MapPairPresenter, path: TokenStream2, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, path, conversions }
    }
    pub fn add_conversion(&mut self, name: TokenStream2, conversion: TokenStream2) {
        self.conversions.push((self.conversion_presenter)(name, conversion));
    }
}

pub struct DropComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    conversions_presenter: IteratorPresenter,
    conversion_presenter: MapPresenter,
    conversions: Vec<TokenStream2>,
}
impl DropComposer {
    pub const fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, conversions_presenter: IteratorPresenter, conversion_presenter: MapPresenter, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, conversions }
    }

    pub fn add_conversion(&mut self, conversion: TokenStream2) {
        let value = (self.conversion_presenter)(conversion);
        self.conversions.push(value);
    }
}


pub struct FieldsComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    context_presenter: ComposerPresenter<ItemComposer, TokenStream2>,
    pub root_presenter: OwnerIteratorPresenter,
    pub field_presenter: ScopeTreeFieldTypedPresenter,

    pub fields: Vec<OwnedItemPresenterContext>,
}
impl FieldsComposer {
    pub const fn new(root_presenter: OwnerIteratorPresenter, context_presenter: ComposerPresenter<ItemComposer, TokenStream2>, field_presenter: ScopeTreeFieldTypedPresenter, fields: Vec<OwnedItemPresenterContext>) -> Self {
        Self { parent: None, root_presenter, context_presenter, field_presenter, fields }
    }

    pub fn add_conversion(&mut self, field_type: FieldTypeConversion) {
        let value = (self.field_presenter)(field_type);
        self.fields.push(value);
    }
}
pub struct NameComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub name: Path,
}

impl NameComposer {
    pub const fn new(name: Path) -> Self {
        Self { parent: None, name }
    }
}
pub struct AttrsComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub attrs: AttrsComposition
}
impl AttrsComposer {
    pub const fn new(attrs: AttrsComposition) -> Self {
        Self { parent: None, attrs }
    }
}

#[macro_export]
macro_rules! composer_impl {
    ($name:ident, $output:ty, $composition:expr) => {
        impl Composer for $name {
            type Item = $output;
            fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
                self.parent = Some(Rc::clone(root));
            }
            #[allow(clippy::redundant_closure_call)]
            fn compose(&self, context: &ScopeContext) -> Self::Item {
                $composition(self, context)
            }
        }
    }
}

composer_impl!(FFIContextComposer, TokenStream2, |context_composer: &FFIContextComposer, _context: &ScopeContext|
    (context_composer.composer)(
        (context_composer.context_presenter)(
            &context_composer.parent.as_ref().unwrap().borrow())));
composer_impl!(ConversionComposer, TokenStream2, |itself: &ConversionComposer, context: &ScopeContext|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            (
                itself.path.to_token_stream(),
                itself.conversions
                .iter()
                .map(|c|
                    OwnedItemPresenterContext::Conversion(c.clone()))
                .collect::<Vec<_>>()))
        .present(context)));

composer_impl!(DropComposer, TokenStream2, |itself: &DropComposer, context: &ScopeContext|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            itself.conversions
            .iter()
            .map(|c|
                OwnedItemPresenterContext::Conversion(c.clone()))
            .collect())
        .present(context)));
composer_impl!(FieldsComposer, TokenStream2, |itself: &FieldsComposer, context: &ScopeContext|
    (itself.root_presenter)(
        ((itself.context_presenter)(&itself.parent.as_ref().unwrap().borrow()),
            itself.fields.clone()))
    .present(context));
composer_impl!(NameComposer, TokenStream2, |itself: &NameComposer, _context: &ScopeContext|
    itself.name.to_token_stream());

composer_impl!(AttrsComposer, Vec<TraitVTablePresentation>, |itself: &AttrsComposer, context: &ScopeContext| {
    let mut trait_types = context.trait_items_from_attributes(&itself.attrs.attrs);
    trait_types.iter_mut()
        .map(|(composition, trait_scope)| {
            // TODO: move to full
            let conversion = TypeConversion::Object(TypeComposition::new(context.scope.to_type(), Some(composition.item.generics.clone())));
            println!("AttrsComposer: {} {} {}", composition.item.ident, trait_scope, conversion);
            composition.implementors.push(conversion);
            implement_trait_for_item((&composition.item, trait_scope), &itself.attrs, context)
        })
        .collect()
});

pub fn implement_trait_for_item(item_trait: (&ItemTrait, &PathHolder), attrs_composition: &AttrsComposition, context: &ScopeContext) -> TraitVTablePresentation {
    let (item_trait, trait_scope) = item_trait;
    let AttrsComposition { ident: item_name, scope: item_scope, .. } = attrs_composition;
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(&item_trait.items, &EMPTY, context);
    let trait_ident = &item_trait.ident;
    let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
    let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));

    let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TokenStream2>, Vec<TokenStream2>) = trait_decomposition.methods.into_iter()
        .map(|signature_decomposition| {
            let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            let fn_name = signature_decomposition.ident.unwrap();
            let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
            let arguments = signature_decomposition.arguments
                .iter()
                // .map(|arg| OwnedItemPresenterContext::Conversion(arg.name_type_original.clone()))
                .map(|arg| arg.name_type_original.clone())
                .collect::<Vec<_>>();
            let mut argument_conversions = vec![OwnedItemPresenterContext::Conversion(quote!(cast_obj))];
            argument_conversions.extend(signature_decomposition.arguments.iter().filter(|arg| arg.name.is_some()).map(|arg| OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())));
            let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments)).present(context);
            let argument_names = IteratorPresentationContext::Round(argument_conversions).present(context);
            (quote!(#name_and_args -> #output_expression {
                let cast_obj = &(*(obj as *const #item_full_ty));
                let obj = <#item_full_ty as #trait_full_ty>::#fn_name #argument_names;
                #output_conversions
            }), quote!(#fn_name: #ffi_method_ident))
        }).unzip();
    let trait_vtable_ident = ffi_vtable_name(trait_ident);
    let trait_object_ident = ffi_trait_obj_name(trait_ident);
    let trait_implementor_vtable_ident = format_ident!("{}_{}", item_name, trait_vtable_ident);
    // let item_module = item_scope.popped();
    println!("implement_trait_for_item: {} {} {}", item_name, trait_ident, trait_scope);
    let (fq_trait_vtable, fq_trait_object) = if item_scope.has_same_parent(&trait_scope) {
        (quote!(#trait_vtable_ident), quote!(#trait_object_ident))
    } else {
        (quote!(#trait_scope::#trait_vtable_ident), quote!(#trait_scope::#trait_object_ident))
    };
    let vtable = quote! {
        #[allow(non_snake_case, non_upper_case_globals)]
        static #trait_implementor_vtable_ident: #fq_trait_vtable = {
            #(#vtable_methods_implentations)*
            #fq_trait_vtable {
                #(#vtable_methods_declarations,)*
            }
        };
    };
    let binding_ident = format_ident!("{}_as_{}", item_name, trait_object_ident);
    let destructor_binding_ident = ffi_destructor_name(&binding_ident);
    let export = quote! {
        /// # Safety
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn #binding_ident(obj: *const #item_full_ty) -> #fq_trait_object {
            #fq_trait_object {
                object: obj as *const (),
                vtable: &#trait_implementor_vtable_ident,
            }
        }
    };
    let destructor = quote! {
        /// # Safety
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn #destructor_binding_ident(obj: #fq_trait_object) {
            ferment_interfaces::unbox_any(obj.object as *mut #item_full_ty);
        }
    };
    TraitVTablePresentation::Full { vtable, export, destructor }
}
