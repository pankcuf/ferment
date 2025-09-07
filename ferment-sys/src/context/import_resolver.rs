use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{parse_quote, Path, PathSegment, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use crate::context::ScopeChain;
use crate::ext::{GenericBoundKey, ToPath};

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
                self.fold_import_tree(scope, tree, current_path);
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
            }
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|use_tree| self.fold_import_tree(scope, use_tree, current_path.clone())),
            _ => {
                // For a glob import, we can't determine the full path statically
                // Just ignore them for now
            }
        }
    }

    pub fn maybe_scope_imports(&self, scope: &ScopeChain) -> Option<&IndexMap<Path, Path>> {
        self.inner.get(scope)
    }
    pub fn maybe_path(&self, scope: &ScopeChain, chunk: &GenericBoundKey) -> Option<&Path> {
        self.maybe_scope_imports(scope)
            .and_then(|imports| match chunk {
                GenericBoundKey::Ident(ident) => imports.get(&ident.to_path()),
                GenericBoundKey::Path(path) => imports.get(path)
            })
    }
    pub fn maybe_import(&self, scope: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        // TODO: check parent scope chain lookup validity as we don't need to have infinite recursive lookup
        // so smth like can_have_more_than_one_grandfather,
        match scope {
            ScopeChain::CrateRoot { .. } |
            ScopeChain::Mod { .. } =>
                self.maybe_path(scope, key),
            ScopeChain::Fn { parent, .. } =>
                self.maybe_fn_import(scope, parent, key),
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } =>
                self.maybe_obj_or_parent(scope, parent, key),
        }
    }

    fn maybe_fn_import(&self, self_scope: &ScopeChain, parent: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        self.maybe_path(self_scope, key)
            .or_else(|| self.maybe_import(parent, key))
    }
    pub fn maybe_obj_or_parent(&self, self_scope: &ScopeChain, parent: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        self.maybe_path(self_scope, key)
            .or_else(|| match parent {
                ScopeChain::CrateRoot { .. } |
                ScopeChain::Mod { .. } =>
                    self.maybe_path(parent, key),
                _ => None,
            })
    }


}