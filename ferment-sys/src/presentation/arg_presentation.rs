use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Arm, Attribute, Expr, Field, FieldMutability, Pat, Type, Visibility};

// Field Type or Fn Arg
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ArgPresentation {
    // Pat(Pat),
    Field(Field),
    Arm(Arm),
    AttrTokens(Vec<Attribute>, TokenStream2),
}

impl ArgPresentation {
    // pub fn expr(attrs: &Vec<Attribute>, expr: TokenStream2) -> Self {
    //     Self::Pat(Pat::Lit(PatLit { attrs: attrs.clone(), expr: Box::new(Expr::Verbatim(expr)),}))
    // }
    //
    pub fn field(attrs: &Vec<Attribute>, vis: Visibility, ident: Option<Ident>, ty: Type) -> Self {
        Self::Field(Field { attrs: attrs.clone(), vis, mutability: FieldMutability::None, ident, colon_token: Default::default(), ty })
    }

    pub fn arm(attrs: &Vec<Attribute>, pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: attrs.clone(), pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })
    }
    pub fn attr_tokens(attrs: &Vec<Attribute>, tokens: TokenStream2) -> Self {
        Self::AttrTokens(attrs.clone(), tokens)
    }

}
impl ToTokens for ArgPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ArgPresentation::Arm(arm) => arm.to_tokens(tokens),
            // ArgPresentation::Pat(pat) => pat.to_tokens(tokens),
            ArgPresentation::Field(field) => field.to_tokens(tokens),
            ArgPresentation::AttrTokens(attr, expr) => {
                quote!(#(#attr)*) .to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}