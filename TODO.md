- Need to catch this aliases at mod.rs level
    ```rust
    pub use self::models::snapshot::LLMQSnapshot;
    ```
    otherwise to support fermentation it needs to be imported as full path:
    ```rust
    use crate::models::snapshot::LLMQSnapshot;
    ```
    and this wouldn't work:
    ```rust
    use crate::models::LLMQSnapshot;
    ```
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
- Need to fix "fermented::" hardcoded in type transposing although different name is specified in Config::with_mod_name()
- Async generic traits (decomposable) (epic)
- Other Languages Support (objc/java) - (at least DashSync’s boilerplate generation) (epic)
- Typealiases for paths (re-export types support)
- Path chunks support (when objects contains paths like this:

    ```rust
    use example_simple::nested;
    use crate::model::snapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(snapshot: snapshot::LLMQSnapshot) -> nested::HashID {
        [0u8; 32]
    }
    ```

- Fix: custom fermented module names (currently no matter what you specified in config – it always expanding in crate::fermented scope)
- Fix: optional primitives (Now Option<bool> expanding to false when bool is really false, or when Option is None, same thing for Option<u32> becomes 0 if None)
- Handle mut vs const in methods arguments
- Expose tokio runtime constructor/destructor
- Improve `Self::` processing
- Somewhat happens while reexporting fermented types while using nested crates: it goes with dash_spv_masternode_processor::some_type::SomeStruct insted of dash_spv_masternode_processor::fermented::types::some_type::SomeStruct 
- Custom structures named Result, Vec, HashMap, BTreeMap may not work properly. Need to add some logic to distinguish between std and custom types here
- Need support for paths containing super or super::super etc
- Minor issue with things like #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityRequest\"]"]
- TypeGroup support
- Algo for determine if type is simple enough to add it to the dictionary of registered types and use orginal one across the FFI
- Such enum has wrong fermentation: 
  ```rust
  #[repr(u8)]
  #[ferment_macro::export]
  pub enum ContractBounds {
    SingleContract { id: Identifier } = 0,
    SingleContractDocumentType { id: Identifier, document_type_name: String } = 1,
  }
  ```