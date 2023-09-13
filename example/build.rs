extern crate cbindgen;

use std::process::Command;

fn main() {
   Command::new("cbindgen")
       .args(&["--config", "cbindgen.toml", "-o", "target/example.h", "target/expanded_reduced.rs"])
       .status()
       .expect("Failed to run cbindgen");
}
