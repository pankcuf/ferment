mod composer;
pub mod builder;
mod error;
mod helper;
mod interface;
mod presentation;
mod visitor;
#[cfg(test)]
mod test;
mod context;
mod formatter;
mod idents;
mod chunk;
mod holder;
mod conversion;
mod composition;
mod tree;

pub use self::builder::Builder;
pub use self::builder::Config;

// It's organized as a sequential process of tree transformation
// Files -> File Tree -> Scope Agnostic Tree -> Full Context Tree -> Expansion