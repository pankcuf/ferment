use quote::quote;
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::interface::{create_struct, CURLY_BRACES_FIELDS_PRESENTER, SIMPLE_PAIR_PRESENTER, SIMPLE_TERMINATED_PRESENTER};
use crate::presentation::context::{OwnedItemPresenterContext, IteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;

pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(TokenStream2, Vec<OwnedItemPresenterContext>),
    RoundBracesFields(TokenStream2, Vec<OwnedItemPresenterContext>),
    MatchFields(TokenStream2, Vec<OwnedItemPresenterContext>),
    NoFields(TokenStream2),
    EnumUnitFields(TokenStream2, Vec<OwnedItemPresenterContext>),
    TypeAlias(TokenStream2, Vec<OwnedItemPresenterContext>),
    TypeAliasFromConversion(Vec<OwnedItemPresenterContext>),
    TypeAliasToConversion(TokenStream2, Vec<OwnedItemPresenterContext>),
    NamedStruct(TokenStream2, Vec<OwnedItemPresenterContext>),
    UnnamedStruct(TokenStream2, Vec<OwnedItemPresenterContext>),
    EnumNamedVariant(TokenStream2, Vec<OwnedItemPresenterContext>),
    EnumUnamedVariant(TokenStream2, Vec<OwnedItemPresenterContext>),
    Enum(TokenStream2, Vec<OwnedItemPresenterContext>),
}

impl ScopeContextPresentable for OwnerIteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> Self::Presentation {
        match self {
            OwnerIteratorPresentationContext::CurlyBracesFields(name, fields) => {
                SIMPLE_PAIR_PRESENTER(quote!(#name), IteratorPresentationContext::Curly(fields.clone()).present(context))
            },
            OwnerIteratorPresentationContext::RoundBracesFields(name, fields) => {
                SIMPLE_PAIR_PRESENTER(quote!(#name), IteratorPresentationContext::Round(fields.clone()).present(context))
            },
            OwnerIteratorPresentationContext::MatchFields(field_path, fields) => {
                SIMPLE_PAIR_PRESENTER(quote!(match #field_path), IteratorPresentationContext::Curly(fields.clone()).present(context))
            },
            OwnerIteratorPresentationContext::NoFields(name) => {
                quote!(#name)
            },
            OwnerIteratorPresentationContext::EnumUnitFields(name, fields) => {
                let items = fields.iter().map(|item| item.present(context));
                quote!(#name = #(#items)*)
            },
            OwnerIteratorPresentationContext::TypeAlias(name, fields) => {
                create_struct(quote!(#name), SIMPLE_TERMINATED_PRESENTER(IteratorPresentationContext::Round(fields.clone()).present(context)))
            },
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields) => {
                let items = fields.iter().map(|item| item.present(context)).collect::<Vec<_>>();
                quote!(#(#items)*)
            },
            OwnerIteratorPresentationContext::TypeAliasToConversion(name, fields) => {
                let items = fields.iter().map(|item| item.present(context)).collect::<Vec<_>>();
                quote!(#name(#(#items),*))
            },
            OwnerIteratorPresentationContext::UnnamedStruct(name, fields) => {
                create_struct(quote!(#name), SIMPLE_TERMINATED_PRESENTER(IteratorPresentationContext::Round(fields.clone()).present(context)))
            },
            OwnerIteratorPresentationContext::NamedStruct(name, fields) => {
                create_struct(quote!(#name), IteratorPresentationContext::Curly(fields.clone()).present(context))

            },
            OwnerIteratorPresentationContext::EnumNamedVariant(name, fields) => {
                SIMPLE_PAIR_PRESENTER(quote!(#name), IteratorPresentationContext::Curly(fields.clone()).present(context))
            },
            OwnerIteratorPresentationContext::EnumUnamedVariant(name, fields) => {
                SIMPLE_PAIR_PRESENTER(quote!(#name), IteratorPresentationContext::Round(fields.clone()).present(context))
            },
            OwnerIteratorPresentationContext::Enum(name, fields) => {
                let enum_presentation = CURLY_BRACES_FIELDS_PRESENTER((quote!(#name), fields.clone()))
                    .present(context);
                quote! {
                    #[repr(C)]
                    #[allow(non_camel_case_types)]
                    #[derive(Clone)]
                    pub enum #enum_presentation
                }
            }
        }
    }
}
