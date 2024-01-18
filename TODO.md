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
    use ferment_example::nested::HashID;
    use crate::model::snapshot::LLMQSnapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: LLMQSnapshot) -> HashID {
        [0u8; 32]
    }
    ```
    instead of
    ```rust
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: crate::model::snapshot::LLMQSnapshot) -> ferment_example::nested::HashID {
        [0u8; 32]
    }
    ```
    or
    ```rust
    use ferment_example::nested;
    use crate::model::snapshot;
    #[ferment_macro::export]
    pub fn get_hash_id_form_snapshot(_snapshot: snapshot::LLMQSnapshot) -> nested::HashID {
        [0u8; 32]
    }
    ```
- Need to fix "fermented::" hardcoded in type transposing although different name is specified in Config::with_mod_name()
- Need to create 'register_type' to hold the dictionary with manual conversions (for std objects & 3rd party crates)
- Async generic traits (decomposable) (epic)
- Other Languages Support (objc/java) - (at least DashSync’s boilerplate generation) (epic)
- Typealiases for paths (re-export types support)
- Manual conversion support (Mechanism for registration of manually fermented objects)

  This needed for:

    - Conversion implementation for several objects from std (or any non-fermentable crates)
    - For cases where special optimisation needed
    - `#[ferment_macro::register(std::time::Duration)] pub struct Duration_FFI { secs: u64, nanos: u32,}`
- Path chunks support (when objects contains paths like this:

    ```rust
    use ferment_example::nested;
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