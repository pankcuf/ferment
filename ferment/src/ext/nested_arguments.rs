use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composition::NestedArgument;

#[allow(unused)]
pub trait NestedArguments {
    fn nested_arguments(&self) -> Punctuated<NestedArgument, Comma>;
}
