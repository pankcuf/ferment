use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Add, Brace, Comma};
use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, parse_quote, Path, PathArguments, PathSegment, spanned::Spanned, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTuple};
use syn::__private::TokenStream2;
use crate::composer::{ConstructorPresentableContext, Depunctuated, ParentComposer};
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, TypeConversion};
use crate::conversion::macro_conversion::merge_attributes;
use crate::ext::{Accessory, DictionaryType, FFIResolve, FFITypeResolve, Mangle, Resolve, ToPath, ToType};
use crate::formatter::format_unique_attrs;
use crate::helper::{path_arguments_to_type_conversions, usize_to_tokenstream};
use crate::interface::create_struct;
use crate::naming::{DictionaryExpression, DictionaryFieldName, Name};
use crate::presentation::context::{BindingPresentableContext, FieldTypePresentableContext, IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::{DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, InterfacePresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::wrapped::Wrapped;

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: FieldTypePresentableContext,
    pub from_conversion: FieldTypePresentableContext,
    pub to_conversion: FieldTypePresentableContext,
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
    pub fn new(ty: Type, destructor: FieldTypePresentableContext, from_conversion: FieldTypePresentableContext, to_conversion: FieldTypePresentableContext) -> Self {
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
impl GenericTypeConversion {
    pub fn arg_conversions(&self) -> (FieldTypePresentableContext, FieldTypePresentableContext) {
        let (from, to) = if let GenericTypeConversion::Optional(..) = self {
            match self.ty() {
                None => unimplemented!("Mixin inside generic"),
                Some(ty) => match TypeConversion::from(ty) {
                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                    TypeConversion::Primitive(_) => (FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()), FieldTypePresentableContext::ToOptPrimitive(FieldTypePresentableContext::O.into())),
                    _ => (FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()), FieldTypePresentableContext::ToOpt(FieldTypePresentableContext::O.into())),
                }
            }
        } else {
            (FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()))
        };
        (
            FieldTypePresentableContext::MapExpression(
                FieldTypePresentableContext::O.into(),
                from.into()),
            FieldTypePresentableContext::MapExpression(
                FieldTypePresentableContext::O.into(),
                to.into())
        )
    }
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
            GenericTypeConversion::Optional(ty) => match ty {
                Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                    Some(last_segment) => match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match args.first() {
                            Some(generic_argument) => match generic_argument {
                                GenericArgument::Type(ty) => Some(ty),
                                _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Non-Type Generic): {}", ty.to_token_stream()),
                            },
                            _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", ty.to_token_stream()),
                        }
                        _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments): {}", ty.to_token_stream()),
                    },
                    None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
            }
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
}

impl GenericTypeConversion {

    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        let scope = source.scope.clone();
        println!("GenericTypeConversion::expand.1: {} ---- {} \n\t {}", self, scope, format_unique_attrs(attrs));
        let attrs = merge_attributes(attrs);
        let attrs = (!attrs.is_empty()).then(|| quote!(#[cfg(#attrs)])).unwrap_or_default();
        println!("GenericTypeConversion::expand.2: {} ----\n\t {:?}", self, attrs);

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
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                        )
                    },
                    [TypeConversion::Primitive(ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = error.to_custom_or_ffi_type(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())
                            )
                        )
                    },
                    [TypeConversion::Primitive(ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(&source);
                        let (arg_1_from, arg_1_to) = generic_error.arg_conversions();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                arg_1_from,
                                arg_1_to),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Primitive(error)] => {
                        let arg_0_ffi_type = ok.to_custom_or_ffi_type(&source);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = ok.ffi_resolve_or_same(&source);
                        let arg_1_ffi_type = error.ffi_resolve_or_same(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                        )
                    },
                    [TypeConversion::Complex(ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = ok.to_custom_or_ffi_type(&source);
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(&source);
                        let (arg_1_from, arg_1_to) = generic_error.arg_conversions();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                arg_1_from,
                                arg_1_to),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Primitive(error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(&source);
                        let (arg_0_from, arg_0_to) = generic_ok.arg_conversions();
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                arg_0_from,
                                arg_0_to),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::AsMut_(FieldTypePresentableContext::O.into())),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Complex(error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(&source);
                        let (arg_0_from, arg_0_to) = generic_ok.arg_conversions();
                        let arg_1_ffi_type = error.to_custom_or_ffi_type(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                arg_0_from,
                                arg_0_to),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                        )
                    },
                    [TypeConversion::Generic(generic_ok), TypeConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = generic_ok.to_custom_or_ffi_type(&source);
                        let (arg_0_from, arg_0_to) = generic_ok.arg_conversions();
                        let arg_1_ffi_type = generic_error.to_custom_or_ffi_type(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                arg_0_from,
                                arg_0_to),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyOpt(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::To(FieldTypePresentableContext::O.into()).into())),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(arg_0_name, ok_type.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, error_type.joined_mut(),  Depunctuated::new()),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Result(from_ok_conversion.present(&source), from_error_conversion.present(&source)),
                                ToConversionPresentation::Result(to_ok_conversion.present(&source), to_error_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([ok_destructor.present(&source), error_destructor.present(&source)]),
                    &source
                )
            },
            GenericTypeConversion::Map(ty) | GenericTypeConversion::IndexMap(ty) | GenericTypeConversion::SerdeJsonMap(ty)=> {
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
                                FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()), 
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into()))
                        )
                    }
                    [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);

                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_1_generic_path_conversion {
                            match arg_1_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from.into()),
                                to)
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
                        println!("Map: Complex x Complex: {} x {}", arg_0_target_path.to_token_stream(), arg_1_target_path.to_token_stream());
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                        )
                    }
                    [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        // println!("Map: Complex x Generic: {} x {}", arg_0_target_path.to_token_stream(), arg_1_generic_path_conversion);
                        let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_1_generic_path_conversion {
                            match arg_1_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from.into()),
                                to),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from.into()),
                                to),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from.into()),
                                to),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
                        )
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from_keys, to_keys) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        let (from_values, to_values) = if let GenericTypeConversion::Optional(..) = arg_1_generic_path_conversion {
                            match arg_1_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::FromOptPrimitive(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToPrimitiveOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::FromOpt(FieldTypePresentableContext::O.into()),
                                        FieldTypePresentableContext::ToComplexOptVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()),
                                FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())
                            )
                        };
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from_keys.into()),
                                to_keys),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
                                FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), from_values.into()),
                                to_values),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Map(from_key_conversion.present(&source), from_value_conversion.present(&source)),
                                ToConversionPresentation::Map(to_key_conversion.present(&source), to_value_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor.present(&source), value_destructor.present(&source)]),
                    &source
                )
            },
            // GenericTypeConversion::IndexMap(ty) => {
            //     let ffi_name = ty.mangle_ident_default();
            //     let ffi_as_type = ffi_name.to_type();
            //     let path: Path = ty.to_path();
            //     let PathSegment { arguments, .. } = path.segments.last().unwrap();
            //     let path_conversions = path_arguments_to_type_conversions(arguments);
            //     let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
            //     let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
            //     let count_name = Name::Dictionary(DictionaryFieldName::Count);
            //     let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into()))
            //             )
            //         }
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         _ => unimplemented!("Generic path arguments conversion error"),
            //     };
            //     let target_type: Type = path.to_type();
            //     let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
            //     let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
            //     compose_generic_presentation(
            //         ffi_name,
            //         attrs.clone(),
            //         Depunctuated::from_iter([
            //             FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
            //             FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
            //             FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
            //         ]),
            //         Depunctuated::from_iter([
            //             InterfacePresentation::Conversion {
            //                 attrs: attrs.clone(),
            //                 types: (ffi_as_type.clone(), target_type.clone()),
            //                 conversions: (
            //                     FromConversionPresentation::Map(from_key_conversion.present(&source), from_value_conversion.present(&source)),
            //                     ToConversionPresentation::Map(to_key_conversion.present(&source), to_value_conversion.present(&source)),
            //                     DestroyPresentation::Default,
            //                     None
            //                 )
            //             }
            //         ]),
            //         Depunctuated::from_iter([key_destructor.present(&source), value_destructor.present(&source)]),
            //         &source
            //     )
            // },
            // GenericTypeConversion::SerdeJsonMap(ty) => {
            //     let ffi_name = ty.mangle_ident_default();
            //     let ffi_as_type = ffi_name.to_type();
            //     let path: Path = ty.to_path();
            //     let PathSegment { arguments, .. } = path.segments.last().unwrap();
            //     let path_conversions = path_arguments_to_type_conversions(arguments);
            //     let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
            //     let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
            //     let count_name = Name::Dictionary(DictionaryFieldName::Count);
            //     let (arg_0_presentation, arg_1_presentation) = match &path_conversions[..] {
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into()))
            //             )
            //         }
            //         [TypeConversion::Primitive(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Complex(arg_0_target_path), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = arg_0_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Primitive(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     FieldTypePresentableContext::ToPrimitiveVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Complex(arg_1_target_path)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_target_path.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         [TypeConversion::Generic(arg_0_generic_path_conversion), TypeConversion::Generic(arg_1_generic_path_conversion)] => {
            //             let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             let arg_1_ffi_type = arg_1_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
            //             (
            //                 GenericArgPresentation::new(
            //                     arg_0_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapKeysCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //                 GenericArgPresentation::new(
            //                     arg_1_ffi_type,
            //                     FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_1_name)).into()),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::From(FieldTypePresentableContext::O.into()).into()),
            //                     FieldTypePresentableContext::ToComplexVec(FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::MapValuesCloned(DictionaryFieldName::Obj.to_token_stream())).into())),
            //             )
            //         }
            //         _ => unimplemented!("Generic path arguments conversion error"),
            //     };
            //     let target_type: Type = path.to_type();
            //     let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
            //     let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
            //     compose_generic_presentation(
            //         ffi_name,
            //         attrs.clone(),
            //         Depunctuated::from_iter([
            //             FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
            //             FieldTypeConversion::Named(arg_0_name,key.joined_mut(), Depunctuated::new()),
            //             FieldTypeConversion::Named(arg_1_name, value.joined_mut(), Depunctuated::new())
            //         ]),
            //         Depunctuated::from_iter([
            //             InterfacePresentation::Conversion {
            //                 attrs: attrs.clone(),
            //                 types: (ffi_as_type.clone(), target_type.clone()),
            //                 conversions: (
            //                     FromConversionPresentation::Map(from_key_conversion.present(&source), from_value_conversion.present(&source)),
            //                     ToConversionPresentation::Map(to_key_conversion.present(&source), to_value_conversion.present(&source)),
            //                     DestroyPresentation::Default,
            //                     None
            //                 )
            //             }
            //         ]),
            //         Depunctuated::from_iter([key_destructor.present(&source), value_destructor.present(&source)]),
            //         &source
            //     )
            // },
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
                            FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) }))))
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveOptBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_primitive_opt_vec(obj.into_iter()) })))
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexOptBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_opt_vec(obj.into_iter()) })))
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexBTreeSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                            )
                        };

                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            from,
                            to)
                    }
                    _ => {
                        return quote!();
                    },
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source)]),
                    &source
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
                let arg_0_presentation = match &path_conversions[..] {
                    [TypeConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveHashSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) }))))
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexHashSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveOptHashSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_primitive_opt_vec(obj.into_iter()) })))
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexOptHashSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_opt_vec(obj.into_iter()) })))
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexHashSet(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                            )
                        };

                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            from,
                            to)
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
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source)]),
                    &source
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
                            FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) }))))
                    }
                    [TypeConversion::Complex(arg_0_target_ty)] => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                    [TypeConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveOptVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_primitive_opt_vec(obj.into_iter()) })))
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexOptVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_opt_vec(obj.into_iter()) })))
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                            )
                        };

                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            from,
                            to)
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = path.to_type();
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source)]),
                    &source
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
                    TypeConversion::Callback(arg_0_target_ty) =>
                        unimplemented!("Callbacks are not implemented in generics: {}", arg_0_target_ty.to_token_stream()),
                    TypeConversion::Primitive(arg_0_target_path) => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveArray(quote!(#arg_0_name), quote!(#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj.to_vec()) }))))
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexArray(quote!(#arg_0_name), quote!(#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        let (from, to) = if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                            match arg_0_generic_path_conversion.ty() {
                                None => unimplemented!("Mixin inside generic"),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic"),
                                    TypeConversion::Primitive(_) => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveOptArray(quote!(#arg_0_name), quote!(#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_primitive_opt_vec(obj.into_iter()) })))
                                    ),
                                    _ => (
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexOptArray(quote!(#arg_0_name), quote!(#count_name))),
                                        FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_opt_vec(obj.into_iter()) })))
                                    ),
                                }
                            }
                        } else {
                            (
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexArray(quote!(#arg_0_name), quote!(#count_name))),
                                FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                            )
                        };

                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            from,
                            to)
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_0_presentation;
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                FromConversionPresentation::Just(from_value_conversion.present(&source)),
                                ToConversionPresentation::Struct(to_value_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                                // Some(parse_quote!(<'a>) )
                            )
                        }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source)]),
                    &source
                )

            },
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_slice: TypeSlice = parse_quote!(#ty);
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let count_name = Name::Dictionary(DictionaryFieldName::Count);
                let elem_type = &type_slice.elem;
                let target_type: Type = parse_quote!(Vec<#elem_type>);
                let arg_0_presentation = match TypeConversion::from(&*type_slice.elem) {
                    TypeConversion::Primitive(arg_0_target_path) => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyPrimitiveContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromPrimitiveVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::boxed_vec(obj) }))))
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        let arg_0_ffi_type = arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name))),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                    TypeConversion::Callback(arg_0_target_ty) =>
                        unimplemented!("Callbacks are not implemented in generics: {}", arg_0_target_ty.to_token_stream()),

                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            FieldTypePresentableContext::DestroyComplexContainer(FieldTypePresentableContext::Simple(quote!(self.#arg_0_name)).into()),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::FromComplexVec(quote!(self.#arg_0_name), quote!(self.#count_name)/*, parse_quote!(#arg_0_generic_path_conversion)*/)),
                            FieldTypePresentableContext::DictionaryExpr(DictionaryExpression::BoxedExpression(quote!(Self { #count_name: obj.len(), #arg_0_name: ferment_interfaces::to_complex_vec(obj.into_iter()) }))))
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::Named(count_name, parse_quote!(usize), Depunctuated::new()),
                        FieldTypeConversion::Named(arg_0_name, value.joined_mut(), Depunctuated::new())
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                                ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source)]),
                    &source
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
                            &source))
                    .collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, _))| FieldTypeConversion::Unnamed(Name::UnnamedArg(index), parse_quote!(#root_path), Depunctuated::new()))),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type, parse_quote!(#ty)),
                            conversions: (
                                FromConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.present(&source))).collect()),
                                ToConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.present(&source))).collect()),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source)))),
                    &source
                )
            }
            GenericTypeConversion::Optional(_) |
            GenericTypeConversion::Box(_) |
            GenericTypeConversion::AnyOther(_) |
            GenericTypeConversion::TraitBounds(_) => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}
fn compose_generic_presentation(
    ffi_name: Ident,
    attrs: TokenStream2,
    field_conversions: Depunctuated<FieldTypeConversion>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_as_path: Path = parse_quote!(#ffi_name);
    let ffi_as_type: Type = parse_quote!(#ffi_name);
    let fields = Punctuated::<_, Comma>::from_iter(field_conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), true)));
    let body = Wrapped::<_, Brace>::new(fields.present(source));
    let object_presentation = create_struct(&ffi_as_path, attrs.clone(), body.to_token_stream());
    let drop_presentation = DropInterfacePresentation::Full { attrs: attrs.clone(), ty: ffi_as_type.clone(), body: drop_body.to_token_stream() };
    let bindings = compose_bindings(&ffi_as_type, attrs.clone(), field_conversions).present(source);
    FFIObjectPresentation::Generic { object_presentation, interface_presentations, drop_presentation, bindings }
}

fn compose_bindings(ffi_type: &Type, attrs: TokenStream2, conversions: Depunctuated<FieldTypeConversion>) -> Depunctuated<BindingPresentableContext> {
    Depunctuated::from_iter([
        BindingPresentableContext::Constructor(
            ConstructorPresentableContext::Default(ffi_type.clone(), attrs.to_token_stream()),
            conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), false)).collect(),
            IteratorPresentationContext::Curly(conversions.iter().map(|field| OwnedItemPresentableContext::DefaultField(field.clone())).collect())),
        BindingPresentableContext::Destructor(ffi_type.clone(), attrs.to_token_stream())
    ])
}

fn dictionary_generic_arg(name: Name, field_name: TokenStream2, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty = ty.resolve(source);
    match TypeConversion::from(&ty) {
        TypeConversion::Primitive(arg_ty) => {
            (arg_ty.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_ty.clone(),
                FieldTypePresentableContext::Empty,
                FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_ty, Depunctuated::new())),
                FieldTypePresentableContext::Named((name.to_token_stream(), FieldTypePresentableContext::ObjFieldName(field_name).into())))]))
        }
        TypeConversion::Complex(arg_type) => {
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type.clone(),
                FieldTypePresentableContext::UnboxAnyTerminated(FieldTypePresentableContext::Simple(quote!(self.#name)).into()),
                FieldTypePresentableContext::From(FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_type, Depunctuated::new())).into()),
                FieldTypePresentableContext::Named((name.to_token_stream(), FieldTypePresentableContext::To(FieldTypePresentableContext::ObjFieldName(field_name).into()).into())))]))
        }
        TypeConversion::Callback(arg_0_target_ty) =>
            unimplemented!("Callbacks are not implemented in generics: {}", arg_0_target_ty.to_token_stream()),

        TypeConversion::Generic(root_path) => {
            // TODO: make sure it does work (actually it doesn't)
            let arg_type: Type = parse_quote!(#root_path);
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type.clone(),
                FieldTypePresentableContext::UnboxAnyTerminated(FieldTypePresentableContext::Simple(quote!(self.#name)).into()),
                FieldTypePresentableContext::From(FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(name.clone(), arg_type, Depunctuated::new())).into()),
                FieldTypePresentableContext::Named((name.to_token_stream(), FieldTypePresentableContext::To(FieldTypePresentableContext::ObjFieldName(field_name).into()).into())))]))
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
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     ffi_to_primitive())
            //             }
            //             TypeConversion::Complex(arg_type) => {
            //                 GenericArgPresentation::new(
            //                     parse_quote!(#arg_type),
            //                     quote!(),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     ffi_to_primitive())
            //             }
            //             TypeConversion::Generic(arg_type) => {
            //                 GenericArgPresentation::new(
            //                     parse_quote!(#arg_type),
            //                     quote!(),
            //                     FieldTypePresentableContext::MapExpression(FieldTypePresentableContext::O.into(), FieldTypePresentableContext::O.into()),
            //                     ffi_to_primitive())
            //             }
            //         }
            //     })
            //     .collect::<Depunctuated<GenericArgPresentation>>())
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
