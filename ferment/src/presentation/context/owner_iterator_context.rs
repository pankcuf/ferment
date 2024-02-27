use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::{Depunctuated, OwnerIteratorLocalContext};
use crate::context::ScopeContext;
use crate::interface::{create_struct, obj, package_boxed_expression, package_unboxed_root, SIMPLE_PAIR_PRESENTER};
use crate::presentation::context::{OwnedItemPresentableContext, FieldTypePresentableContext};
use crate::presentation::ScopeContextPresentable;


#[derive(Clone, Debug)]
pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(OwnerIteratorLocalContext<Comma>),
    Variants((TokenStream2, Punctuated<OwnerIteratorPresentationContext, Comma>)),
    RoundBracesFields(OwnerIteratorLocalContext<Comma>),
    MatchFields((Box<FieldTypePresentableContext>, Punctuated<OwnedItemPresentableContext, Comma>)),
    NoFields(TokenStream2),
    EnumUnitFields(OwnerIteratorLocalContext<Comma>),
    TypeAlias(OwnerIteratorLocalContext<Comma>),
    TypeAliasFromConversion(Depunctuated<OwnedItemPresentableContext>),
    TypeAliasToConversion(OwnerIteratorLocalContext<Comma>),
    NamedStruct(OwnerIteratorLocalContext<Comma>),
    UnnamedStruct(OwnerIteratorLocalContext<Comma>),
    Enum(Box<OwnerIteratorPresentationContext>),
    FromRoot(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    Boxed(Box<OwnerIteratorPresentationContext>),
    Lambda(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    AddrDeref(TokenStream2),
    Obj,
    Empty,
    UnboxedRoot
}

impl ScopeContextPresentable for OwnerIteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            OwnerIteratorPresentationContext::Variants((name, fields)) => {
                let items = fields.present(source);
                SIMPLE_PAIR_PRESENTER((quote!(#name), quote!({ #items })))
            },
            OwnerIteratorPresentationContext::CurlyBracesFields((name, fields)) => {
                let items = fields.present(source);
                SIMPLE_PAIR_PRESENTER((quote!(#name), quote!({ #items })))
            },
            OwnerIteratorPresentationContext::RoundBracesFields((name, fields)) => {
                let items = fields.present(source);
                SIMPLE_PAIR_PRESENTER((quote!(#name), quote!(( #items ))))
            },
            OwnerIteratorPresentationContext::MatchFields((presentation_context, fields)) => {
                let name = FieldTypePresentableContext::Match(presentation_context.clone()).present(source);
                let items = fields.present(source);
                SIMPLE_PAIR_PRESENTER((name, quote!({ #items })))
            },
            OwnerIteratorPresentationContext::TypeAlias((name, fields)) |
            OwnerIteratorPresentationContext::UnnamedStruct((name, fields)) => {
                let items = fields.present(source);
                create_struct(quote!(#name), quote!(( #items );))
            },
            OwnerIteratorPresentationContext::NamedStruct((name, fields)) => {
                let items = fields.present(source);
                create_struct(quote!(#name), quote!({ #items }))
            },
            OwnerIteratorPresentationContext::Enum(context) => {
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    pub enum #enum_presentation
                }
            },
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields) => {
                fields.present(source)
                    .to_token_stream()
            },
            OwnerIteratorPresentationContext::TypeAliasToConversion((name, fields)) => {
                let items = fields.present(source);
                quote!(#name(#items))
            },
            OwnerIteratorPresentationContext::NoFields(name) => {
                quote!(#name)
            },
            OwnerIteratorPresentationContext::EnumUnitFields((name, fields)) => {
                let items = fields.present(source);
                quote!(#name = #items)
            },
            OwnerIteratorPresentationContext::FromRoot(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            OwnerIteratorPresentationContext::Boxed(conversions) => {
                package_boxed_expression(conversions.present(source))
            }
            OwnerIteratorPresentationContext::Lambda(l_value, r_value) => {
                let l_value = l_value.present(source);
                let r_value = r_value.present(source);
                quote!(#l_value => #r_value)

            }
            OwnerIteratorPresentationContext::AddrDeref(field_path) => {
                quote!(&*#field_path)
            }
            OwnerIteratorPresentationContext::Obj => obj(),
            OwnerIteratorPresentationContext::Empty => quote!(),
            OwnerIteratorPresentationContext::UnboxedRoot => package_unboxed_root(),
        }
    }
}
