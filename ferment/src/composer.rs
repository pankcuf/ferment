use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Field, FieldsNamed, FieldsUnnamed, Path, Type, TypePath};
use crate::interface::{DEFAULT_DESTROY_FIELDS_PRESENTER, DEFAULT_DICT_FIELD_PRESENTER, DEFAULT_DICT_FIELD_TYPE_PRESENTER, DEFAULT_DOC_PRESENTER, DEREF_FIELD_PATH, FFI_DEREF_FIELD_NAME, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, IteratorPresenter, LAMBDA_CONVERSION_PRESENTER, MapPairPresenter, MapPresenter, OBJ_FIELD_NAME, OwnerIteratorPresenter, package_unbox_any_expression, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_ITER_PRESENTER, ScopeTreeFieldTypedPresenter, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, STRUCT_DESTROY_PRESENTER, TYPE_ALIAS_CONVERSION_FROM_PRESENTER, TYPE_ALIAS_CONVERSION_TO_PRESENTER, TYPE_ALIAS_PRESENTER};
use crate::interface::{obj};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation, TraitVTablePresentation};
use crate::helper::{destroy_path, destroy_ptr, destroy_reference, ffi_destructor_name, ffi_unnamed_arg_name, from_array, from_path, from_ptr, from_reference, to_array, to_path, to_ptr, to_reference};
use crate::item_conversion::{ItemContext, usize_to_tokenstream};
use crate::path_conversion::PathConversion;

/// Composer Context Presenters
pub type ComposerPresenter<C> = fn(context: &C) -> TokenStream2;

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

#[derive(Clone)]
pub enum FieldType {
    Named(Type, TokenStream2),
    Unnamed(Type, TokenStream2),
    //Itself(Type, TokenStream2),
}
impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldType::Named(ty, field_name) => quote!(#field_name: #ty),
            FieldType::Unnamed(ty, index) => quote!(#index: #ty),
            //FieldType::Itself(ty, field_name) => quote!(#field_name: #ty),
        }.to_tokens(tokens)
    }
}

impl FieldType {
    pub fn ty(&self) -> &Type {
        match self {
            FieldType::Named(ty, _) => ty,
            FieldType::Unnamed(ty, _) => ty,
            //FieldType::Itself(ty, _) => ty
        }
    }
    pub fn name(&self) -> TokenStream2 {
        match self {
            FieldType::Named(_, field_name) => field_name.clone(),
            FieldType::Unnamed(_, field_name) => field_name.clone(),
            //FieldType::Itself(_, field_name) => field_name.clone()
        }

    }
}

impl<'a> ConversionsComposer<'a> {
    pub fn compose(&self, context: &ItemContext) -> Vec<FieldType> {
        match self {
            Self::Empty => vec![],
            Self::NamedStruct(fields) =>
                fields
                    .named
                    .iter()
                    .map(|Field { ident, ty, .. }|
                        FieldType::Named(context.full_type_for(ty), quote!(#ident)))
                    .collect(),
            Self::UnnamedEnumVariant(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldType::Unnamed(context.full_type_for(ty), ffi_unnamed_arg_name(index).to_token_stream()))
                        // (context.full_type_for(ty), ffi_unnamed_arg_name(index).to_token_stream()))
                    .collect(),
            Self::UnnamedStruct(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldType::Unnamed(context.full_type_for(ty), unnamed_struct_fields_comp(ty, index)))
                        // (context.full_type_for(ty), unnamed_struct_fields_comp(ty, index)))
                    .collect(),
            Self::TypeAlias(ty) =>
                vec![FieldType::Unnamed(context.full_type_for(ty), unnamed_struct_fields_comp(ty, 0))],
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
    STRUCT_DESTROY_PRESENTER,
    SIMPLE_PRESENTER,
    vec![]);
pub const DEFAULT_DOC_COMPOSER: FFIContextComposer = FFIContextComposer::new(
    DEFAULT_DOC_PRESENTER,
    |composer| composer.target_name_composer.compose());

pub const FROM_DEREF_FFI_CONTEXT_BY_ADDR_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(&*ffi);
// pub const FROM_DEREF_FFI_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(*ffi);
pub const TO_OBJ_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!(obj);
pub const EMPTY_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |_| quote!();
pub const CONVERSION_FIELDS_FROM_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.fields_from();
pub const CONVERSION_FIELDS_TO_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.fields_to();
pub const FFI_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.ffi_name_composer.compose();
pub const TARGET_NAME_CONTEXT_PRESENTER: ComposerPresenter<ItemComposer> = |composer| composer.target_name_composer.compose();

pub trait Composer where Self: Sized {
    type Item: ToTokens;
    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>);
    fn compose(&self) -> Self::Item;
}

pub struct ItemComposer {
    pub context: ItemContext,
    // pub tree: HashMap<TypeConversion, Type>,
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

    pub(crate) fn type_alias_composer(
        ffi_name: Path,
        target_name: Path,
        context: ItemContext,
        conversions_composer: ConversionsComposer
    ) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
            context,
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
                    target_name.to_token_stream(),
                    vec![]),
                ConversionComposer::new(
                    FFI_TO_ROOT_PRESENTER,
                    TO_OBJ_CONTEXT_PRESENTER,
                    TYPE_ALIAS_CONVERSION_TO_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    ffi_name.to_token_stream(),
                    vec![]),
                DESTROY_STRUCT_COMPOSER,
                DROP_STRUCT_COMPOSER,
                |_| quote!(ffi_ref.0),
                |_| obj(),
                FFIBindingsComposer::new(
                    ROUND_ITER_PRESENTER
                ),
                FFI_DEREF_FIELD_NAME),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn struct_composer(
        ffi_name: Path,
        target_name: Path,
        context: ItemContext,
        root_presenter: OwnerIteratorPresenter,
        field_presenter: ScopeTreeFieldTypedPresenter,
        root_conversion_presenter: OwnerIteratorPresenter,
        conversion_presenter: MapPairPresenter,
        bindings_presenter: IteratorPresenter,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<Self>> {
        Self::new(
            ffi_name.clone(),
            target_name.clone(),
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
                FFIBindingsComposer::new(
                    bindings_presenter,
                ),
                FFI_DEREF_FIELD_NAME
            ),
            conversions_composer
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enum_variant_default_composer(
        ffi_name: Path,
        target_name: Path,
        context: ItemContext,
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
            context,
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
                    DEFAULT_DESTROY_FIELDS_PRESENTER,
                    destroy_presenter,
                    vec![]),
                DEREF_FIELD_PATH,
                SIMPLE_PRESENTER,
                FFIBindingsComposer::new(bindings_iterator_presenter),
                |f| quote!(#f.to_owned())),
            conversions_composer)
    }

    // #[allow(clippy::too_many_arguments)]
    // pub(crate) fn primitive_composer(
    //     ffi_name: Path,
    //     target_name: Path,
    //     context: ItemContext,
    //     root_presenter: OwnerIteratorPresenter,
    //     root_conversion_from_presenter: OwnerIteratorPresenter,
    //     root_conversion_to_presenter: OwnerIteratorPresenter,
    //     destroy_code_context_presenter: MapPresenter,
    //     drop_presenter: MapPairPresenter,
    //     conversions_from: Vec<TokenStream2>,
    //     conversion_to_path: TokenStream2) -> Rc<RefCell<Self>> {
    //     let root = Rc::new(RefCell::new(Self {
    //         need_drop_presentation: false,
    //         context,
    //         ffi_name_composer: NameComposer::new(ffi_name.clone()),
    //         target_name_composer: NameComposer::new(target_name.clone()),
    //         ffi_object_composer: FFIContextComposer::new(
    //             SIMPLE_PRESENTER,
    //             EMPTY_CONTEXT_PRESENTER),
    //         ffi_conversions_composer: FFIConversionComposer::new(
    //             ConversionComposer::new(
    //                 FFI_FROM_ROOT_PRESENTER,
    //                 FROM_DEREF_FFI_CONTEXT_PRESENTER,
    //                 root_conversion_from_presenter,
    //                 EMPTY_PAIR_PRESENTER,
    //                 target_name.to_token_stream(),
    //                 conversions_from),
    //             ConversionComposer::new(
    //                 FFI_TO_ROOT_PRESENTER,
    //                 EMPTY_CONTEXT_PRESENTER,
    //                 root_conversion_to_presenter,
    //                 EMPTY_PAIR_PRESENTER,
    //                 conversion_to_path,
    //                 vec![]),
    //             FFIContextComposer::new(
    //                 destroy_code_context_presenter,
    //                 EMPTY_CONTEXT_PRESENTER),
    //             DropComposer::new(
    //                 drop_presenter,
    //                 EMPTY_CONTEXT_PRESENTER,
    //                 EMPTY_ITERATOR_PRESENTER,
    //                 EMPTY_MAP_PRESENTER,
    //                 vec![]),
    //             EMPTY_MAP_PRESENTER,
    //             EMPTY_MAP_PRESENTER,
    //             FFIBindingsComposer::new(
    //                 EMPTY_ITERATOR_PRESENTER,
    //             ),
    //             EMPTY_MAP_PRESENTER
    //         ),
    //         doc_composer: DEFAULT_DOC_COMPOSER,
    //         fields_from_composer: FieldsComposer::new(
    //             root_presenter,
    //             FFI_NAME_CONTEXT_PRESENTER,
    //             EMPTY_DICT_FIELD_TYPED_PRESENTER,
    //             vec![]),
    //         fields_to_composer: FieldsComposer::new(
    //             root_presenter,
    //             TARGET_NAME_CONTEXT_PRESENTER,
    //             EMPTY_DICT_FIELD_TYPED_PRESENTER,
    //             vec![])
    //     }));
    //     {
    //         let mut root_borrowed = root.borrow_mut();
    //         root_borrowed.setup_composers(&root);
    //     }
    //     root
    // }


    #[allow(clippy::too_many_arguments)]
    fn new(
        ffi_name: Path,
        target_name: Path,
        context: ItemContext,
        root_presenter: OwnerIteratorPresenter,
        doc_composer: FFIContextComposer,
        field_presenter: ScopeTreeFieldTypedPresenter,
        ffi_object_composer: FFIContextComposer,
        ffi_conversions_composer: FFIConversionComposer,
        conversions_composer: ConversionsComposer) -> Rc<RefCell<ItemComposer>> where Self: Sized {

        let root = Rc::new(RefCell::new(Self {
            need_drop_presentation: true,
            context,
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

    fn add_conversion(&mut self, field_type: FieldType) {
        self.ffi_conversions_composer.add_conversion(&field_type);
        self.fields_from_composer.add_conversion(&field_type, &self.context);
        self.fields_to_composer.add_conversion(&field_type, &self.context);
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
    pub(crate) fn make_expansion(&self, traits: Vec<TraitVTablePresentation>) -> Expansion {
        let ffi_name = self.ffi_name_composer.compose();
        // TODO: avoid this
        let ffi_ident = format_ident!("{}", ffi_name.to_string());
        let target_name = self.target_name_composer.compose();
        Expansion::Full {
            comment: DocPresentation::Default(self.doc_composer.compose()),
            ffi_presentation: FFIObjectPresentation::Full(self.ffi_object_composer.compose()),
            conversion: ConversionInterfacePresentation::Interface {
                ffi_type: ffi_name.clone(),
                target_type: target_name.clone(),
                from_presentation: FromConversionPresentation::Struct(self.compose_from()),
                to_presentation: ToConversionPresentation::Struct(self.compose_to()),
                destroy_presentation: self.compose_destroy()
            },
            bindings: vec![
                BindingPresentation::Constructor {
                    ffi_ident: ffi_ident.clone(),
                    ctor_arguments: self.ffi_conversions_composer.bindings_composer.compose_arguments(&self.context),
                    body_presentation: self.ffi_conversions_composer.bindings_composer.compose_field_names()
                },
                BindingPresentation::Destructor {
                    ffi_name: ffi_name.clone(),
                    destructor_ident: ffi_destructor_name(&ffi_ident).to_token_stream()
                }
            ],
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
        destroy_composer:FFIContextComposer,
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
    pub fn add_conversion(&mut self, field_type: &FieldType) {
        let field_path_to = (self.to_presenter)(field_type.name());
        let field_path_from = (self.from_presenter)(field_type.name());
        let field_path_destroy = (self.destructor_presenter)(field_type.name());

        let (converted_field_to, converted_field_from, destructor) = match field_type.ty() {
            Type::Ptr(type_ptr) => (
                to_ptr(field_path_to, type_ptr),
                from_ptr(field_path_from, type_ptr),
                destroy_ptr(field_path_destroy, type_ptr)
            ),
            Type::Path(TypePath { path, .. }) => (
                to_path(field_path_to, path),
                from_path(field_path_from, path),
                destroy_path(field_path_destroy, path),
            ),
            Type::Reference(type_reference) => (
                to_reference(field_path_to, type_reference),
                from_reference(field_path_from, type_reference),
                destroy_reference(field_path_destroy, type_reference)
            ),
            Type::Array(type_array) => (
                to_array(field_path_to, type_array),
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
    field_types: Vec<FieldType>,
}

impl FFIBindingsComposer {
    pub const fn new(root_presenter: IteratorPresenter) -> Self {
        Self { parent: None, root_presenter, field_types: vec![] }
    }
    pub(crate) fn add_conversion(&mut self, field_type: &FieldType) {
        self.field_types.push(field_type.clone());
    }

    pub fn compose_field_names(&self) -> TokenStream2 {
        let fields = self.field_types.iter().map(|field_type| match field_type {
            FieldType::Named(_ty, name) => quote!(#name),
            FieldType::Unnamed(_ty, name) => format_ident!("o_{}", name.to_string()).to_token_stream(),
            //FieldType::Itself(_ty, name) => quote!(#name)
        }).collect::<Vec<_>>();
        (self.root_presenter)(fields.clone())
    }

    pub fn compose_arguments(&self, context: &ItemContext) -> Vec<TokenStream2> {
        self.field_types.iter()
            .map(|field_type| match field_type {
                FieldType::Named(_ty, name) => {
                    let full_ty = DEFAULT_DICT_FIELD_TYPE_PRESENTER(field_type, context);
                    quote!(#name: #full_ty)
                },
                FieldType::Unnamed(_ty, name) => {
                    let field_name = format_ident!("o_{}", name.to_string());
                    let full_ty = DEFAULT_DICT_FIELD_TYPE_PRESENTER(field_type, context);
                    quote!(#field_name: #full_ty)
                },
                // FieldType::Itself(_ty, name) => {
                //     let full_ty = DEFAULT_DICT_FIELD_TYPE_PRESENTER(field_type, context);
                //     quote!(#name: #full_ty)
                // },
            })
            .collect()
    }
}

impl Composer for FFIBindingsComposer {
    type Item = TokenStream2;

    fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
        self.parent = Some(Rc::clone(root));
    }

    fn compose(&self) -> Self::Item {
        (self.root_presenter)(self.field_types.iter().map(|ff| ff.name()).collect::<Vec<_>>())
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

    pub fn add_conversion(&mut self, field_type: &FieldType, context: &ItemContext) {
        // let ffi_type = self.context.ffi_full_type_for(&field_type.ty());
        let value = (self.field_presenter)(field_type, context);
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
#[macro_export]
macro_rules! composer_impl {
    ($name:ident, $composition:expr) => {
        impl Composer for $name {
            type Item = TokenStream2;
            fn set_parent(&mut self, root: &Rc<RefCell<ItemComposer>>) {
                self.parent = Some(Rc::clone(root));
            }
            #[allow(clippy::redundant_closure_call)]
            fn compose(&self) -> Self::Item {
                $composition(self)
            }
        }
    }
}

composer_impl!(FFIContextComposer, |context_composer: &FFIContextComposer|
    (context_composer.composer)(
        (context_composer.context_presenter)(
            &context_composer.parent.as_ref().unwrap().borrow())));
composer_impl!(ConversionComposer, |itself: &ConversionComposer|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            (itself.path.to_token_stream(), itself.conversions.clone()))));

composer_impl!(DropComposer, |itself: &DropComposer|
    (itself.composer)(
        (itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
        (itself.conversions_presenter)(
            itself.conversions.clone())));
composer_impl!(FieldsComposer, |itself: &FieldsComposer|
    (itself.root_presenter)(
        ((itself.context_presenter)(
            &itself.parent.as_ref().unwrap().borrow()),
            itself.fields.clone())));
composer_impl!(NameComposer, |itself: &NameComposer|
    itself.name.to_token_stream());

// composer_impl!(FFIBindingsComposer, |itself: &FFIBindingsComposer|
//     (itself.root_presenter)(
//         ((itself.context_presenter)(
//             &itself.parent.as_ref().unwrap().borrow()),
//             itself.fields.clone())));
