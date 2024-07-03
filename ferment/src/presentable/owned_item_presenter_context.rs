use std::fmt::Debug;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::Type;
use ferment_macro::Display;
use crate::ast::Depunctuated;
use crate::composable::{FieldComposer, FieldTypeConversionKind};
use crate::composer::{Composer, FromConversionComposer, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::Resolve;
use crate::presentable::{Expression, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{ArgPresentation, Expansion, FFIVariable, Name};


#[derive(Clone, Display, Debug)]
pub enum OwnedItemPresentableContext {
    BindingArg(FieldComposer),
    BindingFieldName(FieldComposer),
    Named(FieldComposer, /*is_public:*/ bool),
    DefaultFieldConversion(FieldComposer, FromConversionComposer, Depunctuated<Expansion>),
    DefaultFieldType(Type, Depunctuated<Expansion>),
    Lambda(TokenStream2, TokenStream2, Depunctuated<Expansion>),
    Conversion(TokenStream2, Depunctuated<Expansion>),
    Exhaustive(TokenStream2),
    Expression(Expression, Depunctuated<Expansion>),
    SequenceOutput(SequenceOutput, Depunctuated<Expansion>),
}

fn anonymous_ident(name: &Name) -> Ident {
    format_ident!("o_{}", name.to_token_stream().to_string())
}
impl ScopeContextPresentable for OwnedItemPresentableContext {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        //println!("OwnedItemPresentableContext::present: {}", self);
        match self {
            OwnedItemPresentableContext::Expression(field_type_context, attrs) => {
                println!("OwnedItemPresentableContext::Expression: {}", field_type_context.present(source));

                ArgPresentation::AttributedConversion {
                    attrs: attrs.to_token_stream(),
                    conversion: field_type_context.present(source)
                }
            },
            OwnedItemPresentableContext::SequenceOutput(seq, attrs) => ArgPresentation::AttributedConversion {
                attrs: attrs.to_token_stream(),
                conversion: seq.present(source)
            },
            OwnedItemPresentableContext::Conversion(expr_lit, attrs) => ArgPresentation::AttributedConversion {
                attrs: attrs.to_token_stream(),
                conversion: expr_lit.to_token_stream()
            },
            OwnedItemPresentableContext::DefaultFieldType(field_type, attrs) => ArgPresentation::AttributedConversion {
                attrs: attrs.to_token_stream(),
                conversion: <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()
            },
            OwnedItemPresentableContext::BindingFieldName(FieldComposer { name, named, attrs, .. }) => ArgPresentation::AttributedConversion {
                attrs: attrs.to_token_stream(),
                conversion: match named {
                    true => name.to_token_stream(),
                    false => anonymous_ident(name).to_token_stream()
                }
            },
            OwnedItemPresentableContext::DefaultFieldConversion(FieldComposer { name, .. }, conversion, attrs) => {
                let var = conversion.compose(source);
                println!("OwnedItemPresentableContext::DefaultFieldConversion: {} --- {}", name.to_token_stream(), var.present(source));
                ArgPresentation::NamedType {
                    attrs: attrs.to_token_stream(),
                    name: name.to_token_stream(),
                    var: var.present(source)
                }
            },
            OwnedItemPresentableContext::BindingArg(FieldComposer { name, kind, named, attrs}) => {
                println!("OwnedItemPresentableContext::BindingArg: {} ({}), {}", name.to_token_stream(), name, kind.ty().to_token_stream());
                let (field_name, conversion) = match (kind, named) {
                    (FieldTypeConversionKind::Type(field_type), true) =>
                        (name.to_token_stream(), <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()),
                    (FieldTypeConversionKind::Type(field_type), false) =>
                        (anonymous_ident(name).to_token_stream(), <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()),
                    (FieldTypeConversionKind::Conversion(conversion), _) =>
                        (name.to_token_stream(), conversion.to_token_stream())
                };
                ArgPresentation::NamedType {
                    attrs: attrs.to_token_stream(),
                    name: field_name,
                    var: conversion
                }
            },


            OwnedItemPresentableContext::Lambda(name, value, attrs) => ArgPresentation::Lambda {
                attrs: attrs.to_token_stream(),
                l_value: name.to_token_stream(),
                r_value: value.to_token_stream()
            },

            OwnedItemPresentableContext::Exhaustive(attrs) => ArgPresentation::Lambda {
                attrs: attrs.to_token_stream(),
                l_value: quote!(_),
                r_value: quote!(unreachable!("This is unreachable"))
            },
            OwnedItemPresentableContext::Named(FieldComposer { attrs, name, kind, ..}, is_public) => {
                // println!("OwnedItemPresentableContext::Named: {}", kind.ty().to_token_stream());
                let var = VariableComposer::from(kind.ty()).compose(source);

                // println!("OwnedItemPresentableContext::Named::RESULT: {}", var.to_token_stream());

                ArgPresentation::QualifiedNamedType {
                    attrs: attrs.to_token_stream(),
                    qualifier: (*is_public).then(|| quote!(pub)).unwrap_or_default(),
                    name: name.to_token_stream(),
                    var: var.to_token_stream(),
                }
            },
        }
    }
}
