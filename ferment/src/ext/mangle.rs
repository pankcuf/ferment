use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeTraitObject};

#[derive(Copy, Clone)]
pub enum ManglingRules {
    Default // "::" -> "_"
}

pub trait Mangle {
    fn to_mangled_string(&self, rules: ManglingRules) -> String;
    fn to_mangled_ident(&self, rules: ManglingRules) -> Ident {
        format_ident!("{}", self.to_mangled_string(rules))
    }

    fn to_mangled_ident_default(&self) -> Ident {
        format_ident!("{}", self.to_mangled_string(ManglingRules::Default))
    }
}

impl Mangle for Path {
    fn to_mangled_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                self.segments
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
                                                    let mangled = path.to_mangled_string(rules);
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
                                                        let mangled = path.to_mangled_string(rules);
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

        }
    }
}

impl Mangle for Type {
    fn to_mangled_string(&self, rules: ManglingRules) -> String {
        match rules {
            ManglingRules::Default => {
                match self {
                    // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
                    Type::Path(TypePath { path, .. }) =>
                        path.to_mangled_string(rules),
                    ty => {
                        let p: Path = parse_quote!(#ty);
                        p.get_ident().unwrap().clone().to_string()
                    }
                }

            }
        }
    }
}