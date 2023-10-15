extern crate cbindgen;
extern crate ferment;

fn main() {
   match ferment::build("fermented") {
      Ok(()) => {
         // run cbindgen
      },
      Err(err) => panic!("Can't create FFI expansion: {}", err)
   }

   // let status = Command::new("cargo")
   //     .args(&["fmt", output_path.to_str().unwrap()])
   //     .status()
   //     .expect("Failed to run cargo fmt");
   //
   // if !status.success() {
   //    println!("cargo:warning=cargo fmt failed");
   // }

   // Command::new("cbindgen")
   //     .args(&["--config", "cbindgen.toml", "-o", "target/ferment-example.h", "target/expanded_reduced.rs"])
   //     .status()
   //     .expect("Failed to run cbindgen");
}
