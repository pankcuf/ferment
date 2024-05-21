use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;
use syn::token::{Add, Pub};
use syn::{AngleBracketedGenericArguments, Attribute, Field, Fields, FieldsNamed, GenericArgument, ItemStruct, parse_quote, Path, PathArguments, PathSegment, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTuple, Visibility, VisPublic};
use syn::__private::TokenStream2;
use crate::composer::ParentComposer;
use crate::context::ScopeContext;
use crate::conversion::ItemConversion;
use crate::conversion::macro_conversion::merge_attributes;
use crate::ext::{Accessory, DictionaryType, Mangle, ToPath};
use crate::formatter::format_unique_attrs;
use crate::helper::{path_arguments_to_types};
use crate::naming::{DictionaryFieldName, Name};

#[derive(Clone, PartialEq, Eq)]
pub enum GenericTypeConversion {
    Map(Type),
    IndexMap(Type),
    SerdeJsonMap(Type),
    Vec(Type),
    BTreeSet(Type),
    HashSet(Type),
    Result(Type),
    Box(Type),
    AnyOther(Type),
    Array(Type),
    Slice(Type),
    Tuple(Type),
    Optional(Type),
    TraitBounds(Punctuated<TypeParamBound, Add>)
}
impl Debug for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericTypeConversion::{}({})", match self {
            GenericTypeConversion::Map(_) => "Map",
            GenericTypeConversion::IndexMap(_) => "IndexMap",
            GenericTypeConversion::SerdeJsonMap(_) => "SerdeJsonMap",
            GenericTypeConversion::Vec(_) => "Vec",
            GenericTypeConversion::BTreeSet(_) => "BTreeSet",
            GenericTypeConversion::HashSet(_) => "HashSet",
            GenericTypeConversion::Result(_) => "Result",
            GenericTypeConversion::Box(_) => "Box",
            GenericTypeConversion::AnyOther(_) => "AnyOther",
            GenericTypeConversion::Array(_) => "Array",
            GenericTypeConversion::Slice(_) => "Slice",
            GenericTypeConversion::Tuple(_) => "Tuple",
            GenericTypeConversion::TraitBounds(_) => "TraitBounds",
            GenericTypeConversion::Optional(_) => "Optional"
        }, self.to_token_stream()))
    }
}
impl Display for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for GenericTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Optional(ty) |
            GenericTypeConversion::Tuple(ty) => ty.to_tokens(tokens),
            GenericTypeConversion::TraitBounds(bounds) => bounds.to_tokens(tokens),
        }
    }
}
fn item_struct<I: IntoIterator<Item = Field>>(ident: Ident, attrs: Vec<Attribute>, fields_iterator: I) -> ItemStruct {
    ItemStruct {
        attrs,
        ident,
        fields: Fields::Named(FieldsNamed { brace_token: Default::default(), named: Punctuated::from_iter(fields_iterator) }),
        vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
        struct_token: Default::default(),
        generics: Default::default(),
        semi_token: None,
    }
}
fn field(ident: Ident, ty: Type) -> Field {
    Field { ident: Some(ident), ty, attrs: Default::default(), colon_token: None, vis: Visibility::Public(VisPublic { pub_token: Pub::default() }) }
}

impl GenericTypeConversion {
    pub fn ty(&self) -> Option<&Type> {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Optional(ty) |
            GenericTypeConversion::Tuple(ty) => Some(ty),
            GenericTypeConversion::TraitBounds(_) => {
                // TODO: Make mixin here
                None
            }
        }
    }
    pub fn to_ffi_type(&self) -> Type {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::AnyOther(ty) =>
                single_generic_ffi_type(ty),
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)
            }
            GenericTypeConversion::Tuple(tuple) => {
                let tuple: TypeTuple = parse_quote!(#tuple);
                match tuple.elems.len() {
                    0 => single_generic_ffi_type(tuple.elems.first().unwrap()),
                    _ => {
                        let ffi_name = tuple.mangle_ident_default();
                        parse_quote!(crate::fermented::generics::#ffi_name)
                    }
                }
            }
            GenericTypeConversion::Optional(ty) => {
                match ty {
                    Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                        Some(last_segment) => {
                            match &last_segment.arguments {
                                PathArguments::None => panic!("Empty optional arguments as generic argument (PathArguments::None): {}", ty.to_token_stream()),
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                                    match args.first() {
                                        Some(generic_argument) => match generic_argument {
                                            GenericArgument::Type(ty) => single_generic_ffi_type(ty),
                                            _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Non-Type Generic): {}", ty.to_token_stream()),
                                        },
                                        _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", ty.to_token_stream()),
                                    }
                                }
                                PathArguments::Parenthesized(_) => panic!("TODO: Non-supported optional type as generic argument (PathArguments::Parenthesized): {}", ty.to_token_stream()),
                            }
                        },
                        None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                    },
                    Type::Array(TypeArray { elem, .. }) => single_generic_ffi_type(elem),
                    _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
                }
            }
            GenericTypeConversion::TraitBounds(ty) =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", ty.to_token_stream()),
        }
    }

    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        let scope = source.scope.clone();
        println!("GenericTypeConversion::expand.1: {} ---- {}\n\t {}", self, scope, format_unique_attrs(attrs));
        let attrs = merge_attributes(attrs);
        let attrs = (!attrs.is_empty()).then(|| vec![parse_quote!(#[cfg(#attrs)])]).unwrap_or_default();
        println!("GenericTypeConversion::expand.2: {} ----\n\t {:?}", self, attrs);

        match self {
            GenericTypeConversion::Result(ty) => {
                let path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_types(arguments);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        [
                            field(DictionaryFieldName::Ok.to_ident(), path_conversions[0].clone()),
                            field(DictionaryFieldName::Error.to_ident(), path_conversions[1].clone())]),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()
            },
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) => {
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_types(arguments);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        [
                            field(DictionaryFieldName::Keys.to_ident(), path_conversions[0].clone()),
                            field(DictionaryFieldName::Values.to_ident(), path_conversions[1].clone()),
                            field(DictionaryFieldName::Count.to_ident(), parse_quote!(usize))
                        ]),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()

            },
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Vec(ty) => {
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_types(arguments);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        [
                            field(DictionaryFieldName::Values.to_ident(), path_conversions[0].clone()),
                            field(DictionaryFieldName::Count.to_ident(), parse_quote!(usize))
                        ]),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()
            },
            GenericTypeConversion::Array(ty) => {
                let ty_array: TypeArray = parse_quote!(#ty);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        [
                            field(DictionaryFieldName::Values.to_ident(), ty_array.elem.joined_mut()),
                            field(DictionaryFieldName::Count.to_ident(), parse_quote!(usize))
                        ]),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()
            }
            GenericTypeConversion::Slice(ty) => {
                let ty_slice: TypeSlice = parse_quote!(#ty);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        [
                            field(DictionaryFieldName::Values.to_ident(), ty_slice.elem.joined_mut()),
                            field(DictionaryFieldName::Count.to_ident(), parse_quote!(usize))
                        ]),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()
            },
            GenericTypeConversion::Tuple(ty) => {
                let type_tuple: TypeTuple = parse_quote!(#ty);
                ItemConversion::Struct(
                    item_struct(
                        ty.mangle_ident_default(),
                        attrs,
                        type_tuple.elems.iter()
                            .enumerate()
                            .map(|(index, ty)|
                                field(Name::UnnamedArg(index).mangle_ident_default(), ty.clone()))),
                    scope)
                    .make_expansion(scope_context)
                    .to_token_stream()
            }
            GenericTypeConversion::Optional(_) |
            GenericTypeConversion::Box(_) |
            GenericTypeConversion::AnyOther(_) |
            GenericTypeConversion::TraitBounds(_) => Default::default(),
        }
    }
}
pub fn single_generic_ffi_type(ty: &Type) -> Type {
    let path: Path = parse_quote!(#ty);
    let first_segment = path.segments.first().unwrap();
    let mut cloned_segments = path.segments.clone();
    let first_ident = &first_segment.ident;
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        parse_quote!(#last_ident)
    } else if last_ident.is_any_string() {
        parse_quote!(std::os::raw::c_char)
    } else if last_ident.is_special_generic() || (last_ident.is_result() && path.segments.len() == 1) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
        let ffi_name = path.mangle_ident_default();
        parse_quote!(crate::fermented::generics::#ffi_name)
    } else {
        let new_segments = cloned_segments
            .into_iter()
            .map(|segment| quote_spanned! { segment.span() => #segment })
            .collect::<Vec<_>>();
        parse_quote!(#(#new_segments)::*)
    }
}
