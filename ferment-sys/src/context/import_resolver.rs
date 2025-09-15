use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{Path, PathSegment, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use crate::context::{ScopeChain, ScopeResolver};
use crate::ext::{GenericBoundKey, ToPath};
use crate::ext::{Join, CRATE, SELF, SUPER};

#[derive(Clone, Default)]
pub struct ImportResolver {
    pub inner: IndexMap<ScopeChain, IndexMap<Path, Path>>,
    pub globs: IndexMap<ScopeChain, Vec<Path>>,
    pub materialized_globs: IndexMap<ScopeChain, IndexMap<Path, Path>>,
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
            UseTree::Name(UseName { ident, .. }) => {
                if ident != "_" {
                    current_path.push(ident.clone());
                    let path = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                    self.inner
                        .entry(scope.clone())
                        .or_default()
                        .insert(ident.to_path(), path);
                }
            }
            UseTree::Rename(UseRename { ident, rename, .. }) => {
                if rename != "_" {
                    // Build path using original ident, but store under alias (rename)
                    current_path.push(ident.clone());
                    let path = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                    self.inner
                        .entry(scope.clone())
                        .or_default()
                        .insert(rename.to_path(), path);
                }
            }
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|use_tree| self.fold_import_tree(scope, use_tree, current_path.clone())),
            UseTree::Glob(..) => {
                // Record a glob base path for this scope; names are resolved lazily during refinement.
                let base = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                self.globs.entry(scope.clone()).or_default().push(base);
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
    /// Legacy import resolution method - delegates to enhanced resolver
    pub fn maybe_import(&self, scope: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        // Use enhanced resolution if materialized globs are available, otherwise fall back to legacy
        if !self.materialized_globs.is_empty() {
            self.resolve_import_enhanced(scope, key)
        } else {
            self.maybe_import_legacy(scope, key)
        }
    }

    /// Legacy import resolution method (preserved for compatibility)
    pub fn maybe_import_legacy(&self, scope: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
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


    pub fn maybe_scope_globs(&self, scope: &ScopeChain) -> Option<&Vec<Path>> {
        self.globs.get(scope)
    }


    /// Materialization method that works with a scope resolver
    /// Materializes glob imports after all scope items have been collected
    /// This should be called after the full context tree has been built
    pub fn materialize_globs_with_scope_resolver(&mut self, scope_resolver: &ScopeResolver) {
        for (scope, glob_bases) in &self.globs {
            let mut materialized_map = IndexMap::new();

            for glob_base in glob_bases {
                // Try to find scope chain that matches the glob base
                if let Some(scope_chain) = scope_resolver.maybe_scope(glob_base) {
                    // Get the type chain for this scope to enumerate available items
                    if let Some(type_chain) = scope_resolver.get(scope_chain) {
                        for (type_ref, _object_kind) in &type_chain.inner {
                            // Extract the item name from the type
                            if let Some(path) = type_ref.to_path().segments.last() {
                                let item_ident = &path.ident;
                                let item_path = item_ident.to_path();
                                let full_path = glob_base.joined(&item_path);

                                // Map the item name to its full path
                                materialized_map.insert(item_path, full_path);
                            }
                        }
                    }
                }
            }

            if !materialized_map.is_empty() {
                self.materialized_globs.insert(scope.clone(), materialized_map);
            }
        }
    }

    /// Gets materialized glob imports for a scope
    pub fn maybe_materialized_globs(&self, scope: &ScopeChain) -> Option<&IndexMap<Path, Path>> {
        self.materialized_globs.get(scope)
    }

    fn maybe_from_materialized_globs(&self, scope: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        if let Some(glob_imports) = self.maybe_materialized_globs(scope) {
            match key {
                GenericBoundKey::Ident(ident) => if let Some(glob_path) = glob_imports.get(&ident.to_path()) {
                    return Some(glob_path);
                },
                GenericBoundKey::Path(path) => if let Some(glob_path) = glob_imports.get(path) {
                    return Some(glob_path);
                }
            }
        }
        None
    }

    /// Enhanced unified lookup that combines direct imports, materialized globs, and scope chain resolution
    pub fn resolve_import_enhanced(&self, scope: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        // Priority 1: Direct imports in current scope
        if let Some(direct_path) = self.maybe_path(scope, key) {
            return Some(direct_path);
        }

        // Priority 2: Materialized glob imports in current scope
        if let Some(glob_path) = self.maybe_from_materialized_globs(scope, key) {
            return Some(glob_path);
        }

        // Priority 3: Parent scope chain lookup with enhanced resolution
        match scope {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } => None,
            ScopeChain::Fn { parent, .. } =>
                self.resolve_import_enhanced_fn(scope, parent, key),
            ScopeChain::Trait { parent, .. } |
            ScopeChain::Object { parent, .. } |
            ScopeChain::Impl { parent, .. } =>
                self.resolve_import_enhanced_obj_or_parent(scope, parent, key),
        }
    }

    /// Enhanced function scope resolution with comprehensive fallback
    fn resolve_import_enhanced_fn(&self, self_scope: &ScopeChain, parent: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        // Check function scope directly
        if let Some(path) = self.maybe_path(self_scope, key) {
            return Some(path);
        }

        // Check function scope globs
        if let Some(glob_path) = self.maybe_from_materialized_globs(self_scope, key) {
            return Some(glob_path);
        }

        // Recursively check parent
        self.resolve_import_enhanced(parent, key)
    }

    /// Enhanced object/parent scope resolution with priority ordering
    fn resolve_import_enhanced_obj_or_parent(&self, self_scope: &ScopeChain, parent: &ScopeChain, key: &GenericBoundKey) -> Option<&Path> {
        // Check current scope directly
        if let Some(path) = self.maybe_path(self_scope, key) {
            return Some(path);
        }

        // Check current scope globs
        if let Some(glob_path) = self.maybe_from_materialized_globs(self_scope, key) {
            return Some(glob_path);
        }

        // Only check parent if it's a module or crate root
        match parent {
            ScopeChain::CrateRoot { .. } | ScopeChain::Mod { .. } =>
                self.maybe_path(parent, key)
                    .or_else(|| self.maybe_from_materialized_globs(parent, key)),
            _ => None,
        }
    }

    /// Get all available imports for a scope (direct + globbed)
    pub fn get_all_imports(&self, scope: &ScopeChain) -> IndexMap<Path, Path> {
        let mut all_imports = IndexMap::new();

        // Add direct imports
        if let Some(direct_imports) = self.maybe_scope_imports(scope) {
            all_imports.extend(direct_imports.clone());
        }

        // Add materialized glob imports
        if let Some(glob_imports) = self.maybe_materialized_globs(scope) {
            all_imports.extend(glob_imports.clone());
        }

        all_imports
    }

    /// Refines import paths to fully qualified forms after all scopes are available
    /// This should be called after scope creation but before type refinement
    pub fn refine_import_paths(&mut self) {
        // Create a new refined imports map
        let mut refined_inner = IndexMap::new();
        let mut refined_globs = IndexMap::new();

        // Refine direct imports
        for (scope, imports) in &self.inner {
            let mut refined_scope_imports = IndexMap::new();
            for (alias, path) in imports {
                let refined_path = self.refine_import_path(path, scope);
                refined_scope_imports.insert(alias.clone(), refined_path);
            }
            refined_inner.insert(scope.clone(), refined_scope_imports);
        }

        // Refine glob imports
        for (scope, glob_paths) in &self.globs {
            let mut refined_scope_globs = Vec::new();
            for path in glob_paths {
                let refined_path = self.refine_import_path(path, scope);
                refined_scope_globs.push(refined_path);
            }
            refined_globs.insert(scope.clone(), refined_scope_globs);
        }

        // Replace the old maps with refined ones
        self.inner = refined_inner;
        self.globs = refined_globs;
    }


    /// Refines a single import path using the same normalization logic
    fn refine_import_path(&self, path: &Path, scope: &ScopeChain) -> Path {
        if path.segments.is_empty() {
            return path.clone();
        }

        let first_segment = &path.segments.first().unwrap().ident;
        let first_str = first_segment.to_string();

        match first_str.as_str() {
            CRATE => {
                // Replace "crate" with actual crate ident
                let mut new_path = path.clone();
                new_path.segments.first_mut().unwrap().ident = scope.crate_ident_ref().clone();
                new_path
            },
            SELF => {
                // Replace "self" with current scope path
                let tail_segments = path.segments.iter().skip(1).cloned().collect::<Vec<_>>();
                let mut new_segments = scope.self_path_ref().segments.clone();
                new_segments.extend(tail_segments);
                Path {
                    leading_colon: path.leading_colon,
                    segments: new_segments
                }
            },
            SUPER => {
                // Handle super references by going up scope chain
                let mut super_count = 0;
                for segment in path.segments.iter() {
                    if segment.ident.to_string() == SUPER {
                        super_count += 1;
                    } else {
                        break;
                    }
                }

                let mut base_scope = scope.self_path_ref().clone();
                for _ in 0..super_count {
                    // Pop segments to go up hierarchy
                    if base_scope.segments.len() > 1 {
                        base_scope.segments.pop();
                    }
                }

                let tail_segments = path.segments.iter().skip(super_count).cloned().collect::<Vec<_>>();
                let mut new_segments = base_scope.segments;
                new_segments.extend(tail_segments);
                Path {
                    leading_colon: path.leading_colon,
                    segments: new_segments
                }
            },
            _ => path.clone()
        }
    }
}
