use quote::{format_ident, quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, Type, TypeArray, TypePath, TypePtr, TypeReference};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::interface::{DEREF_FIELD_PATH, destroy_conversion, ffi_from_conversion, ffi_from_opt_conversion, ffi_to_conversion, ffi_to_opt_conversion, FFI_TYPE_PATH_PRESENTER, FROM_OFFSET_MAP_PRESENTER, iter_map_collect, LAMBDA_CONVERSION_PRESENTER, MATCH_FIELDS_PRESENTER, OBJ_FIELD_NAME, package_boxed_expression, package_boxed_vec_expression, package_unbox_any_expression, package_unbox_any_expression_terminated, unwrap_or};
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
        [Type::Path(TypePath { path, .. })] =>
            vec![path],
        [Type::Path(TypePath { path: path_keys, .. }), Type::Path(TypePath { path: path_values, .. })] =>
            vec![path_keys, path_values],
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
    match &path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)[..] {
        [PathConversion::Primitive(..) | PathConversion::Complex(..)] =>
            package_unbox_any_expression_terminated(field_path),
        [PathConversion::Generic(GenericPathConversion::Vec(path))] =>
            destroy_vec(path, field_path),
        _ => panic!("destroy_vec: Bad arguments {} {}", field_path, quote!(#path))
    }
}

#[allow(unused)]
pub fn box_vec(field_path: TokenStream2, values_conversion: TokenStream2) -> TokenStream2 {
    quote!(ferment_interfaces::FFIConversion::ffi_to(vec))
}

pub fn from_simple_vec_conversion(field_path: TokenStream2, _field_type: TokenStream2) -> TokenStream2 {
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

pub fn from_vec(path: &Path, field_path: TokenStream2) -> TokenStream2 {
    let arguments = &path.segments.last().unwrap().arguments;
    let conversion = match &path_arguments_to_path_conversions(arguments)[..] {
        [PathConversion::Primitive(path)] =>
            from_simple_vec_conversion(quote!(vec), path.segments.last().unwrap().ident.to_token_stream()),
        [PathConversion::Complex(_path)] =>
            quote!(ferment_interfaces::from_complex_vec(vec.values, vec.count)),
        [PathConversion::Generic(GenericPathConversion::Vec(..))] =>
            quote! {
                let count = vec.count;
                let values = vec.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            },
        _ => panic!("from_vec: Bad arguments {} {}", field_path, quote!(#arguments)),

    };
    quote!({ let vec = &*#field_path; #conversion })
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
                    let conversion = from_vec(path, field_path.clone());
                    quote!((!#field_path.is_null()).then_some(#conversion))
                }
                _ => ffi_from_opt_conversion(field_path),
            },
            _ => panic!("from_option: Bad arguments {} {}", field_path, quote!(#arguments)),
        },
        _ => panic!("from_option: Bad arguments {} {}", field_path, quote!(#arguments)),
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
        _ => panic!("from_array: unsupported {} {}", field_path, quote!(#type_array)),
    }
}

pub(crate) fn destroy_path(field_path: TokenStream2, path: &Path, _type_ptr: Option<&TypePtr>) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => quote!({}),
        "VarInt" => quote!({}),
        "Option" => match path_arguments_to_paths(&path.segments.last().unwrap().arguments)[..] {
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
                _ => panic!("from_option: Unknown field {} {}", field_path, quote!(#path)),
            },
            _ => panic!("from_option: Bad arguments {} {}", field_path, quote!(#path)),
        },
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
        "Vec" => from_vec(path, field_path),
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
                let ffi_from_conversion =
                    ffi_from_conversion(FROM_OFFSET_MAP_PRESENTER(quote!(*values)));
                quote!((0..count).map(|i| #ffi_from_conversion).collect())
            }
            _ => ffi_from_conversion(field_path),
        },
        Type::Path(type_path) => {
            let field_type = type_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_token_stream();
            quote!(std::slice::from_raw_parts(values as *const #field_type, count).to_vec())
        }
        _ => ffi_from_conversion(field_path),
    };
    println!("from_ptr.2: {}", result);
    result
}

pub(crate) fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
    }
}

pub(crate) fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path, None),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
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
                "Vec" => MATCH_FIELDS_PRESENTER((
                    field_path,
                    vec![
                        LAMBDA_CONVERSION_PRESENTER(quote!(Some(vec)), quote!(ferment_interfaces::FFIConversion::ffi_to(vec))),
                        LAMBDA_CONVERSION_PRESENTER(quote!(None), quote!(std::ptr::null_mut())),
                    ],
                )),
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
        _ => panic!("to_reference: Unknown type {}", quote!(#type_reference)),
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

pub fn ffi_vtable_name(trait_name: &Ident) -> Ident {
    format_ident!("{}_VTable", trait_name)
}

pub fn ffi_trait_obj_name(trait_name: &Ident) -> Ident {
    format_ident!("{}_TraitObject", trait_name)
}

pub fn ffi_destructor_name(item_name: &Ident) -> Ident {
    format_ident!("{}_ffi_destroy", item_name)
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