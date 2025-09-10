mod opposed;
mod wrapped;

use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::{Attribute, Field, FieldMutability, Type, Visibility};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot, FatArrow, Paren, PathSep, Plus, Semi};
pub use opposed::*;
pub use wrapped::*;

#[allow(unused)]
pub type CommaPunctuated<T> = Punctuated<T, Comma>;
#[allow(unused)]
pub type CommaPunctuatedTokens = CommaPunctuated<TokenStream2>;
#[allow(unused)]
pub type Depunctuated<T> = Punctuated<T, Void>;
#[allow(unused)]
pub type BraceWrapped<S, SP> = Wrapped<S, SP, Brace>;
#[allow(unused)]
pub type ParenWrapped<S, SP> = Wrapped<S, SP, Paren>;
#[allow(unused)]
pub type CommaParenWrapped<S> = Wrapped<S, Comma, Paren>;
#[allow(unused)]
pub type SemiPunctuated<T> = Punctuated<T, Semi>;
#[allow(unused)]
pub type SemiPunctuatedTokens = SemiPunctuated<TokenStream2>;
#[allow(unused)]
pub type Colon2Punctuated<T> = Punctuated<T, PathSep>;
#[allow(unused)]
pub type AddPunctuated<T> = Punctuated<T, Plus>;
#[allow(unused)]
pub type DotPunctuated<T> = Punctuated<T, Dot>;

#[allow(unused)]
pub type Assignment<T1, T2> = Opposed<T1, T2, syn::token::Eq>;
#[allow(unused)]
pub type Lambda<T1, T2> = Opposed<T1, T2, FatArrow>;

pub fn field(attrs: Vec<Attribute>, vis: Visibility, ident: Option<Ident>, ty: Type) -> Field {
    Field { attrs, vis, mutability: FieldMutability::None, ident, colon_token: None, ty }
}

fn field_no_attrs(vis: Visibility, ident: Option<Ident>, ty: Type) -> Field {
    field(vec![], vis, ident, ty)
}
pub fn pub_unnamed_field(ty: Type) -> Field {
    field_no_attrs(Visibility::Public(Default::default()), None, ty)
}
pub fn inherited_named_field(ident: Ident, ty: Type) -> Field {
    field_no_attrs(Visibility::Inherited, Some(ident), ty)
}