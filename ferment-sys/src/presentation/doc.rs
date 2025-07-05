use std::marker::PhantomData;
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_quote;
use crate::composer::{Linkable, SourceComposable};
use crate::context::ScopeContextLink;
use crate::formatter::format_token_stream;
use crate::lang::Specification;
use crate::shared::SharedAccess;

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum DocPresentation {
    Empty,
    Default(TokenStream2),
    DefaultT(TokenStream2),
    Direct(TokenStream2),
    Safety(TokenStream2),
}

pub fn default_doc<T: ToTokens>(name: T) -> TokenStream2 {
    let comment = format!("FFI-representation of the [`{}`]", format_token_stream(name));
    // TODO: FFI-representation of the [`{}`](../../path/to/{}.rs)
    parse_quote! { #[doc = #comment] }

}

impl ToTokens for DocPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty => quote!(),
            Self::Direct(target_name) => quote!(#target_name),
            Self::Default(target_name) => default_doc(target_name),
            Self::DefaultT(target_name) => default_doc(target_name),
            Self::Safety(target_name) => {
                let doc = default_doc(target_name);
                quote! {
                    #doc
                    /// # Safety
                }
            }
        }.to_tokens(tokens)
    }
}


pub struct DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub parent: Option<Link>,
    pub ty: TokenStream2,
    _marker: PhantomData<SPEC>,
}

impl<SPEC, Link> DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub fn new(ty: TokenStream2) -> Self {
        Self { parent: None, ty, _marker: PhantomData }
    }
}

impl<SPEC, Link> Linkable<Link> for DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    type Source = ScopeContextLink;
    type Output = TokenStream2;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        default_doc(&self.ty)
    }
}
