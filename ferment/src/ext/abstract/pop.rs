use syn::{Path, PathSegment};
use syn::punctuated::Punctuated;
use crate::ast::{Colon2Punctuated, PathHolder};
use crate::ext::CrateExtension;

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

impl Pop for Colon2Punctuated<PathSegment> {
    fn popped(&self) -> Self {
        self.ident_less()
    }
}
