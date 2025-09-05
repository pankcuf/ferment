use syn::{Path, PathSegment};
use syn::punctuated::Punctuated;

pub trait Split<T> {
    fn split(&self, head_size: usize) -> (T, T);
}

impl Split<Path> for Path {
    fn split(&self, head_size: usize) -> (Path, Path) {
        let segments = self.segments.clone();
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
            Path { leading_colon: None, segments: Punctuated::from_iter(root) },
            Path { leading_colon: None, segments: Punctuated::from_iter(head) },
        )
    }
}

#[test]
pub fn test_split() {
    use syn::parse_quote;
    let current_scope: Path = parse_quote!(aa::bb::cc::dd::ee);
    assert_eq!(current_scope.split(0), (parse_quote!(aa::bb::cc::dd::ee), Path { leading_colon: None, segments: Punctuated::new() }));
    assert_eq!(current_scope.split(1), (parse_quote!(aa::bb::cc::dd), parse_quote!(ee)));
    assert_eq!(current_scope.split(2), (parse_quote!(aa::bb::cc), parse_quote!(dd::ee)));
    assert_eq!(current_scope.split(3), (parse_quote!(aa::bb), parse_quote!(cc::dd::ee)));
}
