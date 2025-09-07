use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;
use crate::ext::CrateExtension;

pub trait PathTransform {
    fn replace_first_with(&mut self, chunk: &Self);
    fn replaced_first_with(&self, chunk: &Self) -> Self;
    fn replace_last_with(&mut self, chunk: &Self);
    fn replaced_last_with(&self, chunk: &Self) -> Self;
    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self);
    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self;
}

impl PathTransform for Path {
    fn replace_first_with(&mut self, chunk: &Self) {
        self.segments.replace_first_with(&chunk.segments)
    }

    fn replaced_first_with(&self, chunk: &Self) -> Self {
        let mut new_path = self.clone();
        new_path.segments = self.segments.replaced_first_with(&chunk.segments);
        new_path
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        self.segments.replace_last_with(&chunk.segments)
    }

    fn replaced_last_with(&self, chunk: &Self) -> Self {
        let mut new_path = self.clone();
        new_path.segments = self.segments.replaced_last_with(&chunk.segments);
        new_path
    }

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self) {
        self.segments.replace_first_and_last_with(&leading_chunk.segments, &trailing_chunk.segments)
    }

    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self {
        let mut new_path = self.clone();
        new_path.segments = self.segments.replaced_first_and_last_with(&leading_chunk.segments, &trailing_chunk.segments);
        new_path
    }
}

impl PathTransform for Colon2Punctuated<PathSegment> {
    fn replace_first_with(&mut self, chunk: &Self) {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        self.clear();
        self.extend(segments);
    }

    fn replaced_first_with(&self, chunk: &Self) -> Self {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        segments
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        if let Some(head) = self.pop() {
            self.extend(chunk.clone());
            if let Some(PathSegment { arguments, .. }) = self.last_mut() {
                *arguments = head.into_value().arguments;
            }
        }
    }
    fn replaced_last_with(&self, chunk: &Self) -> Self {
        let mut new_segments = self.clone();
        new_segments.replace_last_with(chunk);
        new_segments
    }

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self) {
        self.replace_first_with(leading_chunk);
        self.replaced_last_with(trailing_chunk);
    }

    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self {
        self.replaced_first_with(leading_chunk)
            .replaced_last_with(trailing_chunk)
    }
}
