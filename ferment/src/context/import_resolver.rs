use std::collections::{HashMap, HashSet};
use proc_macro2::Ident;
use syn::{Item, parse_quote, Path, PathSegment, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use crate::composition::ImportComposition;
use crate::context::ScopeChain;
use crate::conversion::ImportConversion;
use crate::helper::ItemExtension;
use crate::holder::PathHolder;

// pub trait Maybe<T> {
//     type Context;
//     fn maybe(&self, context: &Self::Context) -> Option<T>;
// }

#[derive(Clone, Default)]
pub struct ImportResolver {
    pub inner: HashMap<ScopeChain, HashMap<PathHolder, Path>>,
}

impl ImportResolver {
    /// Recursively processes Rust use paths to create a mapping
    /// between idents and their fully qualified paths.
    pub(crate) fn fold_import_tree(&mut self, scope: &ScopeChain, use_tree: &UseTree, mut current_path: Vec<Ident>) {
        match use_tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                current_path.push(ident.clone());
                self.fold_import_tree(scope,tree, current_path);
            },
            UseTree::Name(UseName { ident, .. }) |
            UseTree::Rename(UseRename { rename: ident, .. }) => {
                if ident != "_" {
                    current_path.push(ident.clone());
                    // println!("ident: {}", ident);
                    self.inner
                        .entry(scope.clone())
                        .or_default()
                        .insert(parse_quote!(#ident), Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) });
                }
            },
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|use_tree| self.fold_import_tree(scope, use_tree,current_path.clone())),
            UseTree::Glob(_) => {
                // For a glob import, we can't determine the full path statically
                // Just ignore them for now
            }
        }
    }

    pub fn find_used_imports(&self, item: &Item, scope: &ScopeChain) -> Option<HashMap<ImportConversion, HashSet<ImportComposition>>> {
        self.inner
            .get(scope)
            .map(|imports| item.get_used_imports(imports))
    }

    pub fn maybe_scope_imports(&self, scope: &ScopeChain) -> Option<&HashMap<PathHolder, Path>> {
        let result = self.inner.get(scope);
        // println!("maybe_scope_imports: {}:\n{}", scope, scope_imports_dict(&self.inner).join("\n"));
        // if let Some(result) = result {
        //     println!("maybe_scope_imports.result: {}: \n{}", scope, imports_dict(result).join("\n"));
        // }
        result
    }
    pub fn maybe_path(&self, scope: &ScopeChain, chunk: &PathHolder) -> Option<&Path> {
        // println!("maybe_path: {} in [{}]", ident, scope);
        self.maybe_scope_imports(scope)
            .and_then(|imports| imports.get(chunk))
    }
    pub fn maybe_import(&self, scope: &ScopeChain, chunk: &PathHolder) -> Option<&Path> {
        // TODO: check parent scope chain lookup validity as we don't need to have infinite recursive lookup
        // so smth like can_have_more_than_one_grandfather,
        // println!("maybe_import: {} in {}", ident, scope);
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } =>
                self.maybe_path(&scope, chunk),
            ScopeChain::Fn { parent_scope_chain, .. } =>
                self.maybe_fn_import(scope, parent_scope_chain, chunk),
            ScopeChain::Trait { parent_scope_chain, .. } |
            ScopeChain::Object { parent_scope_chain, .. } |
            ScopeChain::Impl { parent_scope_chain, .. } =>
                self.maybe_obj_or_parent(scope, parent_scope_chain, chunk),
        }
    }



    fn maybe_fn_import(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        //println!("maybe_fn_import (fn level): {}", ident);
        self.maybe_path(fn_scope, ident)
            .or({
                match parent_scope {
                    ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                        self.maybe_path(parent_scope, ident),
                    ScopeChain::Fn { parent_scope_chain, .. } =>
                        self.maybe_fn_import(parent_scope, parent_scope_chain, ident),
                    ScopeChain::Trait { parent_scope_chain, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or({
                                //println!("maybe_fn_import (Trait Fn has not imports here): {}", ident);
                                match &**parent_scope_chain {
                                    ScopeChain::CrateRoot { .. } |
                                    ScopeChain::Mod { .. } =>
                                        self.maybe_path(parent_scope_chain, ident),
                                    _ => None,
                                }
                                // if let ScopeChain::Fn { parent_scope_chain: inner_fn_parent_scope_chain, .. } = &**parent_scope_chain {
                                //     self.maybe_fn_import(parent_scope_chain, inner_fn_parent_scope_chain, ident)
                                // } else {
                                //     self.maybe_path(parent_scope, ident)
                                // }
                            }),
                    ScopeChain::Object { parent_scope_chain, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or(match &**parent_scope_chain {
                                ScopeChain::CrateRoot { .. } |
                                ScopeChain::Mod { .. } =>
                                    self.maybe_path(parent_scope_chain, ident),
                                _ => None,
                            }),
                    ScopeChain::Impl { parent_scope_chain, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or(match &**parent_scope_chain {
                                ScopeChain::CrateRoot { .. } |
                                ScopeChain::Mod { .. } =>
                                    self.maybe_path(parent_scope_chain, ident),
                                _ => None,
                            }),
                }
            })
    }
    pub fn maybe_obj_or_parent(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ident: &PathHolder) -> Option<&Path> {
        self.maybe_path(self_scope, ident)
            .or(match parent_chain {
                ScopeChain::CrateRoot { .. } |
                ScopeChain::Mod { .. } =>
                    self.maybe_path(parent_chain, ident),
                _ => None,
            })
    }


}