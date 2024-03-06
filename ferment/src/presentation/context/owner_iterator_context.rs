use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, FatArrow, Paren, Semi};
use syn::{parse_quote, Path, Type};
use crate::composer::{Depunctuated, OwnerAspectIteratorLocalContext, VariantIteratorLocalContext};
use crate::context::ScopeContext;
use crate::ext::Mangle;
use crate::interface::{create_struct, package_unboxed_root};
use crate::naming::DictionaryFieldName;
use crate::opposed::Opposed;
use crate::presentation::context::{OwnedItemPresentableContext, FieldTypePresentableContext};
use crate::presentation::context::name::Aspect;
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::Wrapped;


#[derive(Clone, Debug)]
pub enum OwnerIteratorPresentationContext {
    CurlyBracesFields(OwnerAspectIteratorLocalContext<Comma>),
    Variants((Type, Punctuated<OwnerIteratorPresentationContext, Comma>)),
    CurlyVariantFields(VariantIteratorLocalContext),
    RoundVariantFields(VariantIteratorLocalContext),
    RoundBracesFields(OwnerAspectIteratorLocalContext<Comma>),
    MatchFields((Box<FieldTypePresentableContext>, Punctuated<OwnedItemPresentableContext, Comma>)),
    NoFields(Aspect),
    NoFieldsConversion(Aspect),
    EnumUnitFields(VariantIteratorLocalContext),
    TypeAlias(OwnerAspectIteratorLocalContext<Comma>),
    TypeAliasFromConversion(Depunctuated<OwnedItemPresentableContext>),
    TypeAliasToConversion(OwnerAspectIteratorLocalContext<Comma>),
    NamedStruct(OwnerAspectIteratorLocalContext<Comma>),
    UnnamedStruct(OwnerAspectIteratorLocalContext<Comma>),
    Enum(Box<OwnerIteratorPresentationContext>),
    FromRoot(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    Boxed(Box<OwnerIteratorPresentationContext>),
    Lambda(Box<OwnerIteratorPresentationContext>, Box<OwnerIteratorPresentationContext>),
    AddrDeref(TokenStream2),
    Obj,
    Empty,
    UnboxedRoot,
    StructDropBody(Punctuated<OwnedItemPresentableContext, Semi>),
    DropCode(Punctuated<OwnedItemPresentableContext, Semi>),
}

impl ScopeContextPresentable for OwnerIteratorPresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        // println!("OwnerIteratorPresentationContext::: {:?}", self);
        match self {
            OwnerIteratorPresentationContext::Variants((name, fields)) => {
                let name = name.to_mangled_ident_default();
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                // println!("OwnerIteratorPresentationContext::Variants::present {} --- {}", name.to_token_stream(), presentation.to_token_stream());
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
            OwnerIteratorPresentationContext::TypeAlias((aspect, fields)) |
            OwnerIteratorPresentationContext::UnnamedStruct((aspect, fields)) => {
                let ffi_type = aspect.present(source);
                let wrapped = Wrapped::<_, Paren>::new(fields.present(source)).to_token_stream();
                // println!("OwnerIteratorPresentationContext::UnnamedStruct {}", ffi_type.to_token_stream());
                create_struct(&parse_quote!(#ffi_type), quote!(#wrapped;))
            },
            OwnerIteratorPresentationContext::NamedStruct((aspect, fields)) => {
                let ffi_type = aspect.present(source);
                create_struct(&parse_quote!(#ffi_type), Wrapped::<_, Brace>::new(fields.present(source)).to_token_stream())
            },
            OwnerIteratorPresentationContext::Enum(context) => {
                // println!("OwnerIteratorPresentationContext::present {:?}", context);
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
            OwnerIteratorPresentationContext::TypeAliasToConversion((aspect, fields)) => {
                let name = aspect.present(source);
                let presentation = Wrapped::<_, Paren>::new(fields.present(source));
                quote!(#name #presentation)
            },
            OwnerIteratorPresentationContext::NoFields(aspect) => {
                let name = aspect.present(source);
                let path: Path = parse_quote!(#name);
                let ident = &path.segments.last().unwrap().ident;
                quote!(#ident)
            },
            OwnerIteratorPresentationContext::NoFieldsConversion(aspect) => {
                let name = aspect.present(source);
                // println!("NoFieldsConversion: {:?} {}", aspect, name.to_token_stream());
                quote!(#name)
            },
            OwnerIteratorPresentationContext::EnumUnitFields((name, fields)) => {
                let ty = name.present(source);
                let path: Path = parse_quote!(#ty);
                let ident = &path.segments.last().unwrap().ident;
                Opposed::<_, _, syn::token::Eq>::new(ident, fields.present(source))
                    .to_token_stream()
            },
            OwnerIteratorPresentationContext::FromRoot(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            OwnerIteratorPresentationContext::Boxed(conversions) => {
                DictionaryFieldName::BoxedExpression(conversions.present(source)).to_token_stream()
            }
            OwnerIteratorPresentationContext::Lambda(l_value, r_value) => {
                Opposed::<_, _, FatArrow>::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }

            OwnerIteratorPresentationContext::AddrDeref(field_path) => {
                quote!(&*#field_path)
            }
            OwnerIteratorPresentationContext::Obj => DictionaryFieldName::Obj.to_token_stream(),
            OwnerIteratorPresentationContext::Empty => quote!(),
            OwnerIteratorPresentationContext::UnboxedRoot => package_unboxed_root(),
            OwnerIteratorPresentationContext::StructDropBody(items) => {
                let mut result = Punctuated::<TokenStream2, Semi>::from_iter([quote!(let ffi_ref = self)]);
                result.extend(items.present(source));
                result.to_token_stream()
            },
            OwnerIteratorPresentationContext::DropCode(items) =>
                Wrapped::<_, Brace>::new(items.present(source))
                    .to_token_stream(),

            OwnerIteratorPresentationContext::RoundVariantFields(context) => {
                let (aspect, fields) = context;
                // println!("OwnerIteratorPresentationContext::RoundVariantFields: {:?} ---- {:?}", aspect, fields);
                let name = aspect.present(source);
                let path: Path = parse_quote!(#name);
                let ident = &path.segments.last().unwrap().ident;
                let presentation = Wrapped::<_, Paren>::new(fields.present(source));
                // println!("OwnerIteratorPresentationContext::RoundVariantFields: {} ---- {}", ident.to_token_stream(), presentation.to_token_stream());
                quote!(#ident #presentation)
            }
            OwnerIteratorPresentationContext::CurlyVariantFields(context) => {
                let (aspect, fields) = context;
                let name = aspect.present(source);
                let path: Path = parse_quote!(#name);
                let ident = &path.segments.last().unwrap().ident;
                let presentation = Wrapped::<_, Brace>::new(fields.present(source));
                quote!(#ident #presentation)
            }
        }
    }
}
