use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use ferment_macro::Display;
use crate::ast::{Assignment, BraceWrapped, CommaPunctuated, Lambda, ParenWrapped};
use crate::composer::{AspectCommaPunctuatedArguments, AttrComposable, TypeAspect, VariantComposable, FieldsConversionComposable, SourceComposable, ComposerLinkRef, AspectTerminatedArguments, AspectPresentableArguments};
use crate::context::ScopeContext;
use crate::ext::{LifetimeProcessor, Mangle, ToPath};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::{DictionaryName, InterfacesMethodExpr, present_struct, RustFermentate};


#[derive(Clone, Debug, Display)]
pub enum SeqKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    FromStub(AspectCommaPunctuatedArguments<LANG, SPEC>),
    FromNamedFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    ToNamedFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    FromUnnamedFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    ToUnnamedFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    ToStub(AspectCommaPunctuatedArguments<LANG, SPEC>),
    NamedVariantFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    UnnamedVariantFields(AspectCommaPunctuatedArguments<LANG, SPEC>),
    EnumUnitFields(AspectCommaPunctuatedArguments<LANG, SPEC>),

    Variants(Aspect<SPEC::TYC>, SPEC::Attr, CommaPunctuated<SeqKind<LANG, SPEC>>),
    Unit(Aspect<SPEC::TYC>),
    NoFieldsConversion(Aspect<SPEC::TYC>),
    TypeAliasFromConversion(AspectCommaPunctuatedArguments<LANG, SPEC>),
    NamedStruct(AspectCommaPunctuatedArguments<LANG, SPEC>),
    UnnamedStruct(AspectCommaPunctuatedArguments<LANG, SPEC>),
    StubStruct(AspectCommaPunctuatedArguments<LANG, SPEC>),
    Enum(Box<SeqKind<LANG, SPEC>>),

    StructFrom(Box<SeqKind<LANG, SPEC>>, Box<SeqKind<LANG, SPEC>>),
    StructTo(Box<SeqKind<LANG, SPEC>>, Box<SeqKind<LANG, SPEC>>),

    EnumVariantFrom(Box<SeqKind<LANG, SPEC>>, Box<SeqKind<LANG, SPEC>>),
    EnumVariantTo(Box<SeqKind<LANG, SPEC>>, Box<SeqKind<LANG, SPEC>>),
    EnumVariantDrop(Box<SeqKind<LANG, SPEC>>, Box<SeqKind<LANG, SPEC>>),

    DerefFFI,
    Obj,
    Empty,

    DropStub(AspectTerminatedArguments<LANG, SPEC>),
    StructDropBody(AspectTerminatedArguments<LANG, SPEC>),
    DropCode(AspectTerminatedArguments<LANG, SPEC>),
}

impl<LANG, SPEC> SeqKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn struct_to(field_path: &SeqKind<LANG, SPEC>, conversions: SeqKind<LANG, SPEC>) -> Self {
        Self::StructTo(field_path.clone().into(), conversions.into())
    }
    pub fn struct_from(field_path: &SeqKind<LANG, SPEC>, conversions: SeqKind<LANG, SPEC>) -> Self {
        Self::StructFrom(field_path.clone().into(), conversions.into())
    }
    pub fn variant_from(left: &SeqKind<LANG, SPEC>, right: SeqKind<LANG, SPEC>) -> Self {
        Self::EnumVariantFrom(left.clone().into(), right.clone().into())
    }
    pub fn variant_to(left: &SeqKind<LANG, SPEC>, right: SeqKind<LANG, SPEC>) -> Self {
        Self::EnumVariantTo(left.clone().into(), right.clone().into())
    }
    pub fn variant_drop(left: &SeqKind<LANG, SPEC>, right: SeqKind<LANG, SPEC>) -> Self {
        Self::EnumVariantDrop(left.clone().into(), right.clone().into())
    }
    pub fn struct_drop_post_processor(_: &SeqKind<LANG, SPEC>, right: SeqKind<LANG, SPEC>) -> Self {
        right
    }

    pub fn no_fields<SEP: ToTokens>(((aspect, ..), _): AspectPresentableArguments<LANG, SPEC, SEP>) -> Self {
        Self::NoFieldsConversion(match &aspect {
            Aspect::Target(context) => Aspect::RawTarget(context.clone()),
            _ => aspect.clone(),
        })
    }
    pub fn unit(((aspect, ..), _): &AspectCommaPunctuatedArguments<LANG, SPEC>) -> Self {
        Self::Unit(aspect.clone())
    }
    pub fn variants<C>(composer_ref: &ComposerLinkRef<C>) -> Self
        where C: AttrComposable<SPEC::Attr> + TypeAspect<SPEC::TYC> + VariantComposable<LANG, SPEC> {
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
    pub fn unit_fields(context: &AspectCommaPunctuatedArguments<LANG, SPEC>) -> Self {
        Self::EnumUnitFields(context.clone())
    }
    pub fn brace_variants(context: &AspectCommaPunctuatedArguments<LANG, SPEC>) -> Self {
        Self::NamedVariantFields(context.clone())
    }
    pub fn paren_variants(context: &AspectCommaPunctuatedArguments<LANG, SPEC>) -> Self {
        Self::UnnamedVariantFields(context.clone())
    }
    pub fn empty_root(_: SeqKind<LANG, SPEC>) -> Self {
        Self::Empty
    }
    pub fn bypass(sequence: SeqKind<LANG, SPEC>) -> Self {
        sequence
    }
    pub fn r#enum(context: SeqKind<LANG, SPEC>) -> Self {
        Self::Enum(Box::new(context))
    }
    pub fn fields_from<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<LANG, SPEC> + 'static {
        ctx.fields_from().compose(&())
    }
    pub fn fields_to<C>(ctx: &ComposerLinkRef<C>) -> Self
        where C: FieldsConversionComposable<LANG, SPEC> + 'static {
        ctx.fields_to().compose(&())
    }
}

impl<SPEC> ScopeContextPresentable for SeqKind<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            SeqKind::Empty |
            SeqKind::FromStub(..) |
            SeqKind::ToStub(..) |
            SeqKind::DropStub(..) |
            SeqKind::StubStruct(..) =>
                quote!(),
            SeqKind::FromUnnamedFields(((aspect, _attrs, _generics, _is_round), fields)) |
            SeqKind::ToUnnamedFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                let name = aspect.present(source);
                let presentation = fields.present(source);
                quote!(#name ( #presentation ) )
            },
            SeqKind::FromNamedFields(((aspect, ..), fields)) |
            SeqKind::ToNamedFields(((aspect, ..), fields)) => {
                let name = aspect.present(source);
                let cleaned_name = name.lifetimes_cleaned();
                let presentation = fields.present(source);
                quote!(#cleaned_name { #presentation })
            },
            SeqKind::TypeAliasFromConversion((_, fields)) => {
                fields.present(source)
                    .to_token_stream()
            },
            SeqKind::UnnamedVariantFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SeqKind::NamedVariantFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SeqKind::Variants(aspect, attrs, fields) => {
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #name #presentation
                }
            },
            SeqKind::UnnamedStruct(((aspect, _attrs, _generics, _is_round), fields)) => {
                let ffi_type = aspect.present(source);
                let fields = fields.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    quote!((#fields);))
            },
            SeqKind::NamedStruct(((aspect, _attrs, _generics, _is_round), fields)) => {
                let ffi_type = aspect.present(source);
                let fields = fields.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    quote!({#fields}))
            },
            SeqKind::Enum(context) => {
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            SeqKind::Unit(aspect) => {
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
            SeqKind::NoFieldsConversion(aspect) => {
                aspect.present(source)
                    .to_token_stream()
            },
            SeqKind::EnumUnitFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                Assignment::new(
                    aspect.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            SeqKind::StructFrom(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            SeqKind::StructTo(_field_context, conversions) => {
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            SeqKind::EnumVariantFrom(l_value, r_value) |
            SeqKind::EnumVariantTo(l_value, r_value) |
            SeqKind::EnumVariantDrop(l_value, r_value) => {
                Lambda::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            SeqKind::DerefFFI => {
                let field_path = DictionaryName::Ffi;
                quote!(&*#field_path)
            }
            SeqKind::Obj => {
                DictionaryName::Obj.to_token_stream()
            },
            SeqKind::StructDropBody((_, items)) => {
                let destructors = items.present(source);
                quote! {
                    let ffi_ref = self;
                    #destructors
                }
            },
            SeqKind::DropCode((_, items)) => {
                let destructors = items.present(source);
                quote!({ #destructors })
            }
        };
        result
    }
}
