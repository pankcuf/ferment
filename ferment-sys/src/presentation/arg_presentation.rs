use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Arm, Attribute, Expr, Field, Pat, Type, Visibility};

// Field Type or Fn Arg
#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ArgPresentation {
    Field(Field),
    Arm(Arm),
    AttrTokens(Vec<Attribute>, TokenStream2),
}

impl ArgPresentation {
    pub fn field(attrs: &[Attribute], vis: Visibility, ident: Option<Ident>, ty: Type) -> Self {
        Self::Field(crate::ast::field(attrs.to_owned(), vis, ident, ty))
    }
    pub fn inherited_field(attrs: &[Attribute], ident: Ident, ty: Type) -> Self {
        Self::field(attrs, Visibility::Inherited, Some(ident), ty)
    }

    pub fn arm(attrs: &[Attribute], pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: attrs.to_owned(), pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })
    }
    pub fn attr_less_arm(pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: vec![], pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })
    }
    pub fn attr_tokens<T: ToTokens>(attrs: &[Attribute], tokens: T) -> Self {
        Self::AttrTokens(attrs.to_owned(), tokens.to_token_stream())
    }
    pub fn no_attr_tokens<T: ToTokens>(tokens: T) -> Self {
        Self::AttrTokens(vec![], tokens.to_token_stream())
    }

}
impl ToTokens for ArgPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Arm(arm) => arm.to_tokens(tokens),
            Self::Field(field) => field.to_tokens(tokens),
            Self::AttrTokens(attr, expr) => {
                quote!(#(#attr)*) .to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}