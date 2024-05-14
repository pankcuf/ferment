use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Add, Brace, Comma};
use syn::{parse_quote, Path, PathSegment, spanned::Spanned, Type, TypeArray, TypeParamBound, TypeSlice, TypeTuple};
use syn::__private::TokenStream2;
use crate::composer::{ConstructorPresentableContext, Depunctuated};
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, TypeConversion};
use crate::ext::{Accessory, DictionaryType, FFIResolve, FFITypeResolve, Mangle, Resolve, ToPath, ToType};
use crate::helper::{path_arguments_to_type_conversions, usize_to_tokenstream};
use crate::interface::{create_struct, ffi_to_conversion};
use crate::naming::{DictionaryExpression, DictionaryFieldName, Name};
use crate::presentation::context::{BindingPresentableContext, FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::{DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, InterfacePresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::wrapped::Wrapped;

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: TokenStream2,
    pub from_conversion: TokenStream2,
    pub to_conversion: TokenStream2,
}

impl Debug for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl Display for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: TokenStream2, from_conversion: TokenStream2, to_conversion: TokenStream2) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}

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
    TraitBounds(Punctuated<TypeParamBound, Add>)
}
impl Debug for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericTypeConversion({})", self.to_token_stream()))
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
            GenericTypeConversion::Tuple(ty) => ty.to_tokens(tokens),
            GenericTypeConversion::TraitBounds(bounds) => bounds.to_tokens(tokens),
        }
    }
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
            GenericTypeConversion::TraitBounds(ty) =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", ty.to_token_stream()),
        }
    }
}

impl GenericTypeConversion {
    pub fn expand(&self, source: &ScopeContext) -> TokenStream2 {
        println!("GenericTypeConversion::expand: {}", self);
        // println!("\t{}\n\t{}", ffi_type.to_token_stream(), ffi_name.to_token_stream());

        match self {
            GenericTypeConversion::Result(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Error);

                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [TypeConversion::Primitive(ok), TypeConversion::Primitive(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [TypeConversion::Primitive(ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = error.to_custom_or_ffi_type(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [TypeConversion::Primitive(ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Primitive(error)] => {
                        let arg_0_ffi_type = ok.to_custom_or_ffi_type(source);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = ok.ffi_resolve_or_same(source);
                        let arg_1_ffi_type = error.ffi_resolve_or_same(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = ok.to_custom_or_ffi_type(source);
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Primitive(error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(source);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_primitive_enum(),
                                ffi_to_primitive_enum()),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(source);
                        let arg_1_ffi_type = error.to_custom_or_ffi_type(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(source);
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_option(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex()),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(arg_0_name, ok_type.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, error_type.joined_mut(),  Depunctuated::new()),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Result(quote!(#from_ok_conversion), quote!(#from_error_conversion)),
                                ToConversionPresentation::Result(quote!(#to_ok_conversion), quote!(#to_error_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([ok_destructor, error_destructor]),
                    source
                )
            },
            GenericTypeConversion::Map(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values())
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                                ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor, value_destructor]),
                    source
                )
            },
            GenericTypeConversion::IndexMap(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values())
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                                ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor, value_destructor]),
                    source
                )
            },
            GenericTypeConversion::SerdeJsonMap(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values())
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_primitive_vec(quote!(self.#arg_1_name)),
                                ffi_from_primitive(),
                                ffi_to_primitive_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                                ffi_from_complex(),
                                ffi_to_complex_keys()),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                ffi_destroy_complex_vec(quote!(self.#arg_1_name)),
                                ffi_from_complex(),
                                ffi_to_complex_values()),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                                ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor, value_destructor]),
                    source
                )
            },
            GenericTypeConversion::BTreeSet(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let arg_0_presentation = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_btree_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromPrimitiveBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_btree_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_btree_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    _ => {
                        return quote!();
                        // unimplemented!("Generic path arguments conversion error")
                    },
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { types: (ffi_as_type, target_type), decode, encode }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )
            },
            GenericTypeConversion::HashSet(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                println!("HashSet:::: {:?}", path_conversions);
                let arg_0_presentation = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_hash_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromPrimitiveHashSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_hash_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexHashSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_hash_set(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexHashSet(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    _ => {
                        return quote!();
                        // unimplemented!("Generic path arguments conversion error")
                    },
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { types: (ffi_as_type, target_type), decode, encode }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )
            },
            GenericTypeConversion::Vec(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let path: Path = ty.to_path();
                let PathSegment { arguments, .. } = path.segments.last().unwrap();
                let path_conversions = path_arguments_to_type_conversions(arguments);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let arg_0_presentation = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromPrimitiveVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { types: (ffi_as_type, target_type), decode, encode }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )
            },
            GenericTypeConversion::Array(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                // It's for sure Type::Array(TypeArray { elem, len: _, .. })
                // [u8 ; 32] (simple) or [HashID; 2] (complex/generic)
                // so we can simply parse it as array
                let type_array: TypeArray = parse_quote!(#ty);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let arg_0_presentation = match TypeConversion::from(&*type_array.elem) {
                    TypeConversion::Primitive(arg_0_target_path) => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromPrimitiveArray(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj.to_vec()) })).to_token_stream())
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexArray(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_vec(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexArray(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_0_presentation;
                // println!("from_value_conversion: {}", from_value_conversion);
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                FromConversionPresentation::Just(from_value_conversion),
                                ToConversionPresentation::Struct(to_value_conversion),
                                DestroyPresentation::Default,
                                None
                                // Some(parse_quote!(<'a>) )
                            )
                        }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )

            },
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_slice: TypeSlice = parse_quote!(#ty);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let elem_type = &type_slice.elem;
                // let target_type: Type = parse_quote!(&[#elem_type; N]);
                let target_type: Type = parse_quote!(Vec<#elem_type>);
                let arg_0_presentation = match TypeConversion::from(&*type_slice.elem) {
                    TypeConversion::Primitive(arg_0_target_path) => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_primitive_slice(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromPrimitiveVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            // DictionaryExpression::FromPrimitiveSlice(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_slice(quote!(self.#arg_0_name)),
                            // DictionaryExpression::FromComplexSlice(quote!(self.#arg_0_name), quote!(self.#count_name), arg_0_target_ty).to_token_stream(),
                            DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            ffi_destroy_complex_slice(quote!(self.#arg_0_name)),
                            DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)/*, parse_quote!(#arg_0_generic_path_conversion)*/).to_token_stream(),
                            DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                };
                // let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { types: (ffi_as_type, target_type), decode, encode }
                    ]),
                    Depunctuated::from_iter([value_destructor]),
                    source
                )
            },
            GenericTypeConversion::Tuple(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_tuple: TypeTuple = parse_quote!(#ty);
                let tuple_items = type_tuple.elems.iter()
                    .enumerate()
                    .map(|(index, ty)|
                        dictionary_generic_arg(
                            Name::UnnamedArg(index),
                            usize_to_tokenstream(index),
                            ty,
                            source))
                    .collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                compose_generic_presentation(
                    ffi_name,
                    Depunctuated::new(),
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, _))| FieldTypeConversion::Unnamed(Name::UnnamedArg(index), parse_quote!(#root_path), Depunctuated::new()))),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            types: (ffi_as_type, parse_quote!(#ty)),
                            conversions: (
                                FromConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.clone())).collect()),
                                ToConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.clone())).collect()),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.clone()))),
                    source
                )
            }
            GenericTypeConversion::Box(_) |
            GenericTypeConversion::AnyOther(_) |
            GenericTypeConversion::TraitBounds(_) => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}
fn compose_generic_presentation(
    ffi_name: Ident,
    attrs: Depunctuated<Expansion>,
    field_conversions: Depunctuated<FieldTypeConversion>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    // println!("compose_generic_presentation: {}", ffi_name);
    let ffi_as_path: Path = parse_quote!(#ffi_name);
    let ffi_as_type: Type = parse_quote!(#ffi_name);
    let fields = Punctuated::<_, Comma>::from_iter(field_conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), true)));
    let body = Wrapped::<_, Brace>::new(fields.present(&source));
    let object_presentation = create_struct(&ffi_as_path, attrs.clone(), body.to_token_stream());
    let drop_presentation = DropInterfacePresentation::Full { attrs, ty: ffi_as_type.clone(), body: drop_body.to_token_stream() };
    let bindings = compose_bindings(&ffi_as_type, field_conversions).present(&source);
    FFIObjectPresentation::Generic { object_presentation, interface_presentations, drop_presentation, bindings }
}

fn compose_bindings(ffi_type: &Type, conversions: Depunctuated<FieldTypeConversion>) -> Depunctuated<BindingPresentableContext> {
    Depunctuated::from_iter([
        BindingPresentableContext::Constructor(
            ConstructorPresentableContext::Default(ffi_type.clone(), quote! {}),
            conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), false)).collect(),
            IteratorPresentationContext::Curly(conversions.iter().map(|field| OwnedItemPresentableContext::DefaultField(field.clone())).collect())),
        BindingPresentableContext::Destructor(ffi_type.clone())
    ])
}

fn dictionary_generic_arg(name: Name, field_name: TokenStream2, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty = ty.resolve(source);
    match TypeConversion::from(&ty) {
        TypeConversion::Primitive(arg_ty) => {
            let from_conversion = FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_ty.clone(), Depunctuated::new())).present(source);
            let to_conversion = FieldTypePresentableContext::ObjFieldName(field_name).present(source);
            (arg_ty.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_ty,
                quote!(),
                from_conversion,
                quote!(#name: #to_conversion)
            )]))
        }
        TypeConversion::Complex(arg_type) => {
            let from_conversion = FieldTypePresentableContext::From(FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_type.clone(), Depunctuated::new())).into()).present(source);
            let to_conversion = FieldTypePresentableContext::To(FieldTypePresentableContext::ObjFieldName(field_name).into()).present(source);
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type,
                quote!(ferment_interfaces::unbox_any(self.#name);),
                from_conversion,
                quote!(#name: #to_conversion))]))
        }
        TypeConversion::Generic(root_path) => {
            // TODO: make sure it does work (actually it doesn't)
            let arg_type: Type = parse_quote!(#root_path);
            let from_conversion = FieldTypePresentableContext::From(FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_type.clone(), Depunctuated::new())).into()).present(source);
            let to_conversion = FieldTypePresentableContext::To(FieldTypePresentableContext::ObjFieldName(field_name).into()).present(source);
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type,
                quote!(ferment_interfaces::unbox_any(self.#name);),
                from_conversion,
                quote!(#name: #to_conversion))]))
            // println!("dictionary_generic_arg::Generic: {}", root_path.to_token_stream());
            // let path = ty.to_path();
            // let PathSegment { arguments, .. } = path.segments.last().unwrap();
            // let arg_type_conversions = path_arguments_to_type_conversions(arguments);
            // (root_path.to_ffi_type(), arg_type_conversions.iter()
            //     .map(|arg_path_conversion| {
            //         match arg_path_conversion {
            //             TypeConversion::Primitive(arg_type) => {
            //                 GenericArgPresentation::new(
            //                     parse_quote!(#arg_type),
            //                     quote!(),
            //                     ffi_from_primitive(),
            //                     ffi_to_primitive())
            //             }
            //             TypeConversion::Complex(arg_type) => {
            //                 GenericArgPresentation::new(
            //                     parse_quote!(#arg_type),
            //                     quote!(),
            //                     ffi_from_primitive(),
            //                     ffi_to_primitive())
            //             }
            //             TypeConversion::Generic(arg_type) => {
            //                 GenericArgPresentation::new(
            //                     parse_quote!(#arg_type),
            //                     quote!(),
            //                     ffi_from_primitive(),
            //                     ffi_to_primitive())
            //             }
            //         }
            //     })
            //     .collect::<Depunctuated<GenericArgPresentation>>())
        }
    }
}

fn ffi_from_primitive() -> TokenStream2 {
    quote!(|o| o)
}

fn ffi_from_complex() -> TokenStream2 {
    quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o))
}
fn ffi_from_primitive_enum() -> TokenStream2 {
    quote!(|o| *o)
}

fn ffi_to_complex() -> TokenStream2 {
    ffi_to_conversion(quote!(o))
}

fn ffi_to_primitive() -> TokenStream2 {
    quote!(o)
}

fn ffi_to_primitive_enum() -> TokenStream2 {
    quote!(o as *mut _)
}

fn ffi_to_primitive_keys() -> TokenStream2 {
    quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))
}
fn ffi_to_complex_keys() -> TokenStream2 {
    quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))
}
fn ffi_to_primitive_values() -> TokenStream2 {
    quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))
}
fn ffi_to_complex_values() -> TokenStream2 {
    quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))
}
// fn ffi_to_primitive_vec() -> TokenStream2 {
//     quote!(ferment_interfaces::boxed_vec(obj))
// }
// fn ffi_to_complex_vec() -> TokenStream2 {
//     quote!(ferment_interfaces::to_complex_vec(obj.into_iter()))
// }

fn ffi_destroy_primitive_vec(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_primitive_btree_set(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_primitive_hash_set(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_complex_vec(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_any_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_complex_btree_set(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_any_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_complex_hash_set(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_any_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_option(field_name: TokenStream2) -> TokenStream2 {
    quote!(if !(#field_name).is_null() { ferment_interfaces::unbox_any(#field_name); })
}

fn ffi_destroy_primitive_slice(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_vec_ptr(#field_name, self.#count_var);)
}
fn ffi_destroy_complex_slice(field_name: TokenStream2) -> TokenStream2 {
    let count_var = DictionaryFieldName::Count;
    quote!(ferment_interfaces::unbox_any_vec_ptr(#field_name, self.#count_var);)
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
