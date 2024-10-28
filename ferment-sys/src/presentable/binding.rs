use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::{Path, ReturnType};
use syn::__private::TokenStream2;
use crate::composer::{BindingAccessorContext, CommaPunctuatedPresentableArguments, DestructorContext, FunctionContext};
use crate::context::ScopeContext;
use crate::ext::{Mangle, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{BindingPresentation, Name, RustFermentate};

pub enum BindingPresentableContext<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    Constructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen, bool, CommaPunctuatedPresentableArguments<LANG, SPEC>, CommaPunctuatedPresentableArguments<LANG, SPEC>),
    VariantConstructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen, bool, CommaPunctuatedPresentableArguments<LANG, SPEC>, CommaPunctuatedPresentableArguments<LANG, SPEC>),
    Destructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen),
    Getter(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Var, TokenStream2, SPEC::Attr, SPEC::Gen),
    Setter(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Var, TokenStream2, SPEC::Attr, SPEC::Gen),
    RegFn(Path, bool, CommaPunctuatedPresentableArguments<LANG, SPEC>, ReturnType, SeqKind<LANG, SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Gen)
}

impl<LANG, SPEC> BindingPresentableContext<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub fn ctor(context: FunctionContext<LANG, SPEC>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedPresentableArguments<LANG, SPEC>, CommaPunctuatedPresentableArguments<LANG, SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(ffi_type, attrs, generics, is_round, args, names)
    }
    pub fn variant_ctor(context: FunctionContext<LANG, SPEC>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedPresentableArguments<LANG, SPEC>, CommaPunctuatedPresentableArguments<LANG, SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::VariantConstructor(ffi_type, attrs, generics, is_round, args, names)
    }
    pub fn dtor(context: DestructorContext<LANG, SPEC>) -> Self {
        let (ffi_type, attrs, generics, ..) = context;
        BindingPresentableContext::Destructor(ffi_type, attrs, generics)
    }
    pub fn get(context: BindingAccessorContext<LANG, SPEC>) -> Self {
        let (obj_type, field_name, field_type, attrs, generics, ..) = context;
        BindingPresentableContext::Getter(obj_type, field_type, field_name, attrs, generics)
    }
    pub fn set(context: BindingAccessorContext<LANG, SPEC>) -> Self {
        let (obj_type, field_name, field_type, attrs, generics, ..) = context;
        BindingPresentableContext::Setter(obj_type, field_type, field_name, attrs, generics)
    }
    // pub fn reg_fn(context: FunctionContext<LANG, SPEC>) -> Self {
    //     let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
    //     let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
    //     BindingPresentableContext::RegFn(
    //         path,
    //         asyncness.is_some(),
    //         arguments,
    //         return_type_presentation,
    //         input_conversions,
    //         return_type_conversion,
    //         attrs,
    //         generics
    //     )
    // }
}

impl<SPEC> ScopeContextPresentable for BindingPresentableContext<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(ty, attrs, generics, is_round, args, body) => {
                let body = body.present(source);
                let body_presentation = if *is_round {
                    quote!((#body))
                } else {
                    quote!({#body})
                };
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    name: Name::<RustFermentate, SPEC>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::VariantConstructor(ty, attrs, generics, is_round, args, body) => {
                let body = body.present(source);
                let body_presentation = if *is_round {
                    quote!((#body))
                } else {
                    quote!({#body})
                };

                BindingPresentation::VariantConstructor {
                    attrs: attrs.clone(),
                    name: Name::<RustFermentate, SPEC>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::Destructor(ty, attrs, generics) => {
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    name: Name::<RustFermentate, SPEC>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
            BindingPresentableContext::Getter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::<RustFermentate, SPEC>::getter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.to_type(),
                generics: generics.clone(),
            },
            BindingPresentableContext::Setter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::<RustFermentate, SPEC>::setter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.to_type(),
                generics: generics.clone(),
            },
            BindingPresentableContext::RegFn(path, is_async, arguments, return_type, input_conversions, return_type_conversion, attrs, generics) => BindingPresentation::RegularFunction {
                attrs: attrs.clone(),
                is_async: *is_async,
                arguments: arguments.present(&source),
                name: Name::<RustFermentate, SPEC>::ModFn(path.clone()).mangle_tokens_default(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                output_conversions: <SPEC::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            }
        }
    }
}
