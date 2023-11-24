use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Type, TypePath};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, DEFAULT_DESTROY_FIELDS_PRESENTER, DEFAULT_DICT_FIELD_PRESENTER, DEFAULT_DICT_FIELD_TYPE_PRESENTER, DEFAULT_DOC_PRESENTER, DEREF_FIELD_PATH, EMPTY_DESTROY_PRESENTER, EMPTY_DICT_FIELD_TYPED_PRESENTER, EMPTY_ITERATOR_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, FFI_DEREF_FIELD_NAME, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, IteratorPresenter, LAMBDA_CONVERSION_PRESENTER, MapPairPresenter, MapPresenter, NAMED_CONVERSION_PRESENTER, NAMED_DICT_FIELD_TYPE_PRESENTER, NAMED_STRUCT_PRESENTER, NO_FIELDS_PRESENTER, OBJ_FIELD_NAME, OwnerIteratorPresenter, Presentable, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, ScopeTreeFieldTypedPresenter, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, SIMPLE_TERMINATED_PRESENTER, STRUCT_DESTROY_PRESENTER, TYPE_ALIAS_CONVERSION_FROM_PRESENTER, TYPE_ALIAS_CONVERSION_TO_PRESENTER, TYPE_ALIAS_PRESENTER, UNNAMED_STRUCT_PRESENTER};
use crate::interface::{obj};
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, TraitVTablePresentation};
use crate::helper::{destroy_array, destroy_path, destroy_ptr, destroy_reference, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference};
use crate::type_conversion::TypeConversion;

/// Composer Context Presenters
pub type ComposerPresenter<C> = fn(context: &C) -> TokenStream2;

pub const FFI_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    SIMPLE_PRESENTER,
    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER);
pub const DESTROY_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    ROOT_DESTROY_CONTEXT_PRESENTER,
    |_composer| /*composer.drop_composer.as_ref().unwrap().destructors.clone()*/ quote!());
pub const DROP_STRUCT_COMPOSER: DropComposer = DropComposer::new(
    SIMPLE_CONVERSION_PRESENTER,
    EMPTY_CONTEXT_PRESENTER,
    STRUCT_DESTROY_PRESENTER,
    SIMPLE_PRESENTER,
    vec![]);
pub const DEFAULT_DOC_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    DEFAULT_DOC_PRESENTER,
    |composer| composer.target_name_composer.compose());

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(&*ffi);
pub const FROM_DEREF_FFI_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(obj);
pub const EMPTY_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!();
pub const CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.fields_from();
pub const CONVERSION_FIELDS_TO_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.fields_to();
pub const FFI_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.ffi_name_composer.compose();
pub const TARGET_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.target_name_composer.compose();

pub trait Composer where Self: Sized {
    type Item: Presentable;
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>);
    fn compose(&self) -> Self::Item;
}

pub struct ItemComposer {
    pub tree: HashMap<TypeConversion, Type>,
    pub need_drop_presentation: bool,
    pub ffi_name_composer: NameComposer,
    pub target_name_composer: NameComposer,
    pub ffi_conversions_composer: FFIConversionComposer,
    pub fields_from_composer: FieldsComposer,
    pub fields_to_composer: FieldsComposer,
    pub ffi_object_composer: FFIContextComposer,
    pub doc_composer: FFIContextComposer,
}

impl ItemComposer {

    pub(crate) fn type_alias_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        conversions_composer: I
    ) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            tree,
            TYPE_ALIAS_PRESENTER,
            DEFAULT_DOC_COMPOSER,
            DEFAULT_DICT_FIELD_TYPE_PRESENTER,
            FFI_STRUCT_COMPOSER,
            FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
                    TYPE_ALIAS_CONVERSION_FROM_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    target_name.clone(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    TO_OBJ_CONTEXT_PRESENTER,
                    TYPE_ALIAS_CONVERSION_TO_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    ffi_name.clone(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                |_| quote!(ffi_ref.0),
                |_| obj(),
                FFI_DEREF_FIELD_NAME),
            conversions_composer
        )
    }

    fn struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: ScopeTreeFieldTypedPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            tree,
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
                    target_name.clone(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    EMPTY_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.clone(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                FFI_DEREF_FIELD_NAME,
                OBJ_FIELD_NAME,
                FFI_DEREF_FIELD_NAME),
            conversions_composer
        )
    }

    pub(crate) fn unnamed_struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::struct_composer(
            ffi_name,
            target_name,
            tree,
            UNNAMED_STRUCT_PRESENTER,
            DEFAULT_DICT_FIELD_TYPE_PRESENTER,
            ROUND_BRACES_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            conversions_composer
        )
    }

    pub(crate) fn named_struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::struct_composer(
            ffi_name,
            target_name,
            tree,
            NAMED_STRUCT_PRESENTER,
            NAMED_DICT_FIELD_TYPE_PRESENTER,
            CURLY_BRACES_FIELDS_PRESENTER,
            NAMED_CONVERSION_PRESENTER,
            conversions_composer)
    }

    fn enum_variant_default_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            tree,
            root_presenter,
            DEFAULT_DOC_COMPOSER,
            DEFAULT_DICT_FIELD_PRESENTER,
            FFIContextComposer::new(
                |_| quote!(),
                EMPTY_CONTEXT_PRESENTER),
            FFIConversionComposer::new(
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    target_name.clone(),
                    vec![]),
                ConversionComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_TO_CONTEXT_PRESENTER,
                    root_conversion_presenter,
                    conversion_presenter,
                    ffi_name.clone(),
                    vec![]),
                FFIContextComposer::new(
                    destroy_code_context_presenter,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER),
                DropComposer::new(
                    LAMBDA_CONVERSION_PRESENTER,
                    CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER,
                    DEFAULT_DESTROY_FIELDS_PRESENTER,
                    destroy_presenter,
                    vec![]),
                DEREF_FIELD_PATH,
                SIMPLE_PRESENTER,
                |f| quote!(#f.to_owned())),
            conversions_composer)
    }

    fn enum_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        root_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_presenter: MapPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            tree,
            root_presenter,
            root_presenter,
            conversion_presenter,
            ROOT_DESTROY_CONTEXT_PRESENTER,
            destroy_presenter,
            conversions_composer)
    }

    pub(crate) fn enum_unit_variant_composer(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>) -> Rc<RefCell<Self>> {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            tree,
            NO_FIELDS_PRESENTER,
            NO_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            ROOT_DESTROY_CONTEXT_PRESENTER,
            EMPTY_DESTROY_PRESENTER,
            IntoIterator::into_iter(vec![])
        )
    }

    pub(crate) fn enum_unnamed_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            tree,
            ROUND_BRACES_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            SIMPLE_TERMINATED_PRESENTER,
            conversions_composer
        )
    }

    pub(crate) fn enum_named_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            tree,
            CURLY_BRACES_FIELDS_PRESENTER,
            NAMED_CONVERSION_PRESENTER,
            SIMPLE_PRESENTER,
            conversions_composer
        )
    }


    pub(crate) fn primitive_composer(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_from_presenter: OwnerIteratorPresenter,
        root_conversion_to_presenter: OwnerIteratorPresenter,
        destroy_code_context_presenter: MapPresenter,
        drop_presenter: MapPairPresenter,
        conversions_from: Vec<TokenStream2>,
        conversion_to_path: TokenStream2) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: false,
            tree,
            ffi_name_composer: NameComposer::new(ffi_name.clone()),
            target_name_composer: NameComposer::new(target_name.clone()),
            ffi_object_composer: FFIContextComposer::new(
                SIMPLE_PRESENTER,
                EMPTY_CONTEXT_PRESENTER),
            ffi_conversions_composer: FFIConversionComposer::new(
                ConversionComposer::new(
                    FFI_FROM_ROOT_PRESENTER,
                    FROM_DEREF_FFI_CONTEXT_PRESENTER,
                    root_conversion_from_presenter,
                    EMPTY_PAIR_PRESENTER,
                    target_name.clone(),
                    conversions_from),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    EMPTY_CONTEXT_PRESENTER,
                    root_conversion_to_presenter,
                    EMPTY_PAIR_PRESENTER,
                    conversion_to_path,
                    vec![]),
                FFIContextComposer::new(
                    destroy_code_context_presenter,
                    EMPTY_CONTEXT_PRESENTER),
                DropComposer::new(
                    drop_presenter,
                    EMPTY_CONTEXT_PRESENTER,
                    EMPTY_ITERATOR_PRESENTER,
                    EMPTY_MAP_PRESENTER,
                    vec![]),
                EMPTY_MAP_PRESENTER,
                EMPTY_MAP_PRESENTER,
                EMPTY_MAP_PRESENTER
            ),
            doc_composer: DEFAULT_DOC_COMPOSER,
            fields_from_composer: FieldsComposer::new(
                root_presenter,
                FFI_NAME_CONTEXT_PRESENTER,
                EMPTY_DICT_FIELD_TYPED_PRESENTER,
                vec![]),
            fields_to_composer: FieldsComposer::new(
                root_presenter,
                TARGET_NAME_CONTEXT_PRESENTER,
                EMPTY_DICT_FIELD_TYPED_PRESENTER,
                vec![]),
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
        }
        root
    }


    fn new<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        tree: HashMap<TypeConversion, Type>,
        root_presenter: OwnerIteratorPresenter,
        doc_composer: FFIContextComposer,
        field_presenter: ScopeTreeFieldTypedPresenter,
        ffi_object_composer: FFIContextComposer,
        ffi_conversions_composer: FFIConversionComposer,
        conversions_composer: I) -> Rc<RefCell<ItemComposer>> where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: true,
            tree,
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
        self.ffi_name_composer.set_parent(root);
        self.target_name_composer.set_parent(root);
        self.fields_from_composer.set_parent(root);
        self.fields_to_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
        // self.ffi_conversions_composer.set_parent(root);
        self.ffi_conversions_composer.from_conversion_composer.set_parent(root);
        self.ffi_conversions_composer.to_conversion_composer.set_parent(root);
        self.ffi_conversions_composer.destroy_composer.set_parent(root);
        self.ffi_conversions_composer.drop_composer.set_parent(root);
        self.doc_composer.set_parent(root);
    }

    fn setup_conversion<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(&mut self, conversions_composer: I) {
        for (field_type, field_name) in conversions_composer {
            self.add_conversion(&field_type, field_name);
        }
    }

    fn add_conversion(&mut self, field_type: &Type, field_name: TokenStream2) {
        self.ffi_conversions_composer.add_conversion(field_name.clone(), field_type);
        self.fields_from_composer.add_conversion(field_name.clone(), field_type, &self.tree);
        self.fields_to_composer.add_conversion(field_name.clone(), field_type, &self.tree);
    }

    pub(crate) fn fields_from(&self) -> TokenStream2 {
        self.fields_from_composer.compose()
    }

    pub(crate) fn fields_to(&self) -> TokenStream2 {
        self.fields_to_composer.compose()
    }

    pub(crate) fn compose_from(&self) -> TokenStream2 {
        self.ffi_conversions_composer.from_conversion_composer.compose()
    }

    pub(crate) fn compose_to(&self) -> TokenStream2 {
        self.ffi_conversions_composer.to_conversion_composer.compose()
    }

    pub(crate) fn compose_destroy(&self) -> TokenStream2 {
        self.ffi_conversions_composer.destroy_composer.compose()
    }

    pub(crate) fn compose_drop(&self) -> TokenStream2 {
        self.ffi_conversions_composer.drop_composer.compose()
    }
    pub(crate) fn make_expansion(&self, destructor_ident: TokenStream2, traits: Vec<TraitVTablePresentation>) -> Expansion {
        let ffi_name = self.ffi_name_composer.compose();
        let target_name = self.target_name_composer.compose();
        Expansion::Full {
            comment: DocPresentation::Default(self.doc_composer.compose()),
            ffi_presentation: FFIObjectPresentation::Full(self.ffi_object_composer.compose()),
            conversion: ConversionInterfacePresentation::Interface {
                ffi_name: ffi_name.clone(),
                target_name: target_name.clone(),
                from_presentation: self.compose_from(),
                to_presentation: self.compose_to(),
                destroy_presentation: self.compose_destroy()
            },
            destructor: ConversionInterfacePresentation::Destructor {
                ffi_name: ffi_name.clone(),
                destructor_ident
            },
            drop: if self.need_drop_presentation {
                DropInterfacePresentation::Full(self.ffi_name_composer.compose(), self.compose_drop())
            } else {
                DropInterfacePresentation::Empty
            },
            traits
        }
    }
}

pub struct FFIConversionComposer {
    pub from_conversion_composer: ConversionComposer,
    pub to_conversion_composer: ConversionComposer,
    pub destroy_composer: FFIContextComposer,
    pub drop_composer: DropComposer,

    from_presenter: MapPresenter,
    to_presenter: MapPresenter,
    destructor_presenter: MapPresenter,
}

impl FFIConversionComposer {
    pub const fn new(from_conversion_composer: ConversionComposer, to_conversion_composer: ConversionComposer, destroy_composer:FFIContextComposer, drop_composer: DropComposer, from_presenter: MapPresenter, to_presenter: MapPresenter, destructor_presenter: MapPresenter) -> Self {
        Self { from_conversion_composer, to_conversion_composer, destroy_composer, drop_composer, from_presenter, to_presenter, destructor_presenter }
    }
    pub fn add_conversion(&mut self, field_name: TokenStream2, field_type: &Type) {
        let field_path_to = (self.to_presenter)(field_name.clone());
        let field_path_from = (self.from_presenter)(field_name.clone());
        let field_path_destroy = (self.destructor_presenter)(field_name.clone());
        let (converted_field_to, converted_field_from, destructor) = match field_type {
            Type::Ptr(type_ptr) => (
                to_ptr(field_path_to, type_ptr),
                from_ptr(field_path_from, type_ptr),
                destroy_ptr(field_path_destroy, type_ptr)
            ),
            Type::Path(TypePath { path, .. }) => (
                to_path(field_path_to, path, None),
                from_path(field_path_from, path, None),
                destroy_path(field_path_destroy, path, None),
            ),
            Type::Reference(type_reference) => (
                to_reference(field_path_to, type_reference),
                from_reference(field_path_from, type_reference),
                destroy_reference(field_path_destroy, type_reference)
            ),
            Type::Array(type_array) => (
                to_array(field_path_to, type_array),
                from_array(field_path_from, type_array),
                destroy_array(field_path_destroy, type_array)
            ),
            _ => panic!("add_conversion: Unknown field {} {}", field_name, quote!(#field_type)),
        };
        self.to_conversion_composer.add_conversion(field_name.clone(), converted_field_to);
        self.from_conversion_composer.add_conversion(field_name.clone(), converted_field_from);
        self.drop_composer.add_conversion(destructor);
    }

}

pub struct FFIContextComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPresenter,
    context_presenter: ComposerPresenter<ItemComposer>,
}
impl FFIContextComposer {
    pub const fn new(composer: MapPresenter, context_presenter: ComposerPresenter<ItemComposer>) -> Self {
        Self { parent: None, composer, context_presenter }
    }
}

pub struct ConversionComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer>,
    conversions_presenter: OwnerIteratorPresenter,
    conversion_presenter: MapPairPresenter,
    path: TokenStream2,
    conversions: Vec<TokenStream2>,
}
impl ConversionComposer {
    pub fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer>, conversions_presenter: OwnerIteratorPresenter, conversion_presenter: MapPairPresenter, path: TokenStream2, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, path, conversions }
    }
    pub fn add_conversion(&mut self, name: TokenStream2, conversion: TokenStream2) {
        self.conversions.push((self.conversion_presenter)(name, conversion));
    }
}

pub struct DropComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<ItemComposer>,
    conversions_presenter: IteratorPresenter,
    conversion_presenter: MapPresenter,
    conversions: Vec<TokenStream2>,
}
impl DropComposer {
    pub const fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<ItemComposer>, conversions_presenter: IteratorPresenter, conversion_presenter: MapPresenter, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, conversions }
    }

    pub fn add_conversion(&mut self, conversion: TokenStream2) {
        let value = (self.conversion_presenter)(conversion);
        self.conversions.push(value);
    }
}


pub struct FieldsComposer {
    parent: Option<Rc<RefCell<ItemComposer>>>,
    context_presenter: ComposerPresenter<ItemComposer>,
    pub root_presenter: OwnerIteratorPresenter,
    pub field_presenter: ScopeTreeFieldTypedPresenter,

    pub fields: Vec<TokenStream2>,
}
impl FieldsComposer {
    pub const fn new(root_presenter: OwnerIteratorPresenter, context_presenter: ComposerPresenter<ItemComposer>, field_presenter: ScopeTreeFieldTypedPresenter, fields: Vec<TokenStream2>) -> Self {
        Self { parent: None, root_presenter, context_presenter, field_presenter, fields }
    }

    pub fn add_conversion(&mut self, name: TokenStream2, field_type: &Type, tree: &HashMap<TypeConversion, Type>) {
        let value = (self.field_presenter)(name, field_type, tree);
        self.fields.push(value);
    }
}
pub struct NameComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub name: TokenStream2,
}

impl NameComposer {
    pub const fn new(name: TokenStream2) -> Self {
        Self { parent: None, name }
    }
}

#[macro_export]
macro_rules! composer_impl {
    ($name:ident, $composition:expr) => {
        impl Composer for $name {
            type Item = TokenStream2;
            fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
                self.parent = Some(Rc::clone(root));
            }
            fn compose(&self) -> Self::Item {
                $composition(self)
            }
        }
    }
}

composer_impl!(FFIContextComposer, |context_composer: &FFIContextComposer|
    (context_composer.composer)(
        (context_composer.context_presenter)(
            &*context_composer.parent.as_ref().unwrap().borrow())));
composer_impl!(ConversionComposer, |itself: &ConversionComposer|
    (itself.composer)(
        (itself.context_presenter)(
            &*itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            (itself.path.to_token_stream(), itself.conversions.clone()))));

composer_impl!(DropComposer, |itself: &DropComposer|
    (itself.composer)(
        (itself.context_presenter)(
            &*itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            itself.conversions.clone())));
composer_impl!(FieldsComposer, |itself: &FieldsComposer|
    (itself.root_presenter)(
        ((itself.context_presenter)(
            &*itself.parent.as_ref().unwrap().borrow()),
            itself.fields.clone())));
composer_impl!(NameComposer, |itself: &NameComposer|
    itself.name.clone());