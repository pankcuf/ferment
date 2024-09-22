use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::lang::objc::composers::AttrWrapper;

#[derive(Clone, Debug)]
pub struct ArgPresentation {
    pub attr: AttrWrapper,
    pub objc_ty: TokenStream2,
    pub c_ty: TokenStream2,
    pub name: TokenStream2,
}

impl ToTokens for ArgPresentation {
    fn to_tokens(&self, _tokens: &mut TokenStream2) {
        // let Self { attr, objc_ty, c_ty, name } = self;
        // quote! {
        //
        // }
    }
}