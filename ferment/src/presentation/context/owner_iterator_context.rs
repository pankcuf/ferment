use quote::quote;
use syn::__private::TokenStream2;
use crate::composer::OwnerIteratorLocalContext;
use crate::context::ScopeContext;
use crate::interface::{create_struct, CURLY_BRACES_FIELDS_PRESENTER, SIMPLE_PAIR_PRESENTER, SIMPLE_TERMINATED_PRESENTER};
use crate::presentation::context::{OwnedItemPresenterContext, IteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;


#[derive(Clone, Debug)]
pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(OwnerIteratorLocalContext),
    RoundBracesFields(OwnerIteratorLocalContext),
    MatchFields(OwnerIteratorLocalContext),
    NoFields(TokenStream2),
    EnumUnitFields(OwnerIteratorLocalContext),
    TypeAlias(OwnerIteratorLocalContext),
    TypeAliasFromConversion(Vec<OwnedItemPresenterContext>),
    TypeAliasToConversion(OwnerIteratorLocalContext),
    NamedStruct(OwnerIteratorLocalContext),
    UnnamedStruct(OwnerIteratorLocalContext),
    EnumNamedVariant(OwnerIteratorLocalContext),
    EnumUnamedVariant(OwnerIteratorLocalContext),
    Enum(OwnerIteratorLocalContext),
}

impl ScopeContextPresentable for OwnerIteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> Self::Presentation {
        match self {
            OwnerIteratorPresentationContext::CurlyBracesFields((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(context))),
            OwnerIteratorPresentationContext::RoundBracesFields((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name),
                    IteratorPresentationContext::Round(fields.clone())
                        .present(context))),
            OwnerIteratorPresentationContext::MatchFields((field_path, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(match #field_path),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(context))),
            OwnerIteratorPresentationContext::EnumNamedVariant((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name), IteratorPresentationContext::Curly(fields.clone())
                        .present(context))),
            OwnerIteratorPresentationContext::EnumUnamedVariant((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name), IteratorPresentationContext::Round(fields.clone())
                        .present(context))),
            OwnerIteratorPresentationContext::TypeAlias((name, fields)) |
            OwnerIteratorPresentationContext::UnnamedStruct((name, fields)) => {
                create_struct(
                    quote!(#name),
                    SIMPLE_TERMINATED_PRESENTER(IteratorPresentationContext::Round(fields.clone())
                        .present(context)))
            },
            OwnerIteratorPresentationContext::NamedStruct((name, fields)) => {
                create_struct(
                    quote!(#name),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(context))
            },
            OwnerIteratorPresentationContext::Enum((name, fields)) => {
                let enum_presentation = CURLY_BRACES_FIELDS_PRESENTER((quote!(#name), fields.clone()))
                    .present(context);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    pub enum #enum_presentation
                }
            },
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields) => {
                let items = fields.iter().map(|item| item.present(context));
                quote!(#(#items)*)
            },
            OwnerIteratorPresentationContext::TypeAliasToConversion((name, fields)) => {
                let items = fields.iter().map(|item| item.present(context));
                quote!(#name(#(#items),*))
            },
            OwnerIteratorPresentationContext::NoFields(name) => {
                quote!(#name)
            },
            OwnerIteratorPresentationContext::EnumUnitFields((name, fields)) => {
                let items = fields.iter().map(|item| item.present(context));
                quote!(#name = #(#items)*)
            },
        }
    }
}
