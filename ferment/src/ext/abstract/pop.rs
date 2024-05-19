use syn::{Path, PathSegment};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::ext::CrateExtension;
use crate::holder::PathHolder;

pub trait Pop {
    fn popped(&self) -> Self;
}
// pub trait Pull {
//     fn pulled(&self) -> Self;
// }

impl Pop for PathHolder {
    fn popped(&self) -> Self {
        PathHolder::from(self.0.popped())
    }
}

impl Pop for Path {
    fn popped(&self) -> Self {
        Path { leading_colon: None, segments: Punctuated::from_iter(self.segments.popped()) }
    }
}

impl Pop for Punctuated<PathSegment, Colon2> {
    fn popped(&self) -> Self {
        self.ident_less()
        // Punctuated::from_iter(self.into_iter().take(self.len() - 1).cloned())
    }
}

// impl Pull for PathHolder {
//     fn pulled(&self) -> Self {
//         PathHolder::from(self.0.pulled())
//     }
// }
// impl Pull for Path {
//     fn pulled(&self) -> Self {
//         Path { leading_colon: None, segments: Punctuated::from_iter(self.segments.pulled()) }
//     }
// }
// impl Pull for Punctuated<PathSegment, Colon2> {
//     fn pulled(&self) -> Self {
//         self.iter().skip(1).collect()
//     }
// }
