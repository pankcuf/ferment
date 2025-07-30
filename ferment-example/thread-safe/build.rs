extern crate cbindgen;
extern crate ferment_sys;

pub const SELF_NAME: &str = "example_thread_safe";
fn main() {

   let c_header = format!("target/{}.h", SELF_NAME);
   match ferment_sys::Ferment::with_crate_name(SELF_NAME)
       .with_default_mod_name()
       .generate() {
      Ok(()) => match std::process::Command::new("cbindgen")
          .args(["--config", "cbindgen.toml", "-o", c_header.as_str()])
          .status() {
         Ok(status) => println!("[cbindgen] [ok] generated into {c_header} with status: {status}"),
         Err(err) => panic!("[cbindgen] [error] {}", err)
      }
      Err(err) => panic!("[ferment-sys] Can't create FFI fermentate: {}", err)
   }
}