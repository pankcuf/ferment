use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Type, TypePath};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, DEFAULT_DESTROY_FIELDS_PRESENTER, DEFAULT_DOC_PRESENTER, DEFAULT_FIELD_PRESENTER, DEFAULT_FIELD_TYPE_PRESENTER, DEREF_FIELD_PATH, EMPTY_DESTROY_PRESENTER, EMPTY_FIELD_TYPED_PRESENTER, EMPTY_ITERATOR_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, FFI_DEREF_FIELD_NAME, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, FieldTypedPresenter, IteratorPresenter, LAMBDA_CONVERSION_PRESENTER, MapPairPresenter, MapPresenter, NAMED_CONVERSION_PRESENTER, NAMED_FIELD_TYPE_PRESENTER, NAMED_STRUCT_PRESENTER, NO_FIELDS_PRESENTER, OBJ_FIELD_NAME, OwnerIteratorPresenter, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, SIMPLE_TERMINATED_PRESENTER, STRUCT_DESTROY_PRESENTER, TYPE_ALIAS_CONVERSION_FROM_PRESENTER, TYPE_ALIAS_CONVERSION_TO_PRESENTER, TYPE_ALIAS_PRESENTER, UNNAMED_STRUCT_PRESENTER};
use crate::interface::{obj};
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation};
use crate::helper::{destroy_array, destroy_path, destroy_ptr, destroy_reference, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference};

/// Composer Context Presenters
pub type ComposerPresenter<C> = fn(context: &C) -> TokenStream2;

pub const FFI_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(SIMPLE_PRESENTER, CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER);
pub const DESTROY_STRUCT_COMPOSER: FFIContextComposer = FFIContextComposer::new(ROOT_DESTROY_CONTEXT_PRESENTER, |_composer| /*composer.drop_composer.as_ref().unwrap().destructors.clone()*/ quote!());
pub const DROP_STRUCT_COMPOSER: DropComposer = DropComposer::new(SIMPLE_CONVERSION_PRESENTER, EMPTY_CONTEXT_PRESENTER, STRUCT_DESTROY_PRESENTER, SIMPLE_PRESENTER, vec![]);
pub const DEFAULT_DOC_COMPOSER: FFIContextComposer = FFIContextComposer::new(DEFAULT_DOC_PRESENTER, |composer| composer.target_name_composer.compose());

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ComposerPresenter<RootComposer> = |_| quote!(&*ffi);
pub const FROM_DEREF_FFI_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |_| quote!(*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |_| quote!(obj);
pub const EMPTY_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |_| quote!();
pub const CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |composer| composer.fields_from();
pub const CONVERSION_FIELDS_TO_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |composer| composer.fields_to();
pub const FFI_NAME_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |composer| composer.ffi_name_composer.compose();
pub const TARGET_NAME_CONTEXT_PRESENTER: ComposerPresenter<RootComposer> = |composer| composer.target_name_composer.compose();

pub trait Composer where Self: Sized {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>);
    fn compose(&self) -> TokenStream2;
}

pub struct RootComposer {
    pub ffi_name_composer: NameComposer,
    pub target_name_composer: NameComposer,
    pub fields_from_composer: FieldsComposer,
    pub fields_to_composer: FieldsComposer,
    pub ffi_object_composer: FFIContextComposer,
    pub from_conversion_composer: ConversionComposer,
    pub to_conversion_composer: ConversionComposer,
    pub destroy_composer: FFIContextComposer,
    pub drop_composer: DropComposer,
    pub doc_composer: FFIContextComposer,

    pub from: MapPresenter,
    pub to: MapPresenter,
    pub destroy: MapPresenter,
}

impl RootComposer {
    pub(crate) fn type_alias_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        conversions_composer: I
    ) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            TYPE_ALIAS_PRESENTER,
            DEFAULT_FIELD_TYPE_PRESENTER,
            FFI_STRUCT_COMPOSER,
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
            DEFAULT_DOC_COMPOSER,
            |_| quote!(ffi_ref.0),
            |_| obj(),
            FFI_DEREF_FIELD_NAME,
            conversions_composer
        )
    }

    fn struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: FieldTypedPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            root_presenter,
            field_presenter,
            FFI_STRUCT_COMPOSER,
            ConversionComposer::new(FFI_FROM_ROOT_PRESENTER, FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER, root_conversion_presenter, conversion_presenter, target_name.clone(), vec![]),
            ConversionComposer::new(FFI_TO_ROOT_PRESENTER, EMPTY_CONTEXT_PRESENTER, root_conversion_presenter, conversion_presenter, ffi_name.clone(), vec![]),
            DESTROY_STRUCT_COMPOSER,
            DROP_STRUCT_COMPOSER,
            DEFAULT_DOC_COMPOSER,
            FFI_DEREF_FIELD_NAME,
            OBJ_FIELD_NAME,
            FFI_DEREF_FIELD_NAME,
            conversions_composer
        )
    }

    pub(crate) fn unnamed_struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::struct_composer(
            ffi_name,
            target_name,
            UNNAMED_STRUCT_PRESENTER,
            DEFAULT_FIELD_TYPE_PRESENTER,
            ROUND_BRACES_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            conversions_composer
        )
    }

    pub(crate) fn named_struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::struct_composer(
            ffi_name,
            target_name,
            NAMED_STRUCT_PRESENTER,
            NAMED_FIELD_TYPE_PRESENTER,
            CURLY_BRACES_FIELDS_PRESENTER,
            NAMED_CONVERSION_PRESENTER,
            conversions_composer)
    }

    // pub(crate) fn generic_struct_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
    //     ffi_name: TokenStream2,
    //     target_name: TokenStream2,
    //     from_conversions_presenter: OwnerIteratorPresenter,
    //     to_conversions_presenter: OwnerIteratorPresenter,
    //     conversion_presenter: MapPairPresenter,
    //     drop_conversions_presenter: IteratorPresenter,
    //     doc_presenter: MapPresenter,
    //     from_conversions: Vec<TokenStream2>,
    //     to_conversions: Vec<TokenStream2>,
    //     conversions_composer: I) -> Rc<RefCell<Self>> {
    //     let root = Rc::new(RefCell::new(Self {
    //         ffi_name: ffi_name.clone(),
    //         target_name: target_name.clone(),
    //         fields_from_composer: FieldsComposer::new(NAMED_STRUCT_PRESENTER, FFI_NAME_CONTEXT_PRESENTER, NAMED_FIELD_TYPE_PRESENTER, vec![]),
    //         fields_to_composer: FieldsComposer::new(NAMED_STRUCT_PRESENTER, TARGET_NAME_CONTEXT_PRESENTER, NAMED_FIELD_TYPE_PRESENTER, vec![]),
    //         ffi_object_composer: FFI_STRUCT_COMPOSER,
    //         from_conversion_composer: ConversionComposer::new(
    //             FFI_FROM_ROOT_PRESENTER,
    //             FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER,
    //             from_conversions_presenter,
    //             conversion_presenter,
    //             target_name.clone(),
    //             from_conversions),
    //         to_conversion_composer: ConversionComposer::new(
    //             FFI_TO_ROOT_PRESENTER,
    //             EMPTY_CONTEXT_PRESENTER,
    //             to_conversions_presenter,
    //             conversion_presenter,
    //             ffi_name.clone(),
    //             to_conversions),
    //         destroy_composer: DESTROY_STRUCT_COMPOSER,
    //         drop_composer: DropComposer::new(SIMPLE_CONVERSION_PRESENTER, EMPTY_CONTEXT_PRESENTER, drop_conversions_presenter, SIMPLE_PRESENTER, vec![]),
    //         doc_composer: FFIContextComposer::new(doc_presenter, |composer| composer.target_name.clone()),
    //         from: FFI_DEREF_FIELD_NAME,
    //         to: OBJ_FIELD_NAME,
    //         destroy: FFI_DEREF_FIELD_NAME,
    //     }));
    //     {
    //         let mut root_borrowed = root.borrow_mut();
    //         root_borrowed.setup_composers(&root);
    //         root_borrowed.setup_generic_conversion(conversions_composer);
    //     }
    //     root
    //
    // }

    fn enum_variant_default_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        object_presenter: MapPresenter,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_code_context_presenter: MapPresenter,
        destroy_presenter: MapPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            root_presenter,
            DEFAULT_FIELD_PRESENTER,
            FFIContextComposer::new(object_presenter, EMPTY_CONTEXT_PRESENTER),
            ConversionComposer::new(LAMBDA_CONVERSION_PRESENTER, CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER, root_conversion_presenter, conversion_presenter, target_name.clone(), vec![]),
            ConversionComposer::new(LAMBDA_CONVERSION_PRESENTER, CONVERSION_FIELDS_TO_CONTEXT_PRESENTER, root_conversion_presenter, conversion_presenter, ffi_name.clone(), vec![]),
            FFIContextComposer::new(destroy_code_context_presenter, CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER),
            DropComposer::new(LAMBDA_CONVERSION_PRESENTER, CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER, DEFAULT_DESTROY_FIELDS_PRESENTER, destroy_presenter, vec![]),
            DEFAULT_DOC_COMPOSER,
            DEREF_FIELD_PATH,
            SIMPLE_PRESENTER,
            |f| quote!(#f.to_owned()),
            conversions_composer)
    }

    fn enum_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        object_presenter: MapPresenter,
        root_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        destroy_presenter: MapPresenter,
        conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            object_presenter,
            root_presenter,
            root_presenter,
            conversion_presenter,
            ROOT_DESTROY_CONTEXT_PRESENTER,
            destroy_presenter,
            conversions_composer)
    }

    pub(crate) fn enum_unit_variant_composer(ffi_name: TokenStream2, target_name: TokenStream2, object_presenter: MapPresenter) -> Rc<RefCell<Self>> {
        Self::enum_variant_default_composer(
            ffi_name,
            target_name,
            object_presenter,
            NO_FIELDS_PRESENTER,
            NO_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            ROOT_DESTROY_CONTEXT_PRESENTER,
            EMPTY_DESTROY_PRESENTER,
            IntoIterator::into_iter(vec![])
        )
    }

    pub(crate) fn enum_unnamed_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(ffi_name: TokenStream2, target_name: TokenStream2, variant_presenter: MapPresenter, conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            variant_presenter,
            ROUND_BRACES_FIELDS_PRESENTER,
            SIMPLE_CONVERSION_PRESENTER,
            SIMPLE_TERMINATED_PRESENTER,
            conversions_composer
        )
    }

    pub(crate) fn enum_named_variant_composer<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(ffi_name: TokenStream2, target_name: TokenStream2, object_presenter: MapPresenter, conversions_composer: I) -> Rc<RefCell<Self>> {
        Self::enum_variant_composer(
            ffi_name,
            target_name,
            object_presenter,
            CURLY_BRACES_FIELDS_PRESENTER,
            NAMED_CONVERSION_PRESENTER,
            SIMPLE_PRESENTER,
            conversions_composer
        )
    }


    pub(crate) fn primitive_composer(
        ffi_name: TokenStream2,
        target_name: TokenStream2,
        root_presenter: OwnerIteratorPresenter,
        root_conversion_from_presenter: OwnerIteratorPresenter,
        root_conversion_to_presenter: OwnerIteratorPresenter,
        destroy_code_context_presenter: MapPresenter,
        drop_presenter: MapPairPresenter,
        conversions_from: Vec<TokenStream2>,
        conversion_to_path: TokenStream2) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Self {
            // ffi_name: ffi_name.clone(),
            // target_name: target_name.clone(),
            ffi_name_composer: NameComposer::new(ffi_name.clone()),
            target_name_composer: NameComposer::new(target_name.clone()),
            // field_presenter: EMPTY_FIELD_TYPED_PRESENTER,
            // root_presenter,
            ffi_object_composer: FFIContextComposer::new(SIMPLE_PRESENTER, EMPTY_CONTEXT_PRESENTER),
            from_conversion_composer: ConversionComposer::new(FFI_FROM_ROOT_PRESENTER, FROM_DEREF_FFI_CONTEXT_PRESENTER, root_conversion_from_presenter, EMPTY_PAIR_PRESENTER, target_name.clone(), conversions_from),
            to_conversion_composer: ConversionComposer::new(FFI_TO_ROOT_PRESENTER, EMPTY_CONTEXT_PRESENTER, root_conversion_to_presenter, EMPTY_PAIR_PRESENTER, conversion_to_path, vec![]),
            destroy_composer: FFIContextComposer::new(destroy_code_context_presenter, EMPTY_CONTEXT_PRESENTER),
            drop_composer: DropComposer::new(drop_presenter, EMPTY_CONTEXT_PRESENTER, EMPTY_ITERATOR_PRESENTER, EMPTY_MAP_PRESENTER, vec![]),
            doc_composer: DEFAULT_DOC_COMPOSER,
            from: EMPTY_MAP_PRESENTER,
            to: EMPTY_MAP_PRESENTER,
            destroy: EMPTY_MAP_PRESENTER,
            // fields: vec![],
            fields_from_composer: FieldsComposer::new(root_presenter, FFI_NAME_CONTEXT_PRESENTER, EMPTY_FIELD_TYPED_PRESENTER, vec![]),
            fields_to_composer: FieldsComposer::new(root_presenter, TARGET_NAME_CONTEXT_PRESENTER, EMPTY_FIELD_TYPED_PRESENTER, vec![]),
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
        root_presenter: OwnerIteratorPresenter,
        field_presenter: FieldTypedPresenter,
        ffi_object_composer: FFIContextComposer,
        from_conversion_composer: ConversionComposer,
        to_conversion_composer: ConversionComposer,
        destroy_composer: FFIContextComposer,
        drop_composer: DropComposer,
        doc_composer: FFIContextComposer,
        from: MapPresenter,
        to: MapPresenter,
        destroy: MapPresenter,
        conversions_composer: I) -> Rc<RefCell<RootComposer>> where Self: Sized {
        let root = Rc::new(RefCell::new(Self {
            // ffi_name: ffi_name.clone(),
            // target_name: target_name.clone(),
            ffi_name_composer: NameComposer::new(ffi_name),
            target_name_composer: NameComposer::new(target_name),
            fields_from_composer: FieldsComposer::new(root_presenter, FFI_NAME_CONTEXT_PRESENTER, field_presenter, vec![]),
            fields_to_composer: FieldsComposer::new(root_presenter, TARGET_NAME_CONTEXT_PRESENTER, field_presenter, vec![]),
            ffi_object_composer,
            from_conversion_composer,
            to_conversion_composer,
            destroy_composer,
            drop_composer,
            doc_composer,
            from,
            to,
            destroy,
        }));
        {
            let mut root_borrowed = root.borrow_mut();
            root_borrowed.setup_composers(&root);
            root_borrowed.setup_conversion(conversions_composer);
        }
        root
    }

    fn setup_composers(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.fields_from_composer.set_parent(root);
        self.fields_to_composer.set_parent(root);
        self.ffi_object_composer.set_parent(root);
        self.from_conversion_composer.set_parent(root);
        self.to_conversion_composer.set_parent(root);
        self.destroy_composer.set_parent(root);
        self.drop_composer.set_parent(root);
        self.doc_composer.set_parent(root);
    }

    // fn setup_generic_conversion<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(&mut self, conversions_composer: I) {
    //     for (field_type, field_name) in conversions_composer {
    //         self.add_generic_conversion(&field_type, field_name);
    //     }
    // }

    fn setup_conversion<'a, I: IntoIterator<Item = (&'a Type, TokenStream2)>>(&mut self, conversions_composer: I) {
        for (field_type, field_name) in conversions_composer {
            self.add_conversion(&field_type, field_name);
        }
    }

    // fn add_generic_conversion(&mut self, field_type: &Type, field_name: TokenStream2) {
    //     println!("add_generic_conversion: {}: {}", quote!(#field_name), quote!(#field_type));
    //     // let field_path_to = (self.to)(field_name.clone());
    //     // let field_path_from = (self.from)(field_name.clone());
    //     // let field_path_destroy = (self.destroy)(field_name.clone());
    //
    //     let field_path_to = field_name.clone();
    //     let field_path_from = field_name.clone();
    //     let field_path_destroy = field_name.clone();
    //
    //     // match field_type {
    //     //     Type::Ptr(TypePtr { elem, .. }) => match &**elem {
    //     //         Type::Ptr(TypePtr { elem, .. }) => match &**elem {
    //     //             Type::Path(TypePath { path, .. }) => {
    //     //                 match PathConversion::from(path) {
    //     //                     PathConversion::Simple(path) => {
    //     //                         quote!(std::slice::from_raw_parts(#field_path.values as *const #field_type, #field_path.count).to_vec())
    //     //                     },
    //     //                     PathConversion::Complex(path) => {
    //     //
    //     //                         ffi_from_conversion()
    //     //                     },
    //     //                     _ => panic!("cfff")
    //     //                 }
    //     //             },
    //     //             Type::Array(TypeArray { elem, len, .. }) => {
    //     //
    //     //             },
    //     //             _ => panic!("add_generic_conversion: Unknown ptr ptr field {:?}", field_type),
    //     //         },
    //     //         Type::Path(TypePath { path, .. }) => {},
    //     //         Type::Array(TypeArray { elem, len, .. }) => {},
    //     //         _ => panic!("add_generic_conversion: Unknown ptr field {:?}", field_type),
    //     //     },
    //     //     Type::Path(TypePath { path, .. }) => {},
    //     //     _ => panic!("add_generic_conversion: Unknown field {:?}", field_type),
    //     // }
    //
    //     let (converted_field_to, converted_field_from, destructor) = match field_type {
    //         Type::Ptr(type_ptr) => (
    //             to_ptr(field_path_to, type_ptr),
    //             from_ptr(field_path_from, type_ptr),
    //             destroy_ptr(field_path_destroy, type_ptr)
    //         ),
    //         Type::Path(TypePath { path, .. }) => (
    //             to_path(field_path_to, path, None),
    //             from_path(field_path_from, path, None),
    //             destroy_path(field_path_destroy, path, None),
    //         ),
    //         Type::Reference(type_reference) => (
    //             to_reference(field_path_to, type_reference),
    //             from_reference(field_path_from, type_reference),
    //             destroy_reference(field_path_destroy, type_reference)
    //         ),
    //         Type::Array(type_array) => (
    //             to_array(field_path_to, type_array),
    //             from_array(field_path_from, type_array),
    //             destroy_array(field_path_destroy, type_array)
    //         ),
    //         _ => panic!("add_conversion: Unknown field {:?} {:?}", field_name, field_type),
    //     };
    //
    //     self.fields_from_composer.add_conversion(field_name.clone(), field_type);
    //     self.fields_to_composer.add_conversion(field_name.clone(), field_type);
    //     self.to_conversion_composer.add_conversion(field_name.clone(), converted_field_to);
    //     self.from_conversion_composer.add_conversion(field_name.clone(), converted_field_from);
    //     self.drop_composer.add_conversion(destructor);
    // }


    fn add_conversion(&mut self, field_type: &Type, field_name: TokenStream2) {
        // println!("add_conversion: {}: {}", quote!(#field_name), quote!(#field_type));
        let field_path_to = (self.to)(field_name.clone());
        let field_path_from = (self.from)(field_name.clone());
        let field_path_destroy = (self.destroy)(field_name.clone());
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

        self.fields_from_composer.add_conversion(field_name.clone(), field_type);
        self.fields_to_composer.add_conversion(field_name.clone(), field_type);
        self.to_conversion_composer.add_conversion(field_name.clone(), converted_field_to);
        self.from_conversion_composer.add_conversion(field_name.clone(), converted_field_from);
        self.drop_composer.add_conversion(destructor);
    }

    pub(crate) fn fields_from(&self) -> TokenStream2 {
        self.fields_from_composer.compose()
    }

    fn fields_to(&self) -> TokenStream2 {
        self.fields_to_composer.compose()
    }

    pub(crate) fn compose_doc(&self) -> TokenStream2 {
        self.doc_composer.compose()
    }

    pub(crate) fn compose_ffi_object(&self) -> TokenStream2 {
        self.ffi_object_composer.compose()
    }

    pub(crate) fn compose_from(&self) -> TokenStream2 {
        self.from_conversion_composer.compose()
    }

    pub(crate) fn compose_to(&self) -> TokenStream2 {
        self.to_conversion_composer.compose()
    }

    pub(crate) fn compose_destroy(&self) -> TokenStream2 {
        self.destroy_composer.compose()
    }

    pub(crate) fn compose_drop(&self) -> TokenStream2 {
        self.drop_composer.compose()
    }
    pub(crate) fn make_expansion(&self, input: TokenStream2) -> Expansion {
        Expansion::Full {
            input,
            comment: DocPresentation::Default(self.compose_doc()),
            ffi_presentation: FFIObjectPresentation::Full(self.compose_ffi_object()),
            conversion: ConversionInterfacePresentation::Interface {
                ffi_name: self.ffi_name_composer.compose(),
                target_name: self.target_name_composer.compose(),
                from_presentation: self.compose_from(),
                to_presentation: self.compose_to(),
                destroy_presentation: self.compose_destroy()
            },
            drop: DropInterfacePresentation::Full(
                self.ffi_name_composer.compose(),
                self.compose_drop()),
        }
    }
}

pub struct FFIContextComposer {
    parent: Option<Rc<RefCell<RootComposer>>>,
    composer: MapPresenter,
    context_presenter: ComposerPresenter<RootComposer>,
}
impl FFIContextComposer {
    pub const fn new(composer: MapPresenter, context_presenter: ComposerPresenter<RootComposer>) -> Self {
        Self { parent: None, composer, context_presenter }
    }
}
impl Composer for FFIContextComposer {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    fn compose(&self) -> TokenStream2 {
        let context = (self.context_presenter)(&*self.parent.as_ref().unwrap().borrow());
        (self.composer)(context)
    }
}

pub struct ConversionComposer {
    parent: Option<Rc<RefCell<RootComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<RootComposer>,

    conversions_presenter: OwnerIteratorPresenter,
    conversion_presenter: MapPairPresenter,
    path: TokenStream2,
    conversions: Vec<TokenStream2>,
}
impl ConversionComposer {
    pub fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<RootComposer>, conversions_presenter: OwnerIteratorPresenter, conversion_presenter: MapPairPresenter, path: TokenStream2, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, path, conversions }
    }
    pub fn add_conversion(&mut self, name: TokenStream2, conversion: TokenStream2) {
        self.conversions.push((self.conversion_presenter)(name, conversion));
    }
}
impl Composer for ConversionComposer {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.parent = Some(Rc::clone(root));
    }

    fn compose(&self) -> TokenStream2 {
        let context = (self.context_presenter)(&*self.parent.as_ref().unwrap().borrow());
        let conversions = (self.conversions_presenter)((self.path.to_token_stream(), self.conversions.clone()));
        let composition = (self.composer)(context, conversions);
        composition
    }
}

pub struct DropComposer {
    parent: Option<Rc<RefCell<RootComposer>>>,
    composer: MapPairPresenter,
    context_presenter: ComposerPresenter<RootComposer>,
    conversions_presenter: IteratorPresenter,
    conversion_presenter: MapPresenter,
    conversions: Vec<TokenStream2>,
}
impl DropComposer {
    pub const fn new(composer: MapPairPresenter, context_presenter: ComposerPresenter<RootComposer>, conversions_presenter: IteratorPresenter, conversion_presenter: MapPresenter, conversions: Vec<TokenStream2>) -> Self {
        Self { parent: None, composer, context_presenter, conversions_presenter, conversion_presenter, conversions }
    }

    pub fn add_conversion(&mut self, conversion: TokenStream2) {
        let value = (self.conversion_presenter)(conversion);
        self.conversions.push(value);
    }
}
impl Composer for DropComposer {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    fn compose(&self) -> TokenStream2 {
        let context = (self.context_presenter)(&*self.parent.as_ref().unwrap().borrow());
        let conversions = (self.conversions_presenter)(self.conversions.clone());
        (self.composer)(context, conversions)
    }
}


pub struct FieldsComposer {
    parent: Option<Rc<RefCell<RootComposer>>>,
    context_presenter: ComposerPresenter<RootComposer>,
    pub root_presenter: OwnerIteratorPresenter,
    pub field_presenter: FieldTypedPresenter,

    pub fields: Vec<TokenStream2>,
}
impl FieldsComposer {
    pub const fn new(root_presenter: OwnerIteratorPresenter, context_presenter: ComposerPresenter<RootComposer>, field_presenter: FieldTypedPresenter, fields: Vec<TokenStream2>) -> Self {
        Self { parent: None, root_presenter, context_presenter, field_presenter, fields }
    }

    pub fn add_conversion(&mut self, name: TokenStream2, field_type: &Type) {
        let value = (self.field_presenter)(name, field_type);
        self.fields.push(value);
    }
}
impl Composer for FieldsComposer {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    fn compose(&self) -> TokenStream2 {
        let context = (self.context_presenter)(&*self.parent.as_ref().unwrap().borrow());
        (self.root_presenter)((context, self.fields.clone()))
    }
}

pub struct NameComposer {
    pub parent: Option<Rc<RefCell<RootComposer>>>,
    pub name: TokenStream2,
}

impl NameComposer {
    pub const fn new(name: TokenStream2) -> Self {
        Self { parent: None, name }
    }

}
impl Composer for NameComposer {
    fn set_parent(&mut self, root: &Rc<RefCell<RootComposer>>) {
        self.parent = Some(Rc::clone(root));
    }
    fn compose(&self) -> TokenStream2 {
        self.name.clone()
    }
}


