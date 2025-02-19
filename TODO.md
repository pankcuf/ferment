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
- improve: `Self::`, `&Self` processing
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
- improve types wrapped into smart pointers (Box, etc) (in terms of memory use?)
- fix: Vec<&str> becomes Vec_, also can't use smth like ['a ['a str]]
- static methods for impls are broken (if they are non-opaque), so currently it's possible to use only instance methods
- fix: HashSet<[u8; 32]> becomes std_collections_HashSet_u8 instead of std_collections_HashSet_Arr_u8_32 (+)
- fix: opaque types which impl are exported can't use `fn some_fn(self)` (trying to dereference it)
- fix: Fn without non-receiver arguments is not supported