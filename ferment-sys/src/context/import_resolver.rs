use indexmap::IndexMap;
use std::collections::HashMap;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Path, PathSegment, UseGroup, UseName, UsePath, UseRename, UseTree};
use syn::punctuated::Punctuated;
use crate::context::{ScopeChain, ScopeResolver};
use crate::ext::{GenericBoundKey, ToPath};
use crate::ext::{Join, CRATE, SELF, SUPER};

fn dbg_enabled() -> bool {
    std::env::var("FERMENT_DEBUG_IMPORT_REFINE").is_ok()
}
macro_rules! refine_import_dbg {
    ($($arg:tt)*) => {
        if dbg_enabled() { println!($($arg)*); }
    }
}

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
                        for (type_ref, object_kind) in &type_chain.inner {
                            // Only materialize actual items defined in this scope, not type references
                            if let crate::kind::ObjectKind::Item(_, _) = object_kind {
                                // Extract the item name from the type
                                if let Some(path) = type_ref.to_path().segments.last() {
                                    let item_ident = &path.ident;
                                    let item_path = item_ident.to_path();

                                    // Find where this item is actually defined, not just reexported
                                    let actual_definition_path = self.find_actual_item_definition(&item_path, scope_resolver)
                                        .unwrap_or_else(|| glob_base.joined(&item_path));

                                    // Map the item name to its actual definition path
                                    materialized_map.insert(item_path, actual_definition_path);
                                }
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
        //refine_import_dbg!("resolve_imports_for_scope: {}", scope.fmt_short());
        // 1. Process direct imports from inner map
        if let Some(scope_imports) = self.inner.get(scope) {
            for (alias_path, import_path) in scope_imports {
                let resolved = self.determine_full_qualified_path(import_path, scope, scope_resolver);
                if alias_path.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("resolve_imports_for_scope.direct.insert {} as {} in {}", alias_path.to_token_stream(), resolved.to_token_stream(), scope.fmt_short());
                }
                self.resolved_imports.insert((scope.clone(), alias_path.clone()), resolved);
            }
        }

        // 2. Process materialized glob imports
        if let Some(glob_imports) = self.materialized_globs.get(scope).cloned() {
            for (alias_path, import_path) in glob_imports {
                let resolved = self.determine_full_qualified_path(&import_path, scope, scope_resolver);
                if alias_path.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("resolve_imports_for_scope.glob.insert {} as {} in {}", alias_path.to_token_stream(), resolved.to_token_stream(), scope.fmt_short());
                }
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
        if import_path.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("determine_full_qualified_path: {} in {}", import_path.to_token_stream(), declaring_scope.fmt_short());
        }
        // Step 1: Normalize relative imports (crate/self/super)
        let normalized = self.normalize_import_path(import_path, declaring_scope);
        if normalized.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("normalized: {}", normalized.to_token_stream());
        }
        // Step 2: Follow reexport chains to final destination
        let reexport_resolved = self.resolve_through_reexports(&normalized, scope_resolver);
        if reexport_resolved.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("reexport_resolved: {}", reexport_resolved.to_token_stream());
        }
        // Step 3: Validate against scope registry
        let validated = self.validate_against_scope_registry(&reexport_resolved, scope_resolver);
        if validated.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("validated: {}", validated.to_token_stream());
        }
        validated
    }

    /// Normalizes relative import paths to absolute paths
    pub fn normalize_import_path(&self, path: &Path, scope: &ScopeChain) -> Path {
        if path.segments.is_empty() {
            if path.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                refine_import_dbg!("normalize_import_path:empty {} in {}", path.to_token_stream(), scope.fmt_short());
            }
            return path.clone();
        }

        let first_ident = &path.segments.first().unwrap().ident;
        match first_ident.to_string().as_str() {
            CRATE => {
                // crate::foo::Bar -> actual_crate_name::foo::Bar
                let mut normalized = path.clone();
                normalized.segments.first_mut().unwrap().ident = scope.crate_ident_ref().clone();
                if normalized.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("normalize_import_path:crate {} in {}", normalized.to_token_stream(), scope.fmt_short());
                }
                normalized
            },
            SELF => {
                // self::foo::Bar -> current_scope::foo::Bar
                let tail: Vec<_> = path.segments.iter().skip(1).cloned().collect();
                let mut segments = scope.self_path_ref().segments.clone();
                segments.extend(tail);
                if segments.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("normalize_import_path:self {} in {}", segments.to_token_stream(), scope.fmt_short());
                }
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
                if base.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("normalize_import_path:super {} in {}", base.to_token_stream(), scope.fmt_short());
                }
                base
            },
            _ => {
                // Already absolute or external crate
                if path.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("normalize_import_path:abs_or_ext {} in {}", path.to_token_stream(), scope.fmt_short());
                }
                path.clone()
            }
        }
    }

    /// Resolves through reexport chains by finding actual item definitions
    /// This implements proper verification that items actually exist
    fn resolve_through_reexports(&self, path: &Path, scope_resolver: &ScopeResolver) -> Path {
        // if path.to_token_stream().to_string().contains("ProtocolError") {
        //     refine_import_dbg!("resolve_through_reexports: {}", path.to_token_stream());
        // }

        // Single-segment path: search all scopes for actual item definition
        if path.segments.len() == 1 {
            if let Some(actual_definition) = self.find_verified_item_definition(path, scope_resolver) {
                if actual_definition.to_token_stream().to_string().contains("ProtocolError") {
                    refine_import_dbg!("resolve_through_reexports:found_verified_item: {}", actual_definition.to_token_stream());
                }
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
                        if let crate::kind::ObjectKind::Item(_, _) = object_kind {
                            if let Some(item_segment) = type_ref.to_path().segments.last() {
                                if item_segment.ident == *target_ident {
                                    if path.to_token_stream().to_string().contains("ProtocolError") {
                                        refine_import_dbg!("resolve_through_reexports:verified_exact_path: {}", path.to_token_stream());
                                    }
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
                            if let crate::kind::ObjectKind::Item(_, _) = object_kind {
                                if let Some(item_segment) = type_ref.to_path().segments.last() {
                                    if item_segment.ident == *target_ident {
                                        // Found the target item in this scope
                                        let verified_path = scope_chain.self_path_ref().joined(&target_ident.to_path());
                                        if verified_path.to_token_stream().to_string().contains("ProtocolError") {
                                            refine_import_dbg!("resolve_through_reexports:verified_in_base: {}", verified_path.to_token_stream());
                                        }
                                        return verified_path;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Last resort: search all scopes for this specific item
            if let Some(verified_definition) = self.find_verified_item_definition(&target_ident.to_path(), scope_resolver) {
                // if verified_definition.to_token_stream().to_string().contains("ProtocolError") {
                //     refine_import_dbg!("resolve_through_reexports:verified_fallback: {}", verified_definition.to_token_stream());
                // }
                return verified_definition;
            }
        }

        // Only return original path if we couldn't verify the item exists anywhere
        if path.to_token_stream().to_string().contains("ProtocolError") {
            refine_import_dbg!("resolve_through_reexports:unverified_fallback: {}", path.to_token_stream());
        }
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
                if let crate::kind::ObjectKind::Item(_, _) = object_kind {
                    if let Some(last_segment) = type_ref.to_path().segments.last() {
                        if last_segment.ident == *target_ident {
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
                if alias.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("resolve_direct_imports_pass.insert {} as {} in {}", alias.to_token_stream(), resolved.to_token_stream(), scope.fmt_short());
                }
                self.resolved_imports.insert(key, resolved);
            }
        }
    }

    fn resolve_glob_imports_pass(&mut self, scope_resolver: &ScopeResolver) {
        let all_glob_imports: Vec<_> = self.materialized_globs.iter()
            .flat_map(|(scope, imports)| {
                imports.iter().map(|(alias, import_path)| {
                    (scope.clone(), alias.clone(), import_path.clone())
                })
            })
            .collect();

        for (scope, alias, import_path) in all_glob_imports {
            let key = (scope.clone(), alias.clone());

            // Skip if already resolved
            if self.resolved_imports.contains_key(&key) {
                continue;
            }

            if let Some(resolved) = self.resolve_import_path_fully(&import_path, &scope, scope_resolver) {
                if alias.to_token_stream().to_string().contains("ProtocolError") && scope.to_string().contains("emergency_action") {
                    refine_import_dbg!("resolve_glob_imports_pass.insert {} as {} in {}", alias.to_token_stream(), resolved.to_token_stream(), scope.fmt_short());
                }
                self.resolved_imports.insert(key, resolved);
            }
        }
    }

    /// Resolve import path using only normalized paths and existing resolved imports
    fn resolve_import_path_fully(&self, import_path: &Path, declaring_scope: &ScopeChain, scope_resolver: &ScopeResolver) -> Option<Path> {
        // Use the corrected determine_full_qualified_path which includes proper reexport resolution
        if import_path.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("resolve_import_path_fully: {} in {}", import_path.to_token_stream(), declaring_scope.fmt_short());
        }

        let resolved = self.determine_full_qualified_path(import_path, declaring_scope, scope_resolver);

        if resolved.to_token_stream().to_string().contains("ProtocolError") && declaring_scope.to_string().contains("emergency_action") {
            refine_import_dbg!("resolve_import_path_fully:final_result: {}", resolved.to_token_stream());
        }

        Some(resolved)
    }

    // /// Resolve through import chains without using ReexportSeek
    // fn resolve_through_import_chains(&self, path: &Path, declaring_scope: &ScopeChain) -> Option<Path> {
    //     if path.segments.is_empty() {
    //         return None;
    //     }
    //
    //     // Check if the first segment is an import in the current scope or parent scopes
    //     let first_segment = &path.segments[0].ident;
    //     let first_segment_path = first_segment.to_path();
    //
    //     // Try current scope first
    //     if let Some(resolved_first) = self.resolve_first_segment_in_scope(&first_segment_path, declaring_scope) {
    //         let replaced = self.replace_first_segment(path, &resolved_first);
    //         if (replaced.to_token_stream().to_string().contains("ProtocolError")) && declaring_scope.to_string().contains("emergency_action") {
    //             refine_import_dbg!("resolve_import_path_fully:found_item_definition_in_scopes {}", replaced.to_token_stream());
    //         }
    //         return Some(replaced);
    //     }
    //
    //     // Try parent scopes
    //     let mut current_scope = declaring_scope;
    //     loop {
    //         match current_scope {
    //             ScopeChain::CrateRoot { .. } => break,
    //             ScopeChain::Mod { parent, .. } |
    //             ScopeChain::Fn { parent, .. } |
    //             ScopeChain::Trait { parent, .. } |
    //             ScopeChain::Object { parent, .. } |
    //             ScopeChain::Impl { parent, .. } => {
    //                 if let Some(resolved_first) = self.resolve_first_segment_in_scope(&first_segment_path, parent) {
    //                     let replaced = self.replace_first_segment(path, &resolved_first);
    //                     if (replaced.to_token_stream().to_string().contains("ProtocolError")) && declaring_scope.to_string().contains("emergency_action") {
    //                         refine_import_dbg!("resolve_import_path_fully:found_item_definition_in_scopes {}", replaced.to_token_stream());
    //                     }
    //                     return Some(replaced);
    //                 }
    //                 current_scope = parent;
    //             }
    //         }
    //     }
    //
    //     None
    // }
    //
    // fn resolve_first_segment_in_scope(&self, first_segment_path: &Path, scope: &ScopeChain) -> Option<Path> {
    //     // Check direct imports
    //     if let Some(imports) = self.inner.get(scope) {
    //         if let Some(resolved) = imports.get(first_segment_path) {
    //             return Some(resolved.clone());
    //         }
    //     }
    //
    //     // Check materialized glob imports
    //     if let Some(globs) = self.materialized_globs.get(scope) {
    //         if let Some(resolved) = globs.get(first_segment_path) {
    //             return Some(resolved.clone());
    //         }
    //     }
    //
    //     // Check already resolved imports
    //     let key = (scope.clone(), first_segment_path.clone());
    //     if let Some(resolved) = self.resolved_imports.get(&key) {
    //         return Some(resolved.clone());
    //     }
    //
    //     None
    // }
    //
    // fn replace_first_segment(&self, original_path: &Path, replacement: &Path) -> Path {
    //     let mut new_segments = replacement.segments.clone();
    //
    //     // Add remaining segments from original path (skip first segment)
    //     for segment in original_path.segments.iter().skip(1) {
    //         new_segments.push(segment.clone());
    //     }
    //
    //     Path {
    //         leading_colon: original_path.leading_colon,
    //         segments: new_segments,
    //     }
    // }
    //
    // fn find_valid_base_path(&self, path: &Path, scope_resolver: &ScopeResolver) -> Option<Path> {
    //     let mut test_path = path.clone();
    //     while test_path.segments.len() > 1 {
    //         test_path.segments.pop();
    //         if scope_resolver.maybe_scope(&test_path).is_some() {
    //             // Reconstruct with validated base + remaining segments
    //             let remaining: Vec<_> = path.segments.iter()
    //                 .skip(test_path.segments.len())
    //                 .cloned()
    //                 .collect();
    //             test_path.segments.extend(remaining);
    //             return Some(test_path);
    //         }
    //     }
    //
    //     // If no valid base found, return the original normalized path
    //     Some(path.clone())
    // }

    /// Fast O(1) lookup during refinement
    pub fn resolve_import_in_scope(&self, scope: &ScopeChain, import_path: &Path) -> Option<&Path> {
        self.resolved_imports.get(&(scope.clone(), import_path.clone()))
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

    // /// Search through all scopes to find where a single-segment item (like TxOut) is actually defined
    // fn find_item_definition_in_scopes(&self, item_name: &Path, scope_resolver: &ScopeResolver) -> Option<Path> {
    //     if item_name.segments.len() != 1 {
    //         return None;
    //     }
    //
    //     let target_ident = &item_name.segments[0].ident;
    //     let mut candidates = Vec::new();
    //
    //     // Search through all scopes in the scope resolver to find all potential definitions
    //     for (scope_path, type_chain) in scope_resolver.inner.iter() {
    //         // Check if this scope contains the target item
    //         for (type_ref, object_kind) in &type_chain.inner {
    //             // Only look for actual items, not type references
    //             if let crate::kind::ObjectKind::Item(_, _) = object_kind {
    //                 // Check if this item matches our target
    //                 if let Some(last_segment) = type_ref.to_path().segments.last() {
    //                     if last_segment.ident == *target_ident {
    //                         // Found a potential match! Store the full path
    //                         let full_path = scope_path.self_path_ref().joined(&target_ident.to_path());
    //                         candidates.push((full_path, scope_path.clone()));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // If we found candidates, prefer the most specific (longest) path
    //     // This ensures we get the actual definition, not reexports
    //     if !candidates.is_empty() {
    //         candidates.sort_by(|a, b| b.0.segments.len().cmp(&a.0.segments.len()));
    //         return Some(candidates[0].0.clone());
    //     }
    //
    //     None
    // }

    /// Find where an item is actually defined (not just reexported)
    /// This returns the most specific (longest) path where the item is defined
    fn find_actual_item_definition(&self, item_name: &Path, scope_resolver: &ScopeResolver) -> Option<Path> {
        if item_name.segments.len() != 1 {
            return None;
        }


        let target_ident = &item_name.segments[0].ident;
        let mut definition_paths = Vec::new();

        // Search through all scopes to find where this item is actually defined
        for (scope_path, type_chain) in scope_resolver.inner.iter() {
            for (type_ref, object_kind) in &type_chain.inner {
                // Only look for actual items, not type references
                if let crate::kind::ObjectKind::Item(_, _) = object_kind {
                    // Check if this item matches our target
                    if let Some(last_segment) = type_ref.to_path().segments.last() {
                        if last_segment.ident == *target_ident {
                            // Found a definition! Store the full path
                            let full_definition_path = scope_path.self_path_ref().joined(&target_ident.to_path());
                            definition_paths.push(full_definition_path);
                        }
                    }
                }
            }
        }

        // Return the most specific (longest) path - this should be the actual definition
        // rather than intermediate reexports
        if !definition_paths.is_empty() {
            definition_paths.sort_by(|a, b| b.segments.len().cmp(&a.segments.len()));
            return Some(definition_paths[0].clone());
        }

        None
    }
}
