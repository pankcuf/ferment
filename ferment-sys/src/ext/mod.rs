mod r#abstract;
mod args_transform;
mod collection;
mod constraints;
mod generic_bound_key;
mod maybe_attrs;
mod maybe_generics;
mod maybe_ident;
mod path_transform;
mod present;
mod refine;
mod resolve;
mod visitor;

use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;
pub use self::r#abstract::*;
pub use self::args_transform::*;
pub use self::constraints::*;
pub use self::maybe_ident::*;
pub use self::generic_bound_key::*;
pub use self::maybe_attrs::*;
pub use self::maybe_generics::*;
pub use self::path_transform::*;
pub use self::present::*;
pub use self::refine::*;
pub use self::resolve::*;
pub use self::visitor::*;

pub trait CrateBased {
    fn is_crate_based(&self) -> bool;
    fn crate_named(&self, crate_name: &Self) -> Self where Self: Sized + Clone + PathTransform {
        if self.is_crate_based() {
            self.replaced_first_with(crate_name)
        } else {
            self.clone()
        }
    }

}

impl CrateBased for Path {
    fn is_crate_based(&self) -> bool {
        self.segments.is_crate_based()
    }
}

impl CrateBased for Colon2Punctuated<PathSegment> {
    fn is_crate_based(&self) -> bool {
        self.first()
            .map(|PathSegment { ident, .. }| ident.eq(CRATE))
            .unwrap_or_default()
    }
}
pub trait CrateExtension {
    fn crate_less(&self) -> Self;
    fn ident_less(&self) -> Self;
    fn crate_and_ident_less(&self) -> Self;
}
impl CrateExtension for Colon2Punctuated<PathSegment> {
    fn crate_less(&self) -> Self {
        self.iter().skip(1).cloned().collect()
    }

    fn ident_less(&self) -> Self {
        self.iter().take(self.len() - 1).cloned().collect()
    }

    fn crate_and_ident_less(&self) -> Self {
        self.iter().take(self.len() - 1).skip(1).cloned().collect()
    }
}
impl CrateExtension for Path {
    fn crate_less(&self) -> Self {
        let mut path = self.clone();
        path.segments = self.segments.crate_less();
        path
    }
    fn ident_less(&self) -> Self {
        let mut path = self.clone();
        path.segments = self.segments.ident_less();
        path
    }

    fn crate_and_ident_less(&self) -> Self {
        let mut path = self.clone();
        path.segments = self.segments.crate_and_ident_less();
        path
    }
}
