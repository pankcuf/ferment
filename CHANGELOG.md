**v0.1.4:**
- [Custom conversion support](https://github.com/pankcuf/ferment/blob/main/ferment-example/src/asyn/query.rs#L1-L26)

**v0.1.3:**
- Support async fn (tokio::runtime)
- Expose Enum Variant constructors
- Refactoring

**v0.1.1:**

- fix: merge imports (when multiple items in one mod)
- feat: multiple crates support
- example: nested fermented crates expansion
```rust
extern crate cbindgen;
extern crate ferment;

use std::process::Command;
/// Now we can use `with_crates` to pass names of crates that use `[ferment::export]`
fn main() {
    match ferment::Builder::new()
        .with_crates(vec!["ferment_example".to_string()])
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