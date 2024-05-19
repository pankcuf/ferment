use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use syn::{Attribute, Item, Lit, Meta, MetaList, NestedMeta, parse_quote, Path};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::helper::ItemExtension;
use crate::holder::TypeHolder;

pub enum MacroType {
    Export,
    Register(TypeHolder)
}
#[allow(unused)]
pub fn non_cfg_test(attrs: &Vec<Attribute>) -> bool {
    !cfg_test(attrs)
}
#[allow(unused)]
pub fn cfg_test(attrs: &Vec<Attribute>) -> bool {
    let result = attrs.iter().any(|attr| {
        if attr.path.is_ident("cfg") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested
                    .iter()
                    .any(|nested| matches!(nested, NestedMeta::Meta(Meta::Path(path)) if path.is_ident("test")))
            } else {
                false
            }
        } else {
            false
        }
    });
    result
}

impl TryFrom<&Item> for MacroType {
    type Error = ();

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value.maybe_attrs()
            .and_then(|attrs| attrs.iter().find_map(|attr| {
                let path = &attr.path;
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path.clone());
                        }
                    });
                }
                match path.segments.last().unwrap().ident.to_string().as_str() {
                    "export" =>
                        Some(MacroType::Export),
                    "register" => {
                        let first_path = arguments.first().unwrap();
                        Some(MacroType::Register(parse_quote!(#first_path)))
                    },
                    _ =>
                        None
                }
            })) {
                Some(macro_type) => Ok(macro_type),
                None => Err(())
            }
    }
}


pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum CfgMacroType {
    Feature(String),
    Test,
    Not(Box<CfgMacroType>),
    Any(Vec<CfgMacroType>),
    All(Vec<CfgMacroType>),
}

impl Display for CfgMacroType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CfgMacroType::Feature(feat) =>
                format!("feature = \"{}\"", feat),
            CfgMacroType::Test => "test".to_string(),
            CfgMacroType::Not(cond) =>
                format!("not({})", cond.to_string()),
            CfgMacroType::Any(conds) =>
                format!("any({})", conds.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")),
            CfgMacroType::All(conds) =>
                format!("all({})", conds.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")),
        }.as_str())
    }
}

impl CfgMacroType {
    fn to_meta(&self) -> Meta {
        match self {
            CfgMacroType::Feature(feat) => {
                Meta::NameValue(syn::MetaNameValue {
                    path: syn::Path::from(syn::Ident::new("feature", proc_macro2::Span::call_site())),
                    lit: Lit::Str(syn::LitStr::new(feat, proc_macro2::Span::call_site())),
                    eq_token: Default::default(),
                })
            }
            CfgMacroType::Test => {
                Meta::Path(syn::Path::from(syn::Ident::new("test", proc_macro2::Span::call_site())))
            }
            CfgMacroType::Not(cond) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("not", proc_macro2::Span::call_site())),
                    paren_token: Default::default(),
                    nested: vec![NestedMeta::Meta(cond.to_meta())].into_iter().collect(),
                })
            }
            CfgMacroType::Any(conds) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("any", proc_macro2::Span::call_site())),
                    paren_token: Default::default(),
                    nested: conds.iter().map(|c| NestedMeta::Meta(c.to_meta())).collect(),
                })
            }
            CfgMacroType::All(conds) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("all", proc_macro2::Span::call_site())),
                    paren_token: Default::default(),
                    nested: conds.iter().map(|c| NestedMeta::Meta(c.to_meta())).collect(),
                })
            }
        }
    }

    fn from_meta(meta: &Meta) -> Vec<Self> {
        match meta {
            Meta::List(MetaList { path, nested, .. }) if path.is_ident("any") => {
                vec![CfgMacroType::Any(
                    nested.iter().flat_map(|nested_meta| match nested_meta {
                        NestedMeta::Meta(meta) => CfgMacroType::from_meta(meta),
                        _ => vec![],
                    }).collect()
                )]
            }
            Meta::List(MetaList { path, nested, .. }) if path.is_ident("all") => {
                vec![CfgMacroType::All(
                    nested.iter().flat_map(|nested_meta| match nested_meta {
                        NestedMeta::Meta(meta) => CfgMacroType::from_meta(meta),
                        _ => vec![],
                    }).collect()
                )]
            }
            Meta::List(MetaList { path, nested, .. }) if path.is_ident("not") => {
                nested.iter().flat_map(|nested_meta| match nested_meta {
                    NestedMeta::Meta(meta) => CfgMacroType::from_meta(meta).into_iter().map(|o| CfgMacroType::Not(o.into())).collect(),
                    _ => vec![],
                }).collect()
            }
            Meta::NameValue(syn::MetaNameValue { path, lit, .. }) if path.is_ident("feature") => {
                if let Lit::Str(lit_str) = lit {
                    vec![CfgMacroType::Feature(lit_str.value())]
                } else {
                    vec![]
                }
            }
            Meta::Path(path) if path.is_ident("test") => {
                vec![CfgMacroType::Test]
            }
            _ => vec![],
        }
    }
}
fn merge_cfg_conditions(conditions: Vec<CfgMacroType>) -> Vec<CfgMacroType> {
    let mut features = HashSet::new();
    let mut tests = false;
    let mut not_conditions = vec![];
    let mut any_conditions = vec![];
    let mut all_conditions = vec![];

    for condition in conditions {
        match condition {
            CfgMacroType::Feature(feature) => {
                features.insert(feature);
            }
            CfgMacroType::Test => {
                tests = true;
            }
            CfgMacroType::Not(cond) => {
                not_conditions.push(*cond);
            }
            CfgMacroType::Any(conds) => {
                any_conditions.extend(conds);
            }
            CfgMacroType::All(conds) => {
                all_conditions.extend(conds);
            }
        }
    }

    if !features.is_empty() {
        any_conditions.push(CfgMacroType::Any(features.into_iter().map(CfgMacroType::Feature).collect()));
    }
    if tests {
        any_conditions.push(CfgMacroType::Test);
    }
    if !not_conditions.is_empty() {
        all_conditions.push(CfgMacroType::All(not_conditions));
    }

    if any_conditions.is_empty() {
        all_conditions
    } else {
        vec![CfgMacroType::Any(any_conditions)]
    }
}
pub fn merge_attributes(attrs: &HashSet<Option<Attribute>>) -> Punctuated<Meta, Comma> {
    if attrs.contains(&None) {
        Punctuated::new()
    } else {
        let mut all_conditions = vec![];
        for attr in attrs {
            if let Some(attr) = &attr {
                if attr.path.is_ident("cfg") {
                    match attr.parse_meta() {
                        Ok(Meta::List(meta_list)) => {
                            meta_list.nested.iter().for_each(|meta| {
                                if let NestedMeta::Meta(meta) = meta {
                                    all_conditions.extend(CfgMacroType::from_meta(meta));
                                }
                            });
                        },
                        Ok(meta) => {
                            all_conditions.extend(CfgMacroType::from_meta(&meta));

                        },
                        _ => {}
                    }
                }
            }
        }
        merge_cfg_conditions(all_conditions)
            .iter()
            .map(CfgMacroType::to_meta)
            .collect()
    }
}
