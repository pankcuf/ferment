cargo build --release --verbose
cargo fmt
cargo expand | sed '/#!/d' > target/expanded.rs
cbindgen --config cbindgen.toml -o target/example.h target/expanded.rs
