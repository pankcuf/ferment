extern crate cbindgen;
extern crate ferment;
use std::process::Command;

fn main() {
   let c_header = "target/example.h";
   match ferment::Builder::new()
       .with_mod_name("fermented")
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header])
          .status() {
         Ok(status) => println!("Bindings generated into {c_header} with status: {status}"),
         Err(err) => panic!("Can't generate bindings: {}", err)
      }
      Err(err) => panic!("Can't create FFI expansion: {}", err)
   }
}
