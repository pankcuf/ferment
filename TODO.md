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
