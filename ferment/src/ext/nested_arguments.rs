use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composition::NestedArgument;

pub trait NestedArguments {
    fn nested_arguments(&self) -> Punctuated<NestedArgument, Comma>;
}
