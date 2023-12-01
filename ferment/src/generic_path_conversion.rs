use quote::{quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use syn::{Ident, parse_quote, Path, PathSegment};
use crate::helper::{destroy_map, destroy_result, destroy_vec, path_arguments_to_path_conversions};
use crate::interface::{ffi_to_conversion, MapPresenter, package_boxed_expression};
use crate::item_conversion::ItemContext;
use crate::path_conversion::PathConversion;
use crate::presentation::{DropInterfacePresentation, FFIObjectPresentation};
use crate::scope::Scope;

pub const PRIMITIVE_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_vec_ptr(#p, self.count););
pub const COMPLEX_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_any_vec_ptr(#p, self.count););

pub const UNBOX_OPTION: MapPresenter = |p| quote!(if !#p.is_null() { ferment_interfaces::unbox_any(#p); });

// macro_rules! format_mangled_ident {
//     ($fmt:expr, $path_presentation:expr) => {
//         format_ident!($fmt, format!("{}", $path_presentation))
//     };
// }
//
pub enum GenericPathConversion {
    Map(Path),
    Vec(Path),
    Result(Path)
}

impl GenericPathConversion {
    pub fn prefix(&self) -> String {
        match self {
            GenericPathConversion::Map(_) => format!("Map_"),
            GenericPathConversion::Vec(_) => format!("Vec_"),
            GenericPathConversion::Result(_) => format!("Result_")
        }
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
            GenericPathConversion::Vec(path) =>
                path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)
                    .first()
                    .unwrap()
                    .mangled_vec_arguments(context),
            GenericPathConversion::Result(path) =>
                match &path_arguments_to_path_conversions(&path.segments.last().unwrap().arguments)[..] {
                    [ok_conversion, error_conversion] => {
                        let ident_string = format!("ok_{}_err_{}", ok_conversion.mangled_map_ident(context), error_conversion.mangled_map_ident(context));
                        syn::LitInt::new(&ident_string, Span::call_site()).to_token_stream()
                    },
                    _ => panic!("arguments_presentation: Map nested in Vec not supported yet")
                }
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

    pub fn destroy_field(&self, field_path: TokenStream2) -> TokenStream2 {
        match self {
            GenericPathConversion::Map(path) => destroy_map(path, field_path),
            GenericPathConversion::Vec(path) => destroy_vec(path, field_path),
            GenericPathConversion::Result(path) => destroy_result(path, field_path)
        }
    }
}

fn from_result(ok_conversion: TokenStream2, err_conversion: TokenStream2) -> TokenStream2 {
    quote! {
        let ffi_ref = &*ffi;
        if ffi_ref.error.is_null() {
            Ok(#ok_conversion)
        } else {
            Err(#err_conversion)
        }
    }
}

fn to_result(ok_conversion: TokenStream2, err_conversion: TokenStream2) -> TokenStream2 {
    let result = package_boxed_expression(quote!(Self { ok, error }));
    quote! {
        let (ok, error) = match obj {
            Ok(obj) => (#ok_conversion, std::ptr::null_mut()),
            Err(err) => (std::ptr::null_mut(), #err_conversion)
        };
        #result
    }
}

impl GenericPathConversion {

    pub fn expand(&self, ffi_name: Ident) -> TokenStream2 {
        match self {
            GenericPathConversion::Result(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();

                let from_ok_simple_conversion = quote!(*ffi_ref.ok);
                let from_ok_complex_conversion = quote!(ferment_interfaces::FFIConversion::ffi_from_const(ffi_ref.ok));
                let from_error_simple_conversion = quote!(*ffi_ref.error);
                let from_error_complex_conversion = quote!(ferment_interfaces::FFIConversion::ffi_from_const(ffi_ref.error));

                let to_ok_simple_conversion = quote!(obj as *mut _);
                let to_error_simple_conversion = quote!(err as *mut _);
                let to_ok_complex_conversion = ffi_to_conversion(quote!(obj));
                let to_error_complex_conversion = ffi_to_conversion(quote!(err));

                let (ok, error, from, to, drop_code) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(ok), PathConversion::Primitive(error)] => {
                        (
                            quote!(#ok), quote!(#error),
                            from_result(from_ok_simple_conversion, from_error_simple_conversion),
                            to_result(to_ok_simple_conversion, to_error_simple_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Complex(error)] => {
                        (
                            quote!(#ok), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_simple_conversion, from_error_complex_conversion),
                            to_result(to_ok_simple_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Generic(generic_error)] => {
                        (
                            quote!(#ok), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_simple_conversion, from_error_complex_conversion),
                            to_result(to_ok_simple_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), quote!(#error),
                            from_result(from_ok_complex_conversion, from_error_simple_conversion),
                            to_result(to_ok_complex_conversion, to_error_simple_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Complex(error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), quote!(#error),
                            from_result(from_ok_complex_conversion, from_error_simple_conversion),
                            to_result(to_ok_complex_conversion, to_error_simple_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Complex(error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![UNBOX_OPTION(quote!(self.ok)), UNBOX_OPTION(quote!(self.error))]
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                FFIObjectPresentation::Result {
                    target_type: quote!(#path),
                    ffi_type: quote!(#ffi_name),
                    ok_type: ok,
                    error_type: error,
                    from_conversion: from,
                    to_conversion: to,
                    drop_presentation: DropInterfacePresentation::Full(quote!(#ffi_name), quote!(#(#drop_code)*))
                }
            },
            GenericPathConversion::Map(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();
                let (key, value, from, to, drop_code) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(k), PathConversion::Primitive(v)] => {
                        (
                            quote!(#k), quote!(#v),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()), values: ferment_interfaces::to_simple_vec(obj.values().cloned().collect()) })),
                            vec![PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.keys)), PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Primitive(k), PathConversion::Complex(v)] => {
                        let value_path = Scope::ffi_type_converted_or_same(&parse_quote!(#v));
                        (
                            quote!(#k), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))]
                        )
                    },
                    [PathConversion::Primitive(k), PathConversion::Generic(v_conversion)] => {
                        let v = v_conversion.as_path();
                        let value_path = PathConversion::from(v).as_ffi_path();
                        (
                            quote!(#k), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::to_simple_vec(obj.keys().cloned().collect()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))]
                        )
                    },
                    [PathConversion::Complex(k), PathConversion::Primitive(v)] => {
                        let key_path = Scope::ffi_type_converted_or_same(&parse_quote!(#k));
                        (
                            quote!(*mut #key_path), quote!(#v),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::to_simple_vec(obj.values().cloned().collect::<Vec<_>>()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Complex(k), PathConversion::Complex(v)] => {
                        let key_path = Scope::ffi_type_converted_or_same(&parse_quote!(#k));
                        let value_path = Scope::ffi_type_converted_or_same(&parse_quote!(#v));
                        (
                            quote!(*mut #key_path), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Complex(k), PathConversion::Generic(v_conversion)] => {
                        let key_path = Scope::ffi_type_converted_or_same(&parse_quote!(#k));
                        let v = v_conversion.as_path();
                        let value_path = PathConversion::from(v).as_ffi_path();
                        (
                            quote!(*mut #key_path), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Generic(k_conversion), PathConversion::Primitive(v)] => {
                        let k = k_conversion.as_path();
                        let key_path = PathConversion::from(k).as_ffi_path();
                        (
                            quote!(*mut #key_path), quote!(#v),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::to_simple_vec(obj.values().cloned().collect::<Vec<_>>()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Generic(k_conversion), PathConversion::Complex(v)] => {
                        let k = k_conversion.as_path();
                        let key_path = PathConversion::from(k).as_ffi_path();
                        let value_path = Scope::ffi_type_converted_or_same(&parse_quote!(#v));
                        (
                            quote!(*mut #key_path), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    [PathConversion::Generic(k_conversion), PathConversion::Generic(v_conversion)] => {
                        let k = k_conversion.as_path();
                        let v = v_conversion.as_path();
                        let key_path = PathConversion::from(k).as_ffi_path();
                        let value_path = PathConversion::from(v).as_ffi_path();
                        (
                            quote!(*mut #key_path), quote!(*mut #value_path),
                            quote!(let ffi_ref = &*ffi; ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)),
                            package_boxed_expression(quote!(Self { count: obj.len(), keys: ferment_interfaces::complex_vec_iterator::<#k, #key_path>(obj.keys().cloned()), values: ferment_interfaces::complex_vec_iterator::<#v, #value_path>(obj.values().cloned()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.keys)), COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))],
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                FFIObjectPresentation::Map { target_type: quote!(#path), ffi_type: ffi_name.to_token_stream(), key, value, from, to, drop_presentation: DropInterfacePresentation::Full(ffi_name.to_token_stream(), quote!(#(#drop_code)*)) }
            },
            GenericPathConversion::Vec(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();
                let (target_arg_type, ffi_arg_type, decode, encode, drop_code) = match &path_arguments_to_path_conversions(arguments)[..] {
                    [PathConversion::Primitive(t)] => {
                        (
                            quote!(#t),
                            quote!(#t),
                            quote!(ferment_interfaces::from_simple_vec(self.values as *const Self::Value, self.count)),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::boxed_vec(obj) })),
                            vec![PRIMITIVE_VEC_DROP_PRESENTER(quote!(self.values))]
                        )
                    },
                    [PathConversion::Complex(t)] => {
                        let value_path = Scope::ffi_type_converted_or_same(&parse_quote!(#t));
                        (
                            quote!(#t),
                            quote!(*mut #value_path),
                            quote!({ let count = self.count; let values = self.values; (0..count).map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i))).collect() }),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::complex_vec_iterator::<Self::Value, #value_path>(obj.into_iter()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))]
                        )
                    },
                    [PathConversion::Generic(t_conversion)] => {
                        let t = t_conversion.as_path();
                        let value_path = PathConversion::from(t).as_ffi_path();
                        (
                            quote!(#t),
                            quote!(*mut #value_path),
                            quote!({ let count = self.count; let values = self.values; (0..count).map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i))).collect() }),
                            package_boxed_expression(quote!(Self { count: obj.len(), values: ferment_interfaces::complex_vec_iterator::<Self::Value, #value_path>(obj.into_iter()) })),
                            vec![COMPLEX_VEC_DROP_PRESENTER(quote!(self.values))]
                        )
                    },
                    _ => unimplemented!("Generic path arguments conversion error"),
                };
                FFIObjectPresentation::Vec {
                    target_arg_type,
                    ffi_type: quote!(#ffi_name),
                    ffi_arg_type,
                    decode,
                    encode,
                    drop_presentation: DropInterfacePresentation::Full(ffi_name.to_token_stream(), quote!(#(#drop_code)*)),
                }
            }
        }.to_token_stream()
    }
}