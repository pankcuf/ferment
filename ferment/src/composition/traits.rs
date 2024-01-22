use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::{Ident, Path, Signature, TraitBound, TraitItem, TraitItemMethod, TraitItemType, Type, TypeParamBound};
use syn::__private::TokenStream2;
use crate::formatter::format_trait_decomposition_part1;
use crate::composition::FnSignatureDecomposition;
use crate::context::ScopeContext;
use crate::holder::PathHolder;

#[derive(Clone, Debug)]
pub struct TraitBoundDecomposition {
    pub path: Path,
}

impl ToTokens for TraitBoundDecomposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.path.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub struct TraitTypeDecomposition {
    pub ident: Ident,
    pub trait_bounds: Vec<TraitBoundDecomposition>,
}

impl ToTokens for TraitTypeDecomposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TraitTypeDecomposition { ident, trait_bounds } = self;
        quote!(#ident: #(#trait_bounds),*).to_tokens(tokens)
    }
}

impl TraitTypeDecomposition {
    pub fn from_item_type(item_type: &TraitItemType) -> Self {
        Self {
            ident: item_type.ident.clone(),
            trait_bounds: item_type.bounds.iter()
                .filter_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(TraitBoundDecomposition { path: path.clone() }),
                    TypeParamBound::Lifetime(_lt) =>
                        None
                })
                .collect()
        }
    }
}
// For use in Scope Agnostic Tree
#[derive(Clone, Debug)]
pub struct TraitDecompositionPart1 {
    pub ident: Ident,
    pub consts: HashMap<Ident, Type>,
    pub methods: HashMap<Ident, Signature>,
    pub types: HashMap<Ident, TraitTypeDecomposition>
}

impl Display for TraitDecompositionPart1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("TraitDecompositionPart1({})", format_trait_decomposition_part1(self)).as_str())
    }
}


impl TraitDecompositionPart1 {
    pub fn from_trait_items(ident: &Ident, trait_items: &[TraitItem]) -> Self {
        let mut methods = HashMap::new();
        let mut types = HashMap::new();
        let mut consts = HashMap::new();
        trait_items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(TraitItemMethod { sig, .. } ) => {
                    methods.insert(sig.ident.clone(), sig.clone());
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeDecomposition::from_item_type(trait_item_type));
                },
                TraitItem::Const(trait_item_const) => {
                    consts.insert(trait_item_const.ident.clone(), trait_item_const.ty.clone());
                },
                _ => {}
            });
        TraitDecompositionPart1 { ident: ident.clone(), methods, types, consts }
    }
}

// For use in Full Context Tree
#[derive(Clone, Debug)]
pub struct TraitDecompositionPart2 {
    pub methods: Vec<FnSignatureDecomposition>,
    pub types: HashMap<Ident, TraitTypeDecomposition>,
}

// impl Display for TraitDecompositionPart2 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str()
//     }
// }
// impl TraitDecompositionPart2 {
//     pub fn from_scope_agnostic_tree(decomposition: &TraitDecompositionPart1, scope: &PathHolder, context: &ScopeContext) -> Self {
//         TraitDecompositionPart2 {
//             methods: decomposition.methods
//                 .iter()
//                 .map(|(_ident, sig)| FnSignatureDecomposition::from_signature(sig, scope.clone(), context))
//                 .collect(),
//             types: decomposition.types.clone()
//         }
//     }
// }

impl TraitDecompositionPart2 {
    pub fn from_trait_items(trait_items: &[TraitItem], scope: &PathHolder, context: &ScopeContext) -> Self {
        let mut methods = vec![];
        let mut types = HashMap::new();
        trait_items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(TraitItemMethod { sig, .. } ) => {
                    methods.push(FnSignatureDecomposition::from_signature(sig, scope.clone(), context));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeDecomposition::from_item_type(trait_item_type));
                },
                _ => {}
            });
        TraitDecompositionPart2 { methods, types }
    }

    pub fn present_trait_vtable_inner_functions(self, context: &ScopeContext) -> Vec<TokenStream2> {
        self.methods.into_iter().map(|m| FnSignatureDecomposition::present_trait_vtable_inner_fn(m, context)).collect()
    }
}
