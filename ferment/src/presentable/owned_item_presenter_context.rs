use std::fmt::Debug;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{Arm, Attribute, Expr, Field, Pat, PatLit, PatWild, Type, Visibility};
use ferment_macro::Display;
use crate::composable::{FieldComposer, FieldTypeConversionKind};
use crate::composer::{Composer, FromConversionComposer, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::presentable::{Expression, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{ArgPresentation, FFIVariable, Name};


#[derive(Clone, Display, Debug)]
pub enum OwnedItemPresentableContext {
    BindingArg(FieldComposer),
    BindingFieldName(FieldComposer),
    Named(FieldComposer, Visibility),
    DefaultFieldConversion(Name, Vec<Attribute>, FromConversionComposer),
    DefaultFieldType(Type, Vec<Attribute>),
    Lambda(TokenStream2, TokenStream2, Vec<Attribute>),
    Exhaustive(Vec<Attribute>),
    SequenceOutput(SequenceOutput, Vec<Attribute>),
    Expression(Expression, Vec<Attribute>),
    // ConversionType(ConversionType, Vec<Attribute>),
    PatLitExpr(Expr, Vec<Attribute>)
}

fn anonymous_ident(name: &Name) -> Ident {
    format_ident!("o_{}", name.to_token_stream().to_string())
}

impl ScopeContextPresentable for OwnedItemPresentableContext {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            OwnedItemPresentableContext::PatLitExpr(expr, attrs) => {
                // println!("OwnedItemPresentableContext::PatLitExpr({})", expr.to_token_stream());
                ArgPresentation::Pat(Pat::Lit(PatLit { attrs: attrs.clone(), expr: Box::new(expr.clone()) }))
            },
            OwnedItemPresentableContext::Expression(field_type_context, attrs) => {
                // println!("OwnedItemPresentableContext::Expression({})", field_type_context);
                Self::PatLitExpr(Expr::Verbatim(field_type_context.present(source)), attrs.clone())
                    .present(source)
            },
            // OwnedItemPresentableContext::ConversionType(conversion_type, attrs) => {
            //     Self::PatLitExpr(Expr::Verbatim(conversion_type.compose(source).present(source)), attrs.clone())
            //         .present(source)
            // }

            OwnedItemPresentableContext::SequenceOutput(seq, attrs) => {
                // println!("OwnedItemPresentableContext::SequenceOutput({})", seq);
                Self::PatLitExpr(Expr::Verbatim(seq.present(source)), attrs.clone())
                    .present(source)
            },
            OwnedItemPresentableContext::DefaultFieldType(field_type, attrs) => {
                // println!("OwnedItemPresentableContext::DefaultFieldType({})", field_type.to_token_stream());
                Self::PatLitExpr(Expr::Verbatim(<Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()), attrs.clone())
                    .present(source)
            },
            OwnedItemPresentableContext::BindingFieldName(FieldComposer { name, named, attrs, .. }) => {
                // println!("OwnedItemPresentableContext::BindingFieldName({})", name.to_token_stream());
                Self::PatLitExpr(Expr::Verbatim(named.then(|| name.to_token_stream()).unwrap_or(anonymous_ident(name).to_token_stream())), attrs.clone())
                    .present(source)
            },
            OwnedItemPresentableContext::DefaultFieldConversion(name, attrs, composer) => {
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.1: {} ({}), {}", name.to_token_stream(), name, composer);
                let from_conversion_expr = composer.compose(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.2: {} ({}), {}", name.to_token_stream(), name, from_conversion_expr);
                let from_conversion_presentation = from_conversion_expr.present(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.3: {} ({}), {}", name.to_token_stream(), name, from_conversion_presentation);
                ArgPresentation::Field(Field {
                    attrs: attrs.clone(),
                    vis: Visibility::Inherited,
                    ident: Some(name.mangle_ident_default()),
                    colon_token: Some(Default::default()),
                    ty: Type::Verbatim(from_conversion_presentation),
                })
            },
            OwnedItemPresentableContext::BindingArg(FieldComposer { name, kind, named, attrs}) => {
                // println!("OwnedItemPresentableContext::BindingArg: {} ({}), {}", name.to_token_stream(), name, kind.ty().to_token_stream());
                let (ident, ty) = match kind {
                    FieldTypeConversionKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(anonymous_ident(name))),
                        <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_type()
                    ),
                    FieldTypeConversionKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), Type::Verbatim(conversion.clone()))
                };

                ArgPresentation::Field(Field {
                    attrs: attrs.clone(),
                    vis: Visibility::Inherited,
                    ident,
                    colon_token: Default::default(),
                    ty
                })
            },
            OwnedItemPresentableContext::Named(FieldComposer { attrs, name, kind, ..}, visibility) => {
                // println!("OwnedItemPresentableContext::Named: {}", kind.ty().to_token_stream());
                let ty = VariableComposer::from(kind.ty())
                    .compose(source)
                    .to_type();
                // println!("OwnedItemPresentableContext::Named::RESULT: {}", ty.to_token_stream());
                ArgPresentation::Field(Field { attrs: attrs.clone(), vis: visibility.clone(), ident: Some(name.mangle_ident_default()), colon_token: Some(Default::default()), ty })
            },
            OwnedItemPresentableContext::Lambda(name, value, attrs) => {
                // println!("OwnedItemPresentableContext::Lambda({}, {})", name, value);
                ArgPresentation::Arm(Arm {
                    attrs: attrs.clone(),
                    pat: Pat::Verbatim(name.clone()),
                    guard: None,
                    fat_arrow_token: Default::default(),
                    body: Box::new(Expr::Verbatim(value.clone())),
                    comma: None,
                })
            },
            OwnedItemPresentableContext::Exhaustive(attrs) => {
                // println!("OwnedItemPresentableContext::Exhaustive({})", quote!(#(#attrs)*));
                ArgPresentation::Arm(Arm {
                    attrs: attrs.clone(),
                    pat: Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }),
                    guard: None,
                    fat_arrow_token: Default::default(),
                    body: Box::new(Expr::Verbatim(quote!(unreachable!("This is unreachable")))),
                    comma: None,
                })
            }
        }
    }
}
