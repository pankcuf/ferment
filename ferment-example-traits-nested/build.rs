extern crate cbindgen;
extern crate ferment;

use std::path::PathBuf;
use std::process::Command;
use ferment::builder::Crate;
const NAME: &str = "ferment_example_traits_nested";
fn main() {
   let c_header = format!("target/{NAME}.h");
   // let crates = vec![Crate::new("ferment_example_traits", PathBuf::from("../ferment-example/src"))];
   match ferment::Builder::new(Crate::current_with_name(NAME))
       .with_mod_name("fermented")
       .with_crates(vec!["ferment-example-traits"])
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
