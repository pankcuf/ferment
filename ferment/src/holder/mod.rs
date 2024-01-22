mod path_holder;
mod type_holder;
mod typepath_holder;
mod type_and_path_holder;
mod generics_holder;

use std::hash::Hash;
use quote::ToTokens;
pub use self::path_holder::PathHolder;
pub use self::path_holder::EMPTY;
pub use self::type_holder::TypeHolder;
pub use self::typepath_holder::TypePathHolder;
pub use self::type_and_path_holder::TypeAndPathHolder;



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
