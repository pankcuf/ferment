mod opposed;
mod path_holder;
mod type_holder;
mod typepath_holder;
mod wrapped;

use std::hash::Hash;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::Generics;
use syn::punctuated::Punctuated;
use syn::token::{Add, Brace, Colon2, Comma, Dot, FatArrow, Paren, Semi};
pub use opposed::*;
pub use path_holder::*;
pub use type_holder::*;
pub use typepath_holder::*;
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
pub type SemiPunctuatedTokens = SemiPunctuated<TokenStream2>;
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

pub trait Holder: Clone + PartialEq + Eq + Hash + std::fmt::Debug + std::fmt::Display {
    type Inner: ToTokens;
    fn inner(&self) -> &Self::Inner;
}


#[macro_export]
macro_rules! impl_holder {
    ($holder_name:ident, $inner_type:ty) => {
        #[derive(Clone)]
        pub struct $holder_name(pub $inner_type);

        impl Holder for $holder_name {
            type Inner = $inner_type;

            fn inner(&self) -> &Self::Inner {
                &self.0
            }
        }

        impl PartialEq for $holder_name {
            fn eq(&self, other: &Self) -> bool {
                self.inner().to_token_stream().to_string() == other.inner().to_token_stream().to_string()
            }
        }

        impl Eq for $holder_name {}

        impl std::hash::Hash for $holder_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.inner().to_token_stream().to_string().hash(state);
            }
        }

        impl std::fmt::Debug for $holder_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(crate::formatter::format_token_stream(self.inner()).as_str())
            }
        }

        impl std::fmt::Display for $holder_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl<'a> From<&'a $inner_type> for $holder_name {
            fn from(value: &'a $inner_type) -> Self {
                Self(value.clone())
            }
        }
        impl From<$inner_type> for $holder_name {
            fn from(value: $inner_type) -> Self {
                Self(value)
            }
        }
        impl syn::parse_quote::ParseQuote for $holder_name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                <$inner_type>::parse(input)
                    .map($holder_name::from)
            }
        }

        impl quote::ToTokens for $holder_name {
            fn to_tokens(&self, tokens: &mut syn::__private::TokenStream2) {
                self.inner().to_tokens(tokens)
            }
        }

    };
}
impl_holder!(GenericsHolder, Generics);