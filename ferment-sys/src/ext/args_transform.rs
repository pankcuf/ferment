use syn::{Path, PathArguments, PathSegment};
use crate::ast::Colon2Punctuated;

pub trait ArgsTransform {
    fn arg_less(&self) -> Self;
}

impl ArgsTransform for Colon2Punctuated<PathSegment> {
    fn arg_less(&self) -> Self {
        let mut s = self.clone();
        if let Some(last) = s.last_mut() {
            last.arguments = PathArguments::None;
        }
        s
    }
}

impl ArgsTransform for Path {
    fn arg_less(&self) -> Self {
        Path { segments: self.segments.arg_less(), leading_colon: self.leading_colon }
    }
}