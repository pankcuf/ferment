**v0.1.4:**
- [Custom conversion support](https://github.com/pankcuf/ferment/blob/ff10bec42c55935a3d2b5c457d50e6b5352b418c/ferment-example/src/asyn/query.rs#L1C1-L26C3)

**v0.1.3:**
- Support async fn (tokio::runtime)
- Expose Enum Variant constructors
- Refactoring

**v0.1.1:**

- fix: merge imports (when multiple items in one mod)
- feat: multiple crates support
- example: nested fermented crates fermentate
```rust
extern crate cbindgen;
extern crate ferment;

use std::process::Command;
/// Now we can use `with_crates` to pass names of crates that use `[ferment_macro::export]`
fn main() {
    match ferment::Builder::new()
        .with_crates(vec!["example-simple".to_string()])
        .generate() {
        Ok(()) => match Command::new("cbindgen")
            .args(&["--config", "cbindgen.toml", "-o", "target/example.h"])
            .status() {
            Ok(status) => println!("Bindings generated into target/example.h with status: {}", status),
            Err(err) => panic!("Can't generate bindings: {}", err)
        }
        Err(err) => panic!("Can't create FFI expansion: {}", err)
    }
}
```