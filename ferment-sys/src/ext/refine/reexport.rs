use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;
use crate::context::{GlobalContext, ScopeChain};
use crate::ext::{Join, PathTransform, Pop, ToPath, CrateBased, CRATE, SELF, SUPER};

// Lightweight debug toggle for reexport resolution
fn reexport_dbg_enabled() -> bool {
    std::env::var("FERMENT_DEBUG_REEXPORT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes"))
        .unwrap_or(false)
}
macro_rules! reexport_dbg {
    ($($arg:tt)*) => {
        if reexport_dbg_enabled() { println!($($arg)*); }
    }
}

pub enum ReexportSeek {
    Absolute,
    Relative,
}

impl ReexportSeek {

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
        reexport_dbg!("[reexport] start: {}", path.to_token_stream());
        // Early deepening for pure-glob cases (e.g., aa::AtBb): derive from original parent if possible
        if let Some(last) = path.segments.last().cloned() {
            let parent = path.popped();
            if !parent.segments.is_empty() && is_uppercase_ident(&last.ident) && source.maybe_globs_scope_ref(&parent).is_some() {
                reexport_dbg!("[reexport] early-deepen: parent={} last={} (glob detected)", parent.to_token_stream(), last.ident.to_string());
                if let Some(derived) = derive_path_via_globs(&parent, &last, source) {
                    let finalized = finalize_to_leaf(derived, &last);
                    reexport_dbg!("[reexport] early-deepen -> {}", finalized.to_token_stream());
                    return Some(finalized);
                }
            }
        }
        // Alias-chain first, then derive deep candidates from glob-capable ancestors; choose best, then canonicalize to leaf.
        let alias_mapped = resolve_import_chain(path.clone(), source);
        let orig_last = path.segments.last().cloned();
        let target = extract_symbol(&alias_mapped, orig_last.as_ref()).or(orig_last.clone());
        reexport_dbg!("[reexport] alias_mapped={} target={}", alias_mapped.to_token_stream(), target.as_ref().map(|t| t.ident.to_string()).unwrap_or("<none>".into()));

        let mut candidates: Vec<Path> = vec![alias_mapped.clone()];
        if let Some(t) = target.as_ref() {
            let mut anc = path.clone();
            while !anc.segments.is_empty() {
                let parent = anc.popped();
                if parent.segments.is_empty() { break; }
                if source.maybe_globs_scope_ref(&parent).is_some() {
                    reexport_dbg!("[reexport] ancestor glob at {}", parent.to_token_stream());
                    if let Some(derived) = derive_path_via_globs(&parent, t, source) { candidates.push(derived); }
                }
                anc = parent;
            }
        }

        reexport_dbg!("[reexport] candidates: [{}]", candidates.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(", "));
        // No descendant alias scan here; prefer real object definitions over shallow aliases only as a fallback below.
        if let Some(best) = candidates.iter().find(|p| source.maybe_scope_item_ref_obj_first(p).is_some()).cloned() {
            let finalized = canonicalize_to_leaf(best, target.as_ref(), source);
            reexport_dbg!("[reexport] select: by-object -> {}", finalized.to_token_stream());
            return Some(finalized);
        }
        if let Some(t) = target.as_ref() {
            let pref = at_module_name_of(t);
            if let Some(best) = candidates.iter().find(|p| p.segments.iter().any(|s| s.ident.eq(&pref))).cloned() {
                // Already deep; just ensure leaf symbol
                let finalized = finalize_to_leaf(best, t);
                reexport_dbg!("[reexport] select: by-atmod -> {}", finalized.to_token_stream());
                return Some(finalized);
            }
        }
        let finalized = canonicalize_to_leaf(alias_mapped.clone(), target.as_ref(), source);
        reexport_dbg!("[reexport] select: fallback -> {}", finalized.to_token_stream());
        if let Some(t) = target.as_ref() {
            // Absolute last resort: scan all known scopes for a defined object named `t` and pick the deepest path
            if let Some(mapped) = find_defined_object_path(t, source) {
                reexport_dbg!("[reexport] select: fallback-by-defined -> {}", mapped.to_token_stream());
                return Some(mapped);
            }
        }
        Some(finalized)
    }
}

fn camel_to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 { out.push('_'); }
            for lc in ch.to_lowercase() { out.push(lc); }
        } else {
            out.push(ch);
        }
    }
    out
}

fn at_module_name_of(target: &PathSegment) -> String {
    let snake = camel_to_snake(&target.ident.to_string());
    if snake.starts_with("at_") { snake } else { format!("at_{}", snake) }
}

fn is_lowercase_ident(ident: &Ident) -> bool {
    ident.to_string().chars().next().map(|c| c.is_lowercase()).unwrap_or_default()
}

fn finalize_to_leaf(p: Path, target: &PathSegment) -> Path {
    // If already ends with target, done
    if p.segments.last().map(|s| s.ident == target.ident).unwrap_or_default() {
        return p;
    }
    let at_mod = at_module_name_of(target);
    let last_is_at_mod = p.segments.last().map(|s| s.ident.eq(&at_mod)).unwrap_or_default();
    if last_is_at_mod {
        // Append the leaf symbol
        return p.joined(&target.ident.to_path());
    }
    // If ends with lowercase (likely module), try to replace it with the leaf symbol when previous is at_mod
    if let Some(last) = p.segments.last().cloned() {
        if is_lowercase_ident(&last.ident) {
            // If previous is at_mod, replace last with target; else append target
            let mut replaced = p.clone();
            let prev_is_at_mod = p.segments.iter().rev().nth(1).map(|s| s.ident.eq(&at_mod)).unwrap_or_default();
            if prev_is_at_mod {
                // replace last with target
                replaced.segments.pop();
                replaced.segments.push(target.clone());
                return replaced;
            } else {
                return p.joined(&target.ident.to_path());
            }
        }
    }
    // Default: append target
    p.joined(&target.ident.to_path())
}

fn is_uppercase_ident(ident: &proc_macro2::Ident) -> bool {
    ident.to_string().chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn extract_symbol(mapped: &Path, orig: Option<&PathSegment>) -> Option<PathSegment> {
    // Prefer the last uppercase segment from the mapped path (true target if rename applied)
    if let Some(seg) = mapped.segments.iter().rev().find_map(|s| if is_uppercase_ident(&s.ident) { Some(s.clone()) } else { None }) { return Some(seg); }
    // Fallback to original if it looks like a type
    if let Some(o) = orig { if is_uppercase_ident(&o.ident) { return Some(o.clone()); } }
    None
}

fn canonicalize_to_leaf(p: Path, orig: Option<&PathSegment>, source: &GlobalContext) -> Path {
    let target = match extract_symbol(&p, orig) { Some(t) => t, None => return p };
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
            let finalized = finalize_to_leaf(derived, &target);
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
                let finalized = finalize_to_leaf(derived, &target);
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
        let last = match path.segments.last().cloned() { Some(s) => s, None => break };
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
fn resolve_by_nested_globs(base_abs: &Path, last: &PathSegment, source: &GlobalContext, depth: usize) -> Option<Path> {
    if depth > 8 { return None; }
    // base::last
    let candidate = base_abs.joined(&last.ident.to_path());
    // If base tail matches the common module naming, accept candidate directly
    let preferred_tail = at_module_name_of(last);
    if base_abs.segments.last().map(|s| s.ident.to_string()) == Some(preferred_tail.clone()) {
        return Some(candidate);
    }
    if source.maybe_scope_item_ref_obj_first(&candidate).is_some() {
        return Some(candidate);
    }
    // Look for globs under this base scope and try each sub-base
    if let Some(base_scope) = source.maybe_globs_scope_ref(base_abs) {
        if let Some(sub_bases) = source.imports.maybe_scope_globs(base_scope) {
            for sub in sub_bases.iter() {
                // Normalize sub-base under this scope
                let sub_abs_rel = ReexportSeek::Absolute.join_reexport(sub, base_scope.self_path_ref(), base_scope.crate_ident_ref(), None);
                let is_abs = sub_abs_rel.is_crate_based() || sub_abs_rel.segments.first().map(|seg| seg.ident.eq(base_scope.crate_ident_ref())).unwrap_or_default();
                let sub_abs = if is_abs { sub_abs_rel } else { base_abs.joined(&sub_abs_rel) };
                // If sub base tail matches preferred, return synthesized path
                if sub_abs.segments.last().map(|s| s.ident.to_string()) == Some(preferred_tail.clone()) {
                    return Some(sub_abs.joined(&last.ident.to_path()));
                }
                if let Some(found) = resolve_by_nested_globs(&sub_abs, last, source, depth + 1) {
                    return Some(found);
                }
            }
        }
    }
    // No match found under this base
    None
}

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
    let preferred_tail = at_module_name_of(last);
    let mut current = start_scope_path.clone();
    let mut guard = 0usize;
    while guard < 8 {
        guard += 1;
        let scope = match source.maybe_globs_scope_ref(&current) { Some(s) => s, None => break };
        let bases = match source.imports.maybe_scope_globs(scope) { Some(b) => b, None => break };
        if bases.is_empty() { break; }
        for b in bases.iter() {
            let base_abs_rel = ReexportSeek::Absolute.join_reexport(b, scope.self_path_ref(), scope.crate_ident_ref(), None);
            let first_is_crate = base_abs_rel.segments.first().map(|seg| seg.ident.eq(scope.crate_ident_ref())).unwrap_or(false) || base_abs_rel.is_crate_based();
            let base_abs = if first_is_crate { base_abs_rel } else { current.joined(&base_abs_rel) };
            if base_abs.segments.last().map(|s| s.ident.to_string()) == Some(preferred_tail.clone()) {
                return Some(base_abs.joined(&last.ident.to_path()));
            }
            if let Some(found) = resolve_by_nested_globs(&base_abs, last, source, 0) { return Some(found); }
        }
        current = current.popped();
        if current.segments.is_empty() { break; }
    }
    None
}

// (Removed legacy: is_descendant_of)

// Try to find a direct alias for the last segment in any ancestor scope of `path`.
// (Removed legacy: resolve_by_ancestor_imports)
// Try to find the scope where item is actually defined
// assuming that 'path' is defined at 'scope' and can be shortened
#[allow(unused)]
pub(crate) fn maybe_closest_known_scope_for_import_in_scope<'a>(path: &'a Path, scope: &'a ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeChain> {
    // First assumption that it is relative import path
    let scope_path = scope.self_path_ref();
    let mut closest_scope: Option<&ScopeChain> = None;

    let mut chunk = path.popped();
    while !chunk.segments.is_empty() {
        let candidate = scope_path.joined(&chunk);
        closest_scope = source.maybe_scope_ref(&candidate);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
    chunk = path.popped();
    // Second assumption that it is global import path;
    while !chunk.segments.is_empty() {
        closest_scope = source.maybe_scope_ref(&chunk);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
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
fn find_defined_object_path(target: &PathSegment, source: &GlobalContext) -> Option<Path> {
    let mut best: Option<(usize, Path)> = None;
    for scope in source.scope_register.inner.keys() {
        let scope_path = scope.self_path_ref();
        let candidate = scope_path.joined(&target.ident.to_path());
        if source.maybe_scope_item_ref_obj_first(&candidate).is_some() {
            let depth = candidate.segments.len();
            match &best { Some((best_depth, _)) if *best_depth >= depth => {}, _ => best = Some((depth, candidate)) }
        }
    }
    best.map(|(_, p)| p)
}
