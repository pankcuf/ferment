use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{parse_quote, FnArg, Generics, Lifetime, Path, Receiver, ReturnType, Signature, Type};
use syn::token::{Const, Semi};
use ferment_macro::ComposerBase;
use crate::ast::CommaPunctuatedTokens;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, BasicComposerLink, CommaPunctuatedArgKinds, ComposerLink, ConversionToComposer, DocComposer, DocsComposable, Linkable, SourceAccessible, SourceComposable, VarComposer, SignatureAspect};
use crate::composer::pat_type::PatTypeComposer;
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{ExpressionComposable, LifetimeProcessor, Resolve, ToType};
use crate::lang::{FromDictionary, LangAttrSpecification, LangLifetimeSpecification, Specification};
use crate::presentable::{ArgKind, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::{DictionaryExpr, DictionaryName, DocPresentation, FFIFullDictionaryPath, FFIFullPath, Name};


#[allow(unused)]
#[derive(ComposerBase)]
pub struct TraitFnImplComposer<SPEC>
where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> TraitFnImplComposer<SPEC>
where SPEC: Specification {

    #[allow(unused)]
    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<SPEC> DocsComposable for TraitFnImplComposer<SPEC>
where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}



pub fn compose_trait_impl_fn<SPEC>(
    path: &Path,
    self_ty: &Type,
    trait_ty: &Type,
    aspect: SignatureAspect<SPEC>,
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
    let mut path = path.clone();
    let last = path.segments.pop().unwrap();
    let last_segment = last.value();
    let path = parse_quote!(#path<#trait_ty>::#last_segment);
    let full_self_ty: Type = self_ty.resolve(source);
    let full_trait_ty: Type = trait_ty.resolve(source);
    let mut used_lifetimes = aspect.1.clone();
    let Signature { output, inputs, asyncness, ident, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (
            ReturnType::Default,
            SPEC::Expr::simple(Semi::default())
        ),
        ReturnType::Type(_, return_ty) => (
            ReturnType::Type(Default::default(), Box::new(VarComposer::<SPEC>::key_ref_in_composer_scope(return_ty).compose(source).to_type())),
            ConversionToComposer::<SPEC>::key_in_composer_scope(Name::dictionary_name(DictionaryName::Obj), return_ty).compose(source)
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
                let expr_composer = match (mutability, reference) {
                    (Some(..), _) => |expr: SPEC::Expr| SPEC::Expr::mut_ref(expr),
                    (_, Some(..)) => |expr: SPEC::Expr| SPEC::Expr::r#ref(expr),
                    _ => |expr: SPEC::Expr| SPEC::Expr::simple_expr(expr),
                };
                let name = Name::dictionary_name(DictionaryName::Self_);
                let tokenized_name = name.to_token_stream();
                argument_names.push(tokenized_name);
                let arg_kind = ArgKind::inherited_named_var(name.clone(), VarComposer::<SPEC>::key_ref_in_composer_scope(trait_ty).compose(source), attrs);
                arguments.push(arg_kind);
                let arg_conversion = expr_composer(SPEC::Expr::dict_expr(DictionaryExpr::self_as_trait(&full_self_ty, mutability.map(|m| m.to_token_stream()).unwrap_or_else(|| Const::default().to_token_stream()))));
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

    BindingPresentableContext::RegFn(
        path,
        (aspect.0, used_lifetimes, aspect.2),
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        SeqKind::TraitImplFnCall(((full_self_ty, full_trait_ty, ident.clone()), argument_conversions)),
        return_type_conversion
    )
}

