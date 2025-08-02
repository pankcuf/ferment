use quote::ToTokens;
use syn::{FnArg, PatType, Receiver, ReturnType, Signature, Type};
use crate::composable::FieldComposer;
use crate::composer::{CommaPunctuatedArgKinds, SourceComposable, VarComposer};
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::Specification;
use crate::presentable::{ArgKind, BindingPresentableContext, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

pub fn compose_trait_inner_fn<SPEC>(
    trait_ty: &Type,
    attrs: &SPEC::Attr,
    sig: &Signature,
    source: &ScopeContext
) -> BindingPresentableContext<SPEC>
where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      SPEC::Lt: IntoIterator + Extend<<SPEC::Lt as IntoIterator>::Item>,
      SPEC::Name: ToTokens,
      CommaPunctuatedArgKinds<SPEC>: Extend<ArgKind<SPEC>>,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType,
      VarComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=SPEC::Var> {
    let Signature { output, inputs, .. } = sig;
    let return_type = match output {
        ReturnType::Default => ReturnType::Default,
        ReturnType::Type(_, ty) => ReturnType::Type(Default::default(), Box::new(VarComposer::<SPEC>::key_ref_in_composer_scope(ty).compose(&source).to_type()))
    };
    let arguments = CommaPunctuatedArgKinds::from_iter(inputs
        .iter()
        .map(|arg| {
            ArgKind::<SPEC>::inherited_named(match arg {
                FnArg::Receiver(Receiver { attrs, .. }) =>
                    FieldComposer::self_var(VarComposer::<SPEC>::key_ref_in_composer_scope(trait_ty).compose(source), attrs),
                FnArg::Typed(PatType { ty, attrs, pat, .. }) =>
                    FieldComposer::named_typed(Name::Pat(*pat.clone()), ty, attrs)
            })
        }));
    BindingPresentableContext::TraitVTableInnerFn(attrs.clone(), sig.ident.clone(), arguments, return_type)
}