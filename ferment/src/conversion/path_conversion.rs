use std::fmt;
use std::fmt::Debug;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, Path, PathArguments, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeTraitObject};
use crate::context::ScopeContext;
use crate::conversion::GenericPathConversion;

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
        println!("path_conversion_from_path: {}", path.to_token_stream());

        match &last_segment.arguments {
            PathArguments::AngleBracketed(args) => {
                match last_segment.ident.to_string().as_str() {
                    "Box" => PathConversion::Generic(GenericPathConversion::Box(path.clone())),
                    "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
                    "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
                    "Result" if path.segments.len() == 1 => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
                    _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                        PathArguments::AngleBracketed(args) =>
                            Some(PathConversion::Generic(GenericPathConversion::AnyOther(path.clone()))),
                        _ => None
                    }).unwrap_or(PathConversion::Complex(path.clone()))
                }

            },
            _ => match last_segment.ident.to_string().as_str() {
                // std convertible
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
                | "isize" | "usize" | "bool" => PathConversion::Primitive(path.clone()),
                "Box" => PathConversion::Generic(GenericPathConversion::Box(path.clone())),
                "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
                "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
                "Result" if path.segments.len() == 1 => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
                _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                    PathArguments::AngleBracketed(args) =>
                        Some(PathConversion::Generic(GenericPathConversion::AnyOther(path.clone()))),
                    _ => None
                }).unwrap_or(PathConversion::Complex(path.clone())),
            }

        }



        // match last_segment.ident.to_string().as_str() {
        //     // std convertible
        //     "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
        //     | "isize" | "usize" | "bool" => PathConversion::Primitive(path.clone()),
        //     "Box" => PathConversion::Generic(GenericPathConversion::Box(path.clone())),
        //     "BTreeMap" | "HashMap" => PathConversion::Generic(GenericPathConversion::Map(path.clone())),
        //     "Vec" => PathConversion::Generic(GenericPathConversion::Vec(path.clone())),
        //     "Result" if path.segments.len() == 1 => PathConversion::Generic(GenericPathConversion::Result(path.clone())),
        //     _ => path.segments.iter().find_map(|ff| match &ff.arguments {
        //         PathArguments::AngleBracketed(args) =>
        //             Some(PathConversion::Generic(GenericPathConversion::AnyOther(path.clone()))),
        //             _ => None
        //     }).unwrap_or(PathConversion::Complex(path.clone())),
        // }

    }
}

impl PathConversion {

    #[cfg(test)]
    pub fn as_generic_arg_type(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) =>
                quote!(#path),
            PathConversion::Complex(path) =>
                context.ffi_path_converted_or_same(path)
                    .to_token_stream(),
            PathConversion::Generic(conversion) =>
                context.convert_to_ffi_path(conversion)
                    .to_token_stream()
        }
    }


    // #[cfg(test)]
    // pub fn as_ffi_type(&self) -> Type {
    //     let path = self.as_path();
    //     match self {
    //         PathConversion::Primitive(..) => parse_quote!(#path),
    //         _ => parse_quote!(*mut #path),
    //     }
    // }

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
                                    .filter_map(|(i, gen_arg)| match gen_arg {
                                        GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                                            let mangled = Self::mangled_inner_generic_ident_string(path);
                                            Some(if is_map {
                                                format!("{}{}", if i == 0 { "keys_" } else { "values_" }, mangled)
                                            } else if is_result {
                                                format!("{}{}", if i == 0 { "ok_" } else { "err_" }, mangled)
                                            } else {
                                                mangled
                                            })
                                        },
                                        GenericArgument::Type(Type::TraitObject(TypeTraitObject { dyn_token: _, bounds })) => {
                                            // TODO: need mixins impl to process multiple bounds
                                            bounds.iter().find_map(|b| match b {
                                                TypeParamBound::Trait(TraitBound { paren_token: _, modifier: _, lifetimes: _, path }) =>
                                                    Some(format!("dyn_trait_{}", path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("_"))),
                                                TypeParamBound::Lifetime(_) => None,
                                            })
                                        },
                                        GenericArgument::Type(Type::Array(TypeArray { elem, len, .. })) => {
                                            if let Type::Path(TypePath { path, .. }) = &**elem {
                                                let mangled = Self::mangled_inner_generic_ident_string(path);
                                                Some(if is_map {
                                                    format!("{}{}{}_{}", if i == 0 { "keys_" } else { "values_" }, "arr_", mangled, quote!(#len).to_string())
                                                } else if is_result {
                                                    format!("{}{}{}_{}", if i == 0 { "ok_" } else { "err_" }, "arr_", mangled, quote!(#len).to_string())
                                                } else {
                                                    mangled
                                                })
                                            } else {
                                                None
                                            }
                                            // format!("arr_{}_count", Self::mangled_inner_generic_ident_string(elem))
                                        },
                                        _ => {
                                            None
                                            // panic!("Unknown generic argument: {}", quote!(#gen_arg))
                                        },
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

    pub fn mangled_map_ident(&self, context: &ScopeContext) -> Ident {
        format_ident!("{}", match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("_"),
            PathConversion::Generic(generic_path_conversion) =>
                format!("{}_{}", generic_path_conversion.prefix(), generic_path_conversion.arguments_presentation(context))
        })
    }

    pub fn mangled_vec_arguments(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                quote!(#path),
            PathConversion::Generic(conversion) =>
                conversion.arguments_presentation(context)
        }
    }
    pub fn mangled_box_arguments(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                quote!(#path),
            PathConversion::Generic(conversion) =>
                conversion.arguments_presentation(context)
        }

    }
}