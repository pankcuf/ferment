use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Arm, Field, Pat};

// Field Type or Fn Arg
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ArgPresentation {
    Pat(Pat),
    Field(Field),
    Arm(Arm),
}
impl ToTokens for ArgPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ArgPresentation::Arm(arm) =>
                arm.to_token_stream(),
            ArgPresentation::Pat(pat) =>
                pat.to_token_stream(),
            ArgPresentation::Field(field) =>
                field.to_token_stream(),
        }.to_tokens(tokens)
    }
}