# Ferment Project Plan

This plan consolidates the existing TODOs and code `TODO/FIXME` markers into a structured roadmap. It groups work by themes, calls out priorities, and proposes success criteria. Use this to open GitHub Issues and (optionally) a GitHub Project board.

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

- Resolve or ticket these notable markers:
  - ext/resolve/mod.rs: TraitObject empty case; trait bounds during generic expansion.
  - ext/refine/reexport.rs: `super::super::` handling
  - ext/visitor/unique_nested_items.rs: Expr support
  - ext/constraints.rs: Const generic and angle-bracketed args coverage
  - lang/rust/composer/callback.rs & composer/bare_fn.rs: trait/generics mixins
  - lang/objc/composer/var.rs & rust/ext/resolve: optional type generics edge cases
  - tree/visitor.rs: fn-level use statements and cfg(test) filters (presentation)

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

