extern crate cbindgen;
extern crate ferment_sys;

use std::process::Command;
use ferment_sys::{Ferment, Lang, ObjC, XCodeConfig};

pub const SELF_NAME: &str = "example_platform";
fn main() {
   let languages = vec![
       Lang::ObjC(ObjC::new(XCodeConfig { class_prefix: "DS".to_string(), framework_name: "Fermented".to_string() }))
   ];
   let c_header = format!("target/{}.h", SELF_NAME);
   match Ferment::with_crate_name(SELF_NAME)
       .with_default_mod_name()
       .with_crates(vec!["platform-value", "platform-version", "dpp"])
       .with_languages(languages)
       .generate() {
      Ok(()) => match Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {}", err)
      }
      Err(err) => panic!("[ferment-sys] [error] {}", err)
   }
}