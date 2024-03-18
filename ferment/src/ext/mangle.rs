use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

#[derive(Copy, Clone)]
pub enum ManglingRules {
    Default, // "::" -> "_"
}

pub trait Mangle {
    fn mangle_string(&self, rules: ManglingRules) -> String;
    fn mangle_ident(&self, rules: ManglingRules) -> Ident {
        format_ident!("{}", self.mangle_string(rules))
    }

    fn mangle_ident_default(&self) -> Ident {
        format_ident!("{}", self.mangle_string(ManglingRules::Default))
    }
}

impl Mangle for Path {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default =>
                self.segments.mangle_string(rules),
        }
    }
}

impl Mangle for TypeTuple {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                format!("Tuple_{}", self.elems.iter().map(|ty| ty.mangle_string(rules)).collect::<Vec<String>>().join("_"))
            }
        }
    }
}

impl Mangle for TraitBound {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                format!("dyn_trait_{}", self.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("_"))
            }
        }
    }
}

impl Mangle for TypeTraitObject {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                // TODO: need mixins impl to process multiple bounds
                self.bounds.iter().find_map(|b| match b {
                    TypeParamBound::Trait(trait_bound) => Some(trait_bound.mangle_string(rules)),
                    TypeParamBound::Lifetime(_) => None,
                })
            }
        }.unwrap_or(format!("Any"))
    }
}

impl Mangle for Type {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                match self {
                    // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
                    Type::Path(TypePath { path, .. }) =>
                        path.mangle_string(rules),
                    Type::Tuple(type_tuple) => type_tuple.mangle_string(rules),
                    ty => {
                        let p: Path = parse_quote!(#ty);
                        p.get_ident().unwrap().clone().to_string()
                    }
                }
            }
        }
    }
}

impl Mangle for Punctuated<PathSegment, Colon2> {
    fn mangle_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default =>
                self
                    .iter()
                    .map(|segment| {
                        let mut segment_str = segment.ident.to_string();
                        let is_map = matches!(segment_str.as_str(), "BTreeMap" | "HashMap");
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
                                                    let mangled = path.mangle_string(rules);
                                                    Some(if is_map {
                                                        format!("{}{}", if i == 0 { "keys_" } else { "values_" }, mangled)
                                                    } else if is_result {
                                                        format!("{}{}", if i == 0 { "ok_" } else { "err_" }, mangled)
                                                    } else {
                                                        mangled
                                                    })
                                                },
                                                GenericArgument::Type(Type::Tuple(type_tuple)) =>
                                                    Some(type_tuple.mangle_string(rules)),
                                                GenericArgument::Type(Type::TraitObject(type_trait_object)) => {
                                                    Some(type_trait_object.mangle_string(rules))
                                                },
                                                GenericArgument::Type(Type::Array(TypeArray { elem, len, .. })) => {
                                                    if let Type::Path(TypePath { path, .. }) = &**elem {
                                                        let mangled = path.mangle_string(rules);
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
                    .join("_"),
        }
    }
}