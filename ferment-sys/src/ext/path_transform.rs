use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;
use crate::ext::CrateExtension;

pub trait PathTransform {
    fn replace_first_with(&mut self, chunk: &Self);
    fn replace_last_with(&mut self, chunk: &Self);
}

impl PathTransform for Path {
    fn replace_first_with(&mut self, chunk: &Self) {
        self.segments.replace_first_with(&chunk.segments)
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        self.segments.replace_last_with(&chunk.segments)
    }
}

impl PathTransform for Colon2Punctuated<PathSegment> {
    fn replace_first_with(&mut self, chunk: &Self) {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        self.clear();
        self.extend(segments);
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        if let Some(head) = self.pop() {
            self.extend(chunk.clone());
            if let Some(PathSegment { arguments, .. }) = self.last_mut() {
                *arguments = head.into_value().arguments;
            }
        }
    }
}
