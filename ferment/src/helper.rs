// use std::vec;
// use syn::__private::Span;
// use syn::{Field, Ident, Path, Type, TypePath, TypePtr};
// use syn::token::Mut;
// use crate::interface::FFI_TYPE_PATH_CONVERTER;
// use crate::mangle_path;
// use crate::path_conversion::PathConversion;
//
// #[allow(unused)]
// fn create_field(name: &str, ty: Type) -> Field {
//     // println!("create_field: name: {:?} type: {:?}", name, &ty);
//     Field {
//         attrs: vec![],
//         vis: syn::Visibility::Inherited,
//         ident: Some(Ident::new(name, Span::call_site())),
//         colon_token: Some(Default::default()),
//         ty
//     }
// }
//
// #[allow(unused)]
// fn create_struct(ident: Ident, fields: Fields) -> ItemStruct {
//     ItemStruct {
//         ident,
//         fields,
//         attrs: vec![],
//         generics: Default::default(),
//         semi_token: None,
//         struct_token: Default::default(),
//         vis: syn::Visibility::Inherited,
//     }
// }
//
// #[allow(unused)]
// fn create_named_struct(ident_suffix: Ident, fields: Vec<Field>) -> ItemStruct {
//     // println!("create_named_struct: ident: {:?}", &ident_suffix);
//     // println!("create_named_struct: fields: {:?}", &fields);
//     create_struct(
//         ident_suffix,
//         Fields::Named(FieldsNamed {
//             named: Punctuated::from_iter(fields),
//             brace_token: Default::default()
//         }))
// }
//
// #[allow(unused)]
// pub fn create_vec_struct(ident: Ident, value_type: Type) -> ItemStruct {
//     // println!("create_vec_struct: ident: {:?}", &ident);
//     // println!("create_vec_struct: value_type: {:?}", &value_type);
//     create_named_struct(
//         ident,
//         vec![
//             create_field("count", Type::Path(syn::parse_quote!(usize))),
//             create_field("values", value_type)])
// }
//
//
// #[allow(unused)]
// pub fn create_map_struct(ident: Ident, key_type: Type, value_type: Type) -> ItemStruct {
//     // println!("create_map_struct: ident: {:?}", &ident);
//     // println!("create_map_struct: key_type: {:?}", &key_type);
//     // println!("create_map_struct: value_type: {:?}", &value_type);
//     create_named_struct(
//         ident,
//         vec![
//             create_field("count", Type::Path(syn::parse_quote!(usize))),
//             create_field("keys", key_type),
//             create_field("values", value_type)])
// }
//
// #[allow(unused)]
// pub fn mangle(path: &Path) -> Type {
//     match PathConversion::from(path) {
//         PathConversion::Simple(path) => Type::Ptr(TypePtr {
//             star_token: Default::default(),
//             const_token: None,
//             mutability: Some(Mut::default()),
//             elem: Box::new(Type::Path(TypePath { qself: None, path })),
//         }),
//         PathConversion::Complex(path) => Type::Ptr(TypePtr {
//             star_token: Default::default(),
//             const_token: None,
//             mutability: Some(Mut::default()),
//             elem: Box::new(Type::Ptr(TypePtr {
//                 star_token: Default::default(),
//                 const_token: None,
//                 mutability: Some(Mut::default()),
//                 elem: Box::new(Type::Path(TypePath { qself: None, path: FFI_TYPE_PATH_CONVERTER(&path) })),
//             })),
//         }),
//         PathConversion::Vec(path) |
//         PathConversion::Map(path) => Type::Ptr(TypePtr {
//             star_token: Default::default(),
//             const_token: None,
//             mutability: Some(Mut::default()),
//             elem: Box::new(Type::Ptr(TypePtr {
//                 star_token: Default::default(),
//                 const_token: None,
//                 mutability: Some(Mut::default()),
//                 elem: Box::new(Type::Path(TypePath { qself: None, path: mangle_path(&path) })),
//             })),
//         })
//     }
// }
//
// fn convert_to_ffi_type(field_type: &Type) -> Type {
//     match field_type {
//         Type::P
//         Type::Path(TypePath { path, .. }) => Type::Path(TypePath { qself: None, path: mangle_simple_generic_type(path.clone()) }),
//         _ => unimplemented!("convert_to_ffi_type: unexpected args: {:?}", field_type)
//     }
// }
//
// #[allow(unused)]
// pub fn transform_vec(path: Path) -> ItemStruct {
//     let value_type = match path_arguments_to_types(&path.segments.last().unwrap().arguments)[..] {
//         [Type::Path(TypePath { path, .. })] => mangle(&path),
//         _ => unimplemented!("transform_vec: unexpected args: {}", quote!(#path))
//     };
//     let struct_name_args = VEC_PATH_PRESENTER(&path);
//     // println!("transform_vec: struct_name_args: {:?}", &struct_name_args);
//     // println!("transform_vec: value_type: {:?}", &value_type);
//     // create_vec_struct(parse_quote!(#struct_name_args), value_type)
//     create_vec_struct(format_ident!("{}FFI", struct_name_args.to_string()), value_type)
// }
//
// #[allow(unused)]
// pub fn transform_map(path: Path) -> ItemStruct {
//     let (key_type, value_type) = match path_arguments_to_types(&path.segments.last().unwrap().arguments)[..] {
//         [Type::Path(TypePath { path: key_path, .. }), Type::Path(TypePath { path: value_path, .. })] =>
//             {
//                 // println!("transform_map: key_path {:?}", key_path);
//                 // println!("transform_map: value_path {:?}", value_path);
//                 let key_type = mangle(key_path);
//                 let value_type = mangle(value_path);
//                 // println!("transform_map: key_type {:?}", &key_type);
//                 // println!("transform_map: value_type {:?}", &value_type);
//                 (key_type, value_type)
//             },
//         _ => unimplemented!("transform_vec: unexpected args: {}", quote!(#path))
//     };
//     let struct_name_args = MAP_PATH_PRESENTER(&path);
//     // println!("transform_map: struct_name_args: {:?}", &struct_name_args);
//     // println!("transform_map: key_type: {:?}", &key_type);
//     // println!("transform_map: value_type: {:?}", &value_type);
//     create_map_struct(
//         parse_quote!(#struct_name_args),
//         key_type,
//         value_type)
// }


// /// Drop Interface Cases for Generics
//
// fn from_conversion_type(conversion_type: PathConversion, vec: TokenStream2) -> TokenStream2 {
//     match conversion_type {
//         PathConversion::Simple(path) => quote!(*values.add(i)),
//         _ => ffi_from_conversion(quote!(*values.add(i)))
//         // _ => ferment_interfaces::FFIConversion::ffi_from(*values.add(i))
//     }
// }
//
// fn drop_conversion_type(conversion_type: PathConversion, vec: TokenStream2) -> TokenStream2 {
//     match conversion_type {
//         PathConversion::Simple(path) => package_unbox_vec_ptr(vec, quote!(self.count)),
//         _ => package_unbox_any_vec_ptr(vec, quote!(self.count))
//     }
// }
//
// fn drop_conversions_map(conversion_types: [PathConversion; 2]) -> Vec<TokenStream2> {
//     match conversion_types {
//         [key_conversion_type, value_conversion_type] =>
//             vec![
//                 drop_conversion_type(key_conversion_type, quote!(self.keys)),
//                 drop_conversion_type(value_conversion_type, quote!(self.values))
//             ]
//     }
// }
//
// fn drop_conversions_vec(value_conversion_type: PathConversion) -> Vec<TokenStream2> {
//     vec![drop_conversion_type(value_conversion_type, quote!(self.values))]
// }
//
// fn drop_conversion_simple_vec(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_vec_ptr(field_path, quote!(self.count))
// }
//
// fn drop_conversion_complex_vec(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_any_vec_ptr(field_path, quote!(self.count))
// }
//
// fn drop_conversion_simple_map(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_vec_ptr(field_path, quote!(self.count))
// }
//
// /// from Interface Cases for Generics
//
// fn from_conversions_map(conversion_types: [PathConversion; 2]) -> Vec<TokenStream2> {
//     match conversion_types {
//         [key_conversion_type, value_conversion_type] =>
//             vec![
//                 from_conversion_type(key_conversion_type, quote!(self.keys)),
//                 from_conversion_type(value_conversion_type, quote!(self.values))
//             ]
//     }
// }




use quote::{format_ident, quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, Type, TypeArray, TypePath, TypePtr, TypeReference};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::interface::{DEREF_FIELD_PATH, destroy_conversion, ffi_from_conversion, ffi_from_map_conversion, ffi_from_opt_conversion, ffi_to_conversion, ffi_to_opt_conversion, FFI_TYPE_PATH_PRESENTER, FROM_OFFSET_MAP_PRESENTER, iter_map_collect, LAMBDA_CONVERSION_PRESENTER, MATCH_FIELDS_PRESENTER, OBJ_FIELD_NAME, package_boxed_expression, package_boxed_vec_expression, package_unbox_any_expression, package_unbox_any_expression_terminated, unwrap_or};
use crate::path_conversion::{GenericPathConversion, PathConversion};

pub fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(value_type)] => vec![value_type],
            [GenericArgument::Type(key_type), GenericArgument::Type(value_type, ..)] => vec![key_type, value_type],
            _ => unimplemented!("path_arguments_to_types: unexpected args: {}", quote!(#args)),
        },
        _ => unimplemented!("path_arguments_to_types: arguments: {} not supported", quote!(#arguments)),
    }
}

pub fn path_arguments_to_paths(arguments: &PathArguments) -> Vec<&Path> {
    match path_arguments_to_types(arguments)[..] {
        [Type::Path(TypePath { path, .. })] => vec![path],
        [Type::Path(TypePath { path: path_keys, .. }), Type::Path(TypePath { path: path_values, .. })] => vec![path_keys, path_values],
        _ => unimplemented!("map_types: unexpected args: {}", quote!(#arguments)),
    }
}

pub fn path_arguments_to_path_conversions(arguments: &PathArguments) -> Vec<PathConversion> {
    path_arguments_to_paths(arguments)
        .into_iter()
        .map(PathConversion::from)
        .collect()
}

pub fn destroy_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(..) | PathConversion::Complex(..)] => package_unbox_any_expression_terminated(field_path),
        [PathConversion::Generic(GenericPathConversion::Vec(path))] => destroy_vec(path, field_path),
        _ => panic!("destroy_vec: Bad arguments {} {}", field_path, quote!(#arguments))
    }
}

#[allow(unused)]
pub fn box_vec(field_path: TokenStream2, values_conversion: TokenStream2) -> TokenStream2 {
    package_boxed_expression(quote!({let vec = #field_path; ferment_interfaces::VecFFI { count: vec.len(), values: #values_conversion }}))
}

pub fn from_simple_vec_conversion(field_path: TokenStream2, _field_type: TokenStream2) -> TokenStream2 {
    // quote!({
    //     let vec = #field_path;
    //     ferment_interfaces::from_simple_vec(vec, vec.len())
    // })
    quote!({
        let vec = #field_path;
        ferment_interfaces::from_simple_vec(vec.values, vec.count)
    })
}

#[allow(unused)]
pub fn from_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!({
        let vec = #field_path;
        (0..vec.count)
            .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*vec.add(i)))
            .collect()
    })
}

pub fn from_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(path)] =>
            from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream(), ),
        [PathConversion::Complex(..)] =>
            from_complex_vec_conversion(quote!(vec)),
        [PathConversion::Generic(GenericPathConversion::Vec(path))] =>
            from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        _ => panic!("from_vec_vec_conversion: Bad arguments {}", quote!(#arguments)
        ),
    };
    conversion
}

#[allow(unused)]
pub fn to_simple_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!(#field_path.clone())
}

#[allow(unused)]
pub fn to_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let conversion = ffi_to_conversion(quote!(o));
    iter_map_collect(quote!(#field_path.into_iter()), quote!(|o| #conversion))
}

#[allow(unused)]
pub fn to_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let values_conversion =
        package_boxed_vec_expression(match &path_arguments_to_path_conversions(arguments)[..] {
            [PathConversion::Primitive(..)] =>
                to_simple_vec_conversion(quote!(vec)),
            [PathConversion::Complex(..)] =>
                to_complex_vec_conversion(quote!(vec)),
            [PathConversion::Generic(GenericPathConversion::Vec(Path { segments, .. }))] =>
                to_vec_vec_conversion(&segments.last().unwrap().arguments),
            _ => panic!("to_vec_conversion: bad arguments {}", quote!(#arguments)),
        });
    let boxed_conversion = box_vec(quote!(o), values_conversion);
    iter_map_collect(quote!(vec.into_iter()), quote!(|o| #boxed_conversion))
}

pub fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    // println!("from_vec: {:?} {}", path, &field_path);
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(path)] =>
            from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream(), ),
        [PathConversion::Complex(..)] =>
            from_complex_vec_conversion(quote!(vec)),
        [PathConversion::Generic(GenericPathConversion::Vec(path))] =>
            from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        [PathConversion::Generic(..)] =>
            panic!("from_vec (Map): Unknown field {} {}", field_path, quote!(#arguments)),
        _ => panic!("from_vec: Bad arguments {} {}", field_path, quote!(#arguments)),
    };
    quote!({
        let vec = &*#field_path;
        #conversion
    })
}

pub fn destroy_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match path_arguments_to_paths(arguments)[..] {
        [_path_keys, _path_values] => match PathConversion::from(path) {
            PathConversion::Primitive(..) |
            PathConversion::Complex(..) |
            PathConversion::Generic(GenericPathConversion::Map(..)) =>
                package_unbox_any_expression_terminated(field_path),
            PathConversion::Generic(GenericPathConversion::Vec(..)) =>
                destroy_vec(path, quote!(#field_path)),
        },
        _ => panic!(
            "destroy_map: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    }
}

pub fn from_vec2(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(path)] => from_simple_vec_conversion(
            quote!(vec),
            path.segments.last().unwrap().ident.to_token_stream(),
        ),
        [PathConversion::Complex(path)] => quote!(ferment_interfaces::FFIConversion::ffi_from(vec) as #path),
        [PathConversion::Generic(GenericPathConversion::Vec(..))] =>
            quote! {
                let count = vec.count;
                let values = vec.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            },
        [PathConversion::Generic(GenericPathConversion::Map(..))] => panic!("from_vec2 nested map"),
        _ => panic!("from_vec2: Bad arguments {} {}", field_path, quote!(#arguments)),

    };
    // let value_path = mangle_path(inner_path_value_path);
    quote!({
        let vec = &*#field_path;
        #conversion
    })
}

#[allow(unused)]
pub fn from_map2(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let field_type = &last_segment.ident;
    let arguments = &last_segment.arguments;
    // let simple_conversion = FROM_OFFSET_MAP_PRESENTER;
    // let key_simple_conversion = simple_conversion(quote!(*map.keys));
    // let value_simple_conversion = simple_conversion(quote!(*map.values));
    // println!("from_map2: {} {}", quote!(#path), &field_path);
    match path_arguments_to_paths(arguments)[..] {
        [inner_path_key_path, inner_path_value_path] => {
            let key_path = mangle_path(inner_path_key_path);
            let value_path = mangle_path(inner_path_value_path);
            ffi_from_map_conversion(
                quote!(#field_path),
                quote!(#field_type),
                quote!(#key_path),
                quote!(#value_path),
            )
        }
        _ => panic!("from_map2: Bad arguments {} {}", field_path, quote!(#arguments)),
    }
}

#[allow(dead_code)]
pub fn from_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let field_type = &last_segment.ident;
    let arguments = &last_segment.arguments;
    let simple_conversion = FROM_OFFSET_MAP_PRESENTER;
    let key_simple_conversion = simple_conversion(quote!(*map.keys));
    let value_simple_conversion = simple_conversion(quote!(*map.values));

    match path_arguments_to_paths(arguments)[..] {
        [inner_path_key_path, inner_path_value_path] => {
            let convert =
                |path: &Path, parent_conversion: TokenStream2| match PathConversion::from(path)
                {
                    PathConversion::Primitive(..) => value_simple_conversion.clone(),
                    PathConversion::Complex(..) => ffi_from_conversion(value_simple_conversion.clone()),
                    PathConversion::Generic(GenericPathConversion::Vec(path)) => from_vec(&path, value_simple_conversion.clone()),
                    PathConversion::Generic(GenericPathConversion::Map(path)) => {
                        let inner_path_last_segment = path.segments.last().unwrap();
                        let field_type = &inner_path_last_segment.ident;
                        match path_arguments_to_paths(&inner_path_last_segment.arguments)[..] {
                            [inner_path_key_path, inner_path_value_path] => {
                                let converter =
                                    |inner_conversion: TokenStream2, inner_path: &Path| {
                                        match PathConversion::from(inner_path) {
                                            PathConversion::Primitive(..) => inner_conversion,
                                            PathConversion::Complex(..) => ffi_from_conversion(inner_conversion),
                                            PathConversion::Generic(GenericPathConversion::Vec(path)) => from_vec(&path, inner_conversion.clone()),
                                            _ => panic!("Vec/Map not supported as Map key")
                                        }
                                    };
                                let key_conversion =
                                    converter(key_simple_conversion.clone(), inner_path_key_path);
                                let value_conversion = converter(
                                    value_simple_conversion.clone(),
                                    inner_path_value_path,
                                );
                                ffi_from_map_conversion(
                                    quote!(((#parent_conversion))),
                                    quote!(#field_type),
                                    key_conversion,
                                    value_conversion,
                                )
                            }
                            _ => panic!("from_map: Unknown field {} {}", field_path, quote!(#arguments)),
                        }
                    }
                };
            let key_conversion = convert(inner_path_key_path, key_simple_conversion.clone());
            let value_conversion = convert(inner_path_value_path, value_simple_conversion.clone());
            ffi_from_map_conversion(
                quote!(#field_path),
                quote!(#field_type),
                key_conversion,
                value_conversion,
            )
        }
        _ => panic!("from_map: Bad arguments {} {}", field_path, quote!(#arguments)),
    }
}

// TODO: doesn't work for some cases
pub fn destroy_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [path] => match path.segments.last() {
            Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                | "isize" | "usize" => quote!({}),
                // TODO: mmm shit that's incorrect (bool = None shouldn't mean bool = false)
                "bool" => quote!({}),
                "Vec" => {
                    let conversion = destroy_vec(path, field_path.clone());
                    quote!(if !#field_path.is_null() { #conversion; })
                }
                _ => {
                    let conversion = package_unbox_any_expression_terminated(field_path.clone());
                    quote!(if !#field_path.is_null() { #conversion })
                }
            },
            _ => panic!("from_option: Unknown field {} {}", field_path, quote!(#arguments)),
        },
        _ => panic!("from_option: Bad arguments {} {}", field_path, quote!(#arguments)),
    }
}

// TODO: Option<Map>
pub fn from_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [path] => match path.segments.last() {
            Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
                // TODO: mmm shit that's incorrect
                "bool" => quote!((#field_path).then_some(#field_path)),
                "Vec" => {
                    let conversion = from_vec2(path, field_path.clone());
                    quote!((!#field_path.is_null()).then_some(#conversion))
                }
                _ => ffi_from_opt_conversion(field_path),
            },
            _ => panic!(
                "from_option: Bad arguments {} {}",
                field_path,
                quote!(#arguments)
            ),
        },
        _ => panic!(
            "from_option: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    }
}

pub(crate) fn from_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) =>
            match segments.last().unwrap().ident.to_string().as_str() {
                "u8" => DEREF_FIELD_PATH(field_path),
                _ => panic!("from_array: unsupported segments {} {}", field_path, quote!(#segments))
            },
        _ => panic!("from_array: unsupported {} {}", field_path, quote!(#type_array)),
    }
}

pub(crate) fn destroy_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(_type_path) => package_unbox_any_expression(quote!(#field_path)),
        _ => panic!(
            "from_array: unsupported {} {}",
            field_path,
            quote!(#type_array)
        ),
    }
}

pub(crate) fn destroy_path(
    field_path: TokenStream2,
    path: &Path,
    _type_ptr: Option<&TypePtr>,
) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => quote!({}),
        "VarInt" => quote!({}),
        "Option" => destroy_option(path, field_path),
        "Vec" => destroy_vec(path, field_path),
        "BTreeMap" | "HashMap" => destroy_map(path, field_path),
        "str" => destroy_conversion(field_path, FFI_TYPE_PATH_PRESENTER(path), quote!(&#path)),
        _ => destroy_conversion(field_path, FFI_TYPE_PATH_PRESENTER(path), quote!(#path)),
    }
}

pub(crate) fn from_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => from_option(path, field_path),
        "Vec" => from_vec2(path, field_path),
        "BTreeMap" | "HashMap" => quote!(ferment_interfaces::FFIConversion::ffi_from(#field_path)),
        _ => ffi_from_conversion(field_path),
    }
}

pub(crate) fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, Some(type_ptr)),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {}", quote!(#type_ptr)),
    }
}

pub(crate) fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    println!("from_ptr.1: {} {}", field_path, quote!(#type_ptr));
    let result = match &*type_ptr.elem {
        // Type::Ptr(type_ptr) => from_ptr(quote!(*#field_path.add(i)), type_ptr),
        Type::Ptr(type_ptr) => match &*type_ptr.elem {
            Type::Path(_type_path) => {
                // quote! {
                //     let key = (0..)
                // };
                let ffi_from_conversion =
                    ffi_from_conversion(FROM_OFFSET_MAP_PRESENTER(quote!(*values)));
                // iter_map_collect(quote!((0..#field_path.count)), quote!(|i| #ffi_from_conversion))

                // from_complex_vec_conversion(field_path)
                //from_ptr(field_path, type_ptr),

                quote! {
                    (0..count).map(|i| #ffi_from_conversion).collect()
                }
            }
            _ => ffi_from_conversion(field_path),
        },
        Type::Path(type_path) => {
            // from_simple_vec_conversion(field_path, type_path.path.segments.last().unwrap().ident.to_token_stream())
            // from_simple_vec_conversion(quote!(ffi_ref), type_path.path.segments.last().unwrap().ident.to_token_stream())
            let field_type = type_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_token_stream();
            quote! {
                // let count = ffi_ref.count;
                // let values = ffi_ref.values;
                std::slice::from_raw_parts(values as *const #field_type, count).to_vec()
            }

            //from_path(field_path, &type_path.path, Some(type_ptr))
        }
        _ => ffi_from_conversion(field_path),
    };
    println!("from_ptr.2: {}", result);
    result
}

pub(crate) fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, None),
        _ => panic!(
            "from_reference: unsupported type: {} {}",
            field_path,
            quote!(#type_reference)
        ),
    }
}

pub(crate) fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path, None),
        _ => panic!(
            "from_reference: unsupported type: {} {}",
            field_path,
            quote!(#type_reference)
        ),
    }
}

pub fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
    args.iter().collect::<Vec<_>>()
}



#[allow(unused)]
fn to_vec_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(..)] =>
            to_simple_vec_conversion(quote!(vec)),
        [PathConversion::Complex(..)] =>
            to_complex_vec_conversion(quote!(vec)),
        [PathConversion::Generic(GenericPathConversion::Vec(path))] =>
            to_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        _ => panic!("to_vec_conversion: Map nested in Vec not supported yet"),
    };
    println!("to_vec_conversion: {} {}", &field_path, &conversion);
    box_vec(field_path, package_boxed_vec_expression(conversion))
}

#[allow(unused)]
fn to_map_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    package_boxed_expression(match path_arguments_to_paths(arguments)[..] {
        [inner_path_key, inner_path_value] => {
            let mapper = |field_path: TokenStream2, path: &Path| {
                let conversion = match PathConversion::from(path) {
                    PathConversion::Primitive(..) =>
                        field_path,
                    PathConversion::Complex(..) =>
                        ffi_to_conversion(field_path),
                    PathConversion::Generic(GenericPathConversion::Vec(path)) =>
                        to_vec_conversion(field_path, &path.segments.last().unwrap().arguments),
                    PathConversion::Generic(GenericPathConversion::Map(path)) => {
                        to_map_conversion(field_path, &path.segments.last().unwrap().arguments)
                    }
                };
                quote!(|o| #conversion)
            };
            let key_mapper = mapper(quote!(o), inner_path_key);
            let value_mapper = mapper(quote!(o), inner_path_value);
            let keys_conversion = package_boxed_vec_expression(iter_map_collect(
                quote!(map.keys().cloned()),
                key_mapper,
            ));
            let values_conversion = package_boxed_vec_expression(iter_map_collect(
                quote!(map.values().cloned()),
                value_mapper,
            ));
            quote!({let map = #field_path; ferment_interfaces::MapFFI { count: map.len(), keys: #keys_conversion, values: #values_conversion }})
        }
        _ => panic!("to_map_conversion: Bad arguments {} {}", field_path, quote!(#arguments)),
    })
}

fn to_option_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match path_arguments_to_paths(arguments)[..] {
        [inner_path] => {
            let last_segment = inner_path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                // TODO: MAX/MIN? use optional primitive?
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                | "isize" | "usize" => unwrap_or(field_path, quote!(0)),
                "bool" => unwrap_or(field_path, quote!(false)),
                "Vec" => match path_arguments_to_paths(&last_segment.arguments)[..] {
                    [path] => {
                        let transformer = match PathConversion::from(path) {
                            PathConversion::Primitive(..) => quote!(clone()),
                            PathConversion::Complex(..) => {
                                let mapper = package_boxed_expression(ffi_to_conversion(quote!(o)));
                                iter_map_collect(quote!(iter()), quote!(|o| #mapper))
                            },
                            PathConversion::Generic(..) => panic!("to_option_conversion: Option<Generic<Generic>> nested in Vec not supported yet"),
                        };
                        MATCH_FIELDS_PRESENTER((
                            field_path,
                            vec![
                                // TODO: it's an old API
                                LAMBDA_CONVERSION_PRESENTER(quote!(Some(vec)), package_boxed_expression(quote!(ferment_interfaces::VecFFI::new(vec.#transformer)))),
                                LAMBDA_CONVERSION_PRESENTER(quote!(None), quote!(std::ptr::null_mut())),
                            ],
                        ))
                    }
                    _ => panic!("to_option_conversion: Unknown args {}", quote!(#last_segment)),
                },
                _ => ffi_to_opt_conversion(field_path),
            }
        }
        _ => panic!("to_option_conversion: Bad arguments {}", quote!(#arguments)),
    }
}

pub(crate) fn to_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        "Vec" => match &path_arguments_to_path_conversions(&last_segment.arguments)[..] {
            [PathConversion::Primitive(..)] =>
                quote!(ferment_interfaces::FFIConversion::ffi_to(#field_path)),
            [PathConversion::Complex(..) | PathConversion::Generic(..)] =>
                quote!(ferment_interfaces::FFIConversion::ffi_to(#field_path)),
            _ => unimplemented!("Generic path arguments conversion error"),
        },
        "BTreeMap" | "HashMap" => quote!(ferment_interfaces::FFIConversion::ffi_to(#field_path)),
        "Option" => to_option_conversion(field_path, &last_segment.arguments),
        _ => ffi_to_conversion(field_path),
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(OBJ_FIELD_NAME(ident), quote!(|o| #expr)))
}

pub(crate) fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    println!("to_ptr: {} {}", field_path, quote!(#type_ptr));
    match &*type_ptr.elem {
        Type::Array(TypeArray { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
            _ => panic!("to_pointer: Unknown type inside Type::Array {}", quote!(#type_ptr)),
        },
        Type::Ptr(TypePtr { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(quote!(*#field_path.add(i)), &type_path.path, Some(type_ptr)),
            Type::Array(type_arr) => to_vec_ptr(field_path, type_ptr, type_arr),
            _ => panic!("to_pointer: Unknown type inside Type::Ptr {}", quote!(#type_ptr)),
        },
        Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
        _ => panic!("to_pointer: Unknown type {}", quote!(#type_ptr)),
    }
}

pub(crate) fn to_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, None),
        _ => panic!("to_reference: Unknown type {}", quote!(#type_reference)
        ),
    }
}

pub(crate) fn to_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => to_path(package_boxed_expression(field_path), &type_path.path, None),
        _ => panic!("to_array: Unknown type {}", quote!(#type_array)),
    }
}

pub fn ffi_struct_name(field_type: &Ident) -> Ident {
    format_ident!("{}_FFI", field_type)
}

pub fn ffi_mangled_ident(ty: &Type) -> Ident {
    let ident = mangle_type(ty);
    ffi_struct_name(&ident)
}

pub fn mangle_type(ty: &Type) -> Ident {
    match ty {
        // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
        Type::Path(TypePath { path, .. }) =>
            PathConversion::from(path)
                .into_mangled_generic_ident(),
        ty => {
            let p: Path = parse_quote!(#ty);
            p.get_ident().unwrap().clone()
        }
    }
}

pub fn mangle_path(path: &Path) -> Path {
    PathConversion::from(path)
        .as_ffi_path()
}