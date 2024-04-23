use syn::{Path, PathSegment};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::ext::CrateExtension;
use crate::holder::PathHolder;

pub trait Pop {
    fn popped(&self) -> Self;
}

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