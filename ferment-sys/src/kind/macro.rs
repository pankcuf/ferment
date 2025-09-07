use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::{Attribute, AttrStyle, Item, Lit, Meta, MetaList, parse_quote, Expr, ExprLit, MacroDelimiter, Type};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::MaybeMacroLabeled;
use crate::ext::MaybeAttrs;

#[derive(Debug)]
pub enum MacroKind {
    Export,
    Opaque,
    Register(Type)
}

// #[allow(unused)]
// pub fn non_cfg_test(attrs: &Vec<Attribute>) -> bool {
//     !cfg_test(attrs)
// }
// #[allow(unused)]
// pub fn cfg_test(attrs: &Vec<Attribute>) -> bool {
//     let result = attrs.iter().any(|attr| {
//         if attr.path().is_ident("cfg") {
//             if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
//                 meta_list.nested
//                     .iter()
//                     .any(|nested| matches!(nested, NestedMeta::Meta(Meta::Path(path)) if path.is_ident("test")))
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     });
//     result
// }

impl TryFrom<&Item> for MacroKind {
    type Error = ();

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value.maybe_attrs()
            .and_then(|attrs| attrs.iter().find_map(MaybeMacroLabeled::maybe_macro_labeled)) {
                Some(macro_type) => Ok(macro_type),
                None => Err(())
            }
    }
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
                    eq_token: Default::default(),
                    value: Expr::Lit(ExprLit { attrs: vec![], lit: Lit::Str(syn::LitStr::new(feat, proc_macro2::Span::call_site())) }),
                })
            }
            CfgMacroType::Test => {
                Meta::Path(syn::Path::from(syn::Ident::new("test", proc_macro2::Span::call_site())))
            }
            CfgMacroType::Not(cond) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("not", proc_macro2::Span::call_site())),
                    delimiter: MacroDelimiter::Paren(Paren::default()),
                    tokens: cond.to_meta().to_token_stream(),
                })
            }
            CfgMacroType::Any(conds) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("any", proc_macro2::Span::call_site())),
                    delimiter: MacroDelimiter::Paren(Paren::default()),
                    tokens: Depunctuated::from_iter(conds.iter().map(|c| c.to_meta())).to_token_stream(),
                })
            }
            CfgMacroType::All(conds) => {
                Meta::List(MetaList {
                    path: syn::Path::from(syn::Ident::new("all", proc_macro2::Span::call_site())),
                    delimiter: MacroDelimiter::Paren(Paren::default()),
                    tokens: Depunctuated::from_iter(conds.iter().map(|c| c.to_meta())).to_token_stream(),
                })
            }
        }
    }

    fn from_meta(meta: &Meta) -> Vec<Self> {
        if let Meta::List(MetaList { path, tokens, .. }) = meta {
            let parsed = CommaPunctuated::<Meta>::parse_terminated.parse2(tokens.clone()).unwrap_or_default();

            if path.is_ident("any") {
                return vec![CfgMacroType::Any(
                    parsed.iter()
                        .flat_map(CfgMacroType::from_meta)
                        .collect()
                )];
            }
            if path.is_ident("all") {
                return vec![CfgMacroType::All(
                    parsed.iter()
                        .flat_map(CfgMacroType::from_meta)
                        .collect()
                )];
            }
            if path.is_ident("not") {
                return parsed.iter()
                    .flat_map(CfgMacroType::from_meta)
                    .map(|c| CfgMacroType::Not(Box::new(c)))
                    .collect();
            }
        }

        if let Meta::NameValue(nv) = meta {
            if nv.path.is_ident("feature") {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                    return vec![CfgMacroType::Feature(s.value())];
                }
            }
        }

        if let Meta::Path(path) = meta {
            if path.is_ident("test") {
                return vec![CfgMacroType::Test];
            }
        }

        vec![]
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
pub fn expand_attributes(attrs: &HashSet<Option<Attribute>>) -> Vec<Attribute> {
    let merged = merge_attributes(attrs);
    if merged.is_empty() {
        return vec![];
    }
    vec![Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: parse_quote!(cfg),
            delimiter: MacroDelimiter::Paren(Default::default()),
            tokens: quote!(#merged),
        }),
    }]
}
pub fn merge_attributes(attrs: &HashSet<Option<Attribute>>) -> CommaPunctuated<Meta> {
    if attrs.contains(&None) {
        Punctuated::new()
    } else {
        let mut all_conditions = vec![];

        for attr in attrs {
            if let Some(attr) = attr {
                if attr.path().is_ident("cfg") {
                    match &attr.meta {
                        Meta::List(meta_list) => {
                            if let Ok(parsed) = CommaPunctuated::<Meta>::parse_terminated.parse2(meta_list.tokens.clone()) {
                                for meta in parsed {
                                    all_conditions.extend(CfgMacroType::from_meta(&meta));
                                }
                            }
                        }
                        meta => {
                            all_conditions.extend(CfgMacroType::from_meta(&meta));
                        }
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
