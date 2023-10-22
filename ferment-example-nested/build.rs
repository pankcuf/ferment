extern crate cbindgen;
extern crate ferment;

use std::process::Command;

fn main() {

   match ferment::Builder::new()
       .with_mod_name("fermented")
       .with_crates(vec!["ferment_example".to_string()])
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(&["--config", "cbindgen.toml", "-o", "target/example.h"])
          .status() {
         Ok(status) => println!("Bindings generated into target/example.h with status: {status}"),
         Err(err) => panic!("Can't generate bindings: {err}")
      }
      Err(err) => panic!("Can't create FFI expansion: {err}")
   }
}
