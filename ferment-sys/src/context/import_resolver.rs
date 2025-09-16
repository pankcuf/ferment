use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use proc_macro2::Ident;
use quote::ToTokens;
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
    /// Maps import paths as defined in scope to their fully qualified resolved paths
    /// Key: (ScopeChain, ImportPath) -> Value: FullyQualifiedPath
    pub resolved_imports: HashMap<(ScopeChain, Path), Path>,
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
            UseTree::Name(UseName { ident, .. }) => if ident != "_" {
                current_path.push(ident.clone());
                let path = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                self.inner
                    .entry(scope.clone())
                    .or_default()
                    .insert(ident.to_path(), path);
            }
            UseTree::Rename(UseRename { ident, rename, .. }) => if rename != "_" {
                // Build path using original ident, but store under alias (rename)
                current_path.push(ident.clone());
                let path = Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) };
                self.inner
                    .entry(scope.clone())
                    .or_default()
                    .insert(rename.to_path(), path);
            }
            UseTree::Group(UseGroup { items, .. }) =>
                items.iter()
                    .for_each(|use_tree| self.fold_import_tree(scope, use_tree, current_path.clone())),
            UseTree::Glob(..) => {
                // Record a glob base path for this scope; names are resolved lazily during refinement.
                // Handle special keywords like 'super' by resolving them relative to current scope
                let resolved_path = if current_path.len() == 1 && current_path[0].to_string() == "super" {
                    // For 'super', we want the parent scope's path
                    // Note: parent_scope() returns None for modules, but modules do have parents
                    // Access the parent directly from the enum structure
                    match scope {
                        ScopeChain::Mod { parent, .. } |
                        ScopeChain::Trait { parent, .. } |
                        ScopeChain::Fn { parent, .. } |
                        ScopeChain::Object { parent, .. } |
                        ScopeChain::Impl { parent, .. } =>
                            parent.self_path_ref().clone(),
                        ScopeChain::CrateRoot { .. } =>
                            Path { leading_colon: None, segments: Punctuated::new() }
                    }
                } else {
                    // Regular path - convert current_path to a Path
                    Path { leading_colon: None, segments: Punctuated::from_iter(current_path.into_iter().map(PathSegment::from)) }
                };

                self.globs.entry(scope.clone()).or_default().push(resolved_path);
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
        if self.materialized_globs.is_empty() {
            self.maybe_import_legacy(scope, key)
        } else {
            self.resolve_import_enhanced(scope, key)
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
    /// Find a scope chain in the import resolver that matches the given path
    fn find_scope_by_path(&self, target_path: &Path) -> Option<&ScopeChain> {
        // Look through all scopes in the import resolver to find one that matches the target path
        self.inner.keys().find(|scope_chain| scope_chain.self_path_ref() == target_path)
    }

    /// Materializes glob imports after all scope items have been collected
    /// This should be called after the full context tree has been built
    /// Uses dependency-ordered processing to ensure transitive imports work correctly
    pub fn materialize_globs_with_scope_resolver(&mut self, scope_resolver: &ScopeResolver) {
        // Build dependency graph from glob imports
        let dependency_graph = self.build_glob_dependency_graph(scope_resolver);
        // Get processing order (dependencies first)
        let processing_order = self.topological_sort(&dependency_graph);
        // Process scopes in dependency order
        for scope in processing_order {
            if let Some(glob_bases) = self.globs.get(&scope).cloned() {
                let mut all_items = IndexMap::new();
                // Step 1: Materialize direct items from immediate glob targets
                for glob_base in &glob_bases {
                    all_items.extend(self.materialize_direct_items_from_glob(&scope, glob_base, scope_resolver));
                }
                // Step 2: Items from dependencies are already included via direct materialization
                // The dependency graph ensures we process dependencies first, so when we
                // materialize globs, we get the complete transitive closure
                // Step 3: Store final materialized items
                if !all_items.is_empty() {
                    self.materialized_globs.insert(scope.clone(), all_items.clone());
                    // Add to resolved_imports for import resolution
                    for (alias_path, resolved_path) in all_items {
                        self.resolved_imports.insert((scope.clone(), alias_path), resolved_path);
                    }
                }
            }
        }
    }

    /// Build dependency graph from glob imports
    /// Returns a map: scope -> list of scopes it depends on (imports from)
    fn build_glob_dependency_graph(&self, scope_resolver: &ScopeResolver) -> HashMap<ScopeChain, Vec<ScopeChain>> {
        let mut graph = HashMap::new();
        for (scope, glob_bases) in &self.globs {
            let mut dependencies = Vec::new();
            for glob_base in glob_bases {
                // Convert relative glob patterns to absolute paths
                let absolute_glob_base = if glob_base.leading_colon.is_some() {
                    glob_base.clone()
                } else {
                    let glob_str = glob_base.to_token_stream().to_string();
                    let crate_name = scope.crate_ident_ref().to_string();
                    if glob_str.starts_with(&crate_name) {
                        glob_base.clone()
                    } else {
                        scope.self_path_ref().joined(glob_base)
                    }
                };
                // Find the scope that matches this glob target
                if let Some(target_scope) = scope_resolver.maybe_scope(&absolute_glob_base)
                    .or_else(|| self.find_scope_by_path(&absolute_glob_base)) {
                    dependencies.push(target_scope.clone());
                }
            }
            if !dependencies.is_empty() {
                graph.insert(scope.clone(), dependencies);
            }
        }
        graph
    }

    /// Perform topological sort on dependency graph
    /// Returns scopes in processing order (dependencies first)
    fn topological_sort(&self, graph: &HashMap<ScopeChain, Vec<ScopeChain>>) -> Vec<ScopeChain> {
        let mut result = Vec::new();
        let mut permanent_mark = HashSet::new();
        let mut temporary_mark = HashSet::new();
        // Get all nodes (both dependencies and dependents)
        let mut all_nodes = HashSet::new();
        for (scope, deps) in graph {
            all_nodes.insert(scope.clone());
            for dep in deps {
                all_nodes.insert(dep.clone());
            }
        }
        // Visit each unmarked node
        for node in all_nodes {
            if !permanent_mark.contains(&node) {
                self.topological_sort_visit(&node, graph, &mut permanent_mark, &mut temporary_mark, &mut result);
            }
        }
        result
    }

    /// Recursive helper for topological sort
    fn topological_sort_visit(
        &self,
        node: &ScopeChain,
        graph: &HashMap<ScopeChain, Vec<ScopeChain>>,
        permanent_mark: &mut HashSet<ScopeChain>,
        temporary_mark: &mut HashSet<ScopeChain>,
        result: &mut Vec<ScopeChain>,
    ) {
        if temporary_mark.contains(node) {
            // Cycle detected - for now, we'll just skip (could add warning)
            return;
        }
        if permanent_mark.contains(node) {
            return;
        }
        temporary_mark.insert(node.clone());
        // Visit dependencies first
        if let Some(dependencies) = graph.get(node) {
            for dep in dependencies {
                self.topological_sort_visit(dep, graph, permanent_mark, temporary_mark, result);
            }
        }
        temporary_mark.remove(node);
        permanent_mark.insert(node.clone());
        result.push(node.clone());
    }

    /// Materialize direct items from a single glob import
    fn materialize_direct_items_from_glob(
        &self,
        scope: &ScopeChain,
        glob_base: &Path,
        scope_resolver: &ScopeResolver,
    ) -> IndexMap<Path, Path> {
        let mut items = IndexMap::new();
        // Convert relative glob patterns to absolute paths
        let absolute_glob_base = if glob_base.leading_colon.is_some() {
            glob_base.clone()
        } else {
            let glob_str = glob_base.to_token_stream().to_string();
            let crate_name = scope.crate_ident_ref().to_string();
            if glob_str.starts_with(&crate_name) {
                glob_base.clone()
            } else {
                scope.self_path_ref().joined(glob_base)
            }
        };
        // Try to find scope chain that matches the absolute glob base
        if let Some(scope_chain) = scope_resolver.maybe_scope(&absolute_glob_base) {
            // Get the type chain for this scope to enumerate available items
            if let Some(type_chain) = scope_resolver.get(scope_chain) {
                // Add items defined in this scope
                for (type_ref, object_kind) in &type_chain.inner {
                    if object_kind.is_item() {
                        if let Some(path) = type_ref.to_path().segments.last() {
                            let item_ident = &path.ident;
                            let item_path = item_ident.to_path();
                            let actual_definition_path = scope_chain.self_path_ref().joined(&item_path);
                            items.insert(item_path, actual_definition_path);
                        }
                    }
                }
                // Add imports available in this scope (for re-exports)
                if let Some(scope_imports) = self.inner.get(scope_chain) {
                    items.extend(scope_imports.clone());
                }
            }
        } else {
            // Fallback: look for scope in import resolver
            if let Some(target_scope_chain) = self.find_scope_by_path(&absolute_glob_base) {
                if let Some(scope_imports) = self.inner.get(target_scope_chain) {
                    for (alias, resolved_path) in scope_imports {
                        let qualified_path = if resolved_path.leading_colon.is_some() {
                            resolved_path.clone()
                        } else {
                            target_scope_chain.self_path_ref().joined(resolved_path)
                        };
                        items.insert(alias.clone(), qualified_path);
                    }
                }
            }
        }
        items
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
        // Priority 0: Check resolved imports map for fast O(1) lookup
        let import_path = match key {
            GenericBoundKey::Ident(ident) => ident.to_path(),
            GenericBoundKey::Path(path) => path.clone(),
        };
        if let Some(resolved) = self.resolve_import_in_scope(scope, &import_path) {
            return Some(resolved);
        }
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

    pub fn refine_imports(&mut self, resolver: &ScopeResolver) {
        // Refine import paths after scope creation but before type refinement
        self.refine_import_paths();
        // Materialize glob imports before type refinement
        self.materialize_globs_with_scope_resolver(resolver);
        // Note: resolved imports map will be built later when GlobalContext is available
    }

    /// Refines a single import path using the same normalization logic
    fn refine_import_path(&self, path: &Path, scope: &ScopeChain) -> Path {
        if path.segments.is_empty() {
            return path.clone();
        }
        match path.segments.first().unwrap().ident.to_string().as_str() {
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

    /// Builds the resolved imports map during preprocessing
    pub fn build_resolved_imports_map(&mut self, scope_resolver: &ScopeResolver) {
        //refine_import_dbg!("build_resolved_imports_map");
        // Clear any existing resolved imports
        self.resolved_imports.clear();
        // Get all scopes from the scope resolver
        let all_scopes: Vec<ScopeChain> = self.inner.keys().cloned()
            .chain(self.materialized_globs.keys().cloned())
            .collect();
        // Process all scopes
        for scope in all_scopes {
            self.resolve_imports_for_scope(&scope, scope_resolver);
        }
    }

    fn resolve_imports_for_scope(&mut self, scope: &ScopeChain, scope_resolver: &ScopeResolver) {
        // 1. Process direct imports from inner map
        if let Some(scope_imports) = self.inner.get(scope) {
            for (alias_path, import_path) in scope_imports {
                let resolved = self.determine_full_qualified_path(import_path, scope, scope_resolver);
                self.resolved_imports.insert((scope.clone(), alias_path.clone()), resolved);
            }
        }

        // 2. Process materialized glob imports
        if let Some(glob_imports) = self.materialized_globs.get(scope).cloned() {
            for (alias_path, import_path) in glob_imports {
                let resolved = self.determine_full_qualified_path(&import_path, scope, scope_resolver);
                self.resolved_imports.insert((scope.clone(), alias_path), resolved);
            }
        }
    }

    /// Main algorithm to determine fully qualified path
    fn determine_full_qualified_path(&self,
        import_path: &Path,
        declaring_scope: &ScopeChain,
        scope_resolver: &ScopeResolver
    ) -> Path {
        // Step 1: Normalize relative imports (crate/self/super)
        let normalized = self.normalize_import_path(import_path, declaring_scope);
        // Step 2: Follow reexport chains to final destination
        let reexport_resolved = self.resolve_through_reexports(&normalized, scope_resolver);
        // Step 3: Validate against scope registry
        self.validate_against_scope_registry(&reexport_resolved, scope_resolver)
    }

    /// Normalizes relative import paths to absolute paths
    pub fn normalize_import_path(&self, path: &Path, scope: &ScopeChain) -> Path {
        if path.segments.is_empty() {
            return path.clone();
        }
        let first_ident = &path.segments.first().unwrap().ident;
        match first_ident.to_string().as_str() {
            CRATE => {
                // crate::foo::Bar -> actual_crate_name::foo::Bar
                let mut normalized = path.clone();
                normalized.segments.first_mut().unwrap().ident = scope.crate_ident_ref().clone();
                normalized
            },
            SELF => {
                // self::foo::Bar -> current_scope::foo::Bar
                let tail: Vec<_> = path.segments.iter().skip(1).cloned().collect();
                let mut segments = scope.self_path_ref().segments.clone();
                segments.extend(tail);
                Path { leading_colon: path.leading_colon, segments }
            },
            SUPER => {
                // super::foo::Bar -> parent_scope::foo::Bar
                // Handle multiple super::super::...
                let mut super_count = 0;
                for seg in path.segments.iter() {
                    if seg.ident == SUPER { super_count += 1; } else { break; }
                }
                let mut base = scope.self_path_ref().clone();
                for _ in 0..super_count {
                    if base.segments.len() > 1 { base.segments.pop(); }
                }
                let tail: Vec<_> = path.segments.iter().skip(super_count).cloned().collect();
                base.segments.extend(tail);
                base
            },
            _ => {
                // Already absolute or external crate
                path.clone()
            }
        }
    }

    /// Resolves through reexport chains by finding actual item definitions
    /// This implements proper verification that items actually exist
    fn resolve_through_reexports(&self, path: &Path, scope_resolver: &ScopeResolver) -> Path {
        // Single-segment path: search all scopes for actual item definition
        if path.segments.len() == 1 {
            if let Some(actual_definition) = self.find_verified_item_definition(path, scope_resolver) {
                return actual_definition;
            }
        }
        // Multi-segment path: check if the exact path exists and contains the target item
        if let Some(last_segment) = path.segments.last() {
            let target_ident = &last_segment.ident;
            // First check if the full path exists as a scope with the target item
            if let Some(scope_chain) = scope_resolver.maybe_scope(path) {
                if let Some(type_chain) = scope_resolver.get(scope_chain) {
                    // Check if this scope actually contains the target item
                    for (type_ref, object_kind) in &type_chain.inner {
                        if object_kind.is_item() {
                            if let Some(item_segment) = type_ref.to_path().segments.last() {
                                if item_segment.ident == *target_ident {
                                    return path.clone();
                                }
                            }
                        }
                    }
                }
            }
            // Try progressive truncation to find valid base scope + target item
            let mut base_path = path.clone();
            while base_path.segments.len() > 1 {
                base_path.segments.pop();
                if let Some(scope_chain) = scope_resolver.maybe_scope(&base_path) {
                    if let Some(type_chain) = scope_resolver.get(scope_chain) {
                        // Check if this scope contains our target item
                        for (type_ref, object_kind) in &type_chain.inner {
                            if object_kind.is_item() {
                                if let Some(item_segment) = type_ref.to_path().segments.last() {
                                    if item_segment.ident.eq(target_ident) {
                                        // Found the target item in this scope
                                        return scope_chain.self_path_ref().joined(&target_ident.to_path());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Last resort: search all scopes for this specific item
            if let Some(verified_definition) = self.find_verified_item_definition(&target_ident.to_path(), scope_resolver) {
                return verified_definition;
            }
        }
        // Only return original path if we couldn't verify the item exists anywhere
        path.clone()
    }

    /// Find verified item definition - ensures the item actually exists in the scope
    fn find_verified_item_definition(&self, item_name: &Path, scope_resolver: &ScopeResolver) -> Option<Path> {
        if item_name.segments.len() != 1 {
            return None;
        }
        let target_ident = &item_name.segments[0].ident;
        let mut best_match: Option<Path> = None;
        let mut max_specificity = 0;
        // Search all scopes to find where this item is actually defined
        for (scope_path, type_chain) in scope_resolver.inner.iter() {
            for (type_ref, object_kind) in &type_chain.inner {
                // Only consider actual items, not type references
                if object_kind.is_item() {
                    if let Some(last_segment) = type_ref.to_path().segments.last() {
                        if last_segment.ident.eq(target_ident) {
                            // Found a verified item definition
                            let scope_self_path = scope_path.self_path_ref();
                            // Check if scope path already ends with the target item
                            let full_path = if scope_self_path.segments.last()
                                .map(|seg| seg.ident == *target_ident)
                                .unwrap_or(false) {
                                // Scope already ends with target, don't duplicate
                                scope_self_path.clone()
                            } else {
                                // Need to append target to scope path
                                scope_self_path.joined(&target_ident.to_path())
                            };
                            let specificity = full_path.segments.len();
                            // Prefer the most specific (longest) path
                            if specificity > max_specificity {
                                max_specificity = specificity;
                                best_match = Some(full_path);
                            }
                        }
                    }
                }
            }
        }
        best_match
    }

    /// Validates path against scope registry to ensure it points to real items
    pub fn validate_against_scope_registry(&self, path: &Path, scope_resolver: &ScopeResolver) -> Path {
        // Check if this exact path exists as a scope
        if scope_resolver.maybe_scope(path).is_some() {
            return path.clone();
        }
        // Try progressive truncation to find the deepest valid scope
        let mut test_path = path.clone();
        while test_path.segments.len() > 1 {
            test_path.segments.pop();
            if scope_resolver.maybe_scope(&test_path).is_some() {
                // Reconstruct with validated base + remaining segments
                let remaining: Vec<_> = path.segments.iter()
                    .skip(test_path.segments.len())
                    .cloned()
                    .collect();
                test_path.segments.extend(remaining);
                return test_path;
            }
        }
        path.clone() // Return original if validation fails
    }

    /// Builds fully qualified resolved imports without circular dependencies
    pub fn build_fully_qualified_imports(&mut self, scope_resolver: &ScopeResolver) {
        // Clear any existing resolved imports
        self.resolved_imports.clear();
        // Build in multiple passes to resolve dependencies
        self.resolve_imports_iteratively(scope_resolver);
    }

    /// Iteratively resolve imports until no more changes occur
    fn resolve_imports_iteratively(&mut self, scope_resolver: &ScopeResolver) {
        let max_iterations = 10; // Prevent infinite loops
        for _iteration in 0..max_iterations {
            let initial_size = self.resolved_imports.len();
            // Resolve direct imports
            self.resolve_direct_imports_pass(scope_resolver);
            // Resolve glob imports
            self.resolve_glob_imports_pass(scope_resolver);
            // If no new resolutions were made, we're done
            if self.resolved_imports.len() == initial_size {
                break;
            }
        }
    }

    fn resolve_direct_imports_pass(&mut self, scope_resolver: &ScopeResolver) {
        let all_direct_imports: Vec<_> = self.inner.iter()
            .flat_map(|(scope, imports)| {
                imports.iter().map(|(alias, import_path)| {
                    (scope.clone(), alias.clone(), import_path.clone())
                })
            })
            .collect();
        for (scope, alias, import_path) in all_direct_imports {
            let key = (scope.clone(), alias.clone());
            // Skip if already resolved
            if self.resolved_imports.contains_key(&key) {
                continue;
            }
            if let Some(resolved) = self.resolve_import_path_fully(&import_path, &scope, scope_resolver) {
                self.resolved_imports.insert(key, resolved);
            }
        }
    }

    fn resolve_glob_imports_pass(&mut self, scope_resolver: &ScopeResolver) {
        let all_glob_imports = Vec::from_iter(self.materialized_globs.iter()
            .flat_map(|(scope, imports)| imports.iter().map(|(alias, import_path)| (scope.clone(), alias.clone(), import_path.clone()))));
        for (scope, alias, import_path) in all_glob_imports {
            let key = (scope.clone(), alias.clone());
            // Skip if already resolved
            if self.resolved_imports.contains_key(&key) {
                continue;
            }
            if let Some(resolved) = self.resolve_import_path_fully(&import_path, &scope, scope_resolver) {
                self.resolved_imports.insert(key, resolved);
            }
        }
    }

    /// Resolve import path using only normalized paths and existing resolved imports
    fn resolve_import_path_fully(&self, import_path: &Path, declaring_scope: &ScopeChain, scope_resolver: &ScopeResolver) -> Option<Path> {
        // Use the corrected determine_full_qualified_path which includes proper reexport resolution
        Some(self.determine_full_qualified_path(import_path, declaring_scope, scope_resolver))
    }

    /// Fast O(1) lookup during refinement
    pub fn resolve_import_in_scope(&self, scope: &ScopeChain, import_path: &Path) -> Option<&Path> {
        // Priority 1: Check direct imports
        if let Some(resolved) = self.resolved_imports.get(&(scope.clone(), import_path.clone())) {
            return Some(resolved);
        }
        // Priority 2: Check materialized glob imports - search through all materialized_globs
        // to find items that could be accessible via glob imports in this scope
        if let Some(glob_bases) = self.globs.get(scope) {
            // Check all materialized_globs to see if any could provide this item through the globs
            for (materialized_scope, materialized_items) in &self.materialized_globs {
                if let Some(resolved) = materialized_items.get(import_path) {
                    // Check if this materialized scope is accessible through any glob in the current scope
                    let materialized_path = materialized_scope.self_path_ref();
                    for glob_base in glob_bases {
                        let target_scope_path = if glob_base.segments.is_empty() {
                            // For empty/super globs
                            if let Some(parent) = scope.parent_scope() {
                                parent.self_path_ref().clone()
                            } else {
                                continue;
                            }
                        } else {
                            // For regular paths like "crate::data_contract::errors" or relative paths like "instant"
                            let normalized = self.normalize_import_path(glob_base, scope);
                            // If the normalized path is still relative (doesn't start with crate name), make it absolute
                            let crate_name = scope.self_path_ref().segments.first().map(|seg| &seg.ident);
                            let is_relative = normalized.segments.first().map_or(true, |seg|
                                crate_name.map_or(true, |crate_ident| seg.ident != *crate_ident));

                            if is_relative {
                                // This is a relative path, make it absolute by joining with current scope
                                let mut absolute_path = scope.self_path_ref().clone();
                                absolute_path.segments.extend(normalized.segments);
                                absolute_path
                            } else {
                                normalized
                            }
                        };
                        if materialized_path == &target_scope_path {
                            return Some(resolved);
                        }
                    }
                }
            }
        }
        None
    }

    /// Resolve any absolute path by checking if it's available as an import in any scope
    /// This replaces ReexportSeek functionality by using our precomputed resolved imports
    pub fn resolve_absolute_path(&self, target_path: &Path, current_scope: &ScopeChain) -> Option<Path> {
        // First try to find if the target_path is an import alias in the current scope
        if let Some(resolved) = self.resolve_import_in_scope(current_scope, target_path) {
            return Some(resolved.clone());
        }
        // If not found as an import alias, check if any import resolves to this target path
        // This handles cases where we're looking up the final resolved path
        for ((scope_chain, _import_path), resolved_path) in &self.resolved_imports {
            if scope_chain == current_scope && resolved_path == target_path {
                return Some(resolved_path.clone());
            }
        }
        // If not found, the path might already be absolute and correct
        Some(target_path.clone())
    }

}
