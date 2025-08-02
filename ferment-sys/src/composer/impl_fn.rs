use quote::ToTokens;
use syn::{parse_quote, FnArg, PatType, Path, Receiver, ReturnType, Signature, Type};
use syn::token::Semi;
use crate::ast::CommaPunctuatedTokens;
use crate::composer::{CommaPunctuatedArgKinds, ConversionFromComposer, ConversionToComposer, NameKind, SourceComposable, VarComposer};
use crate::context::ScopeContext;
use crate::ext::{LifetimeProcessor, ToType};
use crate::lang::{FromDictionary, LangAttrSpecification, LangLifetimeSpecification, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ExpressionComposable, ScopeContextPresentable, SeqKind};
use crate::presentation::{DictionaryName, FFIFullDictionaryPath, FFIFullPath, Name};

pub fn compose_impl_fn<SPEC>(
    path: &Path,
    self_ty: &Type,
    aspect: Aspect<SPEC::TYC>,
    attrs: &SPEC::Attr,
    generics: SPEC::Gen,
    sig: &Signature,
    source: &ScopeContext
) -> BindingPresentableContext<SPEC>
where SPEC: Specification<Name=Name<SPEC>, Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      SPEC::Lt: IntoIterator + Extend<<SPEC::Lt as IntoIterator>::Item>,
      SPEC::Name: ToTokens,
      SPEC::Var: ToType,
      CommaPunctuatedArgKinds<SPEC>: Extend<ArgKind<SPEC>>,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType,
      VarComposer<SPEC>: SourceComposable<Source=ScopeContext, Output=SPEC::Var>
{

    let mut used_lifetimes = SPEC::Lt::default();
    let Signature { output, inputs, asyncness, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (
            ReturnType::Default,
            SPEC::Expr::simple(Semi::default())
        ),
        ReturnType::Type(_, ty) => (
            ReturnType::Type(Default::default(), Box::new(VarComposer::<SPEC>::key_ref_in_composer_scope(ty).compose(source).to_type())),
            ConversionToComposer::<SPEC>::key_in_composer_scope(Name::dictionary_name(DictionaryName::Obj), ty).compose(source)
        )
    };

    let mut arguments = CommaPunctuatedArgKinds::<SPEC>::new();
    let mut argument_names = CommaPunctuatedTokens::new();
    let mut argument_conversions = CommaPunctuatedArgKinds::<SPEC>::new();
    for arg in inputs {
        match arg {
            FnArg::Receiver(Receiver { mutability, reference, attrs, .. }) => {
                if let Some((_, Some(lt))) = reference {
                    SPEC::Lt::add_lifetime(&mut used_lifetimes, lt.clone());
                }
                let qualified_ty = match (mutability, reference) {
                    (Some(..), _) => parse_quote!(&mut #self_ty),
                    (_, Some(..)) => parse_quote!(&#self_ty),
                    (..) => self_ty.clone(),
                };
                let name_type_conversion = ConversionFromComposer::<SPEC>::key_in_composer_scope(Name::dictionary_name(DictionaryName::Self_), &qualified_ty).compose(source);
                let name = Name::dictionary_name(DictionaryName::Self_);
                argument_names.push(name.to_token_stream());
                arguments.push(ArgKind::inherited_named_var(name.clone(), VarComposer::<SPEC>::key_ref_in_composer_scope(self_ty).compose(source), SPEC::Attr::from_cfg_attrs(attrs)));
                argument_conversions.push(ArgKind::expr(name_type_conversion));
            },
            FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                used_lifetimes.extend(SPEC::Lt::from_lifetimes(ty.unique_lifetimes()));
                let name = Name::Pat(*pat.clone());
                argument_names.push(name.to_token_stream());
                arguments.push(ArgKind::inherited_named_type(name.clone(), ty, SPEC::Attr::from_cfg_attrs(attrs)));
                argument_conversions.push(ArgKind::expr(ConversionFromComposer::<SPEC>::key_in_composer_scope(name.clone(), ty).compose(source)));
            }
        }
    }
    let aspect_with_args = (aspect, (attrs.clone(), used_lifetimes.clone(), generics.clone()), NameKind::Named);
    let input_conversions = SeqKind::FromUnnamedFields((aspect_with_args, argument_conversions));
    BindingPresentableContext::RegFn(
        path.clone(),
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        input_conversions,
        return_type_conversion,
        attrs.clone(),
        used_lifetimes,
        generics
    )
}
