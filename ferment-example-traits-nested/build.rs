extern crate cbindgen;
extern crate ferment;

use std::process::Command;

fn main() {
   let c_header = "target/example_nested_traits.h";
   match ferment::Builder::new()
       .with_mod_name("fermented")
       .with_crates(vec!["ferment_example_traits".to_string()])
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {err}")
      }
      Err(err) => panic!("Can't create FFI expansion: {err}")
   }
}
