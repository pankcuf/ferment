extern crate cbindgen;
extern crate rs_ffi_analyzer;

use std::fs::File;
use std::path::Path;
use std::process::Command;

// use std::process::Command;

fn main() {
   let input = Path::new("src/lib.rs");
   let output_path = Path::new("src/ffi_expansions.rs");
   let mut output = File::create(output_path).unwrap();
   match rs_ffi_analyzer::process(input, &mut output) {
      Ok(()) => {
         // run cbindgen
      },
      Err(err) => panic!("Can't create FFI expansion: {}", err)
   }


   let status = Command::new("cargo")
       .args(&["fmt", output_path.to_str().unwrap()])
       .status()
       .expect("Failed to run cargo fmt");

   if !status.success() {
      println!("cargo:warning=cargo fmt failed");
   }

   // Command::new("cbindgen")
   //     .args(&["--config", "cbindgen.toml", "-o", "target/example.h", "target/expanded_reduced.rs"])
   //     .status()
   //     .expect("Failed to run cbindgen");
}
