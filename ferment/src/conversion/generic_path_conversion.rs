use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use quote::{quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{parse_quote, Path, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use crate::composer::{ConstructorPresentableContext, Depunctuated, OwnedFieldTypeComposerRef, ParentComposer, SimpleComposerPresenter};
use crate::conversion::type_conversion::TypeConversion;
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, PathConversion};
use crate::ext::{Accessory, Mangle};
use crate::helper::path_arguments_to_path_conversions;
use crate::interface::{create_struct, ffi_to_conversion};
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::presentation::{ConversionInterfacePresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{BindingPresentableContext, IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::destroy_presentation::DestroyPresentation;
use crate::wrapped::Wrapped;

pub const PRIMITIVE_VEC_DROP_PRESENTER: SimpleComposerPresenter = |p|
    quote!(ferment_interfaces::unbox_vec_ptr(#p, self.count););
pub const COMPLEX_VEC_DROP_PRESENTER: SimpleComposerPresenter = |p|
    quote!(ferment_interfaces::unbox_any_vec_ptr(#p, self.count););
pub const UNBOX_OPTION: SimpleComposerPresenter = |p|
    quote!(if !#p.is_null() { ferment_interfaces::unbox_any(#p); });

pub const SIG_ARG_PRESENTER: OwnedFieldTypeComposerRef =
    |field| OwnedItemPresentableContext::Named(field.clone(), false);
pub const FIELD_PRESENTER: OwnedFieldTypeComposerRef =
    |field| OwnedItemPresentableContext::Named(field.clone(), true);
pub const BODY_ARG_PRESENTER: OwnedFieldTypeComposerRef =
    |field| OwnedItemPresentableContext::DefaultField(field.clone());

#[derive(Clone)]
pub enum GenericPathConversion {
    Map(Path),
    Vec(Path),
    Result(Path),
    Box(Path),
    AnyOther(Path),
    // Arc(Path),
    // Mutex(Path),
}
impl ToTokens for GenericPathConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.as_path().to_tokens(tokens)
    }
}

impl GenericPathConversion {
    pub fn as_path(&self) -> &Path {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) |
            GenericPathConversion::Box(path) |
            GenericPathConversion::AnyOther(path) => path
        }
    }
}

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

// fn result_arg_conversion_from(path: &PathConversion) -> TokenStream2 {
//     match path {
//         PathConversion::Primitive(_) => quote!(|o| *o),
//         _ => quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
//     }
// }
// fn result_arg_conversion_to(path: &PathConversion) -> TokenStream2 {
//     match path {
//         PathConversion::Primitive(_) => quote!(o as *mut _),
//         _ => ffi_to_conversion(quote!(o)),
//     }
// }


impl GenericPathConversion {
    pub fn to_ffi_path(&self) -> Type {
        // println!("convert_to_ffi_path: {}", format_token_stream(generic_path_conversion));
        let path = self.as_path();
        let mut cloned_segments = path.segments.clone();
        let last_segment = cloned_segments.iter_mut().last().unwrap();
        let last_ident = &last_segment.ident;
        match last_ident.to_string().as_str() {
            // simple primitive type
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
            "isize" | "usize" | "bool" => parse_quote!(#last_ident),
            // complex special type
            "str" | "String" => parse_quote!(std::os::raw::c_char),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = path.to_mangled_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)
            },
            "Result" if cloned_segments.len() == 1 => {
                let ffi_name = path.to_mangled_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)

            },
            _ => {
                let new_segments = cloned_segments
                    .into_iter()
                    .map(|segment| quote_spanned! { segment.span() => #segment })
                    .collect::<Vec<_>>();
                parse_quote!(#(#new_segments)::*)
            }
        }
    }

    pub fn expand(&self, full_type: &TypeConversion, context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = context.borrow();
        let ffi_type = full_type.ty();
        let ffi_name = ffi_type.to_mangled_ident_default();
        let ffi_as_path: Path = parse_quote!(#ffi_name);
        let ffi_as_type: Type = parse_quote!(#ffi_name);

        match self {
            GenericPathConversion::Result(path) => {
                let PathSegment { arguments, .. } = path.segments.last().unwrap();

                let arg_0_name = Name::Dictionary(DictionaryFieldName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Error);

                let (arg_0_presentation, arg_1_presentation) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = parse_quote!(#ok);
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(ok);
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(ok);
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(ok);
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = parse_quote!(#error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Complex(error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(error);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        let arg_0_ffi_type = generic_ok.to_ffi_path();
                        let arg_1_ffi_type = generic_error.to_ffi_path();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: ok_type, from_conversion: from_ok_conversion, to_conversion: to_ok_conversion, destructor: ok_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: error_type, from_conversion: from_error_conversion, to_conversion: to_error_conversion, destructor: error_destructor } = arg_1_presentation;
                let source = context.borrow();

                let field_conversions = Depunctuated::from_iter([
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Ok), ok_type.joined_mut()),
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Error), error_type.joined_mut()),
                ]);
                let field_presentation_iter = field_conversions.iter().map(|conversion| FIELD_PRESENTER(conversion));


                let fields = Punctuated::<_, Comma>::from_iter(field_presentation_iter);
                let body = Wrapped::<_, Brace>::new(fields.present(&source));
                FFIObjectPresentation::Generic {
                    object_presentation: create_struct(&ffi_as_path, body.to_token_stream()),
                    conversion_presentation: ConversionInterfacePresentation::Interface {
                        types: (ffi_as_type.clone(), target_type.clone()),
                        conversions: (
                            FromConversionPresentation::Result(quote!(#from_ok_conversion), quote!(#from_error_conversion)),
                            ToConversionPresentation::Result(quote!(#to_ok_conversion), quote!(#to_error_conversion)),
                            DestroyPresentation::Default,
                            None
                        )
                    },
                    drop_presentation: DropInterfacePresentation::Full {
                        ty: ffi_as_type.clone(),
                        body: Depunctuated::from_iter([ok_destructor, error_destructor]).to_token_stream()
                    },
                    bindings: compose_bindings(&ffi_as_type, field_conversions).present(&source),
                    special: None,
                }
            },
            GenericPathConversion::Map(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();

                let arg_0_name = Name::Dictionary(DictionaryFieldName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryFieldName::Values);
                let (arg_0_presentation, arg_1_presentation) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(arg_1_target_path).joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned())))
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(arg_0_target_path).joined_mut();
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(arg_0_target_path).joined_mut();
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(arg_1_target_path).joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(arg_0_target_path).joined_mut();
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = parse_quote!(#arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = source.ffi_path_converted_or_same(arg_1_target_path).joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        let arg_1_ffi_type = arg_1_generic_path_conversion.to_ffi_path().joined_mut();
                        (
                            GenericArgPresentation::new(
                                arg_0_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                arg_1_ffi_type,
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                let source = context.borrow();
                let field_conversions = Depunctuated::from_iter([
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Count), parse_quote!(usize)),
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Keys),key.joined_mut()),
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Values), value.joined_mut())
                ]);

                let field_presentation_iter = field_conversions.iter().map(|conversion| FIELD_PRESENTER(conversion));

                let fields = Punctuated::<_, Comma>::from_iter(field_presentation_iter);

                FFIObjectPresentation::Generic {
                    object_presentation: create_struct(&ffi_as_path, Wrapped::<_, Brace>::new(fields.present(&source)).to_token_stream()),
                    conversion_presentation: ConversionInterfacePresentation::Interface {
                        types: (ffi_as_type.clone(), target_type.clone()),
                        conversions: (
                            FromConversionPresentation::Map(quote!(#from_key_conversion), quote!(#from_value_conversion)),
                            ToConversionPresentation::Map(quote!(#to_key_conversion), quote!(#to_value_conversion)),
                            DestroyPresentation::Default,
                            None
                        )
                    },
                    drop_presentation: DropInterfacePresentation::Full { ty: ffi_as_type.clone(), body: Depunctuated::from_iter([key_destructor, value_destructor]).to_token_stream() },
                    bindings: compose_bindings(&ffi_as_type, field_conversions).present(&source),
                    special: None,
                }
            },
            GenericPathConversion::Vec(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();
                let arg_0_name = Name::Dictionary(DictionaryFieldName::Values);
                let arg_0_presentation = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(arg_0_target_path)] => {
                        let arg_0_ffi_type = parse_quote!(#arg_0_target_path);
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromPrimitiveVec(quote!(self.values), quote!(self.count)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { count: obj.len(), values: ferment_interfaces::boxed_vec(obj) })).to_token_stream())
                    }
                    [PathConversion::Complex(arg_0_target_path)] => {
                        let arg_0_ffi_type = source.ffi_path_converted_or_same(arg_0_target_path).joined_mut();
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromComplexVec(quote!(self.values), quote!(self.count)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { count: obj.len(), values: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = arg_0_generic_path_conversion.to_ffi_path().joined_mut();
                        GenericArgPresentation::new(
                            arg_0_ffi_type,
                            COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            DictionaryFieldName::FromComplexVec(quote!(self.values), quote!(self.count)).to_token_stream(),
                            DictionaryFieldName::BoxedExpression(quote!(Self { count: obj.len(), values: ferment_interfaces::to_complex_vec(obj.into_iter()) })).to_token_stream())
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                let target_type: Type = parse_quote!(#path);
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_0_presentation;
                let source = context.borrow();
                let field_conversions = Depunctuated::from_iter([
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Count), parse_quote!(usize)),
                    FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Values), value.joined_mut())
                ]);
                let field_presentation_iter = field_conversions.iter().map(|conversion| FIELD_PRESENTER(conversion));
                let fields = Punctuated::<_, Comma>::from_iter(field_presentation_iter);
                let body = Wrapped::<_, Brace>::new(fields.present(&source));

                FFIObjectPresentation::Generic {
                    object_presentation: create_struct(&ffi_as_path, body.to_token_stream()),
                    conversion_presentation: ConversionInterfacePresentation::Interface {
                        types: (ffi_as_type.clone(), target_type.clone()),
                        conversions: (
                            FromConversionPresentation::Just(quote!(ferment_interfaces::FFIVecConversion::decode(&*ffi))),
                            ToConversionPresentation::Struct(quote!(ferment_interfaces::FFIVecConversion::encode(obj))),
                            DestroyPresentation::Default,
                            None
                        )
                    },
                    drop_presentation: DropInterfacePresentation::Full { ty: ffi_as_type.clone(), body: Depunctuated::from_iter([value_destructor]).to_token_stream() },
                    bindings: compose_bindings(&ffi_as_type, field_conversions).present(&source),
                    special: Some(quote! {
                        impl ferment_interfaces::FFIVecConversion for #ffi_as_type {
                            type Value = #target_type;
                            unsafe fn decode(&self) -> Self::Value { #from_value_conversion }
                            unsafe fn encode(obj: Self::Value) -> *mut Self { #to_value_conversion }
                        }
                    }),
                }
            },
            GenericPathConversion::Box(_path) | GenericPathConversion::AnyOther(_path) =>
                FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}
// fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>);

fn compose_bindings(ffi_type: &Type, conversions: Depunctuated<FieldTypeConversion>) -> Depunctuated<BindingPresentableContext> {
    Depunctuated::from_iter([
        BindingPresentableContext::Constructor(
            ConstructorPresentableContext::Default(ffi_type.clone()),
            conversions.iter().map(SIG_ARG_PRESENTER).collect(),
            IteratorPresentationContext::Curly(conversions.iter().map(BODY_ARG_PRESENTER).collect())),
        BindingPresentableContext::Destructor(ffi_type.clone())
    ])
}