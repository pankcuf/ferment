use std::fmt;
use std::fmt::Debug;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, PathSegment, Type, TypePath};
use crate::interface::{FFI_GENERIC_TYPE_PRESENTER, MANGLE_INNER_PATH_PRESENTER, MAP_PATH_PRESENTER, PathPresenter, VEC_PATH_PRESENTER};
use crate::helper::path_arguments_to_paths;

macro_rules! format_mangled_ident {
    ($fmt:expr, $path_presentation:expr) => {
        format_ident!($fmt, format!("{}", $path_presentation))
    };
}

pub enum PathConversion {
    Simple(Path),
    Complex(Path),
    Map(Path),
    Vec(Path),
}

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
            | "isize" | "usize" | "bool" => PathConversion::Simple(path.clone()),
            "BTreeMap" | "HashMap" => PathConversion::Map(path.clone()),
            "Vec" => PathConversion::Vec(path.clone()),
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
    pub fn as_ffi_path(&self) -> Path {
        let path = self.as_path();
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
            // vec expanded type
            "Vec" => {
                let ident = format_ident!("{}_FFI", Self::mangled_inner_generic_ident_string(path));
                parse_quote!(#ident)
            }
            // map expanded type
            "BTreeMap" | "HashMap" => {
                let ident = format_ident!("{}_FFI", Self::mangled_inner_generic_ident_string(path));
                parse_quote!(#ident)
            }
            // complex/vec/map generated type
            _ => {
                last_segment.ident = Ident::new(
                    &format!("{}_FFI", last_segment.ident),
                    last_segment.ident.span(),
                );
                let new_segments = cloned_segments
                    .into_iter()
                    .map(|segment| quote_spanned! { segment.span() => #segment })
                    .collect::<Vec<_>>();
                parse_quote!(#(#new_segments)::*)
            }
        }
    }
    pub fn as_ffi_type(&self) -> Type {
        let path = self.as_ffi_path();
        match self {
            PathConversion::Simple(..) => parse_quote!(#path),
            _ => parse_quote!(*mut #path),
        }
    }
}

impl PathConversion {
    pub fn as_path(&self) -> &Path {
        match self {
            PathConversion::Simple(path)
            | PathConversion::Complex(path)
            | PathConversion::Map(path)
            | PathConversion::Vec(path) => path,
        }
    }
    #[allow(unused)]
    pub fn to_mangled_inner_generic_ident_string(&self) -> String {
        Self::mangled_inner_generic_ident_string(self.as_path())
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
                match &segment.arguments {
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                        format!("{}_{}",
                            segment_str,
                            args.iter()
                                .enumerate()
                                .map(|(i, gen_arg)| match gen_arg {
                                    GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                                        let mangled = Self::mangled_inner_generic_ident_string(path);
                                        if is_map {
                                            format!(
                                                "{}{}",
                                                if i == 0 { "keys_" } else { "values_" },
                                                mangled
                                            )
                                        } else {
                                            mangled
                                        }
                                    }
                                    _ => panic!("Unknown generic argument: {}", quote!(#gen_arg)),
                                })
                                .collect::<Vec<_>>()
                                .join("_")
                    ),
                    _ => segment_str,
                }
            })
            .collect::<Vec<_>>()
            .join("_")
    }

    // #[cfg(test)]
    pub fn into_mangled_generic_ident(self) -> Ident {
        format_ident!(
            "{}",
            Self::mangled_inner_generic_ident_string(self.as_path())
        )
    }

    // #[cfg(test)]
    #[allow(unused)]
    pub fn mangled_generic_arguments_types_strings(&self) -> Vec<String> {
        self.mangled_generic_arguments_types()
            .iter()
            .map(|ty| ty.to_token_stream().to_string())
            .collect::<Vec<_>>()
    }

    // #[cfg(test)]
    #[allow(unused)]
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

    #[allow(unused)]
    fn mangled_ident(&self, presenter: PathPresenter) -> Ident {
        match self {
            PathConversion::Simple(path) => format_mangled_ident!("{}", presenter(path)),
            PathConversion::Complex(path) => format_mangled_ident!("{}", presenter(path)),
            PathConversion::Vec(path) => format_mangled_ident!("Vec_{}", presenter(path)),
            PathConversion::Map(path) => format_mangled_ident!("Map_{}", presenter(path)),
        }
    }

    #[allow(unused)]
    pub fn mangled_root_ident(&self) -> Ident {
        self.mangled_ident(match self {
            PathConversion::Vec(..) => FFI_GENERIC_TYPE_PRESENTER,
            PathConversion::Map(..) => FFI_GENERIC_TYPE_PRESENTER,
            _ => unimplemented!("can't mangle type"),
        })
    }

    #[allow(unused)]
    pub fn mangled_map_ident(&self) -> Ident {
        self.mangled_ident(MANGLE_INNER_PATH_PRESENTER)
    }

    #[allow(unused)]
    pub fn mangled_vec_arguments(&self) -> TokenStream2 {
        match self {
            PathConversion::Simple(path) | PathConversion::Complex(path) => {
                quote!(#path)
            }
            PathConversion::Vec(..) => self.mangled_ident(VEC_PATH_PRESENTER).to_token_stream(),
            PathConversion::Map(..) => self.mangled_ident(MAP_PATH_PRESENTER).to_token_stream(),
        }
    }
}

#[allow(unused)]
fn conversion_type_for_path(path: &Path) -> PathConversion {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        // std convertible
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => PathConversion::Simple(path.clone()),
        "BTreeMap" | "HashMap" => PathConversion::Map(path.clone()),
        "Vec" => PathConversion::Vec(path.clone()),
        _ => PathConversion::Complex(path.clone()),
    }
}
