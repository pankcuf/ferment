use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{parse_quote, Path, PathSegment, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use crate::context::ScopeChain;

#[derive(Clone, Default)]
pub struct ImportResolver {
    pub inner: IndexMap<ScopeChain, IndexMap<Path, Path>>,
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
                    let path = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                    self.inner
                        .entry(scope.clone())
                        .or_default()
                        .insert(parse_quote!(#ident), path);
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

    pub fn maybe_scope_imports(&self, scope: &ScopeChain) -> Option<&IndexMap<Path, Path>> {
        self.inner.get(scope)
    }
    pub fn maybe_path(&self, scope: &ScopeChain, chunk: &Path) -> Option<&Path> {
        self.maybe_scope_imports(scope)
            .and_then(|imports| imports.get(chunk))
    }
    pub fn maybe_import(&self, scope: &ScopeChain, chunk: &Path) -> Option<&Path> {
        // TODO: check parent scope chain lookup validity as we don't need to have infinite recursive lookup
        // so smth like can_have_more_than_one_grandfather,
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } =>
                self.maybe_path(&scope, chunk),
            ScopeChain::Fn { parent, .. } =>
                self.maybe_fn_import(scope, parent, chunk),
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } =>
                self.maybe_obj_or_parent(scope, parent, chunk),
        }
    }



    fn maybe_fn_import(&self, fn_scope: &ScopeChain, parent_scope: &ScopeChain, ident: &Path) -> Option<&Path> {
        self.maybe_path(fn_scope, ident)
            .or_else(|| {
                match parent_scope {
                    ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                        self.maybe_path(parent_scope, ident),
                    ScopeChain::Fn { parent, .. } =>
                        self.maybe_fn_import(parent_scope, parent, ident),
                    ScopeChain::Trait { parent, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or_else(|| {
                                match &**parent {
                                    ScopeChain::CrateRoot { .. } |
                                    ScopeChain::Mod { .. } =>
                                        self.maybe_path(parent, ident),
                                    _ => None,
                                }
                            }),
                    ScopeChain::Object { parent, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or_else(|| match &**parent {
                                ScopeChain::CrateRoot { .. } |
                                ScopeChain::Mod { .. } =>
                                    self.maybe_path(parent, ident),
                                _ => None,
                            }),
                    ScopeChain::Impl { parent, .. } =>
                        self.maybe_path(parent_scope, ident)
                            .or_else(|| match &**parent {
                                ScopeChain::CrateRoot { .. } |
                                ScopeChain::Mod { .. } =>
                                    self.maybe_path(parent, ident),
                                _ => None,
                            }),
                }
            })
    }
    pub fn maybe_obj_or_parent(&self, self_scope: &ScopeChain, parent_chain: &ScopeChain, ident: &Path) -> Option<&Path> {
        self.maybe_path(self_scope, ident)
            .or_else(|| match parent_chain {
                ScopeChain::CrateRoot { .. } |
                ScopeChain::Mod { .. } =>
                    self.maybe_path(parent_chain, ident),
                _ => None,
            })
    }


}