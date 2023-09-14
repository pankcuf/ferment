use std::collections::HashSet;
use syn::{Attribute, BareFnArg, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemType, Pat, Path, PathSegment, PatIdent, PatType, ReturnType, Signature, Type, TypeBareFn, TypePath, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::generics::{add_generic_type, TypePathComposition};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_FIELDS_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, ENUM_DESTROY_PRESENTER, ENUM_NAMED_VARIANT_PRESENTER, ENUM_PRESENTER, ENUM_UNIT_FIELDS_PRESENTER, ENUM_UNNAMED_VARIANT_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, FFI_TYPE_PRESENTER, GENERIC_MAP_COMPLEX_PRESENTER, GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER, GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER, GENERIC_MAP_SIMPLE_PRESENTER, GENERIC_VEC_COMPLEX_PRESENTER, GENERIC_VEC_SIMPLE_PRESENTER, MATCH_FIELDS_PRESENTER, NAMED_CONVERSION_PRESENTER, NAMED_VARIANT_FIELD_PRESENTER, NO_FIELDS_PRESENTER, obj, package_unboxed_root, Presentable, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_PAIR_PRESENTER, UNNAMED_VARIANT_FIELD_PRESENTER};
use crate::{ffi_struct_name, from_path, mangle_type, path_arguments_to_path_conversions, path_arguments_to_types, usize_to_tokenstream};
use crate::composer::RootComposer;
use crate::path_conversion::PathConversion;
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation};

pub enum ItemConversion {
    Mod(ItemMod),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Type(ItemType),
    Fn(ItemFn)
}

impl ItemConversion {
    pub const fn r#type(&self) -> &str {
        match self {
            Self::Mod(..) => "ffi_dictionary",
            Self::Struct(..) | Self::Enum(..) => "impl_ffi_conv",
            Self::Type(..) => "impl_ffi_ty_conv",
            Self::Fn(..) => "impl_ffi_fn_conv",
        }
    }
    fn ident(&self) -> Ident {
        format_ident!("{}", self.r#type())
    }

    fn is_labeled_with_macro(&self, path: &Path) -> bool {
        path.segments
            .iter()
            .any(|segment| segment.ident == self.ident())
    }

    fn handle_attributes_with_handler<F: FnMut(&Path)>(&self, attrs: &[Attribute], mut handler: F) {
        attrs.iter()
            .for_each(|Attribute { path, .. }|
                if self.is_labeled_with_macro(path) { handler(path) })
    }
}

impl From<ItemConversion> for Expansion {
    fn from(conversion: ItemConversion) -> Self {
        match &conversion {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item_struct) => struct_expansion(item_struct),
            ItemConversion::Enum(item_enum) => enum_expansion(item_enum),
            ItemConversion::Type(item_type) => type_expansion(item_type),
            ItemConversion::Fn(item_fn) => item_fn_expansion(item_fn)
        }
    }
}

impl From<DeriveInput> for ItemConversion {
    fn from(input: DeriveInput) -> Self {
        let DeriveInput { attrs, vis, ident, generics, data } = input;
        match data {
            Data::Struct(DataStruct { fields,  semi_token, struct_token, .. }) =>
                ItemConversion::Struct(ItemStruct { attrs, vis, ident, generics, fields, semi_token, struct_token }),
            Data::Enum(DataEnum { enum_token, brace_token, variants }) =>
                ItemConversion::Enum(ItemEnum { attrs, vis, ident, generics, variants, enum_token, brace_token }),
            Data::Union(DataUnion { union_token, fields }) =>
                unimplemented!("Unions are not supported yet {:?}: {:?}", union_token, fields),
        }
    }
}

impl<'a> TryFrom<&'a Item> for ItemConversion {
    type Error = ();
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        match value {
            Item::Mod(item_mod) => Ok(Self::Mod(item_mod.clone())),
            Item::Struct(item_struct) => Ok(Self::Struct(item_struct.clone())),
            Item::Enum(item_enum) => Ok(Self::Enum(item_enum.clone())),
            Item::Type(item_type) => Ok(Self::Type(item_type.clone())),
            Item::Fn(item_fn) => Ok(Self::Fn(item_fn.clone())),
            _ => Err(())
        }
    }
}

impl ItemConversion {

    pub fn collect_all_items(&self) -> Vec<ItemConversion> {
        let mut all_labeled_items: Vec<ItemConversion> = Vec::new();
        match self {
            Self::Mod(ItemMod { content: Some((_, items)), .. }) =>
                items.iter()
                    .flat_map(|m| Self::try_from(m))
                    .for_each(|conversion| all_labeled_items.push(conversion)),
            _ => {}
            // &value => all_labeled_items.push(value)
        }
        all_labeled_items
    }

    pub fn collect_compositions(&self) -> Vec<TypePathComposition> {
        let mut type_and_paths: Vec<TypePathComposition> = Vec::new();
        let mut cache_type = |ty: &Type, path: &Path|
            type_and_paths.push(TypePathComposition(ty.clone(), path.clone()));
        let mut cache_fields = |fields: &Fields, path: &Path| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| cache_type(&field.ty, path)),
            Fields::Unit => {}
        };

        match self {
            Self::Mod(ItemMod { content: Some((_, items)), .. }) =>
                items.iter()
                    .flat_map(|m| Self::try_from(m))
                    .for_each(|conversion| type_and_paths.extend(conversion.collect_compositions())),
            Self::Struct(item_struct) =>
                self.handle_attributes_with_handler(&item_struct.attrs, |path| cache_fields(&item_struct.fields, path)),
            Self::Enum(item_enum) =>
                self.handle_attributes_with_handler(&item_enum.attrs, |path| item_enum.variants.iter().for_each(|Variant { fields, .. }| cache_fields(&fields, path))),
            Self::Type(ItemType { attrs, ty, .. }) =>
                self.handle_attributes_with_handler(attrs, |path| cache_type(ty, path)),
            Self::Fn(item_fn) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |path| {
                    item_fn.sig.inputs.iter().for_each(|arg| match arg {
                        FnArg::Typed(PatType { ty, .. }) => cache_type(ty, path),
                        _ => {}
                    });
                    match &item_fn.sig.output {
                        ReturnType::Default => {},
                        ReturnType::Type(_, ty) => match &**ty {
                            Type::Path(TypePath { path, .. }) => cache_type(ty, path),
                            _ => {}
                        }
                    };
                }),
            _ => panic!("WWWWWW")
        }

        type_and_paths
    }

    #[allow(unused)]
    fn find_custom_types_in_compositions(&self, compositions: &Vec<TypePathComposition>) -> HashSet<TypePathComposition> {
        let mut custom_types: HashSet<TypePathComposition> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypePathComposition(field_type, .. )| match field_type {
                Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                    PathConversion::Complex(path) => match path.segments.last().unwrap().ident.to_string().as_str() {
                        "str" | "String" => {},
                        "Option" => match path_arguments_to_types(&path.segments.last().unwrap().arguments)[0] {
                            Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                                PathConversion::Complex(path) => match path.segments.last().unwrap().ident.to_string().as_str() {
                                    "str" | "String" => {},
                                    _ => { custom_types.insert(TypePathComposition(field_type.clone(), path.clone())); }
                                },
                                _ => {},
                            },
                            _ => {},
                        },
                        _ => { custom_types.insert(TypePathComposition(field_type.clone(), path.clone())); }
                    },
                    _ => {}
                },
                _ => {}
            });
        let unique_types = custom_types.iter().map(|TypePathComposition { 0: ty, .. }| quote!(#ty)).collect::<Vec<_>>();
        println!("Unique custom types: {}", quote!(#(#unique_types, )*));
        custom_types
    }

    fn find_generic_types_in_compositions(&self, compositions: &Vec<TypePathComposition>) -> HashSet<TypePathComposition> {
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypePathComposition> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypePathComposition(field_type, .. )| match field_type {
                Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                    PathConversion::Vec(path) | PathConversion::Map(path) => {
                        path_arguments_to_types(&path.segments.last().unwrap().arguments)
                            .iter()
                            .for_each(|field_type| add_generic_type(field_type, &mut generics));
                        generics.insert(TypePathComposition(field_type.clone(), path.clone()));
                    },
                    _ => {}
                },
                _ => {}
            });
        generics
    }

    #[allow(unused)]
    fn find_generic_types(&self) -> HashSet<TypePathComposition> {
        self.find_generic_types_in_compositions(&self.collect_compositions())
    }

    // fn find_all_types(&self) -> HashSet<TypePathComposition> {
    //     let all_macro_labeled_types = self.collect_compositions();
    //     let mut custom_types = self.find_custom_types_in_compositions(&all_macro_labeled_types);
    //     let generics = self.find_generic_types_in_compositions(&all_macro_labeled_types);
    //     custom_types.extend(generics);
    //     custom_types
    // }
    pub fn expand_all_types(&self) -> Vec<TokenStream2> {
        let mut custom_items = self.collect_all_items()
            .into_iter()
            .map(|item| Expansion::from(item).present())
            .collect::<Vec<_>>();
        custom_items.extend(self.expand_types(self.find_generic_types()));
        custom_items
    }

    #[allow(unused)]
    pub fn expand_generic_types(&self) -> Vec<TokenStream2> {
        self.expand_types(self.find_generic_types())
    }

    pub fn expand_types(&self, types: HashSet<TypePathComposition>) -> Vec<TokenStream2> {
        types.into_iter()
            .map(|TypePathComposition { 0:ty, 1: path }| {
                let mangled_type = mangle_type(&ty);
                let ffi_name = ffi_struct_name(&mangled_type).to_token_stream();
                let PathSegment { ident, arguments} = path.segments.last().unwrap();
                match ident.to_string().as_str() {
                    "Vec" | "BTreeMap" | "HashMap" => match &path_arguments_to_path_conversions(arguments)[..] {
                        [PathConversion::Simple(value_path)] =>
                            GENERIC_VEC_SIMPLE_PRESENTER((ffi_name, value_path)),
                        [PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
                            GENERIC_VEC_COMPLEX_PRESENTER((ffi_name, value_path)),
                        [PathConversion::Simple(key_path), PathConversion::Simple(value_path)] =>
                            GENERIC_MAP_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
                        [PathConversion::Simple(key_path), PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
                            GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
                        [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path), PathConversion::Simple(value_path)] =>
                            GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
                        [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path), PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
                            GENERIC_MAP_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
                        _ => unimplemented!("Generic path arguments conversion error"),
                    },
                    _ => quote!(),
                }
            })
            .collect::<Vec<_>>()
    }
}

fn enum_expansion(item_enum: &ItemEnum) -> Expansion {
    let ItemEnum { ident: target_name, variants, .. } = item_enum;
    let variants_count = variants.len();
    let ffi_name = ffi_struct_name(target_name);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    let mut drop_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, .. }| {
        let target_variant_path = quote!(#target_name::#variant_name);
        let ffi_variant_path = quote!(#ffi_name::#variant_name);
        let (variant_presenter, fields_context) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (ENUM_UNIT_FIELDS_PRESENTER, vec![quote!(#lit)]),
            None => match fields {
                Fields::Unit => (NO_FIELDS_PRESENTER, vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    ENUM_UNNAMED_VARIANT_PRESENTER,
                    unnamed
                        .iter()
                        .map(UNNAMED_VARIANT_FIELD_PRESENTER)
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    ENUM_NAMED_VARIANT_PRESENTER,
                    named.iter().map(NAMED_VARIANT_FIELD_PRESENTER).collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        // let ffi_object_presenter = |_| variant_presenter((quote!(#variant_name), fields_context));
        let composer = match fields {
            Fields::Unit => RootComposer::enum_unit_variant_composer(
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                |_| quote!(),
            ),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                RootComposer::enum_unnamed_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    |_| quote!(),
                    unnamed.iter().enumerate().map(|(index, Field { ty, .. })| {
                        (ty, format_ident!("o_{}", index).to_token_stream())
                    }),
                )
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                RootComposer::enum_named_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    |_| quote!(),
                    named.iter().map(|Field { ident, ty, .. }| {
                        (ty, ident.clone().unwrap().to_token_stream())
                    }),
                )
            }
        };
        let composer_owned = composer.borrow();
        variants_fields.push(variant_presenter((quote!(#variant_name), fields_context)));
        conversions_from_ffi.push(composer_owned.compose_from());
        conversions_to_ffi.push(composer_owned.compose_to());
        destroy_fields.push(composer_owned.compose_destroy());
        drop_fields.push(composer_owned.compose_drop());
    },
    );
    let input = quote!(#item_enum);
    let comment = DocPresentation::Default(quote!(#target_name));
    let ffi_presentation =
        FFIObjectPresentation::Full(ENUM_PRESENTER((quote!(#ffi_name), variants_fields)));
    let conversion = ConversionInterfacePresentation::Interface {
        ffi_name: quote!(#ffi_name),
        target_name: quote!(#target_name),
        from_presentation: FFI_FROM_ROOT_PRESENTER(
            quote!(&*ffi),
            MATCH_FIELDS_PRESENTER((quote!(ffi_ref), conversions_from_ffi)),
        ),
        to_presentation: FFI_TO_ROOT_PRESENTER(
            quote!(),
            MATCH_FIELDS_PRESENTER((quote!(obj), conversions_to_ffi)),
        ),
        destroy_presentation: package_unboxed_root(),
    };
    let drop = DropInterfacePresentation::Full(
        // DROP_INTERFACE_PRESENTER,
        quote!(#ffi_name),
        ENUM_DESTROY_PRESENTER(drop_fields),
    );
    Expansion::Full { input, comment, ffi_presentation, conversion, drop }
    // expansion(
    //     quote!(#item_enum),
    //     comment.present(),
    //     ffi_converted_input.present(),
    //     ffi_conversion_presentation.present(),
    //     drop_presentation.present(),
    // )
}

fn struct_expansion(item_struct: &ItemStruct) -> Expansion {
    let ItemStruct { fields: ref f, ident: target_name, .. } = item_struct;
    let composer = match f {
        Fields::Unnamed(ref fields) => match target_name.clone().to_string().as_str() {
            // Hack used to simplify some structures
            // Main problem here that without special dictionary of predefined non-std structures
            // we unable to filter out structures and provide them conversions when they are used as field types inside parent structures
            // Solution would be to write build script to preprocess and collect dictionary before macro expansion
            // in order to match struct field types with this predefined dictionary
            "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768"
            | "VarInt" => {
                let (
                    ffi_name,
                    ffi_from_presenter,
                    ffi_from_presentation_context,
                    ffi_to_presenter,
                    ffi_to_presentation_context,
                    destroy_code_context_presenter,
                ) = match fields.unnamed.first().unwrap().ty.clone() {
                    // VarInt
                    Type::Path(TypePath { path, .. }) => {
                        let ffi_name = ffi_struct_name(&target_name);
                        (
                            quote!(#ffi_name),
                            CURLY_BRACES_FIELDS_PRESENTER,
                            vec![from_path(quote!(ffi_ref.0), &path, None)],
                            CURLY_BRACES_FIELDS_PRESENTER,
                            quote!(#ffi_name),
                            ROOT_DESTROY_CONTEXT_PRESENTER,
                        )
                    }
                    // UInt256 etc
                    Type::Array(type_array) => (
                        quote!(#type_array),
                        ROUND_BRACES_FIELDS_PRESENTER,
                        vec![quote!(ffi_ref)],
                        NO_FIELDS_PRESENTER,
                        quote!(obj.0),
                        EMPTY_MAP_PRESENTER,
                    ),
                    _ => unimplemented!(
                        "from_unnamed_struct: not supported {:?}",
                        quote!(#fields)
                    ),
                };
                RootComposer::primitive_composer(
                    quote!(#ffi_name),
                    quote!(#target_name),
                    EMPTY_FIELDS_PRESENTER,
                    ffi_from_presenter,
                    ffi_to_presenter,
                    destroy_code_context_presenter,
                    EMPTY_PAIR_PRESENTER,
                    ffi_from_presentation_context,
                    ffi_to_presentation_context,
                )
            }
            _ => RootComposer::unnamed_struct_composer(
                ffi_struct_name(target_name).to_token_stream(),
                quote!(#target_name),
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })| (ty, usize_to_tokenstream(index))),
            ),
        },
        Fields::Named(ref fields) => RootComposer::named_struct_composer(
            ffi_struct_name(target_name).to_token_stream(),
            quote!(#target_name),
            fields
                .named
                .iter()
                .map(|Field { ident, ty, .. }| (ty, quote!(#ident))),
        ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    };
    let composer_owned = composer.borrow();
    composer_owned.make_expansion(quote!(#item_struct))
}

fn item_fn_expansion(item_fn: &ItemFn) -> Expansion {
    let Signature {
        output,
        ident: fn_name,
        inputs,
        ..
    } = &item_fn.sig;
    let (output_expression, output_conversions) = match output {
        ReturnType::Default => (quote!(()), quote!(;)),
        ReturnType::Type(_, field_type) => (
            FFI_TYPE_PRESENTER(&field_type),
            match &**field_type {
                Type::Path(TypePath { path, .. }) => crate::to_path(quote!(obj), &path, None),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            },
        ),
    };
    let (fn_args, conversions) = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(PatType { ty, pat, .. }) => (
                NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), FFI_TYPE_PRESENTER(&ty)),
                match (&**ty, &**pat) {
                    (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) => {
                        from_path(quote!(#ident), &path, None)
                    }
                    _ => panic!("error: Arg conversion not supported: {:?}", quote!(#ty)),
                },
            ),
            _ => panic!("Arg type not supported: {:?}", quote!(#arg)),
        })
        .unzip();

    Expansion::Function {
        input: quote!(#item_fn),
        comment: DocPresentation::Safety(quote!(#fn_name)),
        ffi_presentation: FFIObjectPresentation::Function {
            name_and_arguments: ROUND_BRACES_FIELDS_PRESENTER((
                format_ident!("ffi_{}", fn_name).to_token_stream(),
                fn_args,
            )),
            input_conversions: ROUND_BRACES_FIELDS_PRESENTER((quote!(#fn_name), conversions)),
            output_expression,
            output_conversions,
        },
    }
}

fn type_expansion(item_type: &ItemType) -> Expansion {
    let ItemType { ident, ty, .. } = item_type;
    let ffi_name = format_ident!("{}_FFI", ident).to_token_stream();
    match &**ty {
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            Expansion::Callback {
                input: quote!(#item_type),
                comment: DocPresentation::Default(ffi_name.clone()),
                ffi_presentation: FFIObjectPresentation::Callback {
                    name: ffi_name,
                    arguments: inputs
                        .iter()
                        .map(|BareFnArg { ty: field_type, name, .. }|
                            NAMED_CONVERSION_PRESENTER(name.clone().unwrap().0.to_token_stream(), FFI_TYPE_PRESENTER(field_type)))
                        .collect::<Vec<_>>(),
                    output_expression: match output {
                        ReturnType::Default => quote!(),
                        ReturnType::Type(token, field_type) =>
                            SIMPLE_PAIR_PRESENTER(quote!(#token), FFI_TYPE_PRESENTER(&field_type))
                    },
                }
            }
        },
        _ => RootComposer::type_alias_composer(
            ffi_name,
            quote!(#ident),
            IntoIterator::into_iter(vec![(&**ty, match &**ty {
                Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                    PathConversion::Simple(..) => obj(),
                    _ => usize_to_tokenstream(0),
                },
                Type::Array(_type_array) => usize_to_tokenstream(0),
                Type::Ptr(_type_ptr) => obj(),
                _ => unimplemented!("from_type_alias: not supported {:?}", quote!(#ty)) })]))
            .borrow()
            .make_expansion(quote!(#item_type))
    }
}