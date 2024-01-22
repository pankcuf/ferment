use std::cell::RefCell;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use syn::{parse_quote, Path, PathSegment, Type};
use crate::conversion::type_conversion::TypeConversion;
use crate::context::ScopeContext;
use crate::conversion::PathConversion;
use crate::formatter::format_token_stream;
use crate::helper::{ffi_mangled_ident, path_arguments_to_path_conversions};
use crate::interface::{ffi_to_conversion, MapPresenter, package_boxed_expression};
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;

pub const PRIMITIVE_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_vec_ptr(#p, self.count););
pub const COMPLEX_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_any_vec_ptr(#p, self.count););
pub const UNBOX_OPTION: MapPresenter = |p|
    quote!(if !#p.is_null() { ferment_interfaces::unbox_any(#p); });

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
    pub fn prefix(&self) -> String {
        match self {
            GenericPathConversion::Map(_) => "Map_",
            GenericPathConversion::Vec(_) => "Vec_",
            GenericPathConversion::Result(_) => "Result_",
            GenericPathConversion::Box(_) => "",
            GenericPathConversion::AnyOther(_) => "",
        }.to_string()
    }

    pub fn arguments_presentation(&self, context: &ScopeContext) -> TokenStream2 {
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
            GenericPathConversion::Box(path) =>
                path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)
                    .first()
                    .unwrap()
                    .mangled_box_arguments(context),
            GenericPathConversion::AnyOther(path) => {
                path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)
                    .first()
                    .unwrap()
                    .mangled_box_arguments(context)
            }
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            GenericPathConversion::Map(path) |
            GenericPathConversion::Vec(path) |
            GenericPathConversion::Result(path) |
            GenericPathConversion::Box(path) |
            GenericPathConversion::AnyOther(path) => path
        }
    }

    pub fn path(self) -> Path {
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

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: TokenStream2, from_conversion: TokenStream2, to_conversion: TokenStream2) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}

impl GenericPathConversion {

    pub fn expand(&self, full_type: &TypeConversion, context: &Rc<RefCell<ScopeContext>>) -> TokenStream2 {
        let borrowed_context = context.borrow();
        let ffi_name = ffi_mangled_ident(full_type.ty());
        println!("GenericPathConversion::expand: {}: [{}]", full_type, ffi_name);
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
                                borrowed_context.ffi_path_converted_or_same(error),
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
                                borrowed_context.convert_to_ffi_path(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        (
                            GenericArgPresentation::new(
                                borrowed_context.ffi_path_converted_or_same(ok),
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
                        println!("Result Complex x Complex {} x {}", format_token_stream(ok), format_token_stream(error));
                        (
                            GenericArgPresentation::new(
                                borrowed_context.ffi_path_converted_or_same(ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                borrowed_context.ffi_path_converted_or_same(error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        println!("Result Complex x Generic {} x {}", format_token_stream(ok), format_token_stream(generic_error));
                        (
                            GenericArgPresentation::new(
                                borrowed_context.ffi_path_converted_or_same(ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                borrowed_context.convert_to_ffi_path(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        (
                            GenericArgPresentation::new(
                                borrowed_context.convert_to_ffi_path(generic_ok),
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
                                borrowed_context.convert_to_ffi_path(generic_ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                borrowed_context.ffi_path_converted_or_same(error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        (
                            GenericArgPresentation::new(
                                borrowed_context.convert_to_ffi_path(generic_ok),
                                UNBOX_OPTION(quote!(self.#arg_0_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                            GenericArgPresentation::new(
                                borrowed_context.convert_to_ffi_path(generic_error),
                                UNBOX_OPTION(quote!(self.#arg_1_name)),
                                quote!(|o| ferment_interfaces::FFIConversion::ffi_from(o)),
                                ffi_to_conversion(quote!(o))),
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };

                FFIObjectPresentation::Result {
                    target_type: quote!(#path),
                    ffi_type: ffi_name.clone(),
                    ok_presentation: arg_0_presentation,
                    error_presentation: arg_1_presentation,
                    generics: None,
                    context: context.clone()
                }
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
                        let arg_1_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_1_target_path);
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
                        let arg_1_ffi_type = borrowed_context.convert_to_ffi_path(arg_1_generic_path_conversion);
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
                        let arg_0_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_0_target_path);
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
                        let arg_0_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_0_target_path);
                        let arg_1_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_1_target_path);
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
                        let arg_0_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_0_target_path);
                        let arg_1_ffi_type = borrowed_context.convert_to_ffi_path(arg_1_generic_path_conversion);
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
                        let arg_0_ffi_type = borrowed_context.convert_to_ffi_path(arg_0_generic_path_conversion);
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
                        let arg_0_ffi_type = borrowed_context.convert_to_ffi_path(arg_0_generic_path_conversion);
                        let arg_1_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_1_target_path);
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
                        let arg_0_ffi_type = borrowed_context.convert_to_ffi_path(arg_0_generic_path_conversion);
                        let arg_1_ffi_type = borrowed_context.convert_to_ffi_path(arg_1_generic_path_conversion);
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
                    ffi_type: ffi_name,
                    key_presentation: arg_0_presentation,
                    value_presentation: arg_1_presentation,
                    generics: None,
                    context: Rc::clone(context)
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
                        let arg_0_ffi_type = borrowed_context.ffi_path_converted_or_same(arg_0_target_path);
                        GenericArgPresentation::new(
                            parse_quote!(*mut #arg_0_ffi_type),
                            COMPLEX_VEC_DROP_PRESENTER(quote!(self.#arg_0_name)),
                            quote!(ferment_interfaces::from_complex_vec(self.values, self.count)),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::to_complex_vec(obj.into_iter()) })))
                    }
                    [PathConversion::Generic(arg_0_generic_path_conversion)] => {
                        let arg_0_ffi_type = borrowed_context.convert_to_ffi_path(arg_0_generic_path_conversion);
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
                    ffi_type: ffi_name,
                    value_presentation: arg_0_presentation,
                    generics: None,
                    context: Rc::clone(context)
                }
            },
            GenericPathConversion::Box(_path) => {
                FFIObjectPresentation::Empty
            }
            GenericPathConversion::AnyOther(_path) => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}