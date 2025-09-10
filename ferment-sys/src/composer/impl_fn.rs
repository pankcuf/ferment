use quote::ToTokens;
use syn::{FnArg, Path, Receiver, ReturnType, Signature};
use syn::token::{RArrow, Semi};
use crate::ast::CommaPunctuatedTokens;
use crate::composer::{CommaPunctuatedArgKinds, ConversionFromComposer, ConversionToComposer, FnImplContext, NameKind, SignatureAspect, SourceComposable, VarComposer};
use crate::composer::pat_type::PatTypeComposer;
use crate::context::ScopeContext;
use crate::ext::{Accessory, ExpressionComposable, LifetimeProcessor, Resolve, ToType};
use crate::lang::{LangAttrSpecification, LangLifetimeSpecification, Specification};
use crate::presentable::{ArgKind, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};


pub fn compose_impl_fn<SPEC>(
    path: &Path,
    aspect: SignatureAspect<SPEC>,
    context: FnImplContext<SPEC>,
    sig: &Signature,
    source: &ScopeContext
) -> BindingPresentableContext<SPEC>
where SPEC: Specification<Name=Name<SPEC>, Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      SPEC::Lt: IntoIterator + Extend<<SPEC::Lt as IntoIterator>::Item>,
      SPEC::Name: ToTokens,
      CommaPunctuatedArgKinds<SPEC>: Extend<ArgKind<SPEC>>,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType,
      VarComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=SPEC::Var>
{
    let self_ty = context.self_ty();
    let mut used_lifetimes = aspect.1.clone();
    let Signature { output, inputs, asyncness, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (
            ReturnType::Default,
            SPEC::Expr::simple(Semi::default())
        ),
        ReturnType::Type(_, return_ty) => (
            ReturnType::Type(RArrow::default(), Box::new(VarComposer::<SPEC>::key_ref_in_composer_scope(return_ty).compose(source).to_type())),
            ConversionToComposer::<SPEC>::key_in_composer_scope(Name::obj(), return_ty).compose(source)
        )
    };

    let mut arguments = CommaPunctuatedArgKinds::<SPEC>::new();
    let mut argument_names = CommaPunctuatedTokens::new();
    let mut argument_conversions = CommaPunctuatedArgKinds::<SPEC>::new();
    for arg in inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                let Receiver { mutability, reference, attrs, .. } = receiver;
                let lifetimes = SPEC::Lt::from_lifetimes(receiver.unique_lifetimes());
                let attrs = SPEC::Attr::from_cfg_attrs(attrs);
                used_lifetimes.extend(lifetimes);
                let qualified_ty = match (mutability, reference) {
                    (Some(..), _) => self_ty.joined_mut_ref(),
                    (_, Some(..)) => self_ty.joined_ref(),
                    (..) => self_ty.clone(),
                };
                let name = Name::self_();
                let tokenized_name = name.to_token_stream();
                argument_names.push(tokenized_name);
                let arg_kind = ArgKind::inherited_named_var(name.clone(), VarComposer::<SPEC>::key_ref_in_composer_scope(self_ty).compose(source), attrs);
                arguments.push(arg_kind);
                let arg_conversion = ConversionFromComposer::<SPEC>::key_in_composer_scope(name, &qualified_ty).compose(source);
                argument_conversions.push(ArgKind::expr(arg_conversion));
            },
            FnArg::Typed(pat_type) => {
                let (lifetimes, tokenized_name, arg_kind, arg_conversion) = PatTypeComposer::new(pat_type).compose(source);
                used_lifetimes.extend(lifetimes);
                argument_names.push(tokenized_name);
                arguments.push(arg_kind);
                argument_conversions.push(arg_conversion);
            }
        }
    }
    let aspect_ext = (aspect.0, used_lifetimes, aspect.2);
    let sequence = match context {
        FnImplContext::TypeImpl { aspect, .. } =>
            SeqKind::FromUnnamedFields(((aspect, aspect_ext.clone(), NameKind::Named), argument_conversions)),
        FnImplContext::TraitImpl { self_ty, trait_ty } =>
            SeqKind::TraitImplFnCall(((self_ty.resolve(source), trait_ty.resolve(source), sig.ident.clone()), argument_conversions))
    };
    BindingPresentableContext::RegFn(
        path.clone(),
        aspect_ext,
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        sequence,
        return_type_conversion
    )
}
