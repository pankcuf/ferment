use std::fmt;
use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, PathArguments, Type};
use crate::context::ScopeContext;
use crate::conversion::GenericPathConversion;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone)]
pub enum PathConversion {
    Primitive(Path),
    Complex(Path),
    Generic(GenericPathConversion),
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

impl From<Path> for PathConversion {
    fn from(path: Path) -> Self {
        Self::from(&path)
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

impl PathConversion {
    pub fn as_path(&self) -> &Path {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) |
            PathConversion::Generic(GenericPathConversion::Map(path)) |
            PathConversion::Generic(GenericPathConversion::Vec(path)) |
            PathConversion::Generic(GenericPathConversion::Result(path)) |
            PathConversion::Generic(GenericPathConversion::Box(path)) |
            PathConversion::Generic(GenericPathConversion::AnyOther(path)) => path
        }
    }
}

impl ScopeContextPresentable for PathConversion {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            PathConversion::Primitive(path) =>
                parse_quote!(#path),
            PathConversion::Complex(path) =>
            // .joined_mut() for Map/Vec
                source.ffi_path_converted_or_same(path),
            PathConversion::Generic(path_conversion) =>
            // .joined_mut() for Map/Vec
                path_conversion.to_ffi_path()
        }
    }
}