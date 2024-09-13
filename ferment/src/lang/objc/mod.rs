pub(crate) mod composer;
pub(crate) mod composers;
pub(crate) mod constants;
pub(crate) mod conversion;
pub(crate) mod fermentate;
pub(crate) mod presentation;
#[allow(unused)]
mod writer;
mod xcproj;
mod presentable;

use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::error;
use crate::lang::CrateTreeConsumer;
use crate::tree::CrateTree;

pub use fermentate::Fermentate as ObjCFermentate;
pub use writer::Writer as ObjCWriter;
pub use xcproj::Config as XCodeConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub xcode: XCodeConfig,
    // pub targets: [&'static str; 5]
}

impl Config {
    pub fn new(xcode: XCodeConfig) -> Self {
        Self {
            xcode,
            // targets: APPLE_TARGETS
        }
    }
    pub fn class_prefix(&self) -> &str {
        &self.xcode.class_prefix
    }
}


impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[objc::Config]\n\txcode: {}", self.xcode))
    }
}

impl CrateTreeConsumer for Config {
    fn generate(&self, _crate_tree: &CrateTree) -> Result<(), error::Error> {
        // let ff = ObjectPresentation::Interface { name: Name::Index(0), c_type: quote!(), properties: SemiPunctuated::new() };
        // println!("objc:: {}", ff.to_token_stream());
        Ok(())
        // unimplemented!("{:?}", crate_tree)

    }
}


pub enum CategoryKind {
    C,
    Rust,
    Args
}
impl ToTokens for CategoryKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            CategoryKind::C => quote!(C),
            CategoryKind::Rust => quote!(Rust),
            CategoryKind::Args => quote!(Args),
        }.to_tokens(tokens)
    }
}
