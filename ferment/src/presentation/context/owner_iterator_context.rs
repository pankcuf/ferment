use quote::quote;
use syn::__private::TokenStream2;
use crate::composer::OwnerIteratorLocalContext;
use crate::context::ScopeContext;
use crate::interface::{create_struct, CURLY_BRACES_FIELDS_PRESENTER, obj, package_boxed_expression, package_unboxed_root, SIMPLE_PAIR_PRESENTER};
use crate::presentation::context::{OwnedItemPresentableContext, IteratorPresentationContext, FieldTypePresentableContext};
use crate::presentation::ScopeContextPresentable;


#[derive(Clone, Debug)]
pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(OwnerIteratorLocalContext),
    RoundBracesFields(OwnerIteratorLocalContext),
    MatchFields((Box<FieldTypePresentableContext>, Vec<OwnedItemPresentableContext>)),
    NoFields(TokenStream2),
    EnumUnitFields(OwnerIteratorLocalContext),
    TypeAlias(OwnerIteratorLocalContext),
    TypeAliasFromConversion(Vec<OwnedItemPresentableContext>),
    TypeAliasToConversion(OwnerIteratorLocalContext),
    NamedStruct(OwnerIteratorLocalContext),
    UnnamedStruct(OwnerIteratorLocalContext),
    EnumNamedVariant(OwnerIteratorLocalContext),
    EnumUnamedVariant(OwnerIteratorLocalContext),
    Enum(OwnerIteratorLocalContext),
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
            OwnerIteratorPresentationContext::CurlyBracesFields((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(source))),
            OwnerIteratorPresentationContext::RoundBracesFields((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name),
                    IteratorPresentationContext::Round(fields.clone())
                        .present(source))),
            OwnerIteratorPresentationContext::MatchFields((presentation_context, fields)) => {
                SIMPLE_PAIR_PRESENTER((
                    FieldTypePresentableContext::Match(presentation_context.clone()).present(source),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(source)))
            },
            OwnerIteratorPresentationContext::EnumNamedVariant((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name), IteratorPresentationContext::Curly(fields.clone())
                        .present(source))),
            OwnerIteratorPresentationContext::EnumUnamedVariant((name, fields)) =>
                SIMPLE_PAIR_PRESENTER((
                    quote!(#name), IteratorPresentationContext::Round(fields.clone())
                        .present(source))),
            OwnerIteratorPresentationContext::TypeAlias((name, fields)) |
            OwnerIteratorPresentationContext::UnnamedStruct((name, fields)) => {
                create_struct(
                    quote!(#name),
                    {
                        let context = IteratorPresentationContext::Round(fields.clone()).present(source);
                        quote!(#context;)
                    }
                )
            },
            OwnerIteratorPresentationContext::NamedStruct((name, fields)) => {
                create_struct(
                    quote!(#name),
                    IteratorPresentationContext::Curly(fields.clone())
                        .present(source))
            },
            OwnerIteratorPresentationContext::Enum((name, fields)) => {
                let enum_presentation = CURLY_BRACES_FIELDS_PRESENTER((quote!(#name), fields.clone()))
                    .present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    pub enum #enum_presentation
                }
            },
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields) => {
                let items = fields.iter().map(|item| item.present(source));
                quote!(#(#items)*)
            },
            OwnerIteratorPresentationContext::TypeAliasToConversion((name, fields)) => {
                let items = fields.iter().map(|item| item.present(source));
                quote!(#name(#(#items),*))
            },
            OwnerIteratorPresentationContext::NoFields(name) => {
                quote!(#name)
            },
            OwnerIteratorPresentationContext::EnumUnitFields((name, fields)) => {
                let items = fields.iter().map(|item| item.present(source));
                quote!(#name = #(#items)*)
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
