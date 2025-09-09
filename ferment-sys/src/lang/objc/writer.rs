use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::process::Command;
use quote::ToTokens;
use crate::{error, XCodeConfig};
use crate::composer::SourceFermentable;
use crate::lang::objc::dictionary::{INTERFACES, MACROS};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::tree::CrateTree;
use crate::writer::{CrateTreeWrite, Writer};

pub const X86_MAC: Target = Target { arch: Arch::X8664, platform: Platform::AppleDarwin };
pub const ARM_MAC: Target = Target { arch: Arch::AARCH64, platform: Platform::AppleDarwin };
pub const X86_IOS: Target = Target { arch: Arch::X8664, platform: Platform::AppleIOS };
pub const ARM_IOS: Target = Target { arch: Arch::AARCH64, platform: Platform::AppleIOS };
pub const ARM_IOS_SIM: Target = Target { arch: Arch::AARCH64, platform: Platform::AppleIOSSim };

pub struct Target {
    pub arch: Arch,
    pub platform: Platform,
}
impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.arch, self.platform))
    }
}

pub enum OS {
    Ios,
    MacOS
}
impl Display for OS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OS::Ios => "ios",
            OS::MacOS => "macos"
        })
    }
}

pub enum Arch {
    X8664,
    AARCH64
}
impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Arch::X8664 => "x86_64",
            Arch::AARCH64 => "aarch64"
        })
    }
}

pub enum Platform {
    AppleDarwin,
    AppleIOS,
    AppleIOSSim,
}
impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Platform::AppleDarwin => "apple-darwin",
            Platform::AppleIOS => "apple-ios",
            Platform::AppleIOSSim => "apple-ios-sim",
        })
    }
}
pub enum Program {
    Cargo,
    Lipo,
    Xcodebuild,
    Pwd
}

impl Program {
    pub fn run<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(&self, args: I) -> Result<(), error::Error> {
        match self {
            Program::Cargo | Program::Lipo | Program::Xcodebuild | Program::Pwd => {
                Command::new(self)
                    .args(args)
                    .status()
                    .map_err(error::Error::from)
                    .map(|_| ())
                    // .expect(format!("Failed to run: {:?}", self.as_ref()).as_str());
            }
        }
    }
}
impl AsRef<OsStr> for Program {
    fn as_ref(&self) -> &OsStr {
        match self {
            Program::Cargo => "cargo",
            Program::Lipo => "lipo",
            Program::Xcodebuild => "xcodebuild",
            Program::Pwd => "pwd"
        }.as_ref()
    }
}

impl CrateTreeWrite<ObjCSpecification> for Writer {
    fn write(&self, crate_tree: &CrateTree) -> Result<(), error::Error> {
        if let Some(config) = self.config.maybe_objc_config() {
            let fermentate = SourceFermentable::<ObjCFermentate>::ferment(crate_tree).to_token_stream().to_string();
            let XCodeConfig { header_name, framework_name: framework, .. } = &config.xcode;
            let mut writer = String::new();
            let result = writer
                .add("#import <Foundation/Foundation.h>\n")
                .add(format!("#import \"{}.h\"\n", header_name).as_str())
                .add(MACROS)
                .add("NS_ASSUME_NONNULL_BEGIN\n")
                .add(INTERFACES)
                .add(fermentate.to_string().as_str())
                .add("\nNS_ASSUME_NONNULL_END\n");
            let objc_file_name = "objc_wrapper.h";

            Command::new("mkdir")
                .args(["-p", "target/include"])
                .status()?;

            let objc_path= Path::new("target")
                .to_path_buf()
                .join(format!("include/{objc_file_name}").as_str());
            let umbrella_header_path= Path::new("target")
                .to_path_buf()
                .join(format!("include/{framework}.h").as_str());

            let umbrella = Path::new("target")
                .to_path_buf()
                .join("include/module.modulemap");

            // //! Project version number for DashSharedCoreBindings.
            // FOUNDATION_EXPORT double DashSharedCoreBindingsVersionNumber;
            //
            // //! Project version string for DashSharedCoreBindings.
            // FOUNDATION_EXPORT const unsigned char DashSharedCoreBindingsVersionString[];
            write_file_with_string(
                &umbrella_header_path,
                format!("#import <Foundation/Foundation.h>\n#import \"{header_name}.h\"\n#import \"{objc_file_name}\"\nFOUNDATION_EXPORT double {framework}VersionNumber;\nFOUNDATION_EXPORT const unsigned char {framework}VersionString[];"))?;
            write_file_with_string(
                &umbrella,
                format!("framework module {framework} {{\n\tumbrella header \"{framework}.h\"\n\texport *\n\tmodule * {{ export * }}\n\theader \"{objc_file_name}\"\n\theader \"{header_name}.h\" }}"))?;
            write_file_with_string(
                &objc_path,
                result)?;
        }
        Ok(())
    }
}

fn write_file_with_string(path: &PathBuf, string: String) -> Result<(), error::Error> {
    File::create(path)
        .map_err(error::Error::from)
        .and_then(|mut output| output.write_all(string.as_bytes()).map_err(error::Error::from))
}