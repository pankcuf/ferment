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
