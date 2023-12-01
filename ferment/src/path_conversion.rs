use std::fmt;
use std::fmt::Debug;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use crate::generics::{map_ffi_expansion, vec_ffi_exp};
use crate::interface::{ffi_to_conversion, MANGLE_INNER_PATH_PRESENTER, MANGLE_MAP_ARGUMENTS_PRESENTER, MANGLE_RESULT_ARGUMENTS_PRESENTER, MANGLE_VEC_ARGUMENTS_PRESENTER, MapPresenter, package_boxed_expression};
use crate::helper::{destroy_map, destroy_result, destroy_vec, ffi_struct_name, path_arguments_to_path_conversions, path_arguments_to_paths};
use crate::item_conversion::ItemContext;
use crate::presentation::{DropInterfacePresentation, FFIObjectPresentation};
use crate::scope::Scope;

macro_rules! format_mangled_ident {
    ($fmt:expr, $path_presentation:expr) => {
        format_ident!($fmt, format!("{}", $path_presentation))
    };
}
// fn mangle_generic_with_prefix(segment: &PathSegment, segment_str: &str, prefix1: &str, prefix2: &str) -> String {
//     let ss = match &segment.arguments {
//         PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
//             format!("{}_{}",
//                     segment_str,
//                     args.iter()
//                         .enumerate()
//                         .map(|(i, gen_arg)| match gen_arg {
//                             GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
//                                 let mangled = mangled_inner_generic_ident_string(path);
//                                 format!("{}{}", if i == 0 { prefix1 } else { prefix2 }, mangled)
//                             }
//                             _ => panic!("Unknown generic argument: {}", quote!(#gen_arg)),
//                         })
//                         .collect::<Vec<_>>()
//                         .join("_")
//             ),
//         _ => segment_str.to_string(),
//     };
//     println!("mangle_generic_with_prefix: {}: {}", quote!(#segment), ss);
//     ss
// }
// pub fn mangled_inner_generic_ident_string(path: &Path) -> String {
//     path.segments
//         .iter()
//         .map(|segment| {
//             let segment_str = segment.ident.to_string();
//             match segment_str.as_str() {
//                 "BTreeMap" | "HashMap" =>
//                     mangle_generic_with_prefix(segment, "Map", "keys_", "values_"),
//                 "Result" =>
//                     mangle_generic_with_prefix(segment, "Result", "ok_", "err_"),
//                 _ =>
//                     segment.ident.to_string()
//             }
//         })
//         .collect::<Vec<_>>()
//         .join("_")
// }

pub trait Mangle {
    fn mangle(&self) -> TokenStream2;
}

pub enum GenericPathConversion {
    Map(Path),
    Vec(Path),
    Result(Path)
}

impl GenericPathConversion {
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

pub const PRIMITIVE_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_vec_ptr(#p, self.count););
pub const COMPLEX_VEC_DROP_PRESENTER: MapPresenter = |p|
    quote!(ferment_interfaces::unbox_any_vec_ptr(#p, self.count););


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
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Complex(error)] => {
                        (
                            quote!(#ok), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_simple_conversion, from_error_complex_conversion),
                            to_result(to_ok_simple_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Primitive(ok), PathConversion::Generic(generic_error)] => {
                        (
                            quote!(#ok), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_simple_conversion, from_error_complex_conversion),
                            to_result(to_ok_simple_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Primitive(error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), quote!(#error),
                            from_result(from_ok_complex_conversion, from_error_simple_conversion),
                            to_result(to_ok_complex_conversion, to_error_simple_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Complex(error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Complex(ok), PathConversion::Generic(generic_error)] => {
                        (
                            Scope::ffi_type_converted_or_same(&parse_quote!(#ok)).to_token_stream(), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Primitive(error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), quote!(#error),
                            from_result(from_ok_complex_conversion, from_error_simple_conversion),
                            to_result(to_ok_complex_conversion, to_error_simple_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Complex(error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), Scope::ffi_type_converted_or_same(&parse_quote!(#error)).to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
                        )
                    },
                    [PathConversion::Generic(generic_ok), PathConversion::Generic(generic_error)] => {
                        (
                            PathConversion::from(generic_ok.as_path()).as_ffi_path().to_token_stream(), PathConversion::from(generic_error.as_path()).as_ffi_path().to_token_stream(),
                            from_result(from_ok_complex_conversion, from_error_complex_conversion),
                            to_result(to_ok_complex_conversion, to_error_complex_conversion),
                            vec![quote!(if !self.ok.is_null() { ferment_interfaces::unbox_any(self.ok); }), quote!(if !self.error.is_null() { ferment_interfaces::unbox_any(self.error); })]
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
                    drop_presentation: DropInterfacePresentation::Full(
                    quote!(#ffi_name),
                    quote!(#(#drop_code)*))
                        .to_token_stream()
                }.to_token_stream()
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
                map_ffi_expansion(
                    ffi_name.to_token_stream(),
                    quote!(#path),
                    key,
                    value,
                    from,
                    to,
                    DropInterfacePresentation::Full(
                        ffi_name.to_token_stream(),
                        quote!(#(#drop_code)*)))
            },
            GenericPathConversion::Vec(path) => {
                let PathSegment { arguments, ..} = path.segments.last().unwrap();
                let (original, mangled_t, decode, encode, drop_code) = match &path_arguments_to_path_conversions(arguments)[..] {
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
                vec_ffi_exp(
                    ffi_name.to_token_stream(),
                    original,
                    mangled_t,
                    decode,
                    encode,
                    DropInterfacePresentation::Full(ffi_name.to_token_stream(), quote!(#(#drop_code)*)))
            }
        }
    }
}

pub enum PathConversion {
    Primitive(Path),
    Complex(Path),
    Generic(GenericPathConversion),
}

// impl PathConversion {
//     pub fn to_ffi_token_stream(&self) -> TokenStream2 {
//         match self {
//             PathConversion::Primitive(path) =>
//                 quote!(#path),
//             PathConversion::Complex(path) =>
//                 Scope::ffi_type_converted_or_same(&parse_quote!(#path)).to_token_stream(),
//             PathConversion::Generic(generic_path_conversion) =>
//                 PathConversion::from(generic_path_conversion.as_path()).as_ffi_path().to_token_stream()
//         }
//     }
// }

impl Debug for PathConversion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PathConversion")?;
        f.debug_list()
            .entries(self.as_path().to_token_stream())
            .finish()
    }
}

impl PartialEq for PathConversion {
    fn eq(&self, other: &PathConversion) -> bool {
        let self_inner = self.as_path();
        let other_inner = other.as_path();
        let self_inner_str = quote! { #self_inner }.to_string();
        let other_inner_str = quote! { #other_inner }.to_string();
        self_inner_str == other_inner_str
    }
}
impl Eq for PathConversion {}

impl From<&str> for PathConversion {
    fn from(s: &str) -> Self {
        PathConversion::from(&syn::parse_str::<Path>(s).unwrap())
    }
}

impl From<&Path> for PathConversion {
    fn from(path: &Path) -> Self {
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
}

impl From<Path> for PathConversion {
    fn from(path: Path) -> Self {
        Self::from(&path)
    }
}


impl PathConversion {

    pub fn convert_to_ffi_path(path: &Path) -> Type {
        let mut cloned_segments = path.segments.clone();
        let last_segment = cloned_segments.iter_mut().last().unwrap();
        let last_ident = &last_segment.ident;
        match last_ident.to_string().as_str() {
            // simple primitive type
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
            | "isize" | "usize" | "bool" => parse_quote!(#last_ident),
            // complex special type
            "str" | "String" => parse_quote!(std::os::raw::c_char),
            "UInt128" => parse_quote!([u8; 16]),
            "UInt160" => parse_quote!([u8; 20]),
            "UInt256" => parse_quote!([u8; 32]),
            "UInt384" => parse_quote!([u8; 48]),
            "UInt512" => parse_quote!([u8; 64]),
            "UInt768" => parse_quote!([u8; 96]),
            "VarInt" => parse_quote!(u64),
            // generic expanded type
            "Vec" | "BTreeMap" | "HashMap" | "Result" => {
                let ident = format_ident!("{}", PathConversion::mangled_inner_generic_ident_string(path));
                // let result = parse_quote!(#ident);
                let result = parse_quote!(crate::fermented::generics::#ident);
                println!("convert_to_ffi_path (generic): {} -> {}", quote!(#path), quote!(#result));
                result
            }
            // complex/vec/map generated type
            _ => {
                last_segment.ident = ffi_struct_name(&last_segment.ident);
                let ddd = last_segment.ident.clone();
                println!("convert_to_ffi_path (complex): {} --> {}", quote!(#path), quote!(#ddd));
                let new_segments = cloned_segments
                    .into_iter()
                    .map(|segment| quote_spanned! { segment.span() => #segment })
                    .collect::<Vec<_>>();
                parse_quote!(#(#new_segments)::*)
            }
        }
    }

    pub fn as_ffi_path(&self) -> Type {
        Self::convert_to_ffi_path(self.as_path())
    }
    pub fn as_ffi_type(&self) -> Type {
        let path = self.as_path();//.as_ffi_path();
        match self {
            PathConversion::Primitive(..) => parse_quote!(#path),
            _ => parse_quote!(*mut #path),
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) |
            PathConversion::Generic(GenericPathConversion::Map(path)) |
            PathConversion::Generic(GenericPathConversion::Vec(path)) |
            PathConversion::Generic(GenericPathConversion::Result(path)) => path,
        }
    }

    pub fn mangled_inner_generic_ident_string(path: &Path) -> String {
        path.segments
            .iter()
            .map(|segment| {
                let mut segment_str = segment.ident.to_string();
                let is_map = segment_str == "BTreeMap" || segment_str == "HashMap";
                if is_map {
                    segment_str = String::from("Map");
                }
                let is_result = segment_str == "Result";

                match &segment.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                        format!("{}_{}",
                                segment_str,
                                args.iter()
                                    .enumerate()
                                    .map(|(i, gen_arg)| match gen_arg {
                                        GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                                            let mangled = Self::mangled_inner_generic_ident_string(path);
                                            if is_map {
                                                format!("{}{}", if i == 0 { "keys_" } else { "values_" }, mangled)
                                            } else if is_result {
                                                format!("{}{}", if i == 0 { "ok_" } else { "err_" }, mangled)
                                            } else {
                                                mangled
                                            }
                                        }
                                        _ => panic!("Unknown generic argument: {}", quote!(#gen_arg)),
                                    })
                                    .collect::<Vec<_>>()
                                    .join("_")
                        )
                    },
                    _ => segment_str,
                }
            })
            .collect::<Vec<_>>()
            .join("_")
    }

    pub fn into_mangled_generic_ident(self) -> Ident {
        format_ident!("{}", Self::mangled_inner_generic_ident_string(self.as_path()))
    }

    pub fn mangled_generic_arguments_types_strings(&self) -> Vec<String> {
        self.mangled_generic_arguments_types()
            .iter()
            .map(|ty| ty.to_token_stream().to_string())
            .collect::<Vec<_>>()
    }

    pub fn mangled_generic_arguments_types(&self) -> Vec<Type> {
        self.as_path()
            .segments
            .iter()
            .flat_map(|PathSegment { arguments, .. }| {
                path_arguments_to_paths(arguments)
                    .into_iter()
                    // .map(Self::mangled_arg_path)
                    .map(Self::from)
                    .map(|arg| arg.as_ffi_type())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl PathConversion {
    pub fn mangled_map_ident(&self, context: &ItemContext) -> Ident {
        match self {
            PathConversion::Primitive(path) =>
                format_mangled_ident!("{}", MANGLE_INNER_PATH_PRESENTER(path, context)),
            PathConversion::Complex(path) =>
                format_mangled_ident!("{}", MANGLE_INNER_PATH_PRESENTER(path, context)),
            PathConversion::Generic(GenericPathConversion::Vec(path)) =>
                format_mangled_ident!("Vec_{}", MANGLE_INNER_PATH_PRESENTER(path, context)),
            PathConversion::Generic(GenericPathConversion::Map(path)) =>
                format_mangled_ident!("Map_{}", MANGLE_INNER_PATH_PRESENTER(path, context)),
            PathConversion::Generic(GenericPathConversion::Result(path)) =>
                format_mangled_ident!("Result_{}", MANGLE_INNER_PATH_PRESENTER(path, context)),
        }
    }

    pub fn mangled_vec_arguments(&self, context: &ItemContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                quote!(#path),
            PathConversion::Generic(GenericPathConversion::Vec(path)) =>
                MANGLE_VEC_ARGUMENTS_PRESENTER(&path.segments.last().unwrap().arguments, context),
            PathConversion::Generic(GenericPathConversion::Map(path)) =>
                MANGLE_MAP_ARGUMENTS_PRESENTER(&path.segments.last().unwrap().arguments, context),
            PathConversion::Generic(GenericPathConversion::Result(path)) =>
                MANGLE_RESULT_ARGUMENTS_PRESENTER(&path.segments.last().unwrap().arguments, context),
        }
    }
}
