use std::fs;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::process::Command;

use crate::error;
// use crate::lang::objc::ObjCFermentate;
use crate::tree::IWriter;

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
        f.write_str(format!("{}-{}", self.arch, self.platform).as_str())
    }
}

pub enum OS {
    IOS,
    MacOS
}
impl Display for OS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OS::IOS => "ios",
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
}

impl Program {
    pub fn run<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(&self, args: I) {
        match self {
            Program::Cargo | Program::Lipo | Program::Xcodebuild => {
                Command::new(self)
                    .args(args)
                    .status()
                    .expect(format!("Failed to run: {:?}", self.as_ref()).as_str());
            }
        }
    }
}
impl AsRef<OsStr> for Program {
    fn as_ref(&self) -> &OsStr {
        match self {
            Program::Cargo => "cargo",
            Program::Lipo => "lipo",
            Program::Xcodebuild => "xcodebuild"
        }.as_ref()
    }
}


pub struct Writer {
    pub config: super::Config
}

impl Writer {
    pub fn new(config: super::Config) -> Self {
        Self { config }
    }
}

impl IWriter for Writer {
    type Fermentate = crate::lang::objc::fermentate::Fermentate;
    fn write(&self, fermentate: Self::Fermentate) -> Result<(), error::Error> {
        let framework = &self.config.xcode.framework_name;
        let _rust_lib_name = "dash_spv_apple_bindings";
        let _header_name = "dash_shared_core";
        fermentate.objc_files()
            .iter()
            .for_each(|file| cp(format!("../objc/{}", file), format!("{framework}/include/{}", file)));

        // cargo_build(X86_MAC);
        // cargo_build(ARM_MAC);
        // cargo_build(X86_IOS);
        // cargo_build(ARM_IOS);
        // cargo_build(ARM_IOS_SIM);
        // cargo_lipo();
        // mkdir(format!("{framework}/framework"));
        // mkdir(format!("{framework}/include"));
        // mkdir(format!("{framework}/lib/ios"));
        // mkdir(format!("{framework}/lib/ios-simulator"));
        // mkdir(format!("{framework}/lib/macos"));
        // lipo(rust_lib_name, vec![X86_MAC, ARM_MAC], format!("{framework}/lib/macos/lib{header_name}_{}.a", OS::MacOS));
        // lipo(rust_lib_name, vec![X86_IOS, ARM_IOS_SIM], format!("{framework}/lib/ios-simulator/lib{header_name}_{}.a", OS::IOS));
        // cp(header(rust_lib_name), format!("{framework}/include/{header_name}.h"));
        // cp(lib(rust_lib_name, ARM_IOS), format!("{framework}/lib/ios/lib{header_name}_{}.a", OS::IOS));
        // xcframework(framework, header_name);
        Ok(())
    }
}

fn lib(lib_name: &str, target: Target) -> String {
    format!("../target/{target}/release/lib{lib_name}.a")
}

fn header(lib_name: &str) -> String {
    format!("../target/{lib_name}.h")
}
fn cp(from: String, to: String) {
    fs::copy(&from, &to)
        .expect(format!("Failed to copy file {from} to {to}").as_str());
}
fn mkdir(dir: String) {
    fs::create_dir_all(dir)
        .expect("Failed to create directory");

}

fn cargo_lipo() {
    Program::Cargo.run(&["lipo", "--release"]);
}

fn cargo_build(target: Target) {
    Program::Cargo.run(&["build", "--target", format!("{}-{}", target.arch, target.platform).as_str(), "--release"]);
}

fn lipo(rust_lib_name: &str, targets: Vec<Target>, output: String) {
    let mut args = vec![String::from("-create")];
    args.extend(targets.into_iter().map(|t| lib(rust_lib_name, t)));
    args.push(String::from("-output"));
    args.push(output);
    Program::Lipo.run(&args);
}

fn xcframework(framework: &str, header_name: &str,) {
    Program::Xcodebuild.run(&[
        "-create-xcframework",
        "-library",
        format!("{framework}/lib/ios/lib{header_name}_{}.a", OS::IOS).as_str(),
        "-headers",
        format!("{framework}/include").as_str(),
        "-library",
        format!("{framework}/lib/ios-simulator/lib{header_name}_{}.a", OS::IOS).as_str(),
        "-headers",
        format!("{framework}/include").as_str(),
        "-output",
        format!("{framework}/framework/{framework}.xcframework").as_str()
    ]);

}