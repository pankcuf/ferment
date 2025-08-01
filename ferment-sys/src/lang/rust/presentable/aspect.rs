use proc_macro2::{Group, TokenTree};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::token::Comma;
use crate::ast::{DelimiterTrait, Wrapped};
use crate::composer::PunctuatedArgKinds;
use crate::context::ScopeContext;
use crate::lang::RustSpecification;
use crate::presentable::{Aspect, ScopeContextPresentable, TypeContext};

impl Aspect<TypeContext> {
    #[allow(unused)]
    pub fn allocate<I>(&self, fields: Wrapped<PunctuatedArgKinds<RustSpecification, Comma>, Comma, I>, source: &ScopeContext) -> TokenStream2
    where I: DelimiterTrait {
        let aspect_presentation = self.present(source);
        match self {
            Aspect::Target(_context) => {
                let fields_presentation = TokenTree::Group(Group::new(I::delimiter(), fields.content.present(source).to_token_stream()));
                quote! {
                    #aspect_presentation #fields_presentation
                }
            }
            Aspect::FFI(_context) | Aspect::RawTarget(_context) => {
                let fields_presentation = TokenTree::Group(Group::new(I::delimiter(), fields.content.present(source).to_token_stream()));
                quote! {
                    #aspect_presentation #fields_presentation
                }
            }
        }
    }


}