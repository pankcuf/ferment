
mod ast;
mod builder;
mod composable;
mod composer;
mod context;
mod conversion;
mod ext;
mod error;
mod file;
mod formatter;
mod holder;
mod naming;
mod presentable;
mod presentation;
mod shared;
#[cfg(test)]
mod test;
mod tree;

pub use self::error::Error;
pub use self::builder::{Builder, Config, Crate};

// It's organized as a sequential process of tree transformation
// Files -> File Tree -> Scope Agnostic Tree -> Full Context Tree -> Expansion