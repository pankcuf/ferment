extern crate cbindgen;
extern crate ferment_sys;

use ferment_sys::{Ferment, Lang, ObjC, XCodeConfig};

fn main() {
    const SELF_NAME: &str = "example_nested";
    match Ferment::with_crate_name(SELF_NAME)
        .with_default_mod_name()
        .with_cbindgen_config_from_file("cbindgen.toml")
        .with_external_crates(vec!["example-simple"])
        .with_languages(vec![
            Lang::ObjC(ObjC::new(XCodeConfig {
                class_prefix: "DS".to_string(),
                framework_name: "DSExampleNested".to_string(),
                header_name: SELF_NAME.to_string()
            })),
        ])
        .generate() {
        Ok(_) => println!("[ferment] [ok]: {SELF_NAME}"),
        Err(err) => panic!("[ferment] [err]: {}", err)
    }
}
