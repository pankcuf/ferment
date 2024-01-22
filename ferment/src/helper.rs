use quote::{format_ident, quote, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject};
use syn::__private::{Span, TokenStream2};
use crate::context::ScopeContext;
use crate::conversion::PathConversion;
use crate::interface::{DEREF_FIELD_PATH, destroy_conversion, ffi_from_conversion, ffi_from_opt_conversion, ffi_to_conversion, ffi_to_opt_conversion, FROM_OFFSET_MAP_PRESENTER, iter_map_collect, OBJ_FIELD_NAME, package_boxed_expression, package_boxed_vec_expression, package_unbox_any_expression_terminated};
use crate::presentation::context::{OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;

// pub fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
//     match arguments {
//         PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match map_args(args)[..] {
//             [GenericArgument::Type(value_type)] =>
//                 vec![value_type],
//             [GenericArgument::Type(key_type), GenericArgument::Type(value_type, ..)] =>
//                 vec![key_type, value_type],
//             _ => unimplemented!("path_arguments_to_types: unexpected args: {}", quote!(#args)),
//         },
//         _ => unimplemented!("path_arguments_to_types: arguments: {} not supported", quote!(#arguments)),
//     }
// }
pub fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None
            }).collect(),
        _ => Vec::new(),
    }
}

fn path_from_type(ty: &Type) -> Option<&Path> {
    match ty {
        Type::Array(TypeArray { elem, len, .. }) => path_from_type(elem),
        Type::Path(TypePath { path, .. }) => Some(path),
        _ => None,
    }
}

pub fn path_arguments_to_paths(arguments: &PathArguments) -> Vec<&Path> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => path_from_type(ty),
                // GenericArgument::Type(Type::Reference(TypeReference { mutability, elem })) => Some(path),
                _ => None
            }).collect(),
        _ => Vec::new(),
    }

    // match path_arguments_to_types(arguments)[..] {
    //     [Type::TraitObject(obj)] =>
    //         from_type_trait_object(obj),
    //     [Type::Path(TypePath { path, .. })] =>
    //         vec![path],
    //     [Type::Path(TypePath { path: path_keys, .. }), Type::Path(TypePath { path: path_values, .. })] =>
    //         vec![path_keys, path_values],
    //     _ =>
    //         unimplemented!("path_arguments_to_paths: unexpected args: {}", quote!(#arguments)),
    // }
}

fn from_type_trait_object(obj: &TypeTraitObject) -> Vec<&Path> {
    obj.bounds.iter().filter_map(|f| match f {
        TypeParamBound::Trait(TraitBound { path, .. }) =>
            Some(path),
        TypeParamBound::Lifetime(_lifetime) =>
            None
    }).collect()
}


pub fn path_arguments_to_path_conversions(arguments: &PathArguments) -> Vec<PathConversion> {
    path_arguments_to_paths(arguments)
        .into_iter()
        .map(PathConversion::from)
        .collect()
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

pub(crate) fn destroy_path(field_path: TokenStream2, path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => quote!(),
        "VarInt" => quote!(),
        "Option" => match path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => quote!(),
            _ => {
                let conversion = package_unbox_any_expression_terminated(field_path.clone());
                quote!(if !#field_path.is_null() { #conversion })
            }
        },
        "String" =>
            destroy_conversion(field_path, parse_quote!(std::os::raw::c_char), quote!(#path)),
        "str" =>
            destroy_conversion(field_path, parse_quote!(std::os::raw::c_char), quote!(&#path)),
        _ =>
            package_unbox_any_expression_terminated(field_path),
    }
}

pub(crate) fn from_path(field_path: TokenStream2, path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            // std convertible
            // TODO: what to use? 0 or ::MAX
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
            | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
            // TODO: mmm shit that's incorrect
            "bool" =>
                quote!((#field_path).then_some(#field_path)),
            _ =>
                ffi_from_opt_conversion(field_path),
        },
        _ => ffi_from_conversion(field_path),
    }
}

pub(crate) fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {}", quote!(#type_ptr)),
    }
}

pub(crate) fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
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
    }
}

pub(crate) fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
    }
}

pub(crate) fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
    }
}

// pub fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
//     args.iter().collect::<Vec<_>>()
// }

pub(crate) fn to_path(field_path: TokenStream2, path: &Path, context: &ScopeContext) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            // TODO: MAX/MIN? use optional primitive?
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" =>
                quote!(#field_path.unwrap_or(0)),
            "bool" =>
                quote!(#field_path.unwrap_or(false)),
            "Vec" =>
                OwnerIteratorPresentationContext::MatchFields(field_path, vec![
                    OwnedItemPresenterContext::Lambda(quote!(Some(vec)), ffi_to_conversion(quote!(vec))),
                    OwnedItemPresenterContext::Lambda(quote!(None), quote!(std::ptr::null_mut())),
                ]).present(context),
            _ => ffi_to_opt_conversion(field_path),
        },
        _ => ffi_to_conversion(field_path),
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(OBJ_FIELD_NAME(ident), quote!(|o| #expr)))
}

pub(crate) fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr, context: &ScopeContext) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Array(TypeArray { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, context),
            _ => panic!("to_pointer: Unknown type inside Type::Array {}", quote!(#type_ptr)),
        },
        Type::Ptr(TypePtr { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(quote!(*#field_path.add(i)), &type_path.path, context),
            Type::Array(type_arr) => to_vec_ptr(field_path, type_ptr, type_arr),
            _ => panic!("to_pointer: Unknown type inside Type::Ptr {}", quote!(#type_ptr)),
        },
        Type::Path(type_path) => to_path(field_path, &type_path.path, context),
        _ => panic!("to_pointer: Unknown type {}", quote!(#type_ptr)),
    }
}

pub(crate) fn to_reference(field_path: TokenStream2, type_reference: &TypeReference, context: &ScopeContext) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, context),
        _ => panic!("to_reference: Unknown type {}", quote!(#type_reference)),
    }
}

pub(crate) fn to_array(field_path: TokenStream2, type_array: &TypeArray, context: &ScopeContext) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => to_path(package_boxed_expression(field_path), &type_path.path, context),
        _ => panic!("to_array: Unknown type {}", quote!(#type_array)),
    }
}

pub fn ffi_vtable_name(trait_name: &Ident) -> Ident {
    format_ident!("{}_VTable", trait_name)
}

pub fn ffi_trait_obj_name(trait_name: &Ident) -> Ident {
    // format_ident!("{}_TraitObject", trait_name)
    format_ident!("{}", trait_name)
}

pub fn ffi_fn_name(fn_name: &Ident) -> Ident {
    format_ident!("ffi_{}", fn_name)
}

pub fn ffi_unnamed_arg_name(index: usize) -> Ident {
    format_ident!("o_{}", index)
}

pub fn ffi_constructor_name(item_name: &Ident) -> Ident {
    format_ident!("{}_ctor", item_name)
}
pub fn ffi_destructor_name(item_name: &Ident) -> Ident {
    format_ident!("{}_destroy", item_name)
}

pub fn ffi_mangled_ident(ty: &Type) -> Ident {
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
pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}
