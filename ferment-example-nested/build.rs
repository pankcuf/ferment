extern crate cbindgen;
extern crate ferment;

use std::path::PathBuf;
use std::process::Command;
use ferment::builder::{Builder, Crate};

pub const SELF_NAME: &str = "ferment_example_nested";
fn main() {
   let c_header = format!("target/{}.h", SELF_NAME);
   let nested_crates = vec![Crate::new("ferment_example", PathBuf::from("../ferment-example/src"))];

   match Builder::new(Crate::current_with_name(SELF_NAME))
       .with_mod_name("fermented")
       .with_crates(nested_crates)
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {err}")
      }
      Err(err) => panic!("Can't create FFI expansion: {err}")
   }
}
