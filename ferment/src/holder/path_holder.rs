use quote::{format_ident, ToTokens};
use syn::{Ident, parse_quote, Path, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::ext::CrateExtension;
use crate::holder::Holder;
use crate::impl_holder;

pub const EMPTY: PathHolder = PathHolder(Path { leading_colon: None, segments: Punctuated::new() });


impl_holder!(PathHolder, Path);

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
impl CrateExtension for PathHolder {
    fn is_crate_based(&self) -> bool {
        self.0.is_crate_based()
    }

    fn crate_less(&self) -> Self {
        PathHolder(self.0.crate_less())
    }

    fn ident_less(&self) -> Self {
        PathHolder(self.0.ident_less())
    }

    fn crate_and_ident_less(&self) -> Self {
        PathHolder(self.0.crate_and_ident_less())
    }

    fn replace_first_with(&mut self, chunk: &Self) {
        self.0.replace_first_with(&chunk.0)
    }

    fn replaced_first_with_ident(&self, chunk: &Self) -> Self {
        PathHolder(self.0.replaced_first_with_ident(&chunk.0))
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        self.0.replace_last_with(&chunk.0)
    }

    fn replaced_last_with(&self, chunk: &Self) -> Self {
        PathHolder(self.0.replaced_last_with(&chunk.0))
    }

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self) {
        self.0.replace_first_and_last_with(&leading_chunk.0, &trailing_chunk.0)
    }

    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self {
        PathHolder(self.0.replaced_first_and_last_with(&leading_chunk.0, &trailing_chunk.0))
    }
}

impl CrateExtension for Punctuated<PathSegment, Colon2> {
    fn is_crate_based(&self) -> bool {
        self.first().unwrap().ident == format_ident!("crate")
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
        // match self.first_mut() {
        //     Some(first) => first.ident = chunk.clone(),
        //     None => {}
        // }
    }

    fn replaced_first_with_ident(&self, chunk: &Self) -> Self {
        let mut segments = chunk.clone();
        segments.extend(self.crate_less());
        segments
    }

    fn replace_last_with(&mut self, chunk: &Self) {
        // let mut new_segments = self.clone();
        let last_segment = self.pop().unwrap();
        self.extend(chunk.clone());
        self.last_mut().unwrap().arguments = last_segment.into_value().arguments;
    }
    fn replaced_last_with(&self, chunk: &Self) -> Self {
        let mut new_segments = self.clone();
        let last_segment = new_segments.pop().unwrap();
        new_segments.extend(chunk.clone());
        new_segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
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

impl PathHolder {

    pub fn iter(&self) -> syn::punctuated::Iter<PathSegment> {
        self.0.segments.iter()
    }
    pub fn crate_root() -> Self {
        parse_quote!(crate)
    }

    pub fn len(&self) -> usize {
        self.0.segments.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.segments.is_empty()
    }
    pub fn root(&self) -> Ident {
        self.0.segments.first().unwrap().ident.clone()
    }
    pub fn head(&self) -> Ident {
        self.0.segments.last().unwrap().ident.clone()
    }

    pub fn joined(&self, name: &Ident) -> PathHolder {
        let mut segments = self.0.segments.clone();
        segments.push(PathSegment::from(name.clone()));
        PathHolder::from(Path { leading_colon: None, segments })
    }

    pub fn to_type(&self) -> Type {
        parse_quote!(#self)
    }

    pub fn split(&self, head_size: usize) -> (PathHolder, PathHolder) {
        let segments = self.0.segments.clone();
        let size = segments.len();
        let n = size - head_size;
        let mut i = 0;
        let (root, head): (Vec<PathSegment>, Vec<PathSegment>) = segments
            .into_iter()
            .partition(|_| {
                let used = i < n;
                i += 1;
                used
            });
        (
            PathHolder::from(Path { leading_colon: None, segments: Punctuated::from_iter(root) }),
            PathHolder::from(Path { leading_colon: None, segments: Punctuated::from_iter(head) }),
        )
    }
    pub fn split_and_join_self(&self, head_size: usize) -> (PathHolder, PathHolder) {
        let (root, head) = self.split(head_size);
        (root, if head.is_empty() {
            parse_quote!(Self)
        } else {
            parse_quote!(Self::#head)
        })
    }
}

#[test]
pub fn test_split() {
    let current_scope: PathHolder = parse_quote!(aa::bb::cc::dd::ee);
    assert_eq!(current_scope.split(0), (parse_quote!(aa::bb::cc::dd::ee), EMPTY));
    assert_eq!(current_scope.split(1), (parse_quote!(aa::bb::cc::dd), parse_quote!(ee)));
    assert_eq!(current_scope.split(2), (parse_quote!(aa::bb::cc), parse_quote!(dd::ee)));
    assert_eq!(current_scope.split(3), (parse_quote!(aa::bb), parse_quote!(cc::dd::ee)));
}
