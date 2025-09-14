# Ferment Project Plan

This plan consolidates the existing TODOs and code `TODO/FIXME` markers into a structured roadmap. It groups work by themes, calls out priorities, and proposes success criteria. Use this to open GitHub Issues and (optionally) a GitHub Project board.

## Summary 

Ferment Processing Pipeline Overview:

1. Crate Discovery & AST Parsing

Ferment traverses all crates defined in the build configuration, parsing Rust source files into syntax trees using the syn crate. It identifies items marked with
#[ferment_macro::export] for FFI generation.

2. Scope Chain Construction

Ferment builds a comprehensive scope registry mapping module paths to their contents, creating a global context that tracks all available types, functions, traits, and
imports across all analyzed crates.

3. Path Resolution & Import Chain Following

The refinement phase resolves all path references by following import chains, glob imports (use mod::*), and re-exports. This is where the current performance bottleneck
occurs - for each unresolved path, it performs expensive scope traversals to find the actual definition.

4. Type Model Generation

Ferment creates "fermented" equivalents of Rust types - converting structs to C-compatible representations with #[repr(C)], generating vtables for traits, and creating
conversion functions between Rust and FFI types.

5. FFI Binding Generation

The final step generates C bindings via cbindgen, with constructor/destructor functions (_ctor/_destroy), automatic From/To trait implementations, and language-specific
bindings (Objective-C, Java).

The Performance Bottleneck:

The current slowness occurs in step 3 - path resolution. For complex projects with hundreds of modules and thousands of cross-references, the algorithm:
- Performs O(n²) scope traversals for each unresolved path
- Re-scans the entire scope registry repeatedly
- Doesn't build efficient indices for common lookup patterns
- Processes paths individually rather than in batches

## Status Summary

- Core FFI generation: structs, enums, type aliases (with caveats), functions, and traits (vtable + trait objects) — working.
- Path resolution and scope population: robust for most cases, including trait/impl scopes, Self-associated paths, generics propagation, imports (rename/group), slices, maps, options, arrays.
- Logging/formatting: noisy macro markers filtered, empty sections hidden.

## Epics

1) Multi-language support
- ObjC: enhance generators and polish outputs
- Java: initial generator and basic end-to-end path

2) Async generic traits
- Trait decomposition with async functions and generic bounds.

3) Visibility-aware fermentation (mod-level)
- Respect pub/private and support mod-based fermentation.

## High-Priority Fixes

- Custom fermented module name
  - Problem: expansion always uses `crate::fermented` regardless of `Config::with_mod_name()`.
  - Deliverable: plumb configured module name into codegen and example templates. Tests to verify custom names.

- Qualified paths in signatures
  - Problem: using fully/partially qualified types can fail resolution.
  - Deliverable: improve path normalization and resolution; add tests for `crate::...`, `super::...`, multi-segment aliases.

- Trait default methods
  - Problem: methods with default impls are not handled.
  - Deliverable: recognize defaults and still expose consistent bindings (document semantics).

- Enum with repr and struct-like variants
  - Problem: wrong fermentation for enums like in TODO.md.
  - Deliverable: add a targeted encoder for discriminants + payloads; add tests.

## Path Resolution & Imports

- Re-exports & type aliases across crates
  - Improve following alias chains through rename/group/glob imports; strengthen ImportResolver; expand tests.

- Wildcard imports
  - Current: ignored
  - Plan: resolve known items using crate metadata or late binding heuristics (document limitations).

- `super::` and nested supers
  - Add path normalization for super-chains; unit tests under nested modules/impls/traits.

## Type Support & Rules

- String slice collections (`Vec<&str>`) and lifetimes
  - Determine supported patterns; degrade gracefully or provide helper conversions.

- Smart pointers (Box, Rc, Arc) handling refinement
  - Ensure memory semantics are sound and predictable at FFI boundary.

- TypeGroup support
  - Expand to cover missing cases; document mangle rules.

- Heuristic for pass-through types
  - Decide when aliases or simple types can skip dictionary wrappers.

## Docs & Formatting

- Clean up docs with nested attributes (#doc in #doc)
  - Escape/format doc strings to prevent duplication.

- Improve error messages for unsupported generics
  - Replace unimplemented!/panic! with actionable diagnostics.

## Test Coverage

- Add tests for:
  - Fully/partially qualified paths in function signatures (args/returns)
  - `super::` resolution paths
  - Re-exported alias chains across crates
  - Enum repr cases with struct-like variants
  - Trait default methods exposure
  - Wildcard imports (best-effort behavior)

## Code TODO/FIXME Consolidation

- Resolve or ticket these notable markers (file:line):
  - ferment/src/lib.rs:84 — make unbox_any composable of arbitrary type (unbox_any_vec_composer)
  - ferment-sys/src/presentation/doc.rs:18 — improve doc link formatting for generated items
  - ferment-sys/src/kind/generic_type.rs:88 — add mixin implementation for generic type kinds
  - ferment-sys/src/kind/generic_type.rs:92 — non-supported generic kind panic; handle remaining generic kinds
  - ferment-sys/src/ext/collection.rs:52 — implement missing collection helpers (if needed)
  - ferment-sys/src/ext/refine/reexport.rs:14 — handle nested super paths (super::super::)
  - ferment-sys/src/composer/bare_fn.rs:82 — support mixins+traits+generics for bare fn
  - ferment-sys/src/tree/visitor.rs:74 — decide handling for fn-level use statements
  - ferment-sys/src/tree/visitor.rs:202 — nested trait/function scoping edge cases
  - ferment-sys/src/tree/visitor.rs:245 — filter out #[cfg(test)] during traversal (presentation-only)
  - ferment-sys/src/ext/refine/refine_in_scope.rs:16 — refine key types as well (QSelf/associated)
  - ferment-sys/src/ext/refine/refine_in_scope.rs:174 — clarify global generic behavior
  - ferment-sys/src/ext/refine/refine_in_scope.rs:415 — support nested function when necessary
  - ferment-sys/src/kind/type.rs:148 — conversions for opaque types (document/handle absence)
  - ferment-sys/src/composer/attrs.rs:39-40 — trait expansion via attributes disabled; migrate to composable RefinedTree
  - ferment-sys/src/kind/type_model.rs:193 — should we use import chunk here as well?
  - ferment-sys/src/kind/type_model.rs:200 — extend checks to other kinds (e.g., slices)
  - ferment-sys/src/ext/constraints.rs:58 — const generic argument handling
  - ferment-sys/src/ext/constraints.rs:136 — implement AngleBracketedGenericArguments.has_self
  - ferment-sys/src/ext/visitor/visit_scope.rs:355,359 — prevent generic bounds from adding to parent (partially addressed; clean comments/tests)
  - ferment-sys/src/ext/visitor/visit_scope.rs:382 — nondeterministic scope note; revisit after refactor
  - ferment-sys/src/ext/resolve/mod.rs:57,76,113 — optional type generics, trait bounds during generic expansion, and trait object (empty) resolution
  - ferment-sys/src/ext/visitor/unique_nested_items.rs:43 — Expr unique-nested-items extraction (if needed)
  - ferment-sys/src/ext/visitor/visit_scope_type.rs:220 — multiple bounds handling
  - ferment-sys/src/lang/rust/ext/resolve/mod.rs:57,76,93 — optional/generics edge cases and avoid hardcode
  - ferment-sys/src/lang/rust/composer/trait.rs:15 — source.scope or local_scope? clarify
  - ferment-sys/src/lang/rust/composer/callback.rs:74 — mixins+traits+generics
  - ferment-sys/src/lang/objc/* — several TODOs documenting in-progress ObjC support

## Proposed GitHub Project Columns

- Backlog (Epics & large chunks)
- Next (High-priority fixes)
- In Progress
- Review
- Done

## Issue Template (suggested)

- Title: [Area] Short summary
- Area: imports | paths | codegen | traits | enums | objc | java | docs | tests
- Description: …
- Acceptance Criteria: …
- Tests: …

---

Last updated: <set on change>
