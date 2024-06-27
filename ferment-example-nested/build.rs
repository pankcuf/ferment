extern crate cbindgen;
extern crate ferment;

use std::process::Command;
use ferment::{Builder, Crate};

pub const SELF_NAME: &str = "ferment_example_nested";
fn main() {

   let c_header = format!("target/{}.h", SELF_NAME);
   match Builder::new(Crate::current_with_name(SELF_NAME))
       .with_mod_name("fermented")
       // .with_crates(vec!["ferment-example", "platform-value", "dpp"])
       .with_crates(vec!["ferment-example"])
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {err}")
      }
      Err(err) => panic!("[ferment] Can't create FFI expansion: {err}")
   }
}