extern crate cbindgen;
extern crate ferment;
use std::process::Command;
use ferment::builder::{Builder, Crate};

pub const SELF_NAME: &str = "ferment_example_traits";

fn main() {
   let c_header = format!("target/{}.h", SELF_NAME);
   match Builder::new(Crate::current_with_name(SELF_NAME))
       .with_mod_name("fermented")
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("Bindings generated into {c_header} with status: {status}"),
         Err(err) => panic!("Can't generate bindings: {}", err)
      }
      Err(err) => panic!("Can't create FFI expansion: {}", err)
   }
}
