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
    pub fn field(attrs: &Vec<Attribute>, vis: Visibility, ident: Option<Ident>, ty: Type) -> Self {
        Self::Field(Field { attrs: attrs.clone(), vis, mutability: FieldMutability::None, ident, colon_token: Default::default(), ty })
    }
    pub fn inherited_field(attrs: &Vec<Attribute>, ident: Ident, ty: Type) -> Self {
        Self::field(attrs, Visibility::Inherited, Some(ident), ty)
    }

    pub fn arm(attrs: &Vec<Attribute>, pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: attrs.clone(), pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })
    }
    pub fn attr_less_arm(pat: Pat, body: TokenStream2) -> Self {
        Self::Arm(Arm { attrs: vec![], pat, guard: None, fat_arrow_token: Default::default(), body: Box::new(Expr::Verbatim(body)), comma: None })
    }
    pub fn attr_tokens<T: ToTokens>(attrs: &Vec<Attribute>, tokens: T) -> Self {
        Self::AttrTokens(attrs.clone(), tokens.to_token_stream())
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