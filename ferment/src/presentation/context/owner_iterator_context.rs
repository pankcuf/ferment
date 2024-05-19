use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, FatArrow, Paren, Semi};
use syn::{Path, Type};
use crate::composer::{Depunctuated, OwnedStatement, VariantIteratorLocalContext};
use crate::context::ScopeContext;
use crate::ext::{Mangle, ToPath};
use crate::interface::{create_struct, package_unboxed_root};
use crate::naming::{DictionaryExpression, DictionaryFieldName};
use crate::opposed::Opposed;
use crate::presentation::context::{OwnedItemPresentableContext, FieldTypePresentableContext};
use crate::presentation::context::name::Aspect;
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::Wrapped;


#[derive(Clone, Debug)]
pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(VariantIteratorLocalContext),
    Variants(((Type, TokenStream2), Punctuated<OwnerIteratorPresentationContext, Comma>)),
    CurlyVariantFields(VariantIteratorLocalContext),
    RoundVariantFields(VariantIteratorLocalContext),
    RoundBracesFields(VariantIteratorLocalContext),
    MatchFields((Box<FieldTypePresentableContext>, Punctuated<OwnedItemPresentableContext, Comma>)),
    NoFields(Aspect),
    NoFieldsConversion(Aspect),
    EnumUnitFields(VariantIteratorLocalContext),
    TypeAlias(VariantIteratorLocalContext),
    TypeAliasFromConversion(Depunctuated<OwnedItemPresentableContext>),
    TypeAliasToConversion(VariantIteratorLocalContext),
    NamedStruct(VariantIteratorLocalContext),
    UnnamedStruct(VariantIteratorLocalContext),
    Enum(Box<OwnerIteratorPresentationContext>),
    FromRoot(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    Boxed(Box<OwnerIteratorPresentationContext>),
    Lambda(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    AddrDeref(TokenStream2),
    Obj,
    Empty,
    UnboxedRoot,
    StructDropBody(OwnedStatement),
    DropCode(OwnedStatement),
}

impl Display for OwnerIteratorPresentationContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OwnerIteratorPresentationContext::CurlyBracesFields(fields) =>
                format!("CurlyBracesFields({:?})", fields),
            OwnerIteratorPresentationContext::Variants(((ty, attrs), variants)) =>
                format!("Variants((({}, {}), {:?}))", ty.to_token_stream(), attrs, variants),
            OwnerIteratorPresentationContext::CurlyVariantFields(fields) =>
                format!("CurlyVariantFields({:?})", fields),
            OwnerIteratorPresentationContext::RoundVariantFields(fields) =>
                format!("RoundVariantFields({:?})", fields),
            OwnerIteratorPresentationContext::RoundBracesFields(fields) =>
                format!("RoundBracesFields({:?})", fields),
            OwnerIteratorPresentationContext::MatchFields((context, fields)) =>
                format!("MatchFields({}, {:?})", context, fields),
            OwnerIteratorPresentationContext::NoFields(aspect) =>
                format!("NoFields({:?})", aspect),
            OwnerIteratorPresentationContext::NoFieldsConversion(aspect) =>
                format!("NoFieldsConversion({:?})", aspect),
            OwnerIteratorPresentationContext::EnumUnitFields(context) =>
                format!("EnumUnitFields({:?})", context),
            OwnerIteratorPresentationContext::TypeAlias(context) =>
                format!("TypeAlias({:?})", context),
            OwnerIteratorPresentationContext::TypeAliasFromConversion(context) =>
                format!("TypeAliasFromConversion({:?})", context),
            OwnerIteratorPresentationContext::TypeAliasToConversion(context) =>
                format!("TypeAliasToConversion({:?})", context),
            OwnerIteratorPresentationContext::NamedStruct(context) =>
                format!("NamedStruct({:?})", context),
            OwnerIteratorPresentationContext::UnnamedStruct(context) =>
                format!("UnnamedStruct({:?})", context),
            OwnerIteratorPresentationContext::Enum(context) =>
                format!("Enum({})", context),
            OwnerIteratorPresentationContext::FromRoot(context, ctx) =>
                format!("FromRoot({}, {})", context, ctx),
            OwnerIteratorPresentationContext::Boxed(context) =>
                format!("Boxed({})", context),
            OwnerIteratorPresentationContext::Lambda(lv, rv) =>
                format!("Lambda({}, {})", lv, rv),
            OwnerIteratorPresentationContext::AddrDeref(context) =>
                format!("AddrDeref({})", context),
            OwnerIteratorPresentationContext::Obj =>
                format!("Obj"),
            OwnerIteratorPresentationContext::Empty =>
                format!("Empty"),
            OwnerIteratorPresentationContext::UnboxedRoot =>
                format!("UnboxedRoot"),
            OwnerIteratorPresentationContext::StructDropBody(context) =>
                format!("StructDropBody({:?})", context),
            OwnerIteratorPresentationContext::DropCode(context) =>
                format!("DropCode({:?})", context),
        }.as_str())
    }
}

impl ScopeContextPresentable for OwnerIteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            OwnerIteratorPresentationContext::Empty =>
                quote!(),
            OwnerIteratorPresentationContext::Variants(((name, _attrs), fields)) => {
                let name = name.mangle_ident_default();
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::CurlyBracesFields((aspect, fields)) => {
                let name = aspect.present(source);
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::RoundBracesFields((aspect, fields)) => {
                let name = aspect.present(source);
                let presentation = Wrapped::<_, Paren>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::MatchFields((presentation_context, fields)) => {
                let name = FieldTypePresentableContext::Match(presentation_context.clone()).present(source);
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::TypeAliasToConversion((aspect, fields)) => {
                let name = aspect.present(source);
                let presentation = Wrapped::<_, Paren>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::RoundVariantFields(context) => {
                let (aspect, fields) = context;
                let name = aspect.present(source);
                let attrs = aspect.attrs();
                let path: Path = name.to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = Wrapped::<_, Paren>::new(fields.present(source));
                quote! {
                    #attrs
                    #ident #presentation
                }
            }
            OwnerIteratorPresentationContext::CurlyVariantFields(context) => {
                let (aspect, fields) = context;
                let name = aspect.present(source);
                let attrs = aspect.attrs();
                let path = name.to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                quote! {
                    #attrs
                    #ident #presentation
                }
            }
            OwnerIteratorPresentationContext::TypeAlias((aspect, fields)) |
            OwnerIteratorPresentationContext::UnnamedStruct((aspect, fields)) => {
                let ffi_type = aspect.present(source);
                let wrapped = Wrapped::<_, Paren>::new(fields.present(source)).to_token_stream();
                create_struct(
                    &ffi_type.to_path(),
                    aspect.attrs().to_token_stream(),
                    quote!(#wrapped;))
            },
            OwnerIteratorPresentationContext::NamedStruct((aspect, fields)) => {
                let ffi_type = aspect.present(source);
                create_struct(
                    &ffi_type.to_path(),
                    TokenStream2::default(),
                    Wrapped::<_, Brace>::new(fields.present(source)).to_token_stream())
            },
            OwnerIteratorPresentationContext::Enum(context) => {
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            OwnerIteratorPresentationContext::TypeAliasFromConversion(fields) => {
                fields.present(source)
                    .to_token_stream()
            },
            OwnerIteratorPresentationContext::NoFields(aspect) => {
                let attrs = aspect.attrs();
                let path = aspect.present(source)
                    .to_path();

                let last_segment = path.segments
                    .last()
                    .expect("Empty path");

                quote! {
                    #attrs
                    #last_segment
                }
            },
            OwnerIteratorPresentationContext::NoFieldsConversion(aspect) => {
                aspect.present(source)
                    .to_token_stream()
            },
            OwnerIteratorPresentationContext::EnumUnitFields((name, fields)) => {
                Opposed::<_, _, syn::token::Eq>::new(
                    name.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            OwnerIteratorPresentationContext::FromRoot(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            OwnerIteratorPresentationContext::Boxed(conversions) => {
                DictionaryExpression::BoxedExpression(conversions.present(source))
                    .to_token_stream()
            }
            OwnerIteratorPresentationContext::Lambda(l_value, r_value) => {
                Opposed::<_, _, FatArrow>::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            OwnerIteratorPresentationContext::AddrDeref(field_path) => {
                quote!(&*#field_path)
            }
            OwnerIteratorPresentationContext::Obj =>
                DictionaryFieldName::Obj.to_token_stream(),
            OwnerIteratorPresentationContext::UnboxedRoot =>
                package_unboxed_root(),
            OwnerIteratorPresentationContext::StructDropBody(items) => {
                let mut result = Punctuated::<TokenStream2, Semi>::from_iter([quote!(let ffi_ref = self)]);
                result.extend(items.present(source));
                result.to_token_stream()
            },
            OwnerIteratorPresentationContext::DropCode(items) =>
                Wrapped::<_, Brace>::new(items.present(source))
                    .to_token_stream(),
        }
    }
}
