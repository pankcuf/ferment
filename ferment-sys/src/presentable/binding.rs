use quote::{quote, ToTokens};
use syn::{Path, ReturnType, Type};
use syn::__private::TokenStream2;
use crate::composer::{BindingAccessorContext, CommaPunctuatedOwnedItems, DestructorContext, FunctionContext};
use crate::context::ScopeContext;
use crate::ext::ToPath;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{BindingPresentation, Name, RustFermentate};

pub enum BindingPresentableContext<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    Constructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen, bool, CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>),
    VariantConstructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen, bool, CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>),
    Destructor(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, SPEC::Attr, SPEC::Gen),
    Getter(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, Type, TokenStream2, SPEC::Attr, SPEC::Gen),
    Setter(<Aspect<SPEC::TYC> as ScopeContextPresentable>::Presentation, Type, TokenStream2, SPEC::Attr, SPEC::Gen),
    RegFn(Path, bool, CommaPunctuatedOwnedItems<LANG, SPEC>, ReturnType, PresentableSequence<LANG, SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Gen)
    // TraitVTableInnerFn
}

impl<LANG, SPEC> BindingPresentableContext<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub fn ctor(context: FunctionContext<LANG, SPEC>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(ffi_type, attrs, generics, is_round, args, names)
    }
    pub fn variant_ctor(context: FunctionContext<LANG, SPEC>) -> Self {
        let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
        let (args, names): (CommaPunctuatedOwnedItems<LANG, SPEC>, CommaPunctuatedOwnedItems<LANG, SPEC>) = field_pairs.into_iter().unzip();
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
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::Destructor(ty, attrs, generics) => {
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
            BindingPresentableContext::Getter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::getter(obj_type.to_path(), &field_name),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.clone(),
                generics: generics.clone(),
            },
            BindingPresentableContext::Setter(obj_type, field_type, field_name, attrs, generics) => BindingPresentation::Getter {
                attrs: attrs.clone(),
                name: Name::setter(obj_type.to_path(), &field_name),
                field_name: field_name.clone(),
                obj_type: obj_type.clone(),
                field_type: field_type.clone(),
                generics: generics.clone(),
            },
            BindingPresentableContext::RegFn(path, is_async, arguments, return_type, input_conversions, return_type_conversion, attrs, generics) => BindingPresentation::RegularFunction {
                attrs: attrs.clone(),
                is_async: *is_async,
                arguments: arguments.present(&source),
                name: Name::ModFn(path.clone()),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                output_conversions: <SPEC::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            }
        }
    }
}
