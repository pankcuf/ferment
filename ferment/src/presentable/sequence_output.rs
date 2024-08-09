use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Attribute, Expr, ExprLet, Pat, Path, PatLit};
use ferment_macro::Display;
use crate::ast::{Assignment, BraceWrapped, CommaPunctuated, Depunctuated, Lambda, ParenWrapped, SemiPunctuated};
use crate::composer::{CommaPunctuatedOwnedItems, OwnedStatement, OwnerAspectWithCommaPunctuatedItems};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Terminated, ToPath};
use crate::presentable::{Aspect, Expression, OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, create_struct, DictionaryName, InterfacesMethodExpr};


#[derive(Clone, Debug, Display)]
pub enum SequenceOutput {
    CurlyBracesFields(OwnerAspectWithCommaPunctuatedItems),
    RoundBracesFields(OwnerAspectWithCommaPunctuatedItems),
    CurlyVariantFields(OwnerAspectWithCommaPunctuatedItems),
    RoundVariantFields(OwnerAspectWithCommaPunctuatedItems),

    Variants(Aspect, Vec<Attribute>, CommaPunctuated<SequenceOutput>),
    MatchFields((Box<Expression>, CommaPunctuatedOwnedItems)),
    NoFields(Aspect),
    NoFieldsConversion(Aspect),
    EnumUnitFields(OwnerAspectWithCommaPunctuatedItems),
    TypeAliasFromConversion(Depunctuated<OwnedItemPresentableContext>),
    // Struct(VariantIteratorLocalContext, ComposerPresenterByRef<(CommaPunctuated<OwnedItemPresentableContext>, ScopeContext), TokenStream2>),
    NamedStruct(OwnerAspectWithCommaPunctuatedItems),
    UnnamedStruct(OwnerAspectWithCommaPunctuatedItems),
    Enum(Box<SequenceOutput>),
    FromRoot(Box<SequenceOutput>, Box<SequenceOutput>),
    Boxed(Box<SequenceOutput>),
    Lambda(Box<SequenceOutput>, Box<SequenceOutput>),
    AddrDeref(TokenStream2),
    Obj,
    Empty,
    UnboxedRoot,
    StructDropBody(OwnedStatement),
    DropCode(OwnedStatement),
}

impl ScopeContextPresentable for SequenceOutput {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            SequenceOutput::Empty =>
                quote!(),
            SequenceOutput::RoundBracesFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote!(#name #presentation)
            },
            SequenceOutput::CurlyBracesFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote!(#name #presentation)
            },
            SequenceOutput::RoundVariantFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SequenceOutput::CurlyVariantFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SequenceOutput::Variants(aspect, attrs, fields) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #name #presentation
                }
            },
            SequenceOutput::MatchFields((presentation_context, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, presentation_context, fields);
                let name = Expression::Match(presentation_context.clone()).present(source);
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote!(#name #presentation)
            },
            // OwnerIteratorPresentationContext::Struct((aspect, fields), composer) => {
            //     let ffi_type = aspect.present(source);
            //     create_struct(
            //         &ffi_type.to_path().segments.last().unwrap().ident,
            //         aspect.attrs().to_token_stream(),
            //         composer(&(fields.clone(), source.clone())))
            // },
            SequenceOutput::UnnamedStruct((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                create_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    ParenWrapped::new(fields.clone()).present(source).terminated())
            },
            SequenceOutput::NamedStruct((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                create_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    BraceWrapped::new(fields.clone()).present(source))
            },
            SequenceOutput::Enum(context) => {
                //println!("SequenceOutput::{}({:?})", self, context);
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            SequenceOutput::TypeAliasFromConversion(fields) => {
                //println!("SequenceOutput::{}({:?})", self, fields);
                fields.present(source)
                    .to_token_stream()
            },
            SequenceOutput::NoFields(aspect) => {
                //println!("SequenceOutput::{}({})", self, aspect);
                let attrs = aspect.attrs();
                let path = aspect.present(source)
                    .to_path();

                let last_segment = path.segments
                    .last()
                    .expect("Empty path");

                quote! {
                    #(#attrs)*
                    #last_segment
                }
            },
            SequenceOutput::NoFieldsConversion(aspect) => {
                println!("SequenceOutput::{}({})", self, aspect);
                aspect.present(source)
                    .to_token_stream()
            },
            SequenceOutput::EnumUnitFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                Assignment::new(
                    aspect.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            SequenceOutput::FromRoot(field_context, conversions) => {
                //println!("SequenceOutput::{}({}, {:?})", self, field_context, conversions);
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            SequenceOutput::Boxed(conversions) => {
                //println!("SequenceOutput::{}({})", self, conversions);
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            SequenceOutput::Lambda(l_value, r_value) => {
                //println!("SequenceOutput::{}({:?}, {:?})", self, l_value, r_value);
                Lambda::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            SequenceOutput::AddrDeref(field_path) => {
                //println!("SequenceOutput::{}({})", self, field_path);
                quote!(&*#field_path)
            }
            SequenceOutput::Obj => {
                //println!("SequenceOutput::{}", self);
                DictionaryName::Obj.to_token_stream()
            },
            SequenceOutput::UnboxedRoot => {
                //println!("SequenceOutput::{}", self);
                Expression::UnboxAny(Expression::DictionaryName(DictionaryName::Ffi).into()).present(source)
            },
            SequenceOutput::StructDropBody(items) => {
                //println!("SequenceOutput::{}({:?})", self, items);
                let mut result = SemiPunctuated::from_iter([
                    ArgPresentation::Pat(Pat::Lit(PatLit { attrs: vec![], expr: Box::new(Expr::Let(ExprLet {
                        attrs: vec![],
                        let_token: Default::default(),
                        pat: Pat::Verbatim(DictionaryName::FfiRef.to_token_stream()),
                        eq_token: Default::default(),
                        expr: Box::new(Expr::Verbatim(quote!(self))),
                    })) }))
                ]);
                result.extend(items.present(source));
                result.to_token_stream()
            },
            SequenceOutput::DropCode(items) => {
                //println!("SequenceOutput::{}({:?})", self, items);
                BraceWrapped::new(items.clone())
                    .present(source)
            }
        };
        // println!("SequenceOutput::{}({})", self, result);
        result
    }
}
