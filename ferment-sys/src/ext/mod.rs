mod r#abstract;
mod collection;
mod constraints;
mod item;
mod present;
mod refine;
mod resolve;
mod visitor;

use quote::format_ident;
use syn::{Path, PathArguments, PathSegment};
use crate::ast::Colon2Punctuated;
pub use self::r#abstract::*;
pub use self::constraints::*;
pub use self::item::*;
pub use self::present::*;
pub use self::refine::*;
pub use self::resolve::*;
pub use self::visitor::*;

pub trait CrateExtension {
    fn arg_less(&self) -> Self;
    fn is_crate_based(&self) -> bool;
    fn crate_named(&self, crate_name: &Self) -> Self where Self: Sized + Clone {
        if self.is_crate_based() {
            self.replaced_first_with_ident(crate_name)
        } else {
            self.clone()
        }
    }
    fn crate_less(&self) -> Self;
    fn ident_less(&self) -> Self;
    fn crate_and_ident_less(&self) -> Self;
    fn replace_first_with(&mut self, chunk: &Self);
    fn replaced_first_with_ident(&self, chunk: &Self) -> Self;
    fn replace_last_with(&mut self, chunk: &Self);
    fn replaced_last_with(&self, chunk: &Self) -> Self;

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self);
    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self;

}

impl CrateExtension for Path {
    fn is_crate_based(&self) -> bool {
        self.segments.is_crate_based()
    }

    fn crate_less(&self) -> Self {
        Path { segments: self.segments.crate_less(), leading_colon: self.leading_colon }
    }
    fn ident_less(&self) -> Self {
        Path { segments: self.segments.ident_less(), leading_colon: self.leading_colon }
    }

    fn arg_less(&self) -> Self {
        Path { segments: self.segments.arg_less(), leading_colon: self.leading_colon }
    }
    fn crate_and_ident_less(&self) -> Self {
        Path { segments: self.segments.crate_and_ident_less(), leading_colon: self.leading_colon }
    }

    fn replace_first_with(&mut self, chunk: &Self) {
        self.segments.replace_first_with(&chunk.segments)
    }

    fn replaced_first_with_ident(&self, chunk: &Self) -> Self {
        Path { segments: self.segments.replaced_first_with_ident(&chunk.segments), leading_colon: self.leading_colon }
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        self.segments.replace_last_with(&chunk.segments)
    }

    fn replaced_last_with(&self, chunk: &Self) -> Self {
        Path { segments: self.segments.replaced_last_with(&chunk.segments), leading_colon: self.leading_colon }
    }

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self) {
        self.segments.replace_first_and_last_with(&leading_chunk.segments, &trailing_chunk.segments)
    }

    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self {
        Path { segments: self.segments.replaced_first_and_last_with(&leading_chunk.segments, &trailing_chunk.segments), leading_colon: self.leading_colon }
    }
}

impl CrateExtension for Colon2Punctuated<PathSegment> {
    fn arg_less(&self) -> Self {
        let mut s = self.clone();
        if let Some(last) = s.last_mut() {
            last.arguments = PathArguments::None;
        }
        s
    }

    fn is_crate_based(&self) -> bool {
        self.first()
            .map(|PathSegment { ident, .. }| format_ident!("{CRATE}").eq(ident))
            .unwrap_or_default()
    }

    fn crate_less(&self) -> Self {
        self.iter().skip(1).cloned().collect()
    }

    fn ident_less(&self) -> Self {
        self.iter().take(self.len() - 1).cloned().collect()
    }

    fn crate_and_ident_less(&self) -> Self {
        self.iter().take(self.len() - 1).skip(1).cloned().collect()
    }

    fn replace_first_with(&mut self, chunk: &Self) {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        self.clear();
        self.extend(segments);
    }

    fn replaced_first_with_ident(&self, chunk: &Self) -> Self {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        segments
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        if let Some(last_popped_segment) = self.pop() {
            self.extend(chunk.clone());
            if let Some(last_segment) = self.last_mut() {
                last_segment.arguments = last_popped_segment.into_value().arguments;
            }
        }
    }
    fn replaced_last_with(&self, chunk: &Self) -> Self {
        let mut new_segments = self.clone();
        if let Some(last_popped_segment) = new_segments.pop() {
            new_segments.extend(chunk.clone());
            if let Some(last_segment) = new_segments.last_mut() {
                last_segment.arguments = last_popped_segment.into_value().arguments;
            }
        }
        new_segments
    }

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self) {
        self.replace_first_with(leading_chunk);
        self.replaced_last_with(trailing_chunk);
    }

    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self {
        self.replaced_first_with_ident(leading_chunk)
            .replaced_last_with(trailing_chunk)
    }

}
