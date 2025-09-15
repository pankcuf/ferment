use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Path, PathSegment};
use std::collections::{HashMap, HashSet};
use crate::ast::Colon2Punctuated;
use crate::context::GlobalContext;
use crate::ext::{Join, PathTransform, Pop, ToPath, CrateBased, CRATE, SELF, SUPER};
use crate::kind::ScopeItemKind;

// Lightweight debug toggle for reexport resolution
fn dbg_enabled() -> bool {
    std::env::var("FERMENT_DEBUG_REEXPORT").is_ok()
}
macro_rules! reexport_dbg {
    ($($arg:tt)*) => {
        if dbg_enabled() { println!($($arg)*); }
    }
}

// Advanced caching for path resolution results to avoid redundant work
thread_local! {
    static PATH_RESOLUTION_CACHE: std::cell::RefCell<HashMap<String, Option<Path>>> = std::cell::RefCell::new(HashMap::new());
    static ANCESTOR_SEARCH_CACHE: std::cell::RefCell<HashMap<String, Option<Path>>> = std::cell::RefCell::new(HashMap::new());
    static GLOB_SCOPE_CACHE: std::cell::RefCell<HashMap<String, bool>> = std::cell::RefCell::new(HashMap::new());
    static CANDIDATE_CACHE: std::cell::RefCell<HashMap<String, Vec<Path>>> = std::cell::RefCell::new(HashMap::new());
    static SCOPE_INDEX_CACHE: std::cell::RefCell<HashMap<String, HashSet<Ident>>> = std::cell::RefCell::new(HashMap::new());
}

// Performance monitoring for optimization impact
static CACHE_HIT_STATS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[allow(unused)]
pub fn get_cache_hit_count() -> usize {
    CACHE_HIT_STATS.load(std::sync::atomic::Ordering::Relaxed)
}

fn get_cached_resolution(path: &Path) -> Option<Option<Path>> {
    let key = path.to_token_stream().to_string();
    let result = PATH_RESOLUTION_CACHE.with(|cache| cache.borrow().get(&key).cloned());
    if result.is_some() {
        CACHE_HIT_STATS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    result
}

fn cache_resolution(path: &Path, result: Option<Path>) {
    let key = path.to_token_stream().to_string();
    PATH_RESOLUTION_CACHE.with(|cache| {
        cache.borrow_mut().insert(key, result);
    });
}

#[allow(unused)]
pub fn clear_resolution_cache() {
    PATH_RESOLUTION_CACHE.with(|cache| cache.borrow_mut().clear());
    ANCESTOR_SEARCH_CACHE.with(|cache| cache.borrow_mut().clear());
    GLOB_SCOPE_CACHE.with(|cache| cache.borrow_mut().clear());
    CANDIDATE_CACHE.with(|cache| cache.borrow_mut().clear());
    SCOPE_INDEX_CACHE.with(|cache| cache.borrow_mut().clear());
    CACHE_HIT_STATS.store(0, std::sync::atomic::Ordering::Relaxed);
}

// Cached glob scope checking to avoid repeated expensive operations
fn is_glob_scope_cached(path: &Path, source: &GlobalContext) -> bool {
    let key = path.to_token_stream().to_string();
    if let Some(cached) = GLOB_SCOPE_CACHE.with(|cache| cache.borrow().get(&key).cloned()) {
        CACHE_HIT_STATS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return cached;
    }

    let result = source.maybe_globs_scope_ref(path).is_some();
    GLOB_SCOPE_CACHE.with(|cache| {
        cache.borrow_mut().insert(key, result);
    });
    result
}

// Cached candidate generation for path resolution
fn get_cached_candidates(path: &Path, target: Option<&PathSegment>) -> Option<Vec<Path>> {
    let key = format!("{}#{}", path.to_token_stream(), target.as_ref().map(|t| t.ident.to_string()).unwrap_or_default());
    CANDIDATE_CACHE.with(|cache| cache.borrow().get(&key).cloned())
}

fn cache_candidates(path: &Path, target: Option<&PathSegment>, candidates: &[Path]) {
    let key = format!("{}#{}", path.to_token_stream(), target.as_ref().map(|t| t.ident.to_string()).unwrap_or_default());
    CANDIDATE_CACHE.with(|cache| {
        cache.borrow_mut().insert(key, candidates.to_vec());
    });
}

fn get_cached_ancestor_search(path: &Path) -> Option<Option<Path>> {
    let key = path.to_token_stream().to_string();
    ANCESTOR_SEARCH_CACHE.with(|cache| cache.borrow().get(&key).cloned())
}

fn cache_ancestor_search(path: &Path, result: Option<Path>) {
    let key = path.to_token_stream().to_string();
    ANCESTOR_SEARCH_CACHE.with(|cache| {
        cache.borrow_mut().insert(key, result);
    });
}

pub enum ReexportSeek {
    Absolute,
    Relative,
}

impl ReexportSeek {
    pub fn new(is_absolute: bool) -> Self {
        if is_absolute { ReexportSeek::Absolute } else { ReexportSeek::Relative }
    }

    fn join_reexport(&self, import_path: &Path, scope_path: &Path, crate_name: &Ident, chunk: Option<&Path>) -> Path {
        // TODO: deal with "super::super::"
        match self {
            ReexportSeek::Absolute => if let Some(PathSegment { ident, .. }) = import_path.segments.first() {
                match (ident.to_string().as_str(), chunk) {
                    (CRATE, Some(chunk_ref)) => {
                        let mut replaced = import_path.clone();
                        replaced.replace_first_with(&crate_name.to_path());
                        merge_reexport_chunks(replaced, chunk_ref)
                    },
                    (CRATE, None) => {
                        let mut replaced = import_path.clone();
                        replaced.replace_first_with(&crate_name.to_path());
                        replaced
                    },
                    (SELF, _) => {
                        let tail = Colon2Punctuated::from_iter(import_path.segments
                            .iter()
                            .skip(1)
                            .cloned())
                            .to_path();
                        let base = scope_path.joined(&tail);
                        chunk.map(|chunk_ref| base.joined(chunk_ref)).unwrap_or(base)
                    },
                    (SUPER, _) => {
                        // Support chained `super::super::...` by counting leading `super` segments
                        let mut super_count = 0usize;
                        for seg in import_path.segments.iter() {
                            if seg.ident == SUPER { super_count += 1; } else { break; }
                        }
                        let tail_after_super = Colon2Punctuated::from_iter(import_path.segments.iter().skip(super_count).cloned()).to_path();
                        // Pop `super_count` times from the base scope
                        let mut base = scope_path.clone();
                        for _ in 0..super_count { base = base.popped(); }
                        let base = base.joined(&tail_after_super);
                        chunk.map(|chunk_ref| base.joined(chunk_ref)).unwrap_or(base)
                    }
                    // For plain relative/import paths like `at_aa` or `bb::at_bb`, a glob base `base::*`
                    // combined with `chunk` should yield `base::chunk` (do not drop `base`).
                    (_, Some(chunk_ref)) =>
                        import_path.joined(chunk_ref),
                    (_, None) =>
                        import_path.clone()
                }
            } else {
                import_path.clone()
            }
            ReexportSeek::Relative => if let Some(chunk) = chunk {
                import_path.popped().joined(chunk)
            } else {
                import_path.clone()
            }
        }
    }
    pub(crate) fn maybe_reexport(&self, path: &Path, source: &GlobalContext) -> Option<Path> {
        // Check cache first for expensive resolutions
        if let Some(cached_result) = get_cached_resolution(path) {
            reexport_dbg!("[reexport] cache hit: {} -> {:?}", path.to_token_stream(), cached_result.as_ref().map(|p| p.to_token_stream()));
            return cached_result;
        }

        reexport_dbg!("[reexport] start: {}", path.to_token_stream());
        // Early deepening for pure-glob cases (e.g., aa::AtBb): derive from original parent if possible
        if let Some(last) = path.segments.last().cloned() {
            let parent = path.popped();
            if !parent.segments.is_empty() && is_uppercase_ident(&last.ident) && source.maybe_globs_scope_ref(&parent).is_some() {
                reexport_dbg!("[reexport] early-deepen: parent={} last={} (glob detected)", parent.to_token_stream(), last.ident.to_string());
                if let Some(derived) = derive_path_via_globs(&parent, &last, source) {
                    let finalized = finalize_to_leaf(derived, &last, source);
                    reexport_dbg!("[reexport] early-deepen -> {}", finalized.to_token_stream());
                    cache_resolution(path, Some(finalized.clone()));
                    return Some(finalized);
                }
            }
        }
        // Alias-chain first, then derive deep candidates from glob-capable ancestors; choose best, then canonicalize to leaf.
        let alias_mapped = resolve_import_chain(path.clone(), source);
        let orig_last = path.segments.last().cloned();
        let target = extract_symbol(&alias_mapped, orig_last.as_ref()).or(orig_last.clone());
        reexport_dbg!("[reexport] alias_mapped={} target={}", alias_mapped.to_token_stream(), target.as_ref().map(|t| t.ident.to_string()).unwrap_or("<none>".into()));

        // Check cache for candidate generation first
        let candidates = if let Some(cached_candidates) = get_cached_candidates(path, target.as_ref()) {
            reexport_dbg!("[reexport] cache hit for candidates");
            cached_candidates
        } else {
            // Generate candidates with optimized ancestor traversal
            let mut candidates = vec![alias_mapped.clone()];
            if let Some(t) = target.as_ref() {
                // Batch ancestor path generation to reduce repeated work
                let ancestors: Vec<Path> = {
                    let mut ancestors = Vec::new();
                    let mut anc = path.clone();

                    // Generate all ancestors in one pass
                    while !anc.segments.is_empty() {
                        let parent = anc.popped();
                        if parent.segments.is_empty() { break; }
                        ancestors.push(parent.clone());
                        anc = parent;
                    }
                    ancestors
                };

                // Process ancestors with cached glob scope checks
                for parent in ancestors {
                    if is_glob_scope_cached(&parent, source) {
                        reexport_dbg!("[reexport] ancestor glob at {}", parent.to_token_stream());
                        if let Some(derived) = derive_path_via_globs(&parent, t, source) {
                            candidates.push(derived);
                        }
                    }
                }
            }

            // Cache the generated candidates
            cache_candidates(path, target.as_ref(), &candidates);
            candidates
        };
        reexport_dbg!("[reexport] candidates: [{}]", candidates.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(", "));
        // Early pruning: if we have an exact match, return immediately
        for candidate in &candidates {
            if is_exact_path_match(path, candidate) && source.maybe_scope_item_ref_obj_first(candidate).is_some() {
                let finalized = canonicalize_to_leaf(candidate.clone(), target.as_ref(), source);
                reexport_dbg!("[reexport] select: exact-match -> {}", finalized.to_token_stream());
                cache_resolution(path, Some(finalized.clone()));
                return Some(finalized);
            }
        }

        // No descendant alias scan here; prefer real object definitions over shallow aliases only as a fallback below.
        if let Some(best) = candidates.iter().find(|p| source.maybe_scope_item_ref_obj_first(p).is_some()).cloned() {
            let finalized = canonicalize_to_leaf(best, target.as_ref(), source);
            reexport_dbg!("[reexport] select: by-object -> {}", finalized.to_token_stream());
            cache_resolution(path, Some(finalized.clone()));
            return Some(finalized);
        }
        if let Some(t) = target.as_ref() {
            let module_names = find_actual_containing_modules(t, &candidates, source);
            if let Some(best) = candidates.iter().find(|p| p.segments.iter().any(|s| module_names.contains(&s.ident))).cloned() {
                // Already deep; just ensure leaf symbol
                let finalized = finalize_to_leaf(best, t, source);
                reexport_dbg!("[reexport] select: by-module -> {}", finalized.to_token_stream());
                return Some(finalized);
            }
        }
        let finalized = canonicalize_to_leaf(alias_mapped.clone(), target.as_ref(), source);
        reexport_dbg!("[reexport] select: fallback -> {}", finalized.to_token_stream());
        if let Some(t) = target.as_ref() {
            // Absolute last resort: scan all known scopes for a defined object named `t` and pick the best path
            // Use the base path (without the target) as context for better matching
            let base_path_for_context = path.popped(); // Remove the target from the original path
            if let Some(mapped) = find_defined_object_path_with_context(t, &base_path_for_context, source) {
                reexport_dbg!("[reexport] select: fallback-by-defined -> {}", mapped.to_token_stream());
                cache_resolution(path, Some(mapped.clone()));
                return Some(mapped);
            }
        }
        cache_resolution(path, Some(finalized.clone()));
        Some(finalized)
    }
}


fn find_actual_containing_modules(target: &PathSegment, base_paths: &[Path], source: &GlobalContext) -> HashSet<Ident> {
    // Create a cache key that represents this search
    let cache_key = format!("{}#{}",
        target.ident.to_string(),
        base_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(","));

    // Check cache first
    if let Some(cached_modules) = SCOPE_INDEX_CACHE.with(|cache| cache.borrow().get(&cache_key).cloned()) {
        CACHE_HIT_STATS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return cached_modules;
    }

    let mut modules = HashSet::new();

    // Pre-filter scopes to reduce iteration overhead
    let relevant_scopes: Vec<_> = source.scope_register.inner.keys()
        .filter(|scope| {
            let scope_path = scope.self_path_ref();
            // Quick filter: scope must be longer than any base path to be relevant
            base_paths.iter().any(|base_path| scope_path.segments.len() > base_path.segments.len())
        })
        .collect();

    // For each base path, search pre-filtered scopes
    for base_path in base_paths {
        for scope in &relevant_scopes {
            let scope_path = scope.self_path_ref();

            // Early exit conditions for better performance
            if scope_path.segments.len() <= base_path.segments.len() {
                continue;
            }

            // Check if scope_path starts with base_path (optimized comparison)
            let is_descendant = base_path.segments.len() == 0 ||
                base_path.segments.iter()
                    .zip(scope_path.segments.iter())
                    .all(|(base_seg, scope_seg)| base_seg.ident == scope_seg.ident);

            if is_descendant && source.maybe_scope_item_ref_obj_first(&scope_path.joined(&target.ident.to_path())).is_some() {
                // Find the immediate child module of base_path that leads to the target
                if let Some(child_module) = scope_path.segments.get(base_path.segments.len()) {
                    modules.insert(child_module.ident.clone());
                }
            }
        }
    }

    // Cache the result
    SCOPE_INDEX_CACHE.with(|cache| {
        cache.borrow_mut().insert(cache_key, modules.clone());
    });

    modules
}


fn finalize_to_leaf(p: Path, target: &PathSegment, source: &GlobalContext) -> Path {
    // If already ends with target, done
    if p.segments.last().map(|s| s.ident == target.ident).unwrap_or_default() {
        return p;
    }
    // Strategy 1: Look for actual item definitions in registered scopes
    let mut best_path: Option<Path> = None;
    let mut max_depth = 0;
    let mut best_exact_match: Option<Path> = None;

    for scope in source.scope_register.inner.keys() {
        let scope_path = scope.self_path_ref();
        let candidate = scope_path.joined(&target.ident.to_path());

        if source.maybe_scope_item_ref_obj_first(&candidate).is_some() {
            // Check if this scope path is compatible with our base path
            if is_path_compatible(&p, &scope_path) {
                // Check if this candidate path exactly matches what we're looking for
                // by checking if the base path segments appear consecutively in the candidate
                if is_exact_path_match(&p, &candidate) {
                    best_exact_match = Some(candidate.clone());
                }
                if candidate.segments.len() > max_depth {
                    max_depth = candidate.segments.len();
                    best_path = Some(candidate);
                }
            }
        }
    }

    // Prefer exact match to anything else
    if let Some(exact_match) = best_exact_match {
        return exact_match;
    }
    if let Some(best) = best_path {
        return best;
    }
    // Strategy 2: Follow glob imports to find the deepest reachable path
    if let Some(deep_path) = find_deepest_path_via_globs(&p, target, source) {
        reexport_dbg!("[reexport] finalize_to_leaf: found path via globs -> {}", deep_path.to_token_stream());
        return deep_path;
    }
    // Fallback: just append target to the current path
    p.joined(&target.ident.to_path())
}

fn is_exact_path_match(base: &Path, candidate: &Path) -> bool {
    let base_len = base.segments.len();
    let candidate_len = candidate.segments.len();

    if base_len == 0 {
        return true;
    }
    if candidate_len < base_len {
        return false;
    }

    // Skip the first segment (usually the crate name) and look for the rest as a subsequence
    if base_len > 1 {
        let meaningful_base_len = base_len - 1;
        if candidate_len > meaningful_base_len {
            // Use iterator-based approach for better performance
            'outer: for start_pos in 1..=(candidate_len - meaningful_base_len) {
                let mut base_iter = base.segments.iter().skip(1);
                let mut candidate_iter = candidate.segments.iter().skip(start_pos);

                for _i in 0..meaningful_base_len {
                    if let (Some(base_seg), Some(cand_seg)) = (base_iter.next(), candidate_iter.next()) {
                        if base_seg.ident != cand_seg.ident {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }
                return true;
            }
        }
    }

    // Fallback to original exact consecutive matching with iterator optimization
    'fallback: for start_pos in 0..=(candidate_len - base_len) {
        let mut base_iter = base.segments.iter();
        let mut candidate_iter = candidate.segments.iter().skip(start_pos);

        for _i in 0..base_len {
            if let (Some(base_seg), Some(cand_seg)) = (base_iter.next(), candidate_iter.next()) {
                if base_seg.ident != cand_seg.ident {
                    continue 'fallback;
                }
            } else {
                continue 'fallback;
            }
        }
        return true;
    }
    false
}


fn is_path_compatible(base: &Path, scope: &Path) -> bool {
    if base.segments.is_empty() {
        return true;
    }
    // Check if scope contains all segments from base in order
    let mut base_idx = 0;
    for scope_seg in &scope.segments {
        if base_idx < base.segments.len() && scope_seg.ident == base.segments[base_idx].ident {
            base_idx += 1;
        }
    }
    base_idx == base.segments.len()
}

fn find_deepest_path_via_globs(base: &Path, target: &PathSegment, source: &GlobalContext) -> Option<Path> {
    reexport_dbg!("[reexport] find_deepest_path_via_globs: base={}", base.to_token_stream());

    // If there's a direct glob at the base path, follow it
    if let Some(scope) = source.maybe_globs_scope_ref(base) {
        if let Some(glob_bases) = source.imports.maybe_scope_globs(scope) {
            reexport_dbg!("[reexport] find_deepest_path_via_globs: found {} glob bases", glob_bases.len());
            // Try each glob base to see if it leads to a deeper path
            for glob_base in glob_bases {
                reexport_dbg!("[reexport] find_deepest_path_via_globs: trying glob_base={}", glob_base.to_token_stream());
                let expanded_path = ReexportSeek::Absolute.join_reexport(glob_base, scope.self_path_ref(), scope.crate_ident_ref(), None);
                // Normalize the path
                let normalized = if expanded_path.is_crate_based() || expanded_path.segments.first().map(|s| s.ident.eq(scope.crate_ident_ref())).unwrap_or_default() {
                    expanded_path
                } else {
                    base.joined(&expanded_path)
                };
                reexport_dbg!("[reexport] find_deepest_path_via_globs: normalized={}", normalized.to_token_stream());
                // The target would be at this expanded path
                let target_candidate = normalized.joined(&target.ident.to_path());
                reexport_dbg!("[reexport] find_deepest_path_via_globs: returning target_candidate={}", target_candidate.to_token_stream());
                return Some(target_candidate);
            }
        }
    }

    reexport_dbg!("[reexport] find_deepest_path_via_globs: no glob found at base");
    None
}

fn is_uppercase_ident(ident: &Ident) -> bool {
    ident.to_string().chars().next().map(char::is_uppercase).unwrap_or_default()
}

fn extract_symbol(mapped: &Path, orig: Option<&PathSegment>) -> Option<PathSegment> {
    // Prefer the last uppercase segment from the mapped path (true target if rename applied)
    if let Some(seg) = mapped.segments.iter().rev().find(|s| is_uppercase_ident(&s.ident)) {
        return Some(seg.clone());
    }
    // Fallback to original if it looks like a type
    if let Some(o) = orig {
        if is_uppercase_ident(&o.ident) {
            return Some(o.clone());
        }
    }
    None
}

fn canonicalize_to_leaf(p: Path, orig: Option<&PathSegment>, source: &GlobalContext) -> Path {
    let target = match extract_symbol(&p, orig) {
        Some(t) => t,
        None => return p
    };
    let parent = p.popped();
    if source.maybe_globs_scope_ref(&parent).is_some() {
        reexport_dbg!("[reexport] canonicalize: parent glob at {}", parent.to_token_stream());
        if let Some(derived) = derive_path_via_globs(&parent, &target, source) {
            // If the base scope where leaf resides has a rename for `target`, prefer the mapped path.
            let base_scope = derived.popped();
            if let Some((scope, import)) = source.maybe_import_scope_pair_ref(&target, &base_scope) {
                let mapped = ReexportSeek::Absolute.join_reexport(import, scope.self_path_ref(), scope.crate_ident_ref(), None);
                reexport_dbg!("[reexport] canonicalize: parent-mapped-leaf -> {}", mapped.to_token_stream());
                return mapped;
            }
            let finalized = finalize_to_leaf(derived, &target, source);
            reexport_dbg!("[reexport] canonicalize: parent-derived -> {}", finalized.to_token_stream());
            return finalized;
        }
    }
    // Try ancestors
    let mut anc = parent.popped();
    while !anc.segments.is_empty() {
        if source.maybe_globs_scope_ref(&anc).is_some() {
            reexport_dbg!("[reexport] canonicalize: ancestor glob at {}", anc.to_token_stream());
            if let Some(derived) = derive_path_via_globs(&anc, &target, source) {
                // Apply local rename mapping if present at the base scope
                let base_scope = derived.popped();
                if let Some((scope, import)) = source.maybe_import_scope_pair_ref(&target, &base_scope) {
                    let mapped = ReexportSeek::Absolute.join_reexport(import, scope.self_path_ref(), scope.crate_ident_ref(), None);
                    reexport_dbg!("[reexport] canonicalize: ancestor-mapped-leaf -> {}", mapped.to_token_stream());
                    return mapped;
                }
                let finalized = finalize_to_leaf(derived, &target, source);
                reexport_dbg!("[reexport] canonicalize: ancestor-derived -> {}", finalized.to_token_stream());
                return finalized;
            }
        }
        anc = anc.popped();
    }
    // No glob info found up the chain — keep as-is
    // As a last resort for alias names reachable only via nested globs (e.g., dd defines `AtDd as DdAlias`),
    // locate the descendant scope that imports `target` and join its mapped path.
    if let Some(alias_scope) = derive_alias_scope_via_globs(&parent, &target, source) {
        if let Some((scope, import)) = source.maybe_import_scope_pair_ref(&target, &alias_scope) {
            let mapped = ReexportSeek::Absolute.join_reexport(import, scope.self_path_ref(), scope.crate_ident_ref(), None);
            reexport_dbg!("[reexport] canonicalize: alias-scope via globs -> {}", mapped.to_token_stream());
            return mapped;
        }
    }
    p
}

fn resolve_import_chain(mut path: Path, source: &GlobalContext) -> Path {
    // Follow alias mappings for the last segment up the ancestor scopes, repeatedly.
    // Stops when no further mapping is found or a stable path is reached.
    let mut guard = 0usize;
    loop {
        if guard > 16 { break; }
        guard += 1;
        let last = match path.segments.last().cloned() {
            Some(s) => s,
            None => break
        };
        let mut candidate = path.popped();
        let mut mapped: Option<Path> = None;
        // climb ancestors to find the scope where this alias is imported
        loop {
            if let Some((scope, import)) = source.maybe_import_scope_pair_ref(&last, &candidate) {
                let scope_path = scope.self_path_ref();
                let rp = ReexportSeek::Absolute.join_reexport(import, scope_path, scope.crate_ident_ref(), None);
                let is_abs = rp.segments.first().map(|seg| seg.ident.eq(scope.crate_ident_ref())).unwrap_or_default() || rp.is_crate_based();
                mapped = Some(if is_abs { rp.clone() } else { scope_path.joined(&rp) });
                break;
            }
            if candidate.segments.is_empty() { break; }
            candidate = candidate.popped();
        }
        match mapped {
            Some(new_path) if new_path.to_token_stream().to_string() != path.to_token_stream().to_string() => {
                path = new_path;
            }
            _ => break,
        }
    }
    path
}

// Fallback: if a path looks like `...::Module::Type` but alias chain can't be followed (e.g.,
// due to `pub use submod::*;`), scan descendant module scopes of `Module` and find one that
// actually defines `Type`. Returns the full path to that definition if found.
// (Removed legacy: resolve_by_scanning_descendants)

// Depth-first expansion through nested glob reexports.
// Starting from an absolute base scope path (e.g., aa::bb::cc), try to resolve `last` by checking
// `base::last`; if not found, and `base` has its own glob bases, recursively try `base::<sub>::last`.

// Search for a descendant scope reachable via nested globs from `base_abs`
// that defines an alias import for `alias`.
fn find_scope_with_alias_via_globs(base_abs: &Path, alias: &PathSegment, source: &GlobalContext, depth: usize) -> Option<Path> {
    if depth > 8 { return None; }
    if let Some(scope) = source.maybe_imports_scope_ref(base_abs) {
        let key = crate::ext::GenericBoundKey::ident(&alias.ident);
        if source.maybe_import_path_ref(scope, &key).is_some() {
            return Some(base_abs.clone());
        }
    }
    if let Some(base_scope) = source.maybe_globs_scope_ref(base_abs) {
        if let Some(sub_bases) = source.imports.maybe_scope_globs(base_scope) {
            for sub in sub_bases.iter() {
                let sub_abs_rel = ReexportSeek::Absolute.join_reexport(sub, base_scope.self_path_ref(), base_scope.crate_ident_ref(), None);
                let is_abs = sub_abs_rel.is_crate_based() || sub_abs_rel.segments.first().map(|seg| seg.ident.eq(base_scope.crate_ident_ref())).unwrap_or_default();
                let sub_abs = if is_abs { sub_abs_rel } else { base_abs.joined(&sub_abs_rel) };
                if let Some(found) = find_scope_with_alias_via_globs(&sub_abs, alias, source, depth + 1) {
                    return Some(found);
                }
            }
        }
    }
    None
}

fn derive_alias_scope_via_globs(start_scope_path: &Path, alias: &PathSegment, source: &GlobalContext) -> Option<Path> {
    let mut current = start_scope_path.clone();
    let mut guard = 0usize;
    while guard < 8 {
        guard += 1;
        if let Some(found) = find_scope_with_alias_via_globs(&current, alias, source, 0) {
            return Some(found);
        }
        current = current.popped();
        if current.segments.is_empty() { break; }
    }
    None
}

// Deterministically build a plausible reexport path by following glob bases
// and preferring modules named `at_<snake(last)>` when present.
fn derive_path_via_globs(start_scope_path: &Path, last: &PathSegment, source: &GlobalContext) -> Option<Path> {
    derive_path_via_globs_with_depth(start_scope_path, last, source, 0)
}

fn derive_path_via_globs_with_depth(start_scope_path: &Path, last: &PathSegment, source: &GlobalContext, depth: usize) -> Option<Path> {
    if depth > 8 { return None; }

    reexport_dbg!("[reexport] derive_path_via_globs: start_scope_path={} target={} depth={}", start_scope_path.to_token_stream(), last.ident, depth);
    let mut current = start_scope_path.clone();
    let mut guard = 0usize;
    while guard < 8 {
        guard += 1;
        let scope = match source.maybe_globs_scope_ref(&current) {
            Some(s) => s,
            None => {
                reexport_dbg!("[reexport] derive_path_via_globs: no scope at {}", current.to_token_stream());
                break
            }
        };
        let bases = match source.imports.maybe_scope_globs(scope) {
            Some(b) => b,
            None => {
                reexport_dbg!("[reexport] derive_path_via_globs: no glob bases at {}", current.to_token_stream());
                break
            }
        };
        if bases.is_empty() { break; }
        reexport_dbg!("[reexport] derive_path_via_globs: found {} bases at {}", bases.len(), current.to_token_stream());
        for b in bases.iter() {
            reexport_dbg!("[reexport] derive_path_via_globs: trying base {}", b.to_token_stream());
            let base_abs_rel = ReexportSeek::Absolute.join_reexport(b, scope.self_path_ref(), scope.crate_ident_ref(), None);
            let first_is_crate = base_abs_rel.segments.first().map(|seg| seg.ident.eq(scope.crate_ident_ref())).unwrap_or_default() || base_abs_rel.is_crate_based();
            let base_abs = if first_is_crate { base_abs_rel } else { current.joined(&base_abs_rel) };
            reexport_dbg!("[reexport] derive_path_via_globs: base_abs={}", base_abs.to_token_stream());

            // Check if this expanded path has further globs we should follow
            if let Some(deeper) = derive_path_via_globs_with_depth(&base_abs, last, source, depth + 1) {
                reexport_dbg!("[reexport] derive_path_via_globs: found deeper path -> {}", deeper.to_token_stream());
                return Some(deeper);
            }

            // If no deeper path, verify the target actually exists at this location before returning
            let target_path = base_abs.joined(&last.ident.to_path());
            if source.maybe_scope_item_ref_obj_first(&target_path).is_some() {
                reexport_dbg!("[reexport] derive_path_via_globs: verified target exists, returning -> {}", target_path.to_token_stream());
                return Some(target_path);
            } else {
                reexport_dbg!("[reexport] derive_path_via_globs: target does not exist at -> {}", target_path.to_token_stream());
            }
        }
        current = current.popped();
        if current.segments.is_empty() { break; }
    }
    reexport_dbg!("[reexport] derive_path_via_globs: no result found");
    None
}

fn merge_reexport_chunks(mut base: Path, extension: &Path) -> Path {
    let mut base_segments = base.segments.iter().collect::<Vec<_>>();
    let mut ext_segments = extension.segments.iter().collect::<Vec<_>>();
    base_segments.reverse();
    ext_segments.reverse();
    let mut result_segments = vec![];
    let mut skip = 0;
    for (base_segment, ext_segment) in base_segments.iter().zip(ext_segments.iter()) {
        if base_segment.ident == ext_segment.ident {
            skip += 1;
        } else {
            break;
        }
    }
    base_segments.reverse();
    ext_segments.reverse();
    let base_len = base_segments.len();
    result_segments.extend(base_segments.into_iter().take(base_len - skip));
    result_segments.extend(ext_segments);
    base.segments = result_segments.into_iter().cloned().collect();
    base
}

// Find a scope where the leaf symbol is actually defined; return the most specific path.
// Now takes the base path context to prefer exact matches
fn find_defined_object_path_with_context(target: &PathSegment, base_path: &Path, source: &GlobalContext) -> Option<Path> {

    let mut exact_matches: Vec<Path> = Vec::new();
    let mut best_shallow: Option<(usize, Path)> = None;
    let mut best_deep: Option<(usize, Path)> = None;

    // Early termination: if we find an exact match, we can stop searching immediately
    let mut found_exact_match = false;

    for scope in source.scope_register.inner.keys() {
        // Early termination: if we already found an exact match and have fallbacks, we can stop
        if found_exact_match && best_deep.is_some() {
            break;
        }

        let scope_path = scope.self_path_ref();
        let candidate = scope_path.joined(&target.ident.to_path());

        if source.maybe_scope_item_ref_obj_first(&candidate).is_some() {
            let depth = candidate.segments.len();

            // Check if this candidate matches the path structure we're looking for
            if is_exact_path_match(base_path, &candidate) {
                exact_matches.push(candidate.clone());
                found_exact_match = true;
                // Don't continue here - we might want to collect multiple exact matches
            } else {
                // Only track fallbacks if we haven't found exact matches yet
                if !found_exact_match {
                    // Track the deepest path (most specific definition)
                    match &best_deep {
                        Some((deepest_depth, _)) if *deepest_depth <= depth => {},
                        _ => best_deep = Some((depth, candidate.clone()))
                    }
                    // Also track the shallowest path
                    match &best_shallow {
                        Some((best_depth, _)) if *best_depth >= depth => {},
                        _ => best_shallow = Some((depth, candidate.clone()))
                    }
                }
            }
        }
    }

    // Strongly prefer exact matches
    if !exact_matches.is_empty() {
        return Some(exact_matches[0].clone());
    }

    // Fallback to the deepest path (most specific actual definition)
    if let Some((_, deep_path)) = best_deep {
        return Some(deep_path);
    }

    // Fallback to the shallowest path
    if let Some((_, shallow_path)) = best_shallow {
        return Some(shallow_path);
    }

    None
}

// Helper function to find the best path candidate based on path structure matching
fn find_best_path_candidate(target_path: &Path, candidates: &[(Path, Path)]) -> Option<(Path, Path)> {
    if candidates.is_empty() {
        return None;
    }


    // First, try to find exact path structure matches
    for &(ref ancestor_path, ref candidate_path) in candidates.iter() {
        if is_exact_path_match2(target_path, candidate_path) {
            return Some((ancestor_path.clone(), candidate_path.clone()));
        }
    }

    // If no exact match, return the first candidate (fallback to original behavior)
    Some(candidates[0].clone())
}

// Import the exact path matching logic from reexport.rs
fn is_exact_path_match2(base: &Path, candidate: &Path) -> bool {
    // Check if the base path segments appear consecutively in the candidate path
    if base.segments.is_empty() {
        return true;
    }

    if candidate.segments.len() < base.segments.len() {
        return false;
    }

    let base_segments: Vec<_> = base.segments.iter().collect();
    let candidate_segments: Vec<_> = candidate.segments.iter().collect();

    // Skip the first segment (usually the crate name) and look for the rest as a subsequence
    for start_pos in 0..=(candidate_segments.len() - base_segments.len()) {
        let mut matches = true;
        for (i, base_seg) in base_segments.iter().enumerate() {
            if candidate_segments[start_pos + i].ident != base_seg.ident {
                matches = false;
                break;
            }
        }
        if matches {
            return true;
        }
    }

    // If no exact match, try pattern matching for common cases where paths have
    // similar structure but with additional intermediate segments
    if base_segments.len() >= 2 {
        let first_base = &base_segments[0];
        let last_base = base_segments.last().unwrap();

        // Check if first and last segments match, and middle segments appear as a subsequence
        if candidate_segments.first().map(|seg| seg.ident == first_base.ident).unwrap_or_default() &&
            candidate_segments.last().map(|seg| seg.ident == last_base.ident).unwrap_or_default() {

            // Check if the middle segments of base appear as a contiguous subsequence in candidate
            return if base_segments.len() > 2 {
                is_subsequence_contiguous(&base_segments[1..base_segments.len() - 1], &candidate_segments[1..candidate_segments.len() - 1])
            } else {
                true // Only first and last match, which is sufficient
            }
        }
    }

    false
}

// Helper function to check if `needle` appears as a contiguous subsequence in `haystack`
fn is_subsequence_contiguous(needle: &[&syn::PathSegment], haystack: &[&syn::PathSegment]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }

    for start_pos in 0..=(haystack.len() - needle.len()) {
        let mut matches = true;
        for (i, needle_seg) in needle.iter().enumerate() {
            if haystack[start_pos + i].ident != needle_seg.ident {
                matches = false;
                break;
            }
        }
        if matches {
            return true;
        }
    }

    false
}


pub fn find_best_ancestor<'a>(resolved_import_path: &Path, source: &'a GlobalContext) -> Option<&'a ScopeItemKind> {
    // Check cache first - this is an expensive operation
    if let Some(cached_path) = get_cached_ancestor_search(resolved_import_path) {
        return cached_path.and_then(|p| source.maybe_scope_item_ref_obj_first(&p));
    }

    // Find the best existing ancestor scope and scan its descendants for a matching leaf
    let mut anc = resolved_import_path.popped();
    let last_seg = resolved_import_path.segments.last().cloned();
    let mut best_candidate: Option<(usize, Path, Path)> = None; // (depth, ancestor_path, candidate_path)

    while !anc.segments.is_empty() {
        if let Some(ancestor_scope) = source.maybe_scope_ref(&anc) {
            if let Some(last_seg) = last_seg.as_ref() {
                let ancestor_path = ancestor_scope.self_path_ref();

                // Find all possible candidates and prioritize based on path structure
                let mut candidates = Vec::new();
                for scope_chain in source.scope_register.inner.keys() {
                    let scope_path = scope_chain.self_path_ref();
                    if ancestor_path.segments.len() <= scope_path.segments.len()
                        && scope_path.segments.iter().zip(ancestor_path.segments.iter()).all(|(a, b)| a.ident == b.ident) {
                        let already_matches_last = scope_path.segments.last().map(|s| s.ident == last_seg.ident).unwrap_or_default();
                        let candidate = if already_matches_last { scope_path.clone() } else { scope_path.joined(&last_seg.ident.to_path()) };
                        if source.maybe_scope_item_ref_obj_first(&candidate).is_some() {
                            candidates.push((ancestor_path.clone(), candidate));
                        }
                    }
                }

                // Find the best candidate using path structure matching
                if let Some(best_found) = find_best_path_candidate(&resolved_import_path, &candidates) {
                    let depth = best_found.0.segments.len();
                    let should_update = match &best_candidate {
                        None => true,
                        Some((best_depth, _, _)) => {
                            // Prioritize exact matches, then depth
                            depth > *best_depth
                        }
                    };

                    if should_update {
                        best_candidate = Some((depth, best_found.0, best_found.1));
                    }
                }
            }
        }
        anc = anc.popped();
    }

    if let Some((_, ancestor_path, candidate)) = best_candidate {
        reexport_dbg!("Found the best ancestor: {}", ancestor_path.to_token_stream());
        cache_ancestor_search(resolved_import_path, Some(candidate.clone()));
        source.maybe_scope_item_ref_obj_first(&candidate)
    } else {
        cache_ancestor_search(resolved_import_path, None);
        None
    }
}