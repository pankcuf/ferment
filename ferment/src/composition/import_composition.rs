use proc_macro2::{Ident, TokenStream as TokenStream2};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::{ItemUse, PathSegment, UseName, UsePath, UseTree};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::conversion::ImportConversion;
use crate::ext::Pop;
use crate::holder::PathHolder;

#[derive(Clone)]
pub struct ImportComposition {
    pub ident: Ident,
    pub scope: PathHolder,
}


impl<'a> From<(&'a Ident, &'a PathHolder)> for ImportComposition {
    fn from(value: (&'a Ident, &'a PathHolder)) -> Self {
        Self { ident: value.0.clone(), scope: value.1.clone() }
    }
}

impl std::fmt::Debug for ImportComposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        f.write_str(&self.scope.to_string())?;
        f.write_str("]: ")?;
        f.write_str(&self.ident.to_token_stream().to_string())
    }
}

impl std::fmt::Display for ImportComposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for ImportComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.ident.to_token_stream(), self.scope.to_token_stream()];
        let other_tokens = [other.ident.to_token_stream(), other.scope.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(TokenStream2::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for ImportComposition {}

impl Hash for ImportComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.to_token_stream().to_string().hash(state);
        self.scope.to_token_stream().to_string().hash(state);
    }
}


pub fn create_item_use_with_tree(tree: UseTree) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree,
        semi_token: Default::default(),
    }
}

pub fn create_items_use_from_path(path: &PathHolder) -> ItemUse {
    create_item_use_with_tree(UseTree::Path(UsePath {
        ident: path.0.segments[0].ident.clone(),
        colon2_token: Default::default(),
        tree: Box::new(build_nested_use_tree(path.0.segments.clone().into_iter().skip(1).collect())),
    }))
}
fn build_nested_use_tree(segments: Punctuated<PathSegment, Colon2>) -> UseTree {
    if segments.len() == 1 {
        UseTree::Name(UseName { ident: segments[0].ident.clone() })
    } else {
        UseTree::Path(UsePath {
            ident: segments[0].ident.clone(),
            colon2_token: Default::default(),
            tree: Box::new(build_nested_use_tree(segments.into_iter().skip(1).collect())),
        })
    }
}
impl ImportComposition {
    pub fn present(&self, import_type: &ImportConversion) -> ItemUse {
        // UseTree::
        let path = &self.scope;
        match import_type {
            ImportConversion::External | ImportConversion::Original /*| ImportConversion::FfiType | ImportConversion::FfiExternal*/ => {
                create_items_use_from_path(path)
                // self.scope.clone()
            },
            ImportConversion::ExternalChunk => {
                create_items_use_from_path(&path.popped())
                // self.scope.popped()
            },
            // _ => self.scope.joined(&self.ident)
            _ => create_items_use_from_path(&path.joined(&self.ident))
        }

    }
}
