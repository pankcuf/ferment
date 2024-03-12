extern crate cbindgen;
extern crate ferment;
extern crate cargo_metadata;

use std::path::PathBuf;
use std::process::Command;
use cargo_metadata::MetadataCommand;
use ferment::builder::{Builder, Crate};

pub const SELF_NAME: &str = "ferment_example_nested";
fn main() {
   let c_header = format!("target/{}.h", SELF_NAME);
   match Builder::new(Crate::current_with_name(SELF_NAME))
       .with_mod_name("fermented")
       .with_crates(find_crates_paths(vec!["ferment_example"]))
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

fn find_crates_paths(crate_names: Vec<&str>) -> Vec<Crate> {
   let metadata = MetadataCommand::new().exec().unwrap();
   crate_names.into_iter()
       .filter_map(|crate_name| {
           metadata.packages
               .iter()
               .find_map(|p| {
                   if let Some(target) = p.targets.first() {
                       if target.name.as_str() == crate_name {
                           return Some(Crate::new(crate_name,PathBuf::from(target.src_path.parent().unwrap())))
                       }
                   }
                   None
               })
       })
       .collect()
}