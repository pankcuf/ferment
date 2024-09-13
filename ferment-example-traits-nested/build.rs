extern crate cbindgen;
extern crate ferment;

use std::process::Command;
const NAME: &str = "ferment_example_traits_nested";
fn main() {
   let c_header = format!("target/{NAME}.h");
   match ferment::Ferment::with_crate_name(NAME)
       .with_default_mod_name()
       .with_crates(vec!["ferment-example-traits"])
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {}", err)
      }
      Err(err) => panic!("[ferment] [error] {}", err)
   }
}
