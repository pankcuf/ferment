use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;

#[derive(Default, Copy, Clone)]
pub struct MangleDefault; // "::" -> "_"

pub trait Mangle<T: Clone> {
    fn mangle_string(&self, context: T) -> String;
    fn mangle_ident(&self, context: T) -> Ident {
        format_ident!("{}", self.mangle_string(context))
    }
    fn mangle_string_default(&self) -> String where T: Default {
        self.mangle_string(T::default())
    }
    fn mangle_ident_default(&self) -> Ident where T: Default {
        format_ident!("{}", self.mangle_string(T::default()))
    }
}

impl<T, SEP, CTX> Mangle<T> for Punctuated<CTX, SEP>  where T: Clone + Copy + Default, CTX: Mangle<T> {
    fn mangle_string(&self, context: T) -> String {
        self.iter()
            .map(|item| item.mangle_string(context))
            .collect::<Vec<_>>()
            .join("_")
    }
}
impl Mangle<MangleDefault> for Type {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
            Type::Path(TypePath { path, .. }) =>
                path.mangle_string(context),
            Type::Tuple(type_tuple) =>
                type_tuple.mangle_string(context),
            ty => {
                let p: Path = parse_quote!(#ty);
                p.get_ident().unwrap().clone().to_string()
            }
        }
    }
}

impl Mangle<MangleDefault> for Path {
    fn mangle_string(&self, context: MangleDefault) -> String {
        self.segments.mangle_string(context)
    }
}

impl Mangle<MangleDefault> for TraitBound {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("dyn_trait_{}", self.path.segments.mangle_string(context))
    }
}

impl Mangle<MangleDefault> for TypeTuple {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("Tuple_{}", self.elems.mangle_string(context))
    }
}

impl Mangle<MangleDefault> for TypeTraitObject {
    fn mangle_string(&self, context: MangleDefault) -> String {
        // TODO: need mixins impl to process multiple bounds
        self.bounds.iter().find_map(|b| match b {
            TypeParamBound::Trait(trait_bound) => Some(trait_bound.mangle_string(context)),
            TypeParamBound::Lifetime(_) => None,
        }).unwrap_or(format!("Any"))
    }
}


impl Mangle<((bool, bool), usize)> for TypePath {
    fn mangle_string(&self, context: ((bool, bool), usize)) -> String {
        let ((is_map, is_result), i) = context;
        let mangled = self.path.mangle_string_default();
        if is_map {
            format!("{}{}", if i == 0 { "keys_" } else { "values_" }, mangled)
        } else if is_result {
            format!("{}{}", if i == 0 { "ok_" } else { "err_" }, mangled)
        } else {
            mangled
        }

    }
}

impl Mangle<((bool, bool), usize)> for TypeArray {
    fn mangle_string(&self, context: ((bool, bool), usize)) -> String {
        let ((is_map, is_result), ..) = context;
        if let Type::Path(type_path) = &*self.elem {
            let mangled_type_path = type_path.mangle_string(context);
            if is_map || is_result {
                format!("{mangled_type_path}_arr_{}", self.len.to_token_stream().to_string())
            } else {
                mangled_type_path
            }
        } else {
            String::default()
        }
    }
}

impl Mangle<String> for PathArguments {
    fn mangle_string(&self, context: String) -> String {
        let mut segment_str = context.clone();
        let is_map = matches!(segment_str.as_str(), "BTreeMap" | "HashMap");
        if is_map {
            segment_str = String::from("Map");
        }
        let is_result = segment_str == "Result";
        match self {
            PathArguments::AngleBracketed(arguments) =>
                format!("{}_{}", segment_str, arguments.mangle_string((is_map, is_result))),
            _ => segment_str,
        }
    }
}

impl Mangle<MangleDefault> for PathSegment {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        self.arguments.mangle_string(self.ident.to_string())
    }
}

impl Mangle<(bool, bool)> for AngleBracketedGenericArguments {
    fn mangle_string(&self, context: (bool, bool)) -> String {
        self.args.iter()
            .enumerate()
            .filter_map(|(i, gen_arg)| match gen_arg {
                GenericArgument::Type(Type::Path(type_path)) =>
                    Some(type_path.mangle_string((context, i))),
                GenericArgument::Type(Type::Array(type_array)) =>
                    Some(type_array.mangle_string((context, i))),
                GenericArgument::Type(Type::Tuple(type_tuple)) =>
                    Some(type_tuple.mangle_string_default()),
                GenericArgument::Type(Type::TraitObject(type_trait_object)) =>
                    Some(type_trait_object.mangle_string_default()),
                _ => None
            })
            .collect::<Vec<_>>()
            .join("_")
    }
}

