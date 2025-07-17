use quote::{quote, ToTokens};
use syn::{Path, ReturnType, Type};
use syn::__private::TokenStream2;
use crate::ast::CommaPunctuatedTokens;
use crate::composer::{BindingAccessorContext, CommaPunctuatedArgKinds, AspectArgComposers, NameKind, ArgKindPair, OwnerAspectSequence, SemiPunctuatedArgKinds, VariableComposer, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::{Mangle, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable, SeqKind};
use crate::presentation::{BindingPresentation, Name};

pub enum BindingPresentableContext<SPEC>
    where SPEC: Specification {
    Constructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    VariantConstructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    Destructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Gen, NameKind),
    Getter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Gen, VariableComposer<SPEC>, TokenStream2),
    Setter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Gen, VariableComposer<SPEC>, TokenStream2),
    RegFn(Path, bool, CommaPunctuatedArgKinds<SPEC>, ReturnType, SeqKind<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Gen, SPEC::Lt),
    #[allow(unused)]
    RegFn2(Path, bool, CommaPunctuatedTokens, CommaPunctuatedArgKinds<SPEC>, ReturnType, Type, SemiPunctuatedArgKinds<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Gen, SPEC::Lt)
}

impl<SPEC> BindingPresentableContext<SPEC>
    where SPEC: Specification {
    pub fn ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((ffi_type, attrs, generics, name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(ffi_type, attrs, generics, name_kind, args, names)
    }
    pub fn variant_ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((aspect, attrs, generics, name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::VariantConstructor(aspect, attrs, generics, name_kind, args, names)
    }
    pub fn dtor(context: AspectArgComposers<SPEC>) -> Self {
        let ((ffi_type, attrs, generics, name_kind), ..) = context;
        BindingPresentableContext::Destructor(ffi_type, attrs, generics, name_kind)
    }
    pub fn get(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, attrs, generics, field_type, field_name, ..) = context;
        BindingPresentableContext::Getter(obj_type, attrs, generics, field_type, field_name)
    }
    pub fn set(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, attrs, generics, field_type, field_name, ..) = context;
        BindingPresentableContext::Setter(obj_type, attrs, generics, field_type, field_name)
    }
    // pub fn reg_fn(context: FunctionContext<SPEC>) -> Self {
    //     let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
    //     let (args, names): (CommaPunctuatedOwnedItems<SPEC>, CommaPunctuatedOwnedItems<SPEC>) = field_pairs.into_iter().unzip();
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

impl ScopeContextPresentable for BindingPresentableContext<RustSpecification> {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            BindingPresentableContext::Constructor(aspect, attrs, generics, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::VariantConstructor(aspect, attrs, generics, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };


                BindingPresentation::VariantConstructor {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            BindingPresentableContext::Destructor(aspect, attrs, generics, _name_kind) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
            BindingPresentableContext::Getter(obj_aspect, attrs, generics, field_type, field_name) => {
                let obj_type = obj_aspect.present(source);

                BindingPresentation::Getter {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::getter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                    generics: generics.clone(),
                }
            },
            BindingPresentableContext::Setter(obj_aspect, attrs, generics, field_type, field_name, ) => {
                let obj_type = obj_aspect.present(source);
                BindingPresentation::Setter {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::setter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                    generics: generics.clone(),
                }
            },
            BindingPresentableContext::RegFn(path, is_async, arguments, return_type, input_conversions, return_type_conversion, attrs, generics, lifetimes) => BindingPresentation::RegularFunction {
                attrs: attrs.clone(),
                is_async: *is_async,
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                lifetimes: lifetimes.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            },
            BindingPresentableContext::RegFn2(path, is_async, argument_names, arguments, return_type, full_fn_path, input_conversions, return_type_conversion, attrs, generics, lifetimes) => BindingPresentation::RegularFunction2 {
                attrs: attrs.clone(),
                is_async: *is_async,
                argument_names: argument_names.clone(),
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                full_fn_path: full_fn_path.clone(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                lifetimes: lifetimes.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            }
        }
    }
}
