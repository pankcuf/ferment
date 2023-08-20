extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, ItemFn, Meta, NestedMeta, Type, PathArguments, GenericArgument, TypePtr, TypeArray, Ident, TypePath, DataStruct, Fields, FieldsUnnamed, FieldsNamed, DataEnum, Expr, Path, ReturnType, FnArg, PatType, AngleBracketedGenericArguments, Pat, PatIdent, Field, TypeReference, Variant, Item, ItemType};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use quote::__private::Span;
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

fn obj() -> TokenStream2 {
    quote!(obj)
}

fn destroy() -> TokenStream2 {
    quote!(destroy)
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

fn unbox_any() -> TokenStream2 {
    quote!(unbox_any)
}

fn package_boxed() -> TokenStream2 {
    let package = package();
    let boxed = boxed();
    quote!(#package::#boxed)
}

fn package_unbox_any() -> TokenStream2 {
    let package = package();
    let unbox_any = unbox_any();
    quote!(#package::#unbox_any)
}

fn package_unbox_any_expression(expr: TokenStream2) -> TokenStream2 {
    let package_unbox_any = package_unbox_any();
    quote!(#package_unbox_any(#expr))
}

fn package_unboxed_root() -> TokenStream2 {
    package_unbox_any_expression(ffi())
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
    quote!(ffi_ref.#field_name)
}

fn obj_field_name(field_name: TokenStream2) -> TokenStream2 {
    let obj = obj();
    quote!(#obj.#field_name)
}

fn create_struct(name: TokenStream2, fields: Vec<TokenStream2>) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone, Debug)]
        pub struct #name { #(#fields,)* }
    }
}

fn create_unnamed_struct(name: TokenStream2, fields: Vec<TokenStream2>) -> TokenStream2 {
    quote! {
        #[repr(C)]
        #[derive(Clone, Debug)]
        pub struct #name(#(#fields,)*);
    }
}

fn ffi_vec_field_type(value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::VecFFI<#value_type>)
}

fn ffi_map_field_type(key_type: TokenStream2, value_type: TokenStream2) -> TokenStream2 {
    quote!(*mut rs_ffi_interfaces::MapFFI<#key_type, #value_type>)
}

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

fn destroy_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let package = package();
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path, .. }))] => match conversion_type_for_path(path) {
                ConversionType::Simple => {
                    quote!(#package::unbox_any(#field_path);)
                },
                ConversionType::Complex => {
                    quote!(#package::unbox_any(#field_path);)
                },
                ConversionType::Vec => destroy_vec(path, quote!(#field_path)),
                ConversionType::Map => panic!("destroy_vec (Map): Unknown field {:?} {:?}", field_path, args),
            },
            _ => panic!("destroy_vec: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("destroy_vec: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn unbox_vec(var: TokenStream2, field_path: TokenStream2, conversion: TokenStream2) -> TokenStream2 {
    quote!({
        let #var = #field_path;
        #conversion
    })
}

fn box_vec(var: TokenStream2, field_path: TokenStream2, values_conversion: TokenStream2) -> TokenStream2 {
    package_boxed_expression(quote!({
        let #var = #field_path;
        rs_ffi_interfaces::VecFFI { count: #var.len(), values: #values_conversion }
    }))
}
fn from_simple_vec_conversion(field_path: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    quote!(std::slice::from_raw_parts(#field_path.values as *const #field_type, #field_path.count).to_vec())
}

fn from_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let ffi_from_conversion = ffi_from_conversion(quote!(*#field_path.values.add(i)));
    iter_map_collect(quote!((0..#field_path.count)), quote!(|i| #ffi_from_conversion))
}

fn from_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    let conversion = match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path, .. }))] => match conversion_type_for_path(path) {
                ConversionType::Simple => from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream()),
                ConversionType::Complex => from_complex_vec_conversion(quote!(vec)),
                ConversionType::Vec => from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
                _ => panic!("from_vec_vec_conversion: Vec<Map<>> not supported yet")
            },
            _ => panic!("from_vec_vec_conversion: Bad args {:?}", args)
        }
        _ => panic!("from_vec_vec_conversion: Bad arguments {:?}", arguments)
    };
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&**vec.values.add(i)), conversion);
    iter_map_collect(quote!((0..vec.count)), quote!(|i| #unbox_conversion))
}

fn to_simple_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    quote!(#field_path.clone())
}

fn to_complex_vec_conversion(field_path: TokenStream2) -> TokenStream2 {
    let conversion = ffi_to_conversion(quote!(o));
    iter_map_collect(quote!(#field_path.into_iter()),  quote!(|o| #conversion))
}

fn to_vec_vec_conversion(arguments: &PathArguments) -> TokenStream2 {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path))] => {
                let mapper = |path: &Path| {
                    match conversion_type_for_path(path) {
                        ConversionType::Simple => to_simple_vec_conversion(quote!(vec)),
                        ConversionType::Complex => to_complex_vec_conversion(quote!(vec)),
                        ConversionType::Vec => to_vec_vec_conversion(&path.segments.last().unwrap().arguments),
                        _ => panic!("No triple nested vec/map")
                    }
                };
                let values_conversion = package_boxed_vec_expression(mapper(&inner_path.path));
                let boxed_conversion = box_vec(quote!(vec), quote!(o), values_conversion);
                iter_map_collect(quote!(vec.into_iter()), quote!(|o| #boxed_conversion))
            },
            _ => panic!("to_vec_conversion: bad args {:?}", args)
        },
        _ => panic!("to_vec_conversion: bad arguments {:?}", arguments)
    }
}


fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path, .. }))] => match conversion_type_for_path(path) {
                ConversionType::Simple => from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream()),
                ConversionType::Complex => from_complex_vec_conversion(quote!(vec)),
                ConversionType::Vec => from_vec_vec_conversion(&path.segments.last().unwrap().arguments),
                ConversionType::Map => panic!("from_vec (Map): Unknown field {:?} {:?}", field_path, args),
            },
            _ => panic!("from_vec: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_vec: Bad arguments {:?} {:?}", field_path, arguments)
    };
    let unbox_conversion = unbox_vec(quote!(vec), quote!(&*#field_path), conversion);
    unbox_conversion
}
fn destroy_map(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let package = package();
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(TypePath { path: _path_keys, .. })), GenericArgument::Type(Type::Path(TypePath { path: _path_values, .. }))] => match conversion_type_for_path(path) {
                ConversionType::Simple => {
                    quote!(#package::unbox_any(#field_path);)
                },
                ConversionType::Complex => {
                    quote!(#package::unbox_any(#field_path);)
                },
                ConversionType::Vec => destroy_vec(path, quote!(#field_path)),
                ConversionType::Map => quote!(#package::unbox_any(#field_path);),
            },
            _ => panic!("destroy_map: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("destroy_map: Bad arguments {:?} {:?}", field_path, arguments)
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
                    ConversionType::Vec => from_vec(inner_path_value_path, quote!(*map.values.add(#key_index))),
                    ConversionType::Map => panic!("Map not supported as Map key")
                };
                let inner_path_value_path_last_segment = inner_path_value_path.segments.last().unwrap();
                let inner_path_value_path_last_segment_args = &inner_path_value_path_last_segment.arguments;
                let value_conversion = match conversion_type_for_path(inner_path_value_path) {
                    ConversionType::Simple => value_simple_conversion,
                    ConversionType::Complex => ffi_from_conversion(value_simple_conversion),
                    ConversionType::Vec => from_vec(inner_path_value_path, quote!(*map.values.add(#key_index))),
                    ConversionType::Map => {
                        let field_type = &inner_path_value_path_last_segment.ident;
                        match &inner_path_value_path_last_segment_args {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
                                [GenericArgument::Type(Type::Path(TypePath { path: inner_path_key_path, ..})), GenericArgument::Type(Type::Path(TypePath { path: inner_path_value_path, ..}))] => {
                                    let key_index = quote!(i);
                                    let simple_conversion = |buffer: TokenStream2| quote!(#buffer.add(#key_index));
                                    let key_simple_conversion = simple_conversion(quote!(*map.keys));
                                    let value_simple_conversion = simple_conversion(quote!(*map.values));
                                    let key_conversion = match conversion_type_for_path(inner_path_key_path) {
                                        ConversionType::Simple => key_simple_conversion,
                                        ConversionType::Complex  => ffi_from_conversion(key_simple_conversion),
                                        ConversionType::Vec => from_vec(inner_path_key_path, quote!(*map.values.add(#key_index))),
                                        ConversionType::Map => panic!("Vec/Map not supported as Map key")
                                    };
                                    let value_conversion = match conversion_type_for_path(inner_path_value_path) {
                                        ConversionType::Simple => value_simple_conversion,
                                        ConversionType::Complex => ffi_from_conversion(value_simple_conversion),
                                        ConversionType::Vec => from_vec(inner_path_value_path, quote!(*map.values.add(#key_index))),
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

// TODO: doesn't work
fn destroy_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) => match path.segments.last() {
                Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                    // std convertible
                    // TODO: what to use? 0 or ::MAX
                    "i8" | "u8" | "i16" | "u16" |
                    "i32" | "u32" | "i64" | "u64" |
                    "i128" | "u128" | "isize" | "usize" => quote!({}),
                    // TODO: mmm shit that's incorrect
                    "bool" => quote!({}),
                    "Vec" => {
                        let conversion = destroy_vec(path, field_path.clone());
                        quote!(if !#field_path.is_null() { #conversion; })
                    },
                    _ => {
                        let conversion = package_unbox_any_expression(field_path.clone());
                        quote!(if !#field_path.is_null() { #conversion; })
                    }
                },
                _ => panic!("from_option: (type->path) Unknown field {:?} {:?}", field_path, path)
            },
            _ => panic!("from_option: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

// TODO: Option<Map>
fn from_option(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    let arguments = &last_segment.arguments;
    match arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) => match path.segments.last() {
                Some(inner_segment) => match inner_segment.ident.to_string().as_str() {
                    // std convertible
                    // TODO: what to use? 0 or ::MAX
                    "i8" | "u8" | "i16" | "u16" |
                    "i32" | "u32" | "i64" | "u64" |
                    "i128" | "u128" | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
                    // TODO: mmm shit that's incorrect
                    "bool" => quote!((#field_path).then_some(#field_path)),
                    "Vec" => {
                        let conversion = from_vec(path, field_path.clone());
                        quote!((!#field_path.is_null()).then_some(#conversion))
                    },
                    _ => ffi_from_opt_conversion(field_path)
                },
                _ => panic!("from_option: (type->path) Unknown field {:?} {:?}", field_path, path)
            },
            _ => panic!("from_option: Unknown field {:?} {:?}", field_path, args)
        },
        _ => panic!("from_option: Bad arguments {:?} {:?}", field_path, arguments)
    }
}

fn from_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "u8" => quote!(*#field_path),
                _ => panic!("from_array: unsupported ident {:?} {:?}", field_path, last_segment.ident)
            }
        },
        _ => panic!("from_array: unsupported {:?} {:?}", field_path, type_array.elem)
    }
}

fn destroy_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(_type_path) => package_unbox_any_expression(quote!(#field_path)),
        _ => panic!("from_array: unsupported {:?} {:?}", field_path, type_array.elem)
    }
}

fn destroy_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!({}),
        "VarInt" => quote!({}),
        "Option" => destroy_option(path, field_path),
        "Vec" => destroy_vec(path, field_path),
        "BTreeMap" | "HashMap" => destroy_map(path, field_path),
        "str" => destroy_conversion(field_path, convert_path_to_ffi_type(path), quote!(&#path)),
        _ => destroy_conversion(field_path, convert_path_to_ffi_type(path), quote!(#path))
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

fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, Some(type_ptr)),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {:?}", type_ptr)
    }
}

fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => from_ptr(field_path, type_ptr),
        Type::Path(type_path) => from_path(field_path, &type_path.path, Some(type_ptr)),
        _ => ffi_from_conversion(field_path)
    }
}

fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {:?} {:?}", field_path, type_reference)
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

fn to_vec_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path))] => {
                let mapper = |path: &Path| {
                    match conversion_type_for_path(path) {
                        ConversionType::Simple => to_simple_vec_conversion(quote!(vec)),
                        ConversionType::Complex => to_complex_vec_conversion(quote!(vec)),
                        ConversionType::Vec => to_vec_vec_conversion(&path.segments.last().unwrap().arguments),
                        ConversionType::Map => panic!("to_vec_conversion: Map nested in Vec not supported yet"),
                    }
                };
                let values_conversion = package_boxed_vec_expression(mapper(&inner_path.path));
                let boxed_conversion = box_vec(quote!(vec), field_path, values_conversion);
                boxed_conversion
            },
            _ => panic!("to_vec_conversion: bad args {:?}", args)
        },
        _ => panic!("to_vec_conversion: bad arguments {:?}", arguments)
    }
}

fn to_map_conversion(field_path: TokenStream2, arguments: &PathArguments) -> TokenStream2 {
    package_boxed_expression(match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
            [GenericArgument::Type(Type::Path(inner_path_key)), GenericArgument::Type(Type::Path(inner_path_value))] => {
                let mapper = |path: &Path| {
                    let conversion = match conversion_type_for_path(path) {
                        ConversionType::Simple => quote!(o),
                        ConversionType::Complex => ffi_to_conversion(quote!(o)),
                        ConversionType::Vec => to_vec_conversion(quote!(o), &path.segments.last().unwrap().arguments),
                        ConversionType::Map => to_map_conversion(quote!(o), &path.segments.last().unwrap().arguments)
                    };
                    quote!(|o| #conversion)
                };
                let key_mapper = mapper(&inner_path_key.path);
                let value_mapper = mapper(&inner_path_value.path);
                let keys_conversion = package_boxed_vec_expression(quote!(map.keys().cloned().map(#key_mapper).collect()));
                let values_conversion = package_boxed_vec_expression(quote!(map.values().cloned().map(#value_mapper).collect()));
                quote!({let map = #field_path; rs_ffi_interfaces::MapFFI { count: map.len(), keys: #keys_conversion, values: #values_conversion }})
            },
            _ => panic!("to_map_conversion: Bad args {:?} {:?}", field_path, args)
        },
        _ => panic!("to_map_conversion: Bad arguments {:?} {:?}", field_path, arguments)
    })
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
fn destroy_conversion(field_value: TokenStream2, ffi_type: TokenStream2, field_type: TokenStream2) -> TokenStream2 {
    let package = package();
    let interface = interface();
    let destroy = destroy();
    quote!(<#ffi_type as #package::#interface<#field_type>>::#destroy(#field_value))
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

fn to_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => {
            to_path(package_boxed_expression(field_path), &type_path.path, None)
        },
        _ => panic!("to_array: Unknown field {:?} {:?}", field_path, type_array.elem)
    }
}

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

fn convert_path_to_ffi_type(path: &Path) -> TokenStream2 {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let field_type = &last_segment.ident;
    match field_type.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" |
        "i32" | "u32" | "i64" | "u64" |
        "i128" | "u128" | "isize" | "usize" | "bool" => quote!(#field_type),
        "str" | "String" => quote!(std::os::raw::c_char),
        "UInt128" => quote!([u8; 16]),
        "UInt160" => quote!([u8; 20]),
        "UInt256" => quote!([u8; 32]),
        "UInt384" => quote!([u8; 48]),
        "UInt512" => quote!([u8; 64]),
        "UInt768" => quote!([u8; 96]),
        "VarInt" => quote!(u64),
        _ => {
            last_segment.ident = Ident::new(&format!("{}FFI", last_segment.ident), last_segment.ident.span());
            let field_type = cloned_segments.into_iter().map(|segment| quote_spanned! { segment.span() => #segment }).collect::<Vec<_>>();
            let full_path = quote!(#(#field_type)::*);
            quote!(#full_path)
        }
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
        Type::Array(TypeArray { elem, len, .. }) => {
            quote!(*mut [#elem; #len])
        },
        Type::Reference(TypeReference { elem, .. }) => extract_struct_field(&**elem),
        _ => panic!("extract_struct_field: field type {:?} not supported", field_type)
    }
}




fn impl_interface(ffi_name: TokenStream2, target_name: TokenStream2, ffi_from_conversion: TokenStream2, ffi_to_conversion: TokenStream2, destroy_code: TokenStream2) -> TokenStream2 {
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
            unsafe fn destroy(#ffi: *mut #ffi_name) {
                #destroy_code;
            }
        }
    }
}

fn impl_drop(ffi_name: TokenStream2, drop_code: TokenStream2) -> TokenStream2 {
    quote! {
        impl Drop for #ffi_name {
            fn drop(&mut self) {
                unsafe { #drop_code }
            }
        }

    }
}

fn from_unnamed_struct(fields: &FieldsUnnamed, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let obj = obj();
    // assert!(fields.unnamed.len() <= 1, "Unnamed structs with multiple fields not supported yet");
    let ffi_name = ffi_struct_name(&target_name);
    let (ffi_struct, interface_impl, drop_impl) = match target_name.clone().to_string().as_str() {
        "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" | "VarInt" => {

            let (ffi_name, ffi_from_conversion, ffi_to_conversion, destroy_code, drop_code) = match fields.unnamed.first().unwrap().ty.clone() {
                // UInt256 etc
                Type::Path(TypePath { path, .. }) => {
                    // match  { }
                    match path.segments.last().unwrap().ident.to_string().as_str() {
                        "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" => {
                            (
                                quote!(#path),
                                quote!(let ffi_ref = *ffi; #target_name(ffi_ref.0)),
                                package_boxed_expression(quote!(#path(#obj.0))),
                                package_unboxed_root(),
                                None
                            )
                        },
                        _ => {
                            (
                                quote!(#ffi_name),
                                {
                                    let conversion = from_path(quote!(ffi_ref.0), &path, None);
                                    quote!(let ffi_ref = *ffi; #target_name(#conversion))
                                },
                                package_boxed_expression(quote!(#ffi_name(#obj.0))),
                                package_unboxed_root(),
                                None
                            )
                        }
                    }
                },
                // VarInt
                Type::Array(ffi_name) => (
                    quote!(#ffi_name),
                    quote!(let ffi_ref = *ffi; #target_name(ffi_ref)),
                    package_boxed_expression(quote!(#obj.0)),
                    quote!(),
                    None
                ),
                _ => unimplemented!("from_unnamed_struct: not supported {:?}", fields.unnamed.first().unwrap().ty)
            };
            let interface_impl = impl_interface(ffi_name.clone(), quote!(#target_name), ffi_from_conversion, ffi_to_conversion, destroy_code);

            let drop_impl = drop_code.map_or(quote!(), |drop_code| impl_drop(ffi_name, drop_code));
            // let ffi_struct = create_unnamed_struct(quote!(#ffi_name), vec![quote!(#ffi_name)]);
            (quote!(), interface_impl, drop_impl)
        },
        _ => {
            let fields_count = fields.unnamed.len();
            let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(fields_count);
            let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(fields_count);
            let mut struct_fields = Vec::<TokenStream2>::with_capacity(fields_count);
            let mut destroy_fields = Vec::<TokenStream2>::with_capacity(fields_count);
            fields.unnamed.iter().enumerate().for_each(|(index, Field { ty: field_type, .. })| {
                let field_name = usize_to_tokenstream(index);
                let field_path_to = obj_field_name(quote!(#field_name));
                let field_path_from = ffi_deref_field_name(quote!(#field_name));

                let (conversion_to, conversion_from, destroy_field) = match field_type {
                    Type::Ptr(type_ptr) => (
                        to_ptr(field_path_to, type_ptr),
                        from_ptr(field_path_from.clone(), type_ptr),
                        destroy_ptr(field_path_from, type_ptr)
                    ),
                    Type::Path(TypePath { path, .. }) => (
                        to_path(field_path_to, path, None),
                        from_path(field_path_from.clone(), path, None),
                        destroy_path(field_path_from, path, None),
                    ),
                    Type::Reference(type_reference) => (
                        to_reference(field_path_to, type_reference),
                        from_reference(field_path_from.clone(), type_reference),
                        destroy_reference(field_path_from.clone(), type_reference)
                    ),
                    Type::Array(type_array) => (
                        to_array(field_path_to, type_array),
                        from_array(field_path_from.clone(), type_array),
                        destroy_array(field_path_from.clone(), type_array)
                    ),
                    _ => panic!("from_unnamed_struct: Unknown field {:?} {:?}", index, field_type),
                };
                conversions_to_ffi.push(conversion_to);
                conversions_from_ffi.push(conversion_from);
                struct_fields.push(extract_struct_field(field_type));
                destroy_fields.push(destroy_field);
            });
            let interface_impl = impl_interface(
                quote!(#ffi_name),
                quote!(#target_name),
                quote!(let ffi_ref = &*ffi; #target_name(#(#conversions_from_ffi,)*)),
                package_boxed_expression(quote!(#ffi_name(#(#conversions_to_ffi,)*))),
                package_unboxed_root()
            );

            let drop_code = match destroy_fields.len() {
                0 => quote!(),
                _ => {
                    quote! {
                        let ffi_ref = self;
                        #(#destroy_fields;)*
                    }
                }
            };
            let drop_impl = impl_drop(quote!(#ffi_name), drop_code);
            let ffi_struct = create_unnamed_struct(quote!(#ffi_name), struct_fields);
            (ffi_struct, interface_impl, drop_impl)
        }
    };
    let expanded = quote! {
        #input
        #ffi_struct
        #interface_impl
        #drop_impl
    };
    println!("{}", expanded);
    return expanded.into();


}

fn from_named_struct(fields: &FieldsNamed, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let ffi_name = input.ident.clone();
    let fields_count = fields.named.len();
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(fields_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(fields_count);
    let mut struct_fields = Vec::<TokenStream2>::with_capacity(fields_count);
    let mut destroy_fields = Vec::<TokenStream2>::with_capacity(fields_count);
    fields.named.iter().for_each(|Field { ident, ty: field_type, .. }| {
        let field_name = &ident.clone().unwrap().to_token_stream();
        let field_path_to = obj_field_name(field_name.clone());
        let field_path_from = ffi_deref_field_name(field_name.clone());

        let (conversion_to, conversion_from, destroy_field) = match field_type {
            Type::Ptr(type_ptr) => (
                to_ptr(field_path_to, type_ptr),
                from_ptr(field_path_from.clone(), type_ptr),
                destroy_ptr(field_path_from, type_ptr)
            ),
            Type::Path(TypePath { path, .. }) => (
                to_path(field_path_to, path, None),
                from_path(field_path_from.clone(), path, None),
                destroy_path(field_path_from, path, None),
            ),
            Type::Reference(type_reference) => (
                to_reference(field_path_to, type_reference),
                from_reference(field_path_from.clone(), type_reference),
                destroy_reference(field_path_from.clone(), type_reference)
            ),
            _ => panic!("from_named_struct: Unknown field {:?} {:?}", field_name, field_type),
        };
        conversions_to_ffi.push(define_field(field_name.clone(), conversion_to));
        conversions_from_ffi.push(define_field(field_name.clone(),conversion_from));
        struct_fields.push(define_pub_field(field_name.clone(), extract_struct_field(field_type)));
        destroy_fields.push(destroy_field);
    });
    let ffi_name = ffi_struct_name(&ffi_name);
    let ffi_struct = create_struct(quote!(#ffi_name), struct_fields);
    let drop_code = match destroy_fields.len() {
        0 => quote!(),
        _ => {
            quote! {
                let ffi_ref = self;
                #(#destroy_fields;)*
            }
        }
    };
    let interface_impl = impl_interface(
        quote!(#ffi_name),
        quote!(#target_name),
        quote!(let ffi_ref = &*ffi; #target_name { #(#conversions_from_ffi,)* }),
        package_boxed_expression(quote!(#ffi_name { #(#conversions_to_ffi,)* })),
        package_unboxed_root()
    );
    let drop_impl = impl_drop(quote!(#ffi_name), drop_code);

    let expanded = quote! {
        #input
        #ffi_struct
        #interface_impl
        #drop_impl
    };
    println!("{}", expanded);
    TokenStream::from(expanded)
}


fn from_enum(data_enum: &DataEnum, target_name: Ident, input: &DeriveInput) -> TokenStream {
    let variants = &data_enum.variants;
    let variants_count = variants.len();
    let ffi_name = ffi_struct_name(&target_name);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, ..}| {
        let target_variant_path = quote!(#target_name::#variant_name);
        let ffi_variant_path = quote!(#ffi_name::#variant_name);
        let (variant_to_lvalue,
            variant_to_rvalue,
            variant_from_lvalue,
            variant_from_rvalue,
            destroy_command) = match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let mut variant_fields = vec![];
                let mut destroy_fields = vec![];
                let mut converted_fields_to = vec![];
                let mut converted_fields_from = vec![];
                unnamed.iter().enumerate().for_each(|(index, Field { ty, .. })| {
                    let field_indexed = format_ident!("o_{}", index);
                    let (converted_field_to, converted_field_from, destroy_field) = match ty {
                        Type::Path(TypePath { path, .. }) => match conversion_type_for_path(path) {
                            ConversionType::Simple => (
                                quote!(#field_indexed),
                                quote!(*#field_indexed),
                                quote!({})),
                            ConversionType::Complex => (
                                ffi_to_conversion(quote!(#field_indexed)),
                                ffi_from_conversion(quote!(*#field_indexed)),
                                {
                                    let expr = package_unbox_any_expression(quote!(#field_indexed));
                                    quote!(let #field_indexed = #expr;)
                                }
                            ),
                            _ => unimplemented!("Enum with Map/Vec as associated value not supported yet")
                        },
                        _ => unimplemented!("Unsupported field type in enum variant")
                    };
                    variant_fields.push(quote!(#field_indexed));
                    converted_fields_to.push(converted_field_to);
                    converted_fields_from.push(converted_field_from);
                    destroy_fields.push(destroy_field);
                });
                (quote!(#target_variant_path(#(#variant_fields,)*)),
                 quote!(#ffi_variant_path(#(#converted_fields_to,)*)),
                 quote!(#ffi_variant_path(#(#variant_fields,)*)),
                 quote!(#target_variant_path(#(#converted_fields_from,)*)),
                 quote!({#(#destroy_fields)*})
                )
            },
            Fields::Unit => (
                quote!(#target_variant_path),
                quote!(#ffi_variant_path),
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                quote!({})
            ),
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut variant_fields = vec![];
                let mut destroy_fields = vec![];
                let mut converted_fields_to = vec![];
                let mut converted_fields_from = vec![];
                named.iter().for_each(|Field { ident, ty: field_type, .. }| {
                    let field_name = &ident.clone().unwrap().to_token_stream();

                    let (converted_field_to, converted_field_from, destroy_field) = match field_type {
                        Type::Ptr(type_ptr) => (
                            to_ptr(quote!(#field_name), type_ptr),
                            from_ptr(quote!(*#field_name), type_ptr),
                            destroy_ptr(field_name.clone(), type_ptr)
                        ),
                        Type::Path(TypePath { path, .. }) => (
                            to_path(quote!(#field_name), path, None),
                            from_path(quote!(*#field_name), path, None),
                            destroy_path(field_name.clone(), path, None),
                        ),
                        Type::Reference(type_reference) => (
                            to_reference(quote!(#field_name), type_reference),
                            from_reference(quote!(*#field_name), type_reference),
                            destroy_reference(field_name.clone(), type_reference)
                        ),
                        _ => panic!("from_named_struct: Unknown field {:?} {:?}", field_name, field_type),
                    };

                    variant_fields.push(field_name.clone());
                    converted_fields_to.push(define_field(field_name.clone(), converted_field_to));
                    converted_fields_from.push(define_field(field_name.clone(),converted_field_from));
                    destroy_fields.push(destroy_field);
                });
                (quote!(#target_variant_path { #(#variant_fields,)* }),
                 quote!(#ffi_variant_path { #(#converted_fields_to,)* }),
                 quote!(#ffi_variant_path { #(#variant_fields,)* }),
                 quote!(#target_variant_path { #(#converted_fields_from,)* }),
                 quote!({#(#destroy_fields)*})
                )
            },
        };
        let variant_field = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => quote!(#variant_name = #lit),
            None => match fields {
                Fields::Named(ref fields) => {
                    let enum_fields = fields.named.iter().map(|Field { ident, ty, .. }| define_field(quote!(#ident), extract_struct_field(ty))).collect::<Vec<_>>();
                    quote!(#variant_name { #(#enum_fields),* })
                },
                Fields::Unnamed(ref fields) => {
                    let enum_fields = fields.unnamed.iter().map(|Field { ty, .. }| extract_struct_field(ty)).collect::<Vec<_>>();
                    quote!(#variant_name(#(#enum_fields),*))
                },
                Fields::Unit => quote!(#variant_name),
            },
            _ => panic!("Error variant discriminant")
        };

        variants_fields.push(variant_field);
        conversions_to_ffi.push(define_lambda(variant_to_lvalue, variant_to_rvalue));
        conversions_from_ffi.push(define_lambda(variant_from_lvalue.clone(), variant_from_rvalue));
        destroy_fields.push(define_lambda(variant_from_lvalue, destroy_command));
    });
    let obj = obj();

    let drop_code = match destroy_fields.len() {
        0 => quote!(),
        _ => {
            quote! {
                match self {
                    #(#destroy_fields,)*
                }
            }
        }
    };

    let converted = quote! {
        #[repr(C)]
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
        pub enum #ffi_name {
            #(#variants_fields,)*
        }
    };
    let interface_impl = impl_interface(
        quote!(#ffi_name),
        quote!(#target_name),
        quote!(let ffi_ref = &*ffi; match ffi_ref { #(#conversions_from_ffi),* }),
        package_boxed_expression(quote!(match #obj { #(#conversions_to_ffi),* })),
        package_unboxed_root()
    );
    let drop_impl = impl_drop(quote!(#ffi_name), drop_code);

    let expanded = quote! {
        #input
        #converted
        #interface_impl
        #drop_impl
    };
    println!("{}", expanded);
    expanded.into()
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
                Fields::Unit => panic!("Fields::Unit is not supported yet"),
            }
        },
        Data::Enum(ref data_enum) => from_enum(data_enum, target_name, &input),
        Data::Union(ref _data_union) => panic!("Union is not supported yet")
    }
}

#[proc_macro_attribute]
pub fn impl_ffi_ty_conv(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let (target_name, alias_to) = match &input {
        Item::Type(ItemType { ident, ty, .. }) => (ident, ty),
        _ => panic!("Expected a type alias"),
    };
    let ffi_name = format_ident!("{}FFI", target_name);

    let ffi_struct_name = ffi_name.to_token_stream();

    let obj = obj();
    let alias_converted = extract_struct_field(alias_to);
    let (ffi_name, ffi_from_conversion, ffi_to_conversion, destroy_code, drop_code) = match &*alias_to.clone() {
        Type::Path(TypePath { path, .. }) => match conversion_type_for_path(path) {
            ConversionType::Simple => (
                ffi_struct_name.clone(),
                quote!(let ffi_ref = &*ffi; ffi_ref.0),
                package_boxed_expression(quote!(#ffi_struct_name(#obj))),
                package_unboxed_root(),
                None,
            ),
            ConversionType::Complex => (
                ffi_struct_name.clone(),
                quote!(let ffi_ref = &*ffi; ffi_ref.0),
                package_boxed_expression(quote!(#ffi_struct_name(#obj))),
                package_unboxed_root(),
                None
            ),
            ConversionType::Vec => {

                (
                    ffi_struct_name.clone(),
                    from_vec(path, quote!((&*ffi).0)),
                    {
                        let conversion = to_vec_conversion(obj.clone(), &path.segments.last().unwrap().arguments);
                        package_boxed_expression(quote!(#ffi_struct_name(#conversion)))
                    },
                    package_unboxed_root(),
                    Some({
                        let field_path = quote!(self.0);
                        let unboxed = package_unbox_any_expression(field_path);
                        quote!(#unboxed;)
                    })
                )
            },
            _ => unimplemented!("from_type_alias: not supported {:?}", &alias_to)
        },
        Type::Array(_type_array) => (
            ffi_struct_name.clone(),
            quote!(let ffi_ref = &*ffi; *ffi_ref.0),
            {
                let inner_type = package_boxed_expression(obj);
                package_boxed_expression(quote!(#ffi_struct_name(#inner_type)))
            },
            package_unboxed_root(),
            Some({
                let unboxed = package_unbox_any_expression(quote!(self.0));
                quote!(#unboxed;)
            })
        ),
        _ => unimplemented!("from_type_alias: not supported {:?}", &alias_to)
    };
    let interface_impl = impl_interface(ffi_name.clone(), quote!(#target_name), ffi_from_conversion, ffi_to_conversion, destroy_code);
    let drop_impl = drop_code.map_or(quote!(), |drop_code| impl_drop(ffi_name.clone(), drop_code));

    let expanded = quote! {
        #input
        #[repr(C)]
        #[derive(Clone, Debug)]
        pub struct #ffi_name(#alias_converted);
        #interface_impl
        #drop_impl
    };

    println!("{}", expanded);
    expanded.into()
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


fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}
