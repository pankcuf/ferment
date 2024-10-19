- Now you can't specify field type as full or partially qualified (bug). So use this:
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
- fix: "fermented::" hardcoded in type transposing although different name is specified in Config::with_mod_name()
- improve: async generic traits (decomposable) (epic)
- improve: other Languages Support (java) (epic)
- improve: typealiases for paths (re-export types support)
- improve: cross-crates re-exports support 
- improve: internal crate reexports with gaps in the middle of hierarchy (like state_transitions::*)
- improve: support wildcard imports ("*")
- fix: custom fermented module names (currently no matter what you specified in config – it always expanding in crate::fermented scope)
- improve: `Self::` processing
- improve: Need support for paths containing super or super::super etc
- fix: minor issue with things like #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityRequest\"]"]
- improve: TypeGroup support
- improve: public/private fields/mods visibility support + mod-based fermentation (epic)
- improve: algo for determine if type is simple enough to add it to the dictionary of registered types and use original one across the FFI (like we often have for type aliases)
- fix: such enum has wrong fermentation: 
  ```rust
  #[repr(u8)]
  #[ferment_macro::export]
  pub enum ContractBounds {
    SingleContract { id: Identifier } = 0,
    SingleContractDocumentType { id: Identifier, document_type_name: String } = 1,
  }
  ```
  
- improve: refine scope (like trait or object impl when it's defined outside the declaration)
- improve: algo allowing to export only involved objects (to reduce amount of generated code)
- improve: if we have a type implementing a trait which has default implementation for some method – we don't have a scope stack to generate bindings (i.e. involved types are unknown, so we can't generate bindings for those methods)
