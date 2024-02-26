use syn::Path;
use syn::punctuated::Punctuated;
use crate::holder::PathHolder;

pub trait Pop {
    fn popped(&self) -> Self;
}

impl Pop for PathHolder {
    fn popped(&self) -> Self {
        let segments = self.0.segments.clone();
        let n = segments.len() - 1;
        PathHolder::from(Path { leading_colon: None, segments: Punctuated::from_iter(segments.into_iter().take(n)) })
    }
}

impl Pop for Path {
    fn popped(&self) -> Self {
        let segments = self.segments.clone();
        let n = segments.len() - 1;
        Path { leading_colon: None, segments: Punctuated::from_iter(segments.into_iter().take(n)) }
    }
}