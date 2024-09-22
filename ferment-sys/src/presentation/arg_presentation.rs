use proc_macro2::Ident;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Arm, Attribute, Expr, Field, Pat, PatLit, Type, Visibility};

// Field Type or Fn Arg
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ArgPresentation {
    Pat(Pat),
    Field(Field),
    Arm(Arm),
}

impl ArgPresentation {
    pub fn expr(attrs: &Vec<Attribute>, expr: TokenStream2) -> Self {
        Self::Pat(Pat::Lit(PatLit { attrs: attrs.clone(), expr: Box::new(Expr::Verbatim(expr)) }))
    }

    pub fn field(attrs: &Vec<Attribute>, vis: Visibility, ident: Option<Ident>, ty: Type) -> Self {
        Self::Field(Field { attrs: attrs.clone(), vis, ident, colon_token: Default::default(), ty })
    }

    pub fn arm(attrs: &Vec<Attribute>, pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: attrs.clone(), pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })

    }

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