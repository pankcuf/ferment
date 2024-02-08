use quote::{format_ident, quote, ToTokens};
use syn::{Ident, parse_quote, Path, PathSegment, Type, TypePath};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::holder::Holder;
use crate::impl_holder;

pub const EMPTY: PathHolder = PathHolder(Path { leading_colon: None, segments: Punctuated::new() });


impl_holder!(PathHolder, Path);

impl PathHolder {

    pub fn crate_root() -> Self {
        parse_quote!(crate)
    }
    pub fn ffi_expansion_scope() -> Self {
        Self::crate_and(quote!(fermented))
    }
    pub fn ffi_generics_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(generics))
    }
    pub fn ffi_types_scope() -> Self {
        Self::ffi_expansion_scope().joined_path(parse_quote!(types))
    }

    pub fn crate_and(path: TokenStream2) -> Self {
        Self::crate_root().joined_path(parse_quote!(#path))
    }
    pub fn ffi_types_and(path: TokenStream2) -> Self {
        Self::ffi_types_scope().joined_path(parse_quote!(#path))
    }


    pub fn len(&self) -> usize {
        self.0.segments.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.segments.is_empty()
    }

    pub fn extract_type_scope(ty: &Type) -> PathHolder {
        match ty {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) => {
                let new_segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                if new_segments.is_empty() {
                    EMPTY
                } else {
                    parse_quote!(#(#new_segments)::*)
                }
            },
            _ => EMPTY
        }
    }


    pub fn is_crate(&self) -> bool {
        self.0.segments.last().unwrap().ident == format_ident!("crate")
    }

    pub fn has_belong_to_current_crate(&self) -> bool {
        self.0.segments.first().unwrap().ident == format_ident!("crate")
    }

    pub fn root_ident(&self) -> Ident {
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

    // pub fn joined_export_id(&self, name: &ScopeTreeExportID) -> PathHolder {
    //     let mut segments = self.0.segments.clone();
    //     segments.push(PathSegment::from(name.clone()));
    //     PathHolder::from(Path { leading_colon: None, segments })
    // }

    pub fn joined_chunk(&self, chunk: &Vec<Ident>) -> PathHolder {
        let mut segments = self.0.segments.clone();
        chunk.iter().for_each(|name| segments.push(PathSegment::from(name.clone())));
        PathHolder::from(Path { leading_colon: None, segments })
    }

    pub fn joined_path(&self, path: Path) -> PathHolder {
        parse_quote!(#self::#path)
    }

    pub fn popped(&self) -> PathHolder {
        let segments = self.0.segments.clone();
        let n = segments.len() - 1;
        PathHolder::from(Path { leading_colon: None, segments: Punctuated::from_iter(segments.into_iter().take(n)) })
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
