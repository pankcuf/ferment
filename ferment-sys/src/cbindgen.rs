#[cfg(feature = "cbindgen")]
impl crate::writer::Writer {
    pub(crate) fn write_headers(&self) -> Result<(), crate::Error> {
        use std::process::Command;
        let crate::Config { current_crate: crate::Crate { name: framework, .. }, cbindgen_config_from_file, .. } = &self.config;
        let framework = {
            #[cfg(feature = "objc")]
            {
                self.config.maybe_objc_config().map(|config| config.xcode.header_name.clone()).unwrap_or(framework.clone())
            }
            #[cfg(not(feature = "objc"))]
            {
                framework.clone()
            }
        };
        Command::new("mkdir")
            .args(&["-p", "target/include"])
            .status()?;
        Command::new("cbindgen")
            .args([
                "--config", cbindgen_config_from_file.as_ref().map(String::as_str).unwrap_or("cbindgen.toml"),
                "-o", format!("target/include/{framework}.h").as_str()
            ])
            .status()
            .map_err(crate::Error::from)
            .map(|_| ())
    }

}