use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{__private::TokenStream2, parse_quote, Path, spanned::Spanned, Type, TypePath};
use crate::context::Context;
use crate::generic_path_conversion::GenericPathConversion;
use crate::helper::{ffi_mangled_ident, path_arguments_to_paths};
use crate::item_conversion::ItemContext;
use crate::path_conversion::PathConversion;
use crate::type_conversion::TypeConversion;

pub fn ffi_path_converted(path: &Path) -> Option<Type> {
    let segments = &path.segments;
    let first_segment = segments.first().unwrap();
    let first_ident = &first_segment.ident;
    let last_segment = segments.iter().last().unwrap();
    let last_ident = &last_segment.ident;
    match last_ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
        | "isize" | "usize" | "bool" => None,
        "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
        "Vec" | "BTreeMap" | "HashMap" | "Result" => {
            let ffi_name = PathConversion::from(path).into_mangled_generic_ident();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        },
        "Option" => path_arguments_to_paths(&last_segment.arguments)
            .first()
            .cloned()
            .and_then(ffi_path_converted),
        "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
        "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
        _ => {
            let segments: Vec<_> = match first_ident.to_string().as_str() {
                "crate" => segments.iter().take(segments.len() - 1).skip(1).collect(),
                _ => segments.iter().take(segments.len() - 1).collect()
            };
            let ffi_name = if segments.is_empty() {
                quote!(#last_ident)
            } else {
                quote!(#(#segments)::*::#last_ident)
            };
            Some(parse_quote!(crate::fermented::types::#ffi_name))
        }
    }
}

pub fn ffi_external_path_converted(path: &Path, context: &Context) -> Option<Type> {
    let segments = &path.segments;
    let first_segment = segments.first().unwrap();
    let first_ident = &first_segment.ident;

    let last_segment = segments.iter().last().unwrap();
    let last_ident = &last_segment.ident;

    match last_ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
        "isize" | "usize" | "bool" => None,
        "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
        "Vec" | "BTreeMap" | "HashMap" | "Result" => {
            let ffi_name = PathConversion::from(path)
                .into_mangled_generic_ident();
            Some(parse_quote!(crate::fermented::generics::#ffi_name))
        },
        "Option" => path_arguments_to_paths(&last_segment.arguments)
            .first()
            .cloned()
            .and_then(|ty| ffi_external_path_converted(ty, context)),
        "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
        "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
        _ => match first_ident.to_string().as_str() {
            "crate" => {
                let segments: Vec<_> = segments.iter().skip(1).take(segments.len() - 2).collect();
                let ffi_name = if segments.is_empty() {
                    quote!(#last_ident)
                } else {
                    quote!(#(#segments)::*::#last_ident)
                };
                Some(parse_quote!(crate::fermented::types::#ffi_name))
            },
            _ if context.contains_fermented_crate(first_ident) => {
                let segments: Vec<_> = segments.iter().skip(1).take(segments.len() - 2).collect();
                let ffi_name = if segments.is_empty() {
                    quote!(#last_ident)
                } else {
                    quote!(#(#segments)::*::#last_ident)
                };
                Some(parse_quote!(#first_ident::fermented::types::#ffi_name))
            },
            _ => {
                let segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                Some(if segments.is_empty() { parse_quote!(#last_ident) } else { parse_quote!(#(#segments)::*::#last_ident) })
            }
        }
    }
}

pub fn ffi_dictionary_field_type(path: &Path, context: &ItemContext) -> TokenStream2 {
    match path.segments.last().unwrap().ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
        "isize" | "usize" | "bool" =>
            quote!(#path),
        "Option" =>
            ffi_dictionary_field_type(path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap(), context),
        "Vec" | "BTreeMap" | "HashMap" | "Result" =>
            context.scope_types.iter()
                .find_map(|(TypeConversion { 0: other}, full_type)|
                    path.to_token_stream().to_string().eq(other.to_token_stream().to_string().as_str())
                        .then_some(full_type))
                .map_or(quote!(*mut #path), |full_type| {
                    let full_ty = ffi_mangled_ident(full_type);
                    quote!(*mut #full_ty)
                }),
        "OpaqueContext" =>
            quote!(ferment_interfaces::OpaqueContext_FFI),
        "OpaqueContextMut" =>
            quote!(ferment_interfaces::OpaqueContextMut_FFI),
        _ =>
            quote!(*mut #path),
    }
}

pub fn convert_to_ffi_path(path: &Path) -> Type {
    let mut cloned_segments = path.segments.clone();
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    match last_ident.to_string().as_str() {
        // simple primitive type
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
        "isize" | "usize" | "bool" => parse_quote!(#last_ident),
        // complex special type
        "str" | "String" => parse_quote!(std::os::raw::c_char),
        "Vec" | "BTreeMap" | "HashMap" | "Result" => {
            let ffi_name = format_ident!("{}", PathConversion::mangled_inner_generic_ident_string(path));
            parse_quote!(crate::fermented::generics::#ffi_name)
        }
        _ => {
            let new_segments = cloned_segments
                .into_iter()
                .map(|segment| quote_spanned! { segment.span() => #segment })
                .collect::<Vec<_>>();
            parse_quote!(#(#new_segments)::*)
        }
    }
}

pub fn path_conversion_from_path(path: &Path) -> PathConversion {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
        | "isize" | "usize" | "bool" => PathConversion::Primitive(path.clone()),
        "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
        "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
        "Result" => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
        _ => PathConversion::Complex(path.clone()),
    }

}

pub fn ffi_external_type_converted(ty: &Type, context: &Context) -> Option<Type> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            ffi_external_path_converted(path, context),
        _ => None
    }
}

pub fn ffi_path_converted_or_same(path: &Path) -> Type {
    ffi_path_converted(path)
        .unwrap_or(parse_quote!(#path))
}