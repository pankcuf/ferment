use std::fmt;
use std::fmt::Debug;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, parse_quote, Path, PathArguments, Type, TypePath};
use crate::generic_path_conversion::GenericPathConversion;
use crate::idents::{convert_to_ffi_path, ffi_path_converted_or_same, path_conversion_from_path};
use crate::item_conversion::ItemContext;

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
        path_conversion_from_path(path)
    }
}



impl PathConversion {

    pub fn as_generic_arg_type(&self) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) => quote!(#path),
            PathConversion::Complex(path) => ffi_path_converted_or_same(path).to_token_stream(),
            PathConversion::Generic(conversion) => convert_to_ffi_path(conversion.as_path()).to_token_stream()
        }
    }


    pub fn as_ffi_type(&self) -> Type {
        let path = self.as_path();
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

    pub fn mangled_map_ident(&self, context: &ItemContext) -> Ident {
        format_ident!("{}", match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("_"),
            PathConversion::Generic(generic_path_conversion) =>
                format!("{}_{}", generic_path_conversion.prefix(), generic_path_conversion.arguments_presentation(context))
        })
    }

    pub fn mangled_vec_arguments(&self, context: &ItemContext) -> TokenStream2 {
        match self {
            PathConversion::Primitive(path) |
            PathConversion::Complex(path) =>
                quote!(#path),
            PathConversion::Generic(conversion) =>
                conversion.arguments_presentation(context)
        }
    }
}