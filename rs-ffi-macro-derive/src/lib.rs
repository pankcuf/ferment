extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, ItemFn, Meta, NestedMeta, Type, PathArguments, GenericArgument, TypePtr, TypeArray, Ident, TypePath, DataStruct, Fields, FieldsUnnamed, FieldsNamed, DataEnum, Variant, Expr, Path, ReturnType, FnArg, PatType, AngleBracketedGenericArguments, Pat, PatIdent, Field, TypeReference};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;

enum ConversionType {
    Simple,
    Complex,
    Map,
    Vec
}

fn package() -> TokenStream2 {
    quote!(rs_ffi_interfaces)
}

fn interface() -> TokenStream2 {
    quote!(FFIConversion)
}

fn ffi() -> TokenStream2 {
    quote!(ffi)
}

fn ffi_deref() -> TokenStream2 {
    quote!(ffi_ref)
    // let ffi = ffi();
    // quote!(*#ffi)
}

fn obj() -> TokenStream2 {
    quote!(obj)
}

fn ffi_from() -> TokenStream2 {
    quote!(ffi_from)
}

fn ffi_from_opt() -> TokenStream2 {
    quote!(ffi_from_opt)
}

fn ffi_to() -> TokenStream2 {
    quote!(ffi_to)
}

fn ffi_to_opt() -> TokenStream2 {
    quote!(ffi_to_opt)
}

fn boxed() -> TokenStream2 {
    quote!(boxed)
}

fn boxed_vec() -> TokenStream2 {
    quote!(boxed_vec)

}

fn package_boxed() -> TokenStream2 {
    let package = package();
    let boxed = boxed();
    quote!(#package::#boxed)
}

fn package_boxed_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed = package_boxed();
    quote!(#package_boxed(#expr))
}

fn package_boxed_vec() -> TokenStream2 {
    let package = package();
    let boxed_vec = boxed_vec();
    quote!(#package::#boxed_vec)
}

fn package_boxed_vec_expression(expr: TokenStream2) -> TokenStream2 {
    let package_boxed_vec = package_boxed_vec();
    quote!(#package_boxed_vec(#expr))
}

fn iter_map_collect(iter: TokenStream2, mapper: TokenStream2) -> TokenStream2 {
    quote!(#iter.map(#mapper).collect())
}

fn define_field(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    quote!(#l_value: #r_value)
}
fn define_pub_field(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    define_field(quote!(pub #l_value), r_value)
}

fn define_lambda(l_value: TokenStream2, r_value: TokenStream2) -> TokenStream2 {
    quote!(#l_value => #r_value)
}

fn unwrap_or(field_path: TokenStream2, or: TokenStream2) -> TokenStream2 {
    quote!(#field_path.unwrap_or(#or))
}

fn ffi_deref_field_name(field_name: TokenStream2) -> TokenStream2 {
    let ffi_deref = ffi_deref();
    quote!(#ffi_deref.#field_name)
}

fn obj_field_name(field_name: TokenStream2) -> TokenStream2 {
    let obj = obj();
    quote!(#obj.#field_name)
}

fn create_struct(name: TokenStream2, fields: Vec<Box<impl ToTokens>>) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone, Copy, Debug)]
        pub struct #name { #(#fields,)* }
    }
}

fn ffi_vec_field_type(value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::VecFFI<#value_type>)
}

fn ffi_map_field_type(key_type: TokenStream2, value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::MapFFI<#key_type, #value_type>)
}

// fn ffi_to_map_conversion(map_key_path: TokenStream2, key_index: TokenStream2, key_conversion: TokenStream2, value_conversion: TokenStream2) -> TokenStream2 {
//     let keys_conversion = package_boxed_vec_expression(quote!(#map_key_path.keys().cloned().map(|#key_index| #key_conversion).collect()));
//     let values_conversion = package_boxed_vec_expression(quote!(#map_key_path.values().cloned().map(|#key_index| #value_conversion).collect()));
//     package_boxed_expression(quote! {{
//         rs_ffi_interfaces::MapFFI {
//             count: #map_key_path.len(),
//             keys: #keys_conversion,
//             values: #values_conversion,
//         }
//     }})
// }

fn ffi_from_map_conversion(map_key_path: TokenStream2, key_index: TokenStream2, acc_type: TokenStream2, key_conversion: TokenStream2, value_conversion: TokenStream2) -> TokenStream2 {
    quote! {{
        let map = &*#map_key_path;
        (0..map.count).fold(#acc_type::new(), |mut acc, #key_index| {
            let key = #key_conversion;
            let value = #value_conversion;
            acc.insert(key, value);
            acc
        })
    }}
}

fn ffi_from_vec_conversion(vec_key_path: TokenStream2, key_index: TokenStream2, value_conversion: TokenStream2) -> TokenStream2 {
    quote! {{
        let vec = &*#vec_key_path;
        (0..vec.count).map(|#key_index| #value_conversion).collect()
    }}
}

// fn from_vec(path: &Path, field_name: &Ident) -> TokenStream2 {
fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path, .. }))] => {
                let conversion = match conversion_type_for_path(path) {
                    ConversionType::Simple => {
                        let field_type = &path.segments.last().unwrap().ident;
                        quote!(std::slice::from_raw_parts(vec.values as *const #field_type, vec.count).to_vec())
                    },
                    ConversionType::Complex => {
                        let ffi_from_conversion = ffi_from_conversion(quote!(*vec.values.add(i)));
                        quote!((0..vec.count).map(|i| #ffi_from_conversion).collect())
                    },
                    ConversionType::Map => panic!("from_vec (Map): Unknown field {:?} {:?}", field_path, args),
                    ConversionType::Vec => panic!("from_vec (Vec): Unknown field {:?} {:?}", field_path, args)
                };
                // quote!({ let vec = &*#ffi_deref.#field_name; #conversion })
                quote!({ let vec = &*#field_path; #conversion })
            },
            _ => panic!("from_vec: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_vec: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn from_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let field_type = &last_segment.ident;
    let arguments = &last_segment.arguments;
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path: inner_path_key_path, .. })), GenericArgument::Type(Type::Path(TypePath { path: inner_path_value_path, .. }))] => {
                let key_index = quote!(i);
                let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
                let key_simple_conversion = simple_conversion(quote!(*map.keys));
                let value_simple_conversion = simple_conversion(quote!(*map.values));
                let key_conversion = match conversion_type_for_path(inner_path_key_path) {
                    ConversionType::Simple => key_simple_conversion,
                    ConversionType::Complex => ffi_from_conversion(key_simple_conversion),
                    ConversionType::Map | ConversionType::Vec => panic!("Vec/Map not supported as Map key")
                };
                let inner_path_value_path_last_segment = inner_path_value_path.segments.last().unwrap();
                let inner_path_value_path_last_segment_args = &inner_path_value_path_last_segment.arguments;
                let value_conversion = match conversion_type_for_path(inner_path_value_path) {
                    ConversionType::Simple => value_simple_conversion,
                    ConversionType::Complex => ffi_from_conversion(value_simple_conversion),
                    ConversionType::Vec => match inner_path_value_path_last_segment_args {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                            [GenericArgument::Type(Type::Path(TypePath { path, .. }))] => {
                                let key_index = quote!(i);
                                let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
                                let value_simple_conversion = simple_conversion(quote!(*map.values));
                                let value_conversion = match conversion_type_for_path(path) {
                                    ConversionType::Simple => value_simple_conversion,
                                    ConversionType::Complex => ffi_from_conversion(value_simple_conversion),
                                    _ => panic!("3 Nested Map/Vec not supported yet")
                                };
                                let ccc = simple_conversion(quote!(map.values));
                                ffi_from_vec_conversion(quote!(((*#ccc))), key_index, value_conversion)
                            },
                            _ => panic!("from_map: Unknown field {:?} {:?}", field_path, args)
                        },
                        _ => panic!("from_map: Unknown field {:?} {:?}", field_path, inner_path_value_path)
                    },
                    ConversionType::Map => {
                        let field_type = &inner_path_value_path_last_segment.ident;
                        match &inner_path_value_path_last_segment_args {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                                [GenericArgument::Type(Type::Path(inner_path_key)), GenericArgument::Type(Type::Path(inner_path_value))] => {
                                    let key_index = quote!(i);
                                    let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
                                    let key_simple_conversion = simple_conversion(quote!(*map.keys));
                                    let value_simple_conversion = simple_conversion(quote!(*map.values));
                                    let key_conversion = match conversion_type_for_path(&inner_path_key.path) {
                                        ConversionType::Simple => key_simple_conversion,
                                        ConversionType::Complex  => ffi_from_conversion(key_simple_conversion),
                                        ConversionType::Vec | ConversionType::Map => panic!("Vec/Map not supported as Map key")
                                    };
                                    let value_conversion = match conversion_type_for_path(&inner_path_value.path) {
                                        ConversionType::Simple => value_simple_conversion,
                                        ConversionType::Complex => ffi_from_conversion(value_simple_conversion),
                                        _ => panic!("from_map: 3 Nested Map/Vec not supported yet")
                                    };
                                    let ccc = simple_conversion(quote!(map.values));
                                    ffi_from_map_conversion(quote!(((*#ccc))), key_index, quote!(#field_type), key_conversion, value_conversion)
                                },
                                _ => panic!("from_map: Unknown field {:?} {:?}", field_path, args)
                            },
                            _ => panic!("from_map: Unknown field {:?} {:?}", field_path, args)
                        }
                    }
                };
                ffi_from_map_conversion(quote!(#field_path), key_index, quote!(#field_type), key_conversion, value_conversion)
            },
            _ => panic!("from_map: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_map: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn from_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    // let ffi_field_name_quote = ffi_deref_field_name(quote!(#field_name));
    let arguments = &last_segment.arguments;
    match arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(GenericArgument::Type(Type::Path(inner_path))) => {
                let path = &inner_path.path;
                match path.segments.last() {
                    Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                        // std convertible
                        // TODO: what to use? 0 or ::MAX
                        "i8" | "u8" | "i16" | "u16" |
                        "i32" | "u32" | "i64" | "u64" |
                        "i128" | "u128" | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
                        // TODO: mmm shit that's incorrect
                        "bool" => quote!((#field_path > 0).then_some(#field_path)),
                        "Vec" => {
                            let conversion = from_vec(path, field_path.clone());
                            quote!((!#field_path.is_null()).then_some(#conversion))
                        },
                        _ => ffi_from_opt_conversion(field_path)
                    },
                    _ => panic!("from_option: (type->path) Unknown field {:?} {:?}", field_path, inner_path)
                }
            },
            _ => panic!("from_option: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn from_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => from_option(path, field_path),
        "Vec" => from_vec(path, field_path),
        "BTreeMap" | "HashMap" => from_map(path, field_path),
        _ => ffi_from_conversion(field_path)
    }
}

fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => from_ptr(field_path, type_ptr),
        Type::Path(type_path) => from_path(field_path, &type_path.path, Some(type_ptr)),
        _ => ffi_from_conversion(field_path)
    }
}

fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {:?} {:?}", field_path, type_reference)
    }
}


fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
    args.iter().collect::<Vec<_>>()
}

fn mapper_to(path: &Path) -> TokenStream2 {
    let conversion = match conversion_type_for_path(path) {
        ConversionType::Simple => quote!(o),
        ConversionType::Complex => ffi_to_conversion(quote!(o)),
        ConversionType::Vec |
        ConversionType::Map => panic!("mapper_to: Don't support wrapping triple nested Map/Vec")
    };
    quote!(|o| #conversion)
}

fn to_vec_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path))] => {
                let transformer = |path: &Path| {
                    match conversion_type_for_path(path) {
                        ConversionType::Simple => quote!(vec.clone()),
                        ConversionType::Complex => {
                            let conversion = ffi_to_conversion(quote!(o));
                            iter_map_collect(quote!(vec.into_iter()),  quote!(|o| #conversion))
                        },
                        ConversionType::Map => panic!("to_vec_conversion: Map nested in Vec not supported yet"),
                        ConversionType::Vec => panic!("to_vec_conversion: Vec nested in Vec not supported yet"),
                    }
                };
                let values_conversion = package_boxed_vec_expression(transformer(&inner_path.path));
                package_boxed_expression(quote! {{ let vec = #field_path; rs_ffi_interfaces::VecFFI { count: vec.len(), values: #values_conversion } }})
            },
            _ => panic!("to_vec_conversion: bad args {:?}", args)
        },
        _ => panic!("to_vec_conversion: bad arguments {:?}", arguments)
    }
}

fn to_map_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path_key)), GenericArgument::Type(Type::Path(inner_path_value))] => {
                let inner_mapper = |path: &Path| {
                    let conversion = match conversion_type_for_path(path) {
                        ConversionType::Simple => quote!(o),
                        ConversionType::Complex => ffi_to_conversion(quote!(o)),
                        ConversionType::Vec => match &path.segments.last().unwrap().arguments {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                                [GenericArgument::Type(Type::Path(inner_path))] => {
                                    let value_mapper = mapper_to(&inner_path.path);
                                    let values_conversion = package_boxed_vec_expression(quote!(o.values().cloned().map(#value_mapper).collect()));
                                    package_boxed_expression(quote!({rs_ffi_interfaces::VecFFI { count: o.len(), values: #values_conversion }}))
                                },
                                _ => panic!("to_map_conversion (nested_map): Unknown field {:?} {:?}", field_path, args)
                            },
                            _ => panic!("to_map_conversion (nested_vec): Unknown field {:?} {:?}", field_path, args)
                        },
                        ConversionType::Map => match &path.segments.last().unwrap().arguments {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                                [GenericArgument::Type(Type::Path(inner_path_key)), GenericArgument::Type(Type::Path(inner_path_value))] => {
                                    let key_mapper = mapper_to(&inner_path_key.path);
                                    let value_mapper = mapper_to(&inner_path_value.path);
                                    let keys_conversion = package_boxed_vec_expression(quote!(o.keys().cloned().map(#key_mapper).collect()));
                                    let values_conversion = package_boxed_vec_expression(quote!(o.values().cloned().map(#value_mapper).collect()));
                                    package_boxed_expression(quote!({
                                        rs_ffi_interfaces::MapFFI { count: o.len(), keys: #keys_conversion, values: #values_conversion }
                                    }))
                                },
                                _ => panic!("to_map_conversion (nested_map): Unknown field {:?} {:?}", field_path, args)
                            }
                            _ => panic!("to_map_conversion (nested_map): Unknown field {:?} {:?}", field_path, args)
                        }
                    };
                    quote!(|o| #conversion)
                };
                let inner_key_mapper = inner_mapper(&inner_path_key.path);
                let inner_value_mapper = inner_mapper(&inner_path_value.path);
                let keys_conversion = package_boxed_vec_expression(quote!(map.keys().cloned().map(#inner_key_mapper).collect()));
                let values_conversion = package_boxed_vec_expression(quote!(map.values().cloned().map(#inner_value_mapper).collect()));
                package_boxed_expression(quote!({
                    let map = #field_path;
                    rs_ffi_interfaces::MapFFI { count: map.len(), keys: #keys_conversion, values: #values_conversion }
                }))
            },
            _ => panic!("to_map_conversion: Bad args {:?} {:?}", field_path, args)
        },
        _ => panic!("to_map_conversion: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn to_option_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path))] => {
                let last_segment = &inner_path.path.segments.last().unwrap();
                match last_segment.ident.to_string().as_str() {
                    // TODO: MAX/MIN? use optional primitive?
                    "i8" | "u8" | "i16" | "u16" |
                    "i32" | "u32" | "i64" | "u64" |
                    "i128" | "u128" | "isize" | "usize" => unwrap_or(field_path, quote!(0)),
                    "bool" => unwrap_or(field_path, quote!(false)),
                    "Vec" => match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                            [GenericArgument::Type(Type::Path(type_values))] => {
                                // let obj = obj();
                                let transformer = match conversion_type_for_path(&type_values.path) {
                                    ConversionType::Simple => quote!(clone()),
                                    ConversionType::Complex => {
                                        let mapper = package_boxed_expression(ffi_to_conversion(quote!(o)));
                                        iter_map_collect(quote!(iter()), quote!(|o| #mapper))
                                    },
                                    ConversionType::Map => panic!("define_option: Map nested in Vec not supported yet"),
                                    ConversionType::Vec => panic!("define_option: Vec nested in Vec not supported yet"),
                                };
                                let conversion = package_boxed_expression(quote!(rs_ffi_interfaces::VecFFI::new(vec.#transformer)));
                                quote!(match #field_path { Some(vec) => #conversion, None => std::ptr::null_mut()})
                            },
                            _ => panic!("to_option_conversion: bad args {:?}", last_segment)
                        },
                        _ => panic!("to_option_conversion: Unknown args {:?}", last_segment)
                    },
                    _ => ffi_to_opt_conversion(field_path)
                }
            },
            _ => panic!("to_option_conversion: Bad args {:?}", args)
        },
        _ => panic!("to_option_conversion: Bad arguments {:?}", arguments)
    }
}

fn to_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    // let obj_field_name_quote = obj_field_name(ident.clone());
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        "Vec" => to_vec_conversion(field_path, &last_segment.arguments),
        "BTreeMap" | "HashMap" => to_map_conversion(field_path, &last_segment.arguments),
        "Option" => to_option_conversion(field_path, &last_segment.arguments),
        _ => ffi_to_conversion(field_path)
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(obj_field_name(ident), quote!(|o| #expr)))
}

fn ffi_from_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from = ffi_from();
    quote!(#package::#interface::#ffi_from(#field_value))
}

fn ffi_to_conversion(field_path: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to = ffi_to();
    quote!(#package::#interface::#ffi_to(#field_path))
}

fn ffi_from_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_from_opt = ffi_from_opt();
    quote!(#package::#interface::#ffi_from_opt(#field_value))
}

fn ffi_to_opt_conversion(field_value: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi_to_opt = ffi_to_opt();
    quote!(#package::#interface::#ffi_to_opt(#field_value))
}

// fn to_field(field_name: &Ident) -> TokenStream2 {
//     define_field(quote!(#field_name), ffi_to_conversion(obj_field_name(quote!(#field_name))))
// }
//
// fn to_opt_field(field_name: &Ident) -> TokenStream2 {
//     define_field(quote!(#field_name), ffi_to_opt_conversion(obj_field_name(quote!(#field_name))))
// }

fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Array(TypeArray { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
            _ => panic!("to_pointer: Unknown field (arr->) {:?} {:?}", field_path, elem),
        },
        Type::Ptr(TypePtr { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
            // Type::Ptr(type_ptr) => to_vec_ptr(f, type_ptr),
            Type::Array(type_arr) => to_vec_ptr(field_path, type_ptr, type_arr),
            _ => panic!("to_pointer: Unknown field (ptr->) {:?} {:?}", field_path, elem),
        },
        Type::Path(type_path) => to_path(field_path, &type_path.path, Some(type_ptr)),
        _ => panic!("to_pointer: Unknown field (path->) {:?} {:?}", field_path, type_ptr.elem),
    }
}

fn to_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, None),
        _ => panic!("to_reference: Unknown field {:?} {:?}", field_path, type_reference.elem)
    }
}

// fn to_arr(f: &Field, _type_array: &TypeArray) -> TokenStream2 {
//     let field_name = &f.ident.clone().unwrap();
//     if let Type::Path(type_path) = &f.ty {
//         if type_path.path.segments.last().unwrap().ident == "Option" {
//             return to_opt_field(field_name)
//         }
//     }
//     to_field(field_name)
// }

fn conversion_type_for_path(path: &Path) -> ConversionType {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => ConversionType::Simple,
        "BTreeMap" | "HashMap" => ConversionType::Map,
        "Vec" => ConversionType::Vec,
        _ => ConversionType::Complex
    }

}

fn convert_path_to_field_type(path: &Path) -> TokenStream2 {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let field_type = &last_segment.ident;
    match field_type.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!(#field_type),
        "str" | "String" => quote!(*mut std::os::raw::c_char),
        "UInt128" => quote!(*mut [u8; 16]),
        "UInt160" => quote!(*mut [u8; 20]),
        "UInt256" => quote!(*mut [u8; 32]),
        "UInt384" => quote!(*mut [u8; 48]),
        "UInt512" => quote!(*mut [u8; 64]),
        "UInt768" => quote!(*mut [u8; 96]),
        "VarInt" => quote!(u64),
        _ => {
            last_segment.ident = Ident::new(&format!("{}FFI", last_segment.ident), last_segment.ident.span());
            let field_type = cloned_segments.into_iter().map(|segment| quote_spanned! { segment.span() => #segment }).collect::<Vec<_>>();
            let full_path = quote!(#(#field_type)::*);
            quote!(*mut #full_path)
        }
    }
}

fn ffi_struct_name(field_type: &Ident) -> Ident {
    format_ident!("{}FFI", field_type)
}

fn extract_map_arg_type(path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "BTreeMap" | "HashMap" => match &last_segment.arguments {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                [GenericArgument::Type(Type::Path(TypePath { path: path_keys, .. })), GenericArgument::Type(Type::Path(TypePath { path: path_values, .. }))] =>
                    ffi_map_field_type(extract_map_arg_type(path_keys), extract_map_arg_type(path_values)),
                _ => panic!("convert_map_arg_type: bad args {:?}", last_segment)
            },
            _ => panic!("convert_map_arg_type: Unknown args {:?}", last_segment)
        },
        _ => convert_path_to_field_type(path)
    }
}

fn extract_vec_arg_type(path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "Vec" => match &last_segment.arguments {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                [GenericArgument::Type(Type::Path(TypePath { path, .. }))] =>
                    ffi_vec_field_type(extract_vec_arg_type(path)),
                _ => panic!("extract_vec_arg_type: bad args {:?}", path)
            },
            _ => panic!("extract_vec_arg_type: Unknown args {:?}", path)
        },
        _ => convert_path_to_field_type(path)
    }
}

fn extract_struct_field(field_type: &Type) -> TokenStream2 {
    match field_type {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            let arguments = &last_segment.arguments;
            match last_segment.ident.to_string().as_str() {
                "Vec" => match arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                        [GenericArgument::Type(Type::Path(TypePath { path, .. }))] =>
                            ffi_vec_field_type(extract_vec_arg_type(path)),
                        _ => panic!("extract_struct_field: Vec: args: {:?} not supported", args)
                    }
                    _ => panic!("extract_struct_field: Vec: arguments: {:?} not supported", arguments)
                },
                "BTreeMap" | "HashMap" => match arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                        [GenericArgument::Type(Type::Path(TypePath { path: path_keys, .. })), GenericArgument::Type(Type::Path(TypePath { path: path_values, .. }))] =>
                            ffi_map_field_type(extract_map_arg_type(path_keys), extract_map_arg_type(path_values)),
                        _ => panic!("extract_struct_field: Map: args: {:?} not supported", args)
                    }
                    _ => panic!("extract_struct_field: Map: arguments: {:?} not supported", arguments)
                },
                "Option" => match arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                        [GenericArgument::Type(field_type)] =>
                            extract_struct_field(field_type),
                        _ => panic!("extract_struct_field: Option: {:?} not supported", args)
                    }
                    _ => panic!("extract_struct_field: Option: {:?} not supported", arguments)
                },
                _ => convert_path_to_field_type(path),
            }
        },
        Type::Reference(TypeReference { elem, .. }) => extract_struct_field(&**elem),
        _ => panic!("extract_struct_field: field type {:?} not supported", field_type)
    }
}




fn impl_interface(ffi_name: TokenStream2, target_name: TokenStream2, ffi_from_conversion: TokenStream2, ffi_to_conversion: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let ffi = ffi();
    let obj = obj();
    let ffi_from = ffi_from();
    let ffi_to = ffi_to();
    let ffi_from_opt = ffi_from_opt();
    let ffi_to_opt = ffi_to_opt();
    quote! {
        impl #package::#interface<#target_name> for #ffi_name {
            unsafe fn #ffi_from(#ffi: *mut #ffi_name) -> #target_name { #ffi_from_conversion }
            unsafe fn #ffi_to(#obj: #target_name) -> *mut #ffi_name { #ffi_to_conversion }
            unsafe fn #ffi_from_opt(#ffi: *mut #ffi_name) -> Option<#target_name> {
                (!#ffi.is_null()).then_some(<Self as #package::#interface<#target_name>>::#ffi_from(#ffi))
            }
            unsafe fn #ffi_to_opt(#obj: Option<#target_name>) -> *mut #ffi_name {
                #obj.map_or(std::ptr::null_mut(), |o| <Self as #package::#interface<#target_name>>::#ffi_to(o))
            }
        }
    }
}

fn from_unnamed_struct(fields: &FieldsUnnamed, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let obj = obj();
    let ffi_deref = ffi_deref();
    let interface_impl = match fields.unnamed.first().unwrap().ty.clone() {
        Type::Path(ffi_name) => impl_interface(
            quote!(#ffi_name),
            quote!(#target_name),
            quote!(let ffi_ref = *ffi; #target_name(#ffi_deref.0)),
            package_boxed_expression(quote!(#ffi_name(#obj.0)))
        ),
        Type::Array(ffi_name) => impl_interface(
            quote!(#ffi_name),
            quote!(#target_name),
            quote!(let ffi_ref = *ffi; #target_name(#ffi_deref)),
            package_boxed_expression(quote!(#obj.0))
        ),
        _ => panic!("Expected array type")
    };
    let expanded = quote! {
        #input
        #interface_impl
    };
    println!("{}", expanded);
    TokenStream::from(expanded)
}

fn from_named_struct(fields: &FieldsNamed, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let ffi_name = input.ident.clone();
    let conversions_to_ffi = fields.named.iter().map(|Field { ident, ty, .. }| {
        let ident = &ident.clone().unwrap();
        match ty {
            Type::Ptr(type_ptr) => define_field(ident.to_token_stream(),to_ptr(obj_field_name(quote!(#ident)), type_ptr)),
            // Type::Array(type_array) => to_arr(f, type_array),
            Type::Path(type_path) => define_field(ident.to_token_stream(), to_path(obj_field_name(quote!(#ident)), &type_path.path, None)),
            Type::Reference(type_reference) => define_field(ident.to_token_stream(), to_reference(obj_field_name(quote!(#ident)), type_reference)),
            _ => panic!("from_named_struct: Unknown field {:?} {:?}", ident, ty),
        }
    }).collect::<Vec<_>>();
    let conversions_from_ffi = fields.named.iter().map(|Field { ident, ty, .. } | {
        let field_name = &ident.clone().unwrap();
        let field_path = ffi_deref_field_name(quote!(#field_name));
        Box::new(define_field(quote!(#field_name),match ty {
            Type::Ptr(type_ptr) => from_ptr(field_path, type_ptr),
            Type::Path(type_path) => from_path(field_path, &type_path.path, None),
            Type::Reference(type_reference) => from_reference(field_path, type_reference),
            _ => field_path,
        }))
    }).collect::<Vec<_>>();
    let struct_fields = fields.named
        .iter()
        .map(|f| Box::new(define_pub_field(f.ident.clone().unwrap().to_token_stream(), extract_struct_field(&f.ty))))
        .collect::<Vec<_>>();
    let ffi_name = ffi_struct_name(&ffi_name);
    let ffi_struct = create_struct(quote!(#ffi_name), struct_fields);
    let interface_impl = impl_interface(
        quote!(#ffi_name),
        quote!(#target_name),
        quote!(let ffi_ref = &*ffi; #target_name { #(#conversions_from_ffi,)* }),
        package_boxed_expression(quote!(#ffi_name { #(#conversions_to_ffi,)* })),
    );
    let expanded = quote! {
        #input
        #ffi_struct
        #interface_impl
    };
    println!("{}", expanded);
    TokenStream::from(expanded)
}

fn from_enum_variant(variant: &Variant, _index: usize) -> TokenStream2 {
    let variant_name = &variant.ident;
    match &variant.discriminant {
        Some((_, Expr::Lit(lit, ..))) => quote!(#variant_name = #lit),
        None => {
            let extract_associated_type = |field_type: &Type| {
                let converted_type = extract_struct_field(field_type);
                quote!(#variant_name(#converted_type))
            };
            let enum_fields = match &variant.fields {
                Fields::Named(ref fields) => fields.named.iter().map(|f| Box::new(extract_associated_type(&f.ty))).collect::<Vec<_>>(),
                Fields::Unnamed(ref fields) => fields.unnamed.iter().map(|f| Box::new(extract_associated_type(&f.ty))).collect::<Vec<_>>(),
                Fields::Unit => vec![Box::new(quote!(#variant_name))],
            };
            quote!(#(#enum_fields)*)
        },
        _ => panic!("Error variant discriminant")
    }
}

fn from_enum(data_enum: &DataEnum, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let variants = &data_enum.variants;
    let variant_fields = variants
        .iter()
        .enumerate()
        .map(|(index, variant)| from_enum_variant(variant, index))
        .collect::<Vec<_>>();
    let ffi_name = ffi_struct_name(&target_name);

    let converted = quote! {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
        pub enum #ffi_name {
            #(#variant_fields,)*
        }
    };
    let to_ffi_arms = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        match &v.fields {
            Fields::Unnamed(fields) => match &fields.unnamed.first().unwrap().ty {
                Type::Path(TypePath { path, .. }) => {
                    let obj = obj();
                    let conversion = match conversion_type_for_path(path) {
                        ConversionType::Simple => obj.clone(),
                        ConversionType::Complex => ffi_to_conversion(obj.clone()),
                        ConversionType::Vec | ConversionType::Map => unimplemented!("Enum with Map/Vec as associated value not supported yet")
                    };
                    define_lambda(quote!(#target_name::#ident(#obj)), quote!(#ffi_name::#ident(#conversion)))
                },
                _ => panic!("Unsupported field type in enum variant")
            },
            Fields::Unit => define_lambda(quote!(#target_name::#ident), quote!(#ffi_name::#ident)),
            _ => panic!("Unsupported fields in enum variant"),
        }
    });

    let from_ffi_arms = data_enum.variants.iter().map(|v| {
        let ident = &v.ident;
        match &v.fields {
            Fields::Unnamed(fields) => {
                let mut variant_fields = vec![];
                let mut converted_fields = vec![];
                fields.unnamed.iter().enumerate().for_each(|(index, field)| match &field.ty {
                    Type::Path(TypePath { path, .. }) => {
                        let field_indexed = format_ident!("o_{}", index);
                        variant_fields.push(quote!(#field_indexed));
                        converted_fields.push(match conversion_type_for_path(path) {
                            ConversionType::Simple => quote!(#field_indexed),
                            ConversionType::Complex => ffi_from_conversion(quote!(#field_indexed)),
                            ConversionType::Vec | ConversionType::Map => unimplemented!("Enum with Map/Vec as associated value not supported yet")
                        });
                    },
                    _ => panic!("Unsupported field type in enum variant")
                });
                define_lambda(quote!(#ffi_name::#ident(#(#variant_fields,)*)), quote!(#target_name::#ident(#(#converted_fields,)*)))
            },
            Fields::Unit => define_lambda(quote!(#ffi_name::#ident), quote!(#target_name::#ident)),
            _ => panic!("Unsupported fields in enum variant"),
        }
    });
    let obj = obj();
    let ffi_deref = ffi_deref();
    let interface_impl = impl_interface(
        quote!(#ffi_name),
        quote!(#target_name),
        quote!(let ffi_ref = *ffi; match #ffi_deref { #(#from_ffi_arms),* }),
        package_boxed_expression(quote!(match #obj { #(#to_ffi_arms),* })),
    );

    let expanded = quote! {
        #input
        #converted
        #interface_impl
    };
    println!("{}", expanded);
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn impl_ffi_conv(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let target_name = match attrs.first() {
        Some(NestedMeta::Lit(literal)) => format_ident!("{}", literal.to_token_stream().to_string()),
        Some(NestedMeta::Meta(Meta::Path(path))) => path.segments.first().unwrap().ident.clone(),
        _ => {
            // use default rules
            // for unnamed structs like UInt256 -> #target_name = [u8; 32]
            // for named structs -> generate ($StructName)FFI
            input.ident.clone()
        },
    };
    match input.data {
        Data::Struct(DataStruct { fields: ref f, ..}) => {
            match f {
                Fields::Named(ref fields) => from_named_struct(fields, target_name, &input),
                Fields::Unnamed(ref fields) => from_unnamed_struct(fields, target_name, &input),
                Fields::Unit => panic!("Fields::Unit not supported"),
            }
        },
        Data::Enum(ref data_enum) => from_enum(data_enum, target_name, &input),
        _ => panic!("FFIConversion can only be derived for structs & enums")
    }
}

#[proc_macro_attribute]
pub fn impl_ffi_fn_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let signature = &input_fn.sig;
    let fn_name = &signature.ident;
    let ffi_fn_name = format_ident!("ffi_{}", fn_name);
    let obj = obj();
    let output_type = match &signature.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, field_type) => extract_struct_field(&field_type),
    };

    let args_converted = signature.inputs.iter().map(|arg| match arg {
            FnArg::Typed(PatType { ty, pat, .. }) =>
                Box::new(define_field(pat.to_token_stream(), extract_struct_field(&ty))),
            _ => panic!("Arg type {:?} not supported", arg)
        }).collect::<Vec<_>>();

    let args_conversions = signature.inputs.iter().map(|arg| match *arg {
        FnArg::Typed(ref pat_type) => match (&*pat_type.ty, &*pat_type.pat) {
            (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) =>
                from_path(quote!(#ident), path, None),
            _ => panic!("error: arg conversion: {:?}", pat_type.ty)
        },
        _ => panic!("Expected typed function argument")
    }).collect::<Vec<_>>();

    let output_conversion = match &signature.output {
        ReturnType::Default => quote! { ; },
        ReturnType::Type(_, field_type) => match &**field_type {
            Type::Path(TypePath { path, .. }) => to_path(obj.clone(), path, None),
            _ => panic!("error: output conversion: {:?}", field_type),
        },
    };

    let expanded = quote! {
        #input_fn
        #[no_mangle]
        pub unsafe extern "C" fn #ffi_fn_name(#(#args_converted),*) -> #output_type {
            let #obj = #fn_name(#(#args_conversions),*);
            #output_conversion
        }
    };

    println!("{}", expanded);
    expanded.into()
}

// #[proc_macro_attribute]
// pub fn impl_ffi_fn_conv(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let function = parse_macro_input!(item as ItemFn);
//     println!("impl_ffi_fn_conv: input: {:?}", function);
//     let ident = function.sig.ident.clone();
//     // Create the FFI function name
//     let ffi_fn_name = format!("ffi_{}", function.sig.ident);
//
//     // Build the FFI function signature
//     let ffi_fn_ident = Ident::new(&ffi_fn_name, ident.span());
//     let inputs = &function.sig.inputs;
//     let output = &function.sig.output;
//
//     println!("impl_ffi_fn_conv: ffi_fn_ident: {:?}", ffi_fn_ident);
//     println!("impl_ffi_fn_conv: inputs: {:?}", inputs);
//     println!("impl_ffi_fn_conv: output: {:?}", output);
//     println!("impl_ffi_fn_conv: function: {:?}", function);
//     // Generate the FFI function
//     let ffi_fn = quote! {
//         #[no_mangle] pub extern "C" fn #ffi_fn_ident(#inputs) #output {
//             #ident(#inputs)
//         }
//     };
//     let result = ffi_fn.into();
//     println!("impl_ffi_fn_conv: output: {}", result);
//     // Convert the generated function back into a TokenStream and return it
//     result
//
// }

// #[proc_macro_attribute]
// pub fn ffi_chain_settings(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemTrait);
//
//     let name = &input.ident;
//     let ffi_name = format_ident!("{}FFI", name);
//     let methods = input.items.into_iter().filter_map(|item| {
//         if let TraitItem::Method(method) = item {
//             Some(method)
//         } else {
//             None
//         }
//     });
//
//     // Generating FFI methods for the trait
//     let ffi_methods = methods.map(|method| {
//         let name = &method.sig.ident;
//         let ffi_name = format_ident!("{}_ffi", name);
//         let args = method.sig.inputs.iter().map(|arg| {
//             if let FnArg::Typed(arg) = arg {
//                 let arg_name = &arg.pat;
//                 quote! { #arg_name: <#arg_name as FFIConversion<_>>::ffi_to(#arg_name) }
//             } else {
//                 panic!("Unexpected argument type");
//             }
//         });
//
//         quote! {
//             unsafe fn #ffi_name(&self, #(#args),*) -> *mut _ {
//                 <_ as FFIConversion<_>>::ffi_to(self.#name(#(#args)*))
//             }
//         }
//     });
//
//     let expanded = quote! {
//         pub trait #ffi_name {
//             #(#ffi_methods)*
//         }
//
//         impl #name for ChainType {
//             // You need to put actual implementations here...
//         }
//
//         impl #name for DevnetType {
//             // You need to put actual implementations here...
//         }
//
//         impl #ffi_name for ChainType {
//             // FFI implementations...
//         }
//
//         impl #ffi_name for DevnetType {
//             // FFI implementations...
//         }
//     };
//
//     TokenStream::from(expanded)
// }