- Full/partially qualified paths in signatures: some cases still fail to resolve (bug). Work around by importing names locally for now:
    ```rust
    use example_simple::nested::HashID;
    use crate::model::snapshot::LLMQSnapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: LLMQSnapshot) -> HashID {
        [0u8; 32]
    }
    ```
    instead of
    ```rust
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: crate::model::snapshot::LLMQSnapshot) -> example_simple::nested::HashID {
        [0u8; 32]
    }
    ```
    or
    ```rust
    use example_simple::nested;
    use crate::model::snapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: snapshot::LLMQSnapshot) -> nested::HashID {
        [0u8; 32]
    }
    ```
- fix: "fermented::" hardcoded in type transposing although different name is specified in Config::with_mod_name() (needs wiring the configured module name through codegen)
- improve: async generic traits (decomposable) (epic)
- improve: other Languages Support (objc) (epic)
- improve: other Languages Support (java) (epic)
- improve: typealiases for paths (re-export types support). Partially supported; improve across-crate alias chains.
- improve: cross-crates re-exports support. We handle rename/group imports; still improve deep alias chains and wildcard re-exports.
- improve: internal crate reexports with gaps in the middle of hierarchy (like state_transitions::*)
- improve: support wildcard imports ("*")
- fix: custom fermented module names (currently always expanding in crate::fermented scope)
- improve: `Self::`, `&Self` processing (partially implemented: trait/impl scopes capture Self-associated paths; parent scopes exclude them)
- improve: Need support for paths containing super or super::super etc
- fix: minor issue with things like #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityRequest\"]"]
- improve: TypeGroup support
- improve: public/private fields/mods visibility support + mod-based fermentation (epic)
- improve: algo to determine if a type is simple enough to be passed across FFI as-is (vs dictionary-backed), esp. for type aliases
- fix: enum with explicit repr discriminants + struct-like variants is fermented incorrectly: 
  ```rust
  #[repr(u8)]
  #[ferment_macro::export]
  pub enum ContractBounds {
    SingleContract { id: Identifier } = 0,
    SingleContractDocumentType { id: Identifier, document_type_name: String } = 1,
  }
  ```
- improve: types wrapped into smart pointers (Box, etc) (memory & ownership model)
- fix: Vec<&str> becomes Vec_ (lifetime-bearing string slices unsupported)
- improve: expose to_string methods, for items implementing Display
- fix: support trait methods with default implementations

Code TODO Index (from source files)
- ferment/src/lib.rs:84 — make unbox_any composable for arbitrary types (unbox_any_vec_composer)
- ferment-sys/src/presentation/doc.rs:18 — improve doc link formatting for generated items
- ferment-sys/src/kind/generic_type.rs:88 — add mixin implementation for generic type kinds
- ferment-sys/src/kind/generic_type.rs:92 — non-supported generic kind panic; handle remaining generic kinds
- ferment-sys/src/ext/collection.rs:52 — implement missing collection helpers (if needed)
- ferment-sys/src/ext/refine/reexport.rs:14 — handle nested super paths (super::super::)
- ferment-sys/src/composer/bare_fn.rs:82 — support mixins+traits+generics for bare fn
- ferment-sys/src/tree/visitor.rs:74 — decide handling for fn-level use statements
- ferment-sys/src/tree/visitor.rs:202 — nested trait/function scoping edge cases
- ferment-sys/src/tree/visitor.rs:245 — filter out #[cfg(test)] during tree traversal (presentation-only)
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
- ferment-sys/src/lang/objc/* — several TODOs for ObjC support (callbacks opaque, optional generics paths, trait bounds, remove hardcodes)
