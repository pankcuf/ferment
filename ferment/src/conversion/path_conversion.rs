use std::fmt;
use std::fmt::Debug;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, PathArguments};
use crate::conversion::GenericPathConversion;

#[derive(Clone, Eq)]
pub enum PathConversion {
    Primitive(Path),
    Complex(Path),
    Generic(GenericPathConversion),
}

impl Debug for PathConversion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PathConversion")?;
        f.debug_list()
            .entries(self.to_token_stream())
            .finish()
    }
}

impl PartialEq for PathConversion {
    fn eq(&self, other: &PathConversion) -> bool {
        self.to_token_stream().to_string() == other.to_token_stream().to_string()
    }
}
impl From<Path> for PathConversion {
    fn from(path: Path) -> Self {
        Self::from(&path)
    }
}
impl ToTokens for PathConversion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) => path.to_tokens(tokens),
            PathConversion::Generic(generic) => generic.to_tokens(tokens),
        }
    }
}

impl From<&Path> for PathConversion {
    fn from(path: &Path) -> Self {
        let last_segment = path.segments.last().unwrap();
        match &last_segment.arguments {
            PathArguments::AngleBracketed(_) => {
                match last_segment.ident.to_string().as_str() {
                    "Box" => PathConversion::Generic(GenericPathConversion::Box(path.clone())),
                    "Arc" => PathConversion::Generic(GenericPathConversion::AnyOther(path.clone())),
                    "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
                    "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
                    "Result" if path.segments.len() == 1 => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
                    _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                        PathArguments::AngleBracketed(_) =>
                            Some(PathConversion::Generic(GenericPathConversion::AnyOther(path.clone()))),
                        _ => None
                    }).unwrap_or(PathConversion::Complex(path.clone()))
                }
            },
            _ => match last_segment.ident.to_string().as_str() {
                // std convertible
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128"
                | "isize" | "usize" | "bool" => PathConversion::Primitive(path.clone()),
                "Box" => PathConversion::Generic(GenericPathConversion::Box(path.clone())),
                "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
                "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
                "Result" if path.segments.len() == 1 => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
                _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                    PathArguments::AngleBracketed(_) =>
                        Some(PathConversion::Generic(GenericPathConversion::AnyOther(path.clone()))),
                    _ => None
                }).unwrap_or(PathConversion::Complex(path.clone())),
            }
        }
    }
}

// impl ScopeContextPresentable for PathConversion {
//     type Presentation = Type;
//
//     fn present(&self, source: &ScopeContext) -> Self::Presentation {
//         match self {
//             PathConversion::Primitive(path) =>
//                 parse_quote!(#path),
//             PathConversion::Complex(path) =>
//             // .joined_mut() for Map/Vec
//                 source.ffi_path_converted_or_same(path),
//             PathConversion::Generic(path_conversion) =>
//             // .joined_mut() for Map/Vec
//                 path_conversion.to_ffi_path()
//         }
//     }
// }









