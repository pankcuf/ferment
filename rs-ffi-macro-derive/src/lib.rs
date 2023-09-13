mod composer;
mod interface;
mod presentation;
mod util;
#[cfg(test)]
mod test;
mod generics;
mod item_conversion;
mod path_conversion;

extern crate proc_macro;
use crate::interface::{destroy_conversion, ffi_from_conversion, ffi_from_opt_conversion, ffi_to_conversion, ffi_to_opt_conversion};
use interface::{
    ffi_from_map_conversion, iter_map_collect, package_boxed_expression,
    package_boxed_vec_expression, package_unbox_any_expression,
    package_unbox_any_expression_terminated, unwrap_or,
};
use interface::{DEREF_FIELD_PATH, FFI_TYPE_PATH_PRESENTER, FROM_OFFSET_MAP_PRESENTER, LAMBDA_CONVERSION_PRESENTER, MATCH_FIELDS_PRESENTER, OBJ_FIELD_NAME};
use proc_macro::TokenStream;
use quote::__private::Span;
use quote::{format_ident, quote, ToTokens};
use std::string::ToString;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, Item, parse_macro_input, Path, PathArguments, Type, TypeArray, TypePath, TypePtr, TypeReference};
use item_conversion::ItemConversion;
use crate::path_conversion::PathConversion;

fn expansion(
    input: TokenStream2,
    comment: TokenStream2,
    ffi_converted_input: TokenStream2,
    ffi_conversion_presentation: TokenStream2,
    drop_presentation: TokenStream2,
) -> TokenStream2 {
    let expanded = quote! {
        #input
        #comment
        #ffi_converted_input
        #ffi_conversion_presentation
        #drop_presentation
    };
    println!("{}", expanded);
    expanded
}

fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

fn path_arguments_to_generic_arguments(arguments: &PathArguments) -> Vec<&GenericArgument> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            map_args(args)
        }
        _ => unimplemented!(
            "map_arguments: arguments: {} not supported",
            quote!(#arguments)
        ),
    }
}

fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    match path_arguments_to_generic_arguments(arguments)[..] {
        [GenericArgument::Type(value_type)] => vec![value_type],
        [GenericArgument::Type(key_type), GenericArgument::Type(value_type, ..)] => {
            vec![key_type, value_type]
        }
        _ => unimplemented!("map_types: unexpected args: {}", quote!(#arguments)),
    }
}

fn path_arguments_to_paths(arguments: &PathArguments) -> Vec<&Path> {
    match path_arguments_to_types(arguments)[..] {
        [Type::Path(TypePath { path, .. })] => vec![path],
        [Type::Path(TypePath { path: path_keys, .. }), Type::Path(TypePath { path: path_values, .. })] => vec![path_keys, path_values],
        _ => unimplemented!("map_types: unexpected args: {}", quote!(#arguments)),
    }
}

fn path_arguments_to_path_conversions(arguments: &PathArguments) -> Vec<PathConversion> {
    path_arguments_to_paths(arguments)
        .into_iter()
        .map(PathConversion::from)
        .collect()
}

fn destroy_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Simple(..) | PathConversion::Complex(..)] => package_unbox_any_expression_terminated(field_path),
        [PathConversion::Vec(path)] => destroy_vec(path, field_path),
        _ => panic!("destroy_vec: Bad arguments {} {}", field_path, quote!(#arguments))
    }
}

fn unbox_vec(var: TokenStream2, field_path: TokenStream2, conversion: TokenStream2) -> TokenStream2 {
    quote!({
        let #var = #field_path;
        #conversion
    })
}

#[allow(unused)]
fn box_vec(field_path: TokenStream2, values_conversion: TokenStream2) -> TokenStream2 {
    package_boxed_expression(quote!({let vec = #field_path; rs_ffi_interfaces::VecFFI { count: vec.len(), values: #values_conversion }}),
    )
}
#[allow(unused)]
fn from_simple_vec_conversion(field_path: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    quote!({
        let vec = #field_path;
        rs_ffi_interfaces::from_simple_vec(vec.values, vec.count)
    })
}

#[allow(unused)]
fn from_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!({
        let vec = #field_path;
        (0..vec.count)
            .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*vec.add(i)))
            .collect()
    })
    // quote!({
    //     let vec = #field_path;
    //     rs_ffi_interfaces::from_complex_vec(vec.values, vec.count) })



    // let ffi_from_conversion =
    //     ffi_from_conversion(FROM_OFFSET_MAP_PRESENTER(quote!(*#field_path.values)));
    // iter_map_collect(
    //     quote!((0..#field_path.count)),
    //     quote!(|i| #ffi_from_conversion),
    // )
}

fn from_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Simple(path)] => from_simple_vec_conversion(
            quote!(vec),
            path.segments.last().unwrap().ident.to_token_stream(),
        ),
        [PathConversion::Complex(..)] => from_complex_vec_conversion(quote!(vec)),
        [PathConversion::Vec(path)] => {
            from_vec_vec_conversion(&path.segments.last().unwrap().arguments)
        }
        _ => panic!(
            "from_vec_vec_conversion: Bad arguments {}",
            quote!(#arguments)
        ),
    };
    conversion
    // let unbox_conversion = unbox_vec(
    //     quote!(vec),
    //     FROM_OFFSET_MAP_PRESENTER(quote!(&**vec.values)),
    //     conversion,
    // );
    // iter_map_collect(quote!((0..vec.count)), quote!(|i| #unbox_conversion))
}

#[allow(unused)]
fn to_simple_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!(#field_path.clone())
}

#[allow(unused)]
fn to_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let conversion = ffi_to_conversion(quote!(o));
    iter_map_collect(quote!(#field_path.into_iter()), quote!(|o| #conversion))
}

#[allow(unused)]
fn to_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let values_conversion =
        package_boxed_vec_expression(match &path_arguments_to_path_conversions(arguments)[..] {
            [PathConversion::Simple(..)] => to_simple_vec_conversion(quote!(vec)),
            [PathConversion::Complex(..)] => to_complex_vec_conversion(quote!(vec)),
            [PathConversion::Vec(Path { segments, .. })] => {
                to_vec_vec_conversion(&segments.last().unwrap().arguments)
            }
            _ => panic!("to_vec_conversion: bad arguments {}", quote!(#arguments)),
        });
    let boxed_conversion = box_vec(quote!(o), values_conversion);
    iter_map_collect(quote!(vec.into_iter()), quote!(|o| #boxed_conversion))
}

fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    // println!("from_vec: {:?} {}", path, &field_path);
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Simple(path)] => from_simple_vec_conversion(
            quote!(vec),
            path.segments.last().unwrap().ident.to_token_stream(),
        ),
        [PathConversion::Complex(..)] => from_complex_vec_conversion(quote!(vec)),
        [PathConversion::Vec(path)] => {
            from_vec_vec_conversion(&path.segments.last().unwrap().arguments)
        }
        [PathConversion::Map(..)] => panic!(
            "from_vec (Map): Unknown field {} {}",
            field_path,
            quote!(#arguments)
        ),
        _ => panic!(
            "from_vec: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    };
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&*#field_path), conversion);
    unbox_conversion
}
fn destroy_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match path_arguments_to_paths(arguments)[..] {
        [_path_keys, _path_values] => match PathConversion::from(path) {
            PathConversion::Simple(..) => package_unbox_any_expression_terminated(field_path),
            PathConversion::Complex(..) => package_unbox_any_expression_terminated(field_path),
            PathConversion::Vec(..) => destroy_vec(path, quote!(#field_path)),
            PathConversion::Map(..) => package_unbox_any_expression_terminated(field_path),
        },
        _ => panic!(
            "destroy_map: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    }
}

fn from_vec2(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Simple(path)] => from_simple_vec_conversion(
            quote!(vec),
            path.segments.last().unwrap().ident.to_token_stream(),
        ),
        [PathConversion::Complex(path)] => quote!(rs_ffi_interfaces::FFIConversion::ffi_from(vec) as #path),
        // [PathConversion::Vec(path)] => from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
        [PathConversion::Vec(_)] =>
            quote! {
                let count = vec.count;
                let values = vec.values;
                (0..count)
                    .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            },
            // quote!(rs_ffi_interfaces::FFIConversion::ffi_from(vec) as #path)
        [PathConversion::Map(_)] => panic!("from_vec2 nested map"),
        _ => panic!("from_vec2: Bad arguments {} {}", field_path, quote!(#arguments)),

    };
    // let value_path = mangle_path(inner_path_value_path);
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&*#field_path), conversion);
    unbox_conversion
}

#[allow(unused)]
fn from_map2(path: &Path, field_path: TokenStream2) -> TokenStream2 {
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
        _ => panic!(
            "from_map2: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    }
}

#[allow(dead_code)]
fn from_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
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
                    PathConversion::Simple(..) => value_simple_conversion.clone(),
                    PathConversion::Complex(..) => {
                        ffi_from_conversion(value_simple_conversion.clone())
                    }
                    PathConversion::Vec(path) => from_vec(&path, value_simple_conversion.clone()),
                    PathConversion::Map(path) => {
                        let inner_path_last_segment = path.segments.last().unwrap();
                        let field_type = &inner_path_last_segment.ident;
                        match path_arguments_to_paths(&inner_path_last_segment.arguments)[..] {
                            [inner_path_key_path, inner_path_value_path] => {
                                let converter =
                                    |inner_conversion: TokenStream2, inner_path: &Path| {
                                        match PathConversion::from(inner_path) {
                                            PathConversion::Simple(..) => inner_conversion,
                                            PathConversion::Complex(..) => {
                                                ffi_from_conversion(inner_conversion)
                                            }
                                            PathConversion::Vec(path) => {
                                                from_vec(&path, inner_conversion.clone())
                                            }
                                            PathConversion::Map(..) => {
                                                panic!("Vec/Map not supported as Map key")
                                            }
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
                            _ => panic!(
                                "from_map: Unknown field {} {}",
                                field_path,
                                quote!(#arguments)
                            ),
                        }
                    }
                };
            let key_conversion = convert(inner_path_key_path, key_simple_conversion.clone());
            let value_conversion = convert(inner_path_value_path, value_simple_conversion.clone());

            // let key_conversion = match PathConversion::from(inner_path_key_path) {
            //     PathConversion::Simple(..) => key_simple_conversion.clone(),
            //     PathConversion::Complex(..) => ffi_from_conversion(key_simple_conversion.clone()),
            //     PathConversion::Vec(path) => from_vec(&path, key_simple_conversion.clone()),
            //     PathConversion::Map(..) => panic!("Map not supported as Map key")
            // };
            //
            // let value_conversion = match PathConversion::from(inner_path_value_path) {
            //     PathConversion::Simple(..) => value_simple_conversion.clone(),
            //     PathConversion::Complex(..) => ffi_from_conversion(value_simple_conversion.clone()),
            //     PathConversion::Vec(path) => from_vec(&path, value_simple_conversion.clone()),
            //     PathConversion::Map(..) => {
            //         let inner_path_value_path_last_segment = inner_path_value_path.segments.last().unwrap();
            //         let field_type = &inner_path_value_path_last_segment.ident;
            //         match path_arguments_to_paths(&inner_path_value_path_last_segment.arguments)[..] {
            //             [inner_path_key_path, inner_path_value_path] => {
            //
            //                 let converter = |inner_conversion: TokenStream2, inner_path: &Path| match PathConversion::from(inner_path) {
            //                     PathConversion::Simple(..) => inner_conversion,
            //                     PathConversion::Complex(..)  => ffi_from_conversion(inner_conversion),
            //                     PathConversion::Vec(path) => from_vec(&path, value_simple_conversion.clone()),
            //                     PathConversion::Map(..) => panic!("Vec/Map not supported as Map key")
            //                 };
            //
            //                 let key_conversion = converter(key_simple_conversion.clone(), inner_path_key_path);
            //                 let value_conversion = converter(value_simple_conversion.clone(), inner_path_value_path);
            //                 let ccc = value_simple_conversion.clone();
            //                 ffi_from_map_conversion(quote!(((#ccc))), quote!(#field_type), key_conversion, value_conversion)
            //             },
            //             _ => panic!("from_map: Unknown field {:?} {:?}", field_path, arguments)
            //         }
            //     }
            // };
            ffi_from_map_conversion(
                quote!(#field_path),
                quote!(#field_type),
                key_conversion,
                value_conversion,
            )
        }
        _ => panic!(
            "from_map: Bad arguments {} {}",
            field_path,
            quote!(#arguments)
        ),
    }
}

// TODO: doesn't work for some cases
fn destroy_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match path_arguments_to_paths(arguments)[..] {
        [path] => match path.segments.last() {
            Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                | "isize" | "usize" => quote!({}),
                // TODO: mmm shit that's incorrect
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
            _ => panic!(
                "from_option: Unknown field {} {}",
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

// TODO: Option<Map>
fn from_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
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

fn from_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => match segments.last().unwrap().ident.to_string().as_str() {
            "u8" => DEREF_FIELD_PATH(field_path),
            _ => panic!(
                "from_array: unsupported segments {} {}",
                field_path,
                quote!(#segments)
            ),
        },
        _ => panic!(
            "from_array: unsupported {} {}",
            field_path,
            quote!(#type_array)
        ),
    }
}

fn destroy_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(_type_path) => package_unbox_any_expression(quote!(#field_path)),
        _ => panic!(
            "from_array: unsupported {} {}",
            field_path,
            quote!(#type_array)
        ),
    }
}

fn destroy_path(
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

fn from_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => from_option(path, field_path),
        // "Vec" => from_vec2(path, field_path),
        "Vec" => from_vec2(path, field_path),
        // "Vec" => quote!(rs_ffi_interfaces::FFIConversion::ffi_from(#field_path)),
        // "Vec" => quote!(FFIVecConversion::decode(#field_path)),
        "BTreeMap" | "HashMap" => quote!(rs_ffi_interfaces::FFIConversion::ffi_from(#field_path)),
        // "BTreeMap" | "HashMap" => quote!(rs_ffi_interfaces::FFIConversion::ffi_from(#field_path)),
        // "BTreeMap" | "HashMap" => from_map2(path, field_path),
        _ => ffi_from_conversion(field_path),
    }
}

fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, Some(type_ptr)),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {}", quote!(#type_ptr)),
    }
}

fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
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

fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, None),
        _ => panic!(
            "from_reference: unsupported type: {} {}",
            field_path,
            quote!(#type_reference)
        ),
    }
}

fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path, None),
        _ => panic!(
            "from_reference: unsupported type: {} {}",
            field_path,
            quote!(#type_reference)
        ),
    }
}

fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
    args.iter().collect::<Vec<_>>()
}

#[allow(unused)]
fn to_vec_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Simple(..)] => to_simple_vec_conversion(quote!(vec)),
        [PathConversion::Complex(..)] => to_complex_vec_conversion(quote!(vec)),
        [PathConversion::Vec(path)] => {
            to_vec_vec_conversion(&path.segments.last().unwrap().arguments)
        }
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
                    PathConversion::Simple(..) => field_path,
                    PathConversion::Complex(..) => ffi_to_conversion(field_path),
                    PathConversion::Vec(path) => {
                        to_vec_conversion(field_path, &path.segments.last().unwrap().arguments)
                    }
                    PathConversion::Map(path) => {
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
            quote!({let map = #field_path; rs_ffi_interfaces::MapFFI { count: map.len(), keys: #keys_conversion, values: #values_conversion }})
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
                            PathConversion::Simple(..) => quote!(clone()),
                            PathConversion::Complex(..) => {
                                let mapper = package_boxed_expression(ffi_to_conversion(quote!(o)));
                                iter_map_collect(quote!(iter()), quote!(|o| #mapper))
                            },
                            PathConversion::Map(..) => panic!("to_option_conversion: Option<Map<Map>> nested in Vec not supported yet"),
                            PathConversion::Vec(..) => panic!("to_option_conversion: Vec nested in Vec not supported yet"),
                        };
                        MATCH_FIELDS_PRESENTER((
                            field_path,
                            vec![
                                LAMBDA_CONVERSION_PRESENTER(
                                    quote!(Some(vec)),
                                    package_boxed_expression(
                                        quote!(rs_ffi_interfaces::VecFFI::new(vec.#transformer)),
                                    ),
                                ),
                                LAMBDA_CONVERSION_PRESENTER(
                                    quote!(None),
                                    quote!(std::ptr::null_mut()),
                                ),
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

fn to_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        // "Vec" => to_vec_conversion(field_path, &last_segment.arguments),
        "Vec" => match &path_arguments_to_path_conversions(&last_segment.arguments)[..] {
            // [PathConversion::Simple(_value_path)] => quote!(rs_ffi_interfaces::to_simple_vec(#field_path)),
            [PathConversion::Simple(_value_path)] => quote!(rs_ffi_interfaces::FFIConversion::ffi_to(#field_path)),
            // [PathConversion::Complex(_value_path) | PathConversion::Vec(_value_path) | PathConversion::Map(_value_path)] => quote!(rs_ffi_interfaces::to_complex_vec(#field_path)),
            [PathConversion::Complex(_value_path) | PathConversion::Vec(_value_path) | PathConversion::Map(_value_path)] =>
                quote!(rs_ffi_interfaces::FFIConversion::ffi_to(#field_path)),
            _ => unimplemented!("Generic path arguments conversion error"),
        },

        // "BTreeMap" | "HashMap" => to_map_conversion(field_path, &last_segment.arguments),
        "BTreeMap" | "HashMap" => quote!(rs_ffi_interfaces::FFIConversion::ffi_to(#field_path)),
        // "BTreeMap" | "HashMap" => match &path_arguments_to_path_conversions(&last_segment.arguments) {
        //     [PathConversion::Simple(key_path),
        //     PathConversion::Simple(value_path)] =>
        //         quote!(rs_ffi_interfaces::to_simple_vec(obj))
        //     // GENERIC_MAP_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Simple(key_path),
        //     PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
        //         GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path),
        //     PathConversion::Simple(value_path)] =>
        //         GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path),
        //     PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
        //         GENERIC_MAP_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     _ => unimplemented!("Generic path arguments conversion error")
        // },
        // "BTreeMap" | "HashMap" => match &path_arguments_to_path_conversions(&last_segment.arguments) {
        //     [PathConversion::Simple(key_path),
        //     PathConversion::Simple(value_path)] =>
        //         quote!(rs_ffi_interfaces::to_simple_vec(obj))
        //         // GENERIC_MAP_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Simple(key_path),
        //     PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
        //         GENERIC_MAP_SIMPLE_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path),
        //     PathConversion::Simple(value_path)] =>
        //         GENERIC_MAP_COMPLEX_SIMPLE_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     [PathConversion::Complex(key_path) | PathConversion::Vec(key_path) | PathConversion::Map(key_path),
        //     PathConversion::Complex(value_path) | PathConversion::Vec(value_path) | PathConversion::Map(value_path)] =>
        //         GENERIC_MAP_COMPLEX_PRESENTER((ffi_name, &path, key_path, value_path)),
        //     _ => unimplemented!("Generic path arguments conversion error")
        // },
        "Option" => to_option_conversion(field_path, &last_segment.arguments),
        _ => ffi_to_conversion(field_path),
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(OBJ_FIELD_NAME(ident), quote!(|o| #expr)))
}

fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
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

fn to_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, None),
        _ => panic!("to_reference: Unknown type {}", quote!(#type_reference)
        ),
    }
}

fn to_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => to_path(package_boxed_expression(field_path), &type_path.path, None),
        _ => panic!("to_array: Unknown type {}", quote!(#type_array)),
    }
}

fn ffi_struct_name(field_type: &Ident) -> Ident {
    format_ident!("{}_FFI", field_type)
}

fn mangle_type(ty: &Type) -> Ident {
    match ty {
        // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
        Type::Path(TypePath { path, .. }) =>
            PathConversion::from(path)
                .into_mangled_generic_ident(),
        _ => unimplemented!("Can't mangle type"),
    }
}

fn mangle_path(path: &Path) -> Path {
    PathConversion::from(path)
        .as_ffi_path()
}

/// The `impl_ffi_fn_conv` procedural macro facilitates FFI (Foreign Function Interface) conversion
/// for a given function. It handles both input arguments and output types, converting them into a format
/// suitable for FFI boundaries.
///
/// # Syntax
///
/// The macro can be applied to any Rust function:
///
/// ```ignore
/// #[impl_ffi_fn_conv]
/// pub fn my_function(arg1: MyType1, arg2: MyType2) -> MyReturnType {
///     // function implementation
/// }
/// ```
///
/// # Output
///
/// The macro will automatically generate additional FFI-compatible code around the annotated function.
/// It converts the function into a form that can be easily invoked from C/C++ code.
///
/// ## Safety
///
/// This macro generates safety documentation specific to the function, covering the expectations
/// and constraints of the FFI boundary.
///
/// ## Function Conversion
///
/// The macro processes the function's input arguments and return type, performing necessary transformations
/// like memory allocation/deallocation, pointer conversion, etc., to make them FFI-compatible.
///
/// # Panics
///
/// - The macro will panic if any of the function's argument types are not supported for conversion.
/// - The macro will also panic if the function's return type is not supported for conversion.
///
/// # Example
///
/// ```ignore
/// #[impl_ffi_fn_conv]
/// pub fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// ```
///
/// After applying the macro, the function can be safely invoked from C/C++ code.
///
/// # Note
///
/// This macro is intended for internal use and should be used cautiously,
/// understanding the risks associated with FFI calls.
///
/// # See Also
///
/// # Limitations
///
/// - The macro currently does not support Rust async functions.
/// - Nested data structures may not be fully supported.
///
#[proc_macro_attribute]
pub fn impl_ffi_fn_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // TokenStream::from(quote!())
    input
    // Expansion::from(ItemConversion::Fn(parse_macro_input!(input as ItemFn)))
    //     .present()
    //     .into()
}

#[proc_macro_attribute]
pub fn impl_ffi_conv(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // TokenStream::from(quote!())
    // let attrs = parse_macro_input!(attr as AttributeArgs);
    // let target_name = match attrs.first() {
    //     Some(NestedMeta::Lit(literal)) => {
    //         format_ident!("{}", literal.to_token_stream().to_string())
    //     }
    //     Some(NestedMeta::Meta(Meta::Path(path))) => path.segments.first().unwrap().ident.clone(),
    //     _ => {
    //         // use default rules
    //         // for unnamed structs like UInt256 -> #target_name = [u8; 32]
    //         // for named structs -> generate ($StructName)FFI
    //         input.ident.clone()
    //     }
    // };
    item
    // Expansion::from(ItemConversion::from(parse_macro_input!(item as DeriveInput)))
    //     .present()
    //     .into()
}

#[proc_macro_attribute]
pub fn impl_ffi_ty_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // just marker macro
    input
    // TokenStream::from(quote!())
    // Expansion::from(ItemConversion::try_from(&parse_macro_input!(input as Item)).unwrap())
    //     .present()
    //     .into()
}

#[proc_macro_attribute]
pub fn ffi_dictionary(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let expansions = ItemConversion::try_from(&parse_macro_input!(input as Item))
        .map_or(vec![], |conversion| conversion.expand_all_types());
    let expanded = quote! { #(#expansions)* };
    // println!("expansions: {}", expanded);
    TokenStream::from(expanded)
}
