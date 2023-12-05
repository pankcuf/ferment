use quote::{quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use syn::{Ident, parse_quote, Path, PathSegment, Type};
use crate::helper::path_arguments_to_path_conversions;
use crate::idents::{convert_to_ffi_path, ffi_path_converted_or_same};
use crate::interface::{ffi_to_conversion, MapPresenter, package_boxed_expression};
use crate::item_conversion::ItemContext;
use crate::path_conversion::PathConversion;
use crate::presentation::FFIObjectPresentation;

pub const PRIMITIVE_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_vec_ptr(#p, self.count););
pub const COMPLEX_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_any_vec_ptr(#p, self.count););
pub const UNBOX_OPTION: MapPresenter = |p|
    quote!(if !#p.is_null() { ferment_interfaces::unbox_any(#p); });

pub enum GenericPathConversion {
    Map(Path),
    Vec(Path),
    Result(Path)
}

impl GenericPathConversion {
    pub fn prefix(&self) -> String {
        match self {
            GenericPathConversion::Map(_) => "Map_",
            GenericPathConversion::Vec(_) => "Vec_",
            GenericPathConversion::Result(_) => "Result_"
        }.to_string()
    }

    pub fn arguments_presentation(&self, context: &ItemContext) -> TokenStream2 {
        match self {
            GenericPathConversion::Map(path) =>
                match &path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)[..] {
                    [key_conversion, value_conversion] => {
                        let ident_string = format!("keys_{}_values_{}", key_conversion.mangled_map_ident(context), value_conversion.mangled_map_ident(context));
                        syn::LitInt::new(&ident_string, Span::call_site()).to_token_stream()
                    },
                    _ => panic!("arguments_presentation: Map nested in Vec not supported yet"),
                },
            GenericPathConversion::Result(path) =>
                match &path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)[..] {
                    [ok_conversion, error_conversion] => {
                        let ident_string = format!("ok_{}_err_{}", ok_conversion.mangled_map_ident(context), error_conversion.mangled_map_ident(context));
                        syn::LitInt::new(&ident_string, Span::call_site()).to_token_stream()
                    },
                    _ => panic!("arguments_presentation: Map nested in Vec not supported yet")
                },
            GenericPathConversion::Vec(path) =>
                path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)
                    .first()
                    .unwrap()
                    .mangled_vec_arguments(context),
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) => path,
        }
    }

    pub fn path(self) -> Path {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) => path
        }
    }
}


impl<'a> From<&'a GenericPathConversion> for Type {
    fn from(generic_path_conversion: &'a GenericPathConversion) -> Self {
        convert_to_ffi_path(generic_path_conversion.as_path())
    }
}


impl<'a> From<&'a PathConversion> for Type {
    fn from(path_conversion: &'a PathConversion) -> Self {
        match path_conversion {
            PathConversion::Primitive(path) => parse_quote!(#path),
            PathConversion::Complex(path) => ffi_path_converted_or_same(path),
            PathConversion::Generic(generic_path_conversion) => Type::from(generic_path_conversion),
        }
    }
}

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: TokenStream2,
    pub from_conversion: TokenStream2,
    pub to_conversion: TokenStream2,
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: TokenStream2, from_conversion: TokenStream2, to_conversion: TokenStream2) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}

impl GenericPathConversion {

    pub fn expand(&self, ffi_name: Ident) -> TokenStream2 {



        match self {
            GenericPathConversion::Result(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();

                let arg_0_name = quote!(ok);
                let arg_1_name = quote!(error);

                let (arg_0_presentation, arg_1_presentation) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(ok), PathConversion::Primitive(error)] => {
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                parse_quote!(#error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Complex(error)] => {
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Generic(generic_error)] => {
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                            GenericArgPresentation::new(
                                Type::from(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        (
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                parse_quote!(#error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Complex(error)] => {
                        (
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        (
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                Type::from(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        (
                            GenericArgPresentation::new(
                                Type::from(generic_ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                parse_quote!(#error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| *o),
                                quote!(o as *mut _)),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Complex(error)] => {
                        (
                            GenericArgPresentation::new(
                                Type::from(generic_ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                ffi_path_converted_or_same(error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        (
                            GenericArgPresentation::new(
                                Type::from(generic_ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                Type::from(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };

                FFIObjectPresentation::Result { target_type: quote!(#path), ffi_type: quote!(#ffi_name), ok_presentation: arg_0_presentation, error_presentation: arg_1_presentation }
            },
            GenericPathConversion::Map(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();

                let arg_0_name = quote!(keys);
                let arg_1_name = quote!(values);
                let (arg_0_presentation, arg_1_presentation) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#arg_0_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(#arg_1_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_1_ffi_type = ffi_path_converted_or_same(arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#arg_0_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned())))
                        )
                    }
                    [PathConversion::Primitive(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_1_ffi_type = Type::from(arg_1_generic_path_conversion);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(#arg_0_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = ffi_path_converted_or_same(arg_0_target_path);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(#arg_1_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = ffi_path_converted_or_same(arg_0_target_path);
                        let arg_1_ffi_type = ffi_path_converted_or_same(arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Complex(arg_0_target_path), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = ffi_path_converted_or_same(arg_0_target_path);
                        let arg_1_ffi_type = Type::from(arg_1_generic_path_conversion);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Primitive(arg_1_target_path)] => {
                        let arg_0_ffi_type = Type::from(arg_0_generic_path_conversion);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(#arg_1_target_path),
                                PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| o),
                                quote!(ferment_interfaces::to_primitive_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Complex(arg_1_target_path)] => {
                        let arg_0_ffi_type = Type::from(arg_0_generic_path_conversion);
                        let arg_1_ffi_type = ffi_path_converted_or_same(arg_1_target_path);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion), PathConversion::Generic(arg_1_generic_path_conversion)] => {
                        let arg_0_ffi_type = Type::from(arg_0_generic_path_conversion);
                        let arg_1_ffi_type = Type::from(arg_1_generic_path_conversion);
                        (
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_0_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.keys().cloned()))),
                            GenericArgPresentation::new(
                                parse_quote!(*mut #arg_1_ffi_type),
                                COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                quote!(ferment_interfaces::to_complex_vec(obj.values().cloned()))),
                        )
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
               FFIObjectPresentation::Map {
                    target_type: quote!(#path),
                    ffi_type: ffi_name.to_token_stream(),
                    key_presentation: arg_0_presentation,
                    value_presentation: arg_1_presentation,
                }
            },
            GenericPathConversion::Vec(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();
                let arg_0_name = quote!(values);
                let arg_0_presentation = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(arg_0_target_path)] => {
                        GenericArgPresentation::new(
                            parse_quote!(#arg_0_target_path),
                            PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            quote!(ferment_interfaces::from_primitive_vec(self.values, self.count)),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::boxed_vec(obj) })))
                    }
                    [PathConversion::Complex(arg_0_target_path)] => {
                        let arg_0_ffi_type = ffi_path_converted_or_same(arg_0_target_path);
                        GenericArgPresentation::new(
                            parse_quote!(*mut #arg_0_ffi_type),
                            COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            quote!(ferment_interfaces::from_complex_vec(self.values, self.count)),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = Type::from(arg_0_generic_path_conversion);
                        GenericArgPresentation::new(
                            parse_quote!(*mut #arg_0_ffi_type),
                            COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            quote!(ferment_interfaces::from_complex_vec(self.values, self.count)),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                    }
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                FFIObjectPresentation::Vec {
                    target_type: quote!(#path),
                    ffi_type: ffi_name.to_token_stream(),
                    value_presentation: arg_0_presentation,
                }
            }
        }.to_token_stream()
    }
}