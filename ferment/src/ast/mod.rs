mod opposed;
// mod sequence;
mod wrapped;

use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Add, Brace, Colon2, Comma, Dot, FatArrow, Paren, Semi};
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
pub type SemiPunctuated<T> = Punctuated<T, Semi>;
#[allow(unused)]
pub type Colon2Punctuated<T> = Punctuated<T, Colon2>;
#[allow(unused)]
pub type AddPunctuated<T> = Punctuated<T, Add>;
#[allow(unused)]
pub type DotPunctuated<T> = Punctuated<T, Dot>;

#[allow(unused)]
pub type Assignment<T1, T2> = Opposed<T1, T2, syn::token::Eq>;
#[allow(unused)]
pub type Lambda<T1, T2> = Opposed<T1, T2, FatArrow>;
