1. Need to catch this aliases at mod.rs level
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
2. Now you can't specify field type as full or partially qualified (bug). So use this:
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