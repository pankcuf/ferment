use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Expr, ExprLet, Pat, Path, PatLit};
use ferment_macro::Display;
use crate::ast::{Assignment, BraceWrapped, CommaPunctuated, Depunctuated, Lambda, ParenWrapped, SemiPunctuated};
use crate::composer::{AspectCommaPunctuatedArguments, AttrComposable, TypeAspect, VariantComposable, FieldsConversionComposable, SourceComposable, ComposerLinkRef, PresentableSequencePair, AspectTerminatedArguments, CommaPunctuatedPresentableArguments};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Terminated, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, DictionaryName, InterfacesMethodExpr, present_struct, RustFermentate};


#[derive(Clone, Debug, Display)]
pub enum PresentableSequence<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    CurlyBracesFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    RoundBracesFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    CurlyVariantFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    RoundVariantFields(AspectCommaPunctuatedArguments<LANG, SPEC>),

    Variants(Aspect<SPEC::TYC>, SPEC::Attr, CommaPunctuated<PresentableSequence<LANG, SPEC>>),
    NoFields(Aspect<SPEC::TYC>),
    NoFieldsConversion(Aspect<SPEC::TYC>),
    EnumUnitFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    TypeAliasFromConversion(((Aspect<SPEC::TYC>, SPEC::Gen), CommaPunctuatedPresentableArguments<LANG, SPEC>)),
    NamedStruct(AspectCommaPunctuatedArguments<LANG, SPEC>),
    UnnamedStruct(AspectCommaPunctuatedArguments<LANG, SPEC>),
    Enum(Box<PresentableSequence<LANG, SPEC>>),
    FromRoot(Box<PresentableSequence<LANG, SPEC>>, Box<PresentableSequence<LANG, SPEC>>),
    ToRoot(Box<PresentableSequence<LANG, SPEC>>, Box<PresentableSequence<LANG, SPEC>>),
    Boxed(Box<PresentableSequence<LANG, SPEC>>),
    Lambda(Box<PresentableSequence<LANG, SPEC>>, Box<PresentableSequence<LANG, SPEC>>),
    DerefFFI,
    Obj,
    Empty,
    UnboxedRoot,
    StructDropBody(AspectTerminatedArguments<LANG, SPEC>),
    DropCode(AspectTerminatedArguments<LANG, SPEC>),
}

impl<LANG, SPEC> PresentableSequence<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn boxed((_, conversions): &PresentableSequencePair<LANG, SPEC>) -> Self {
        Self::Boxed(conversions.clone().into())
    }
    pub fn ffi_to_root((field_path, conversions): &PresentableSequencePair<LANG, SPEC>) -> Self {
        Self::ToRoot(field_path.clone().into(), conversions.clone().into())
    }
    pub fn ffi_from_root((field_path, conversions): &PresentableSequencePair<LANG, SPEC>) -> Self {
        Self::FromRoot(field_path.clone().into(), conversions.clone().into())
    }
    pub fn lambda((left, right): &PresentableSequencePair<LANG, SPEC>) -> Self {
        Self::Lambda(left.clone().into(), right.clone().into())
    }
    pub fn struct_drop_post_processor((_, right): &PresentableSequencePair<LANG, SPEC>) -> Self {
        right.clone()
    }

    pub fn no_fields(((aspect, _generics), _): ((Aspect<SPEC::TYC>, SPEC::Gen), CommaPunctuatedPresentableArguments<LANG, SPEC>)) -> Self {
        Self::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            _ => aspect.clone(),
        })
    }
    pub fn variants<C>(composer_ref: &ComposerLinkRef<C>) -> Self
        where C: AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + VariantComposable<LANG, SPEC>,
              SPEC::Expr: ScopeContextPresentable {
        Self::Variants(C::target_type_aspect(composer_ref), C::compose_attributes(composer_ref), C::compose_variants(composer_ref))
    }
    pub fn deref_ffi<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::DerefFFI
    }
    pub fn empty<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::Empty
    }
    pub fn obj<C>(_ctx: &ComposerLinkRef<C>) -> Self {
        Self::Obj
    }
    pub fn unboxed_root(_: PresentableSequence<LANG, SPEC>) -> Self {
        Self::UnboxedRoot
    }
    pub fn empty_root(_: PresentableSequence<LANG, SPEC>) -> Self {
        Self::Empty
    }
    pub fn bypass(sequence: PresentableSequence<LANG, SPEC>) -> Self {
        sequence
    }
    pub fn r#enum(context: PresentableSequence<LANG, SPEC>) -> Self {
        Self::Enum(Box::new(context))
    }
    pub fn fields_from<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<LANG, SPEC> + 'static,
              SPEC::Expr: ScopeContextPresentable {
        ctx.fields_from().compose(&())
    }
    pub fn fields_to<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<LANG, SPEC> + 'static,
              SPEC::Expr: ScopeContextPresentable {
        ctx.fields_to().compose(&())
    }
}

// impl<LANG, SPEC> Display for SequenceOutput<LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

impl<SPEC> ScopeContextPresentable for PresentableSequence<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            PresentableSequence::Empty =>
                quote!(),
            PresentableSequence::RoundBracesFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = fields.present(source);
                quote!(#name ( #presentation ) )
            },
            PresentableSequence::CurlyBracesFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = fields.present(source);
                quote!(#name { #presentation })
            },
            PresentableSequence::RoundVariantFields(((aspect, _generics), fields)) => {
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
            PresentableSequence::TypeAliasFromConversion((_, fields)) => {
                //println!("SequenceOutput::{}({:?})", self, fields);
                Depunctuated::from_iter(fields.clone())
                    .present(source)
                    .to_token_stream()
            },
            PresentableSequence::CurlyVariantFields(((aspect, _generics), fields)) => {
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
            PresentableSequence::Variants(aspect, attrs, fields) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #name #presentation
                }
            },
            PresentableSequence::UnnamedStruct(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    ParenWrapped::new(fields.clone()).present(source).terminated())
            },
            PresentableSequence::NamedStruct(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    BraceWrapped::new(fields.clone()).present(source))
            },
            PresentableSequence::Enum(context) => {
                //println!("SequenceOutput::{}({:?})", self, context);
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            PresentableSequence::NoFields(aspect) => {
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
            PresentableSequence::NoFieldsConversion(aspect) => {
                // println!("SequenceOutput::{}({})", self, aspect);
                aspect.present(source)
                    .to_token_stream()
            },
            PresentableSequence::EnumUnitFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                Assignment::new(
                    aspect.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            PresentableSequence::FromRoot(field_context, conversions) => {
                //println!("SequenceOutput::{}({}, {:?})", self, field_context, conversions);
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            PresentableSequence::ToRoot(_field_context, conversions) => {
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            PresentableSequence::Boxed(conversions) => {
                //println!("SequenceOutput::{}({})", self, conversions);
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            PresentableSequence::Lambda(l_value, r_value) => {
                //println!("SequenceOutput::{}({:?}, {:?})", self, l_value, r_value);
                Lambda::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            PresentableSequence::DerefFFI => {
                let field_path = DictionaryName::Ffi;
                //println!("SequenceOutput::{}({})", self, field_path);
                quote!(&*#field_path)
            }
            PresentableSequence::Obj => {
                //println!("SequenceOutput::{}", self);
                DictionaryName::Obj.to_token_stream()
            },
            PresentableSequence::UnboxedRoot => {
                //println!("SequenceOutput::{}", self);
                SPEC::Expr::destroy_complex_tokens(DictionaryName::Ffi)
                    .present(source)
                    .to_token_stream()
            },
            PresentableSequence::StructDropBody((_aspect, items)) => {
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
            PresentableSequence::DropCode((_aspect, items)) => {
                //println!("SequenceOutput::{}({:?})", self, items);
                BraceWrapped::new(items.clone())
                    .present(source)
            }
        };
        // println!("SequenceOutput::{}({})", self, result);
        result
    }
}
