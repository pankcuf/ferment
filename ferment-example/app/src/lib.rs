#[test]
fn test_example_nested() {
    const SELF_NAME: &str = "example_nested";
    match ferment_sys::Ferment::with_crate_name(SELF_NAME)
        .with_default_mod_name()
        .with_cbindgen_config_from_file("cbindgen.toml")
        .with_external_crates(vec![
            // "versioned-feature-core",
            "example-simple",
            // "dashcore",
            // "dpp",
            // "platform-value",
            // "platform-version"
        ])
        .with_languages(vec![
            #[cfg(feature = "objc")]
            ferment_sys::Lang::ObjC(ferment_sys::ObjC::new(ferment_sys::XCodeConfig {
                class_prefix: "DS".to_string(),
                framework_name: "DSExampleNested".to_string(),
                header_name: SELF_NAME.to_string()
            }
            )),
        ])
        .generate() {
        Ok(_) => println!("[ferment] [ok]: {SELF_NAME}"),
        Err(err) => panic!("[ferment] [err]: {}", err)
    }

}