use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{FnArg, Generics, Lifetime, Path, ReturnType, Signature};
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::CommaPunctuatedTokens;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, BasicComposerLink, ComposerLink, DocComposer, DocsComposable, Linkable, SourceAccessible, SourceComposable, CommaPunctuatedArgKinds, VarComposer, ConversionToComposer, NameKind, SignatureAspect};
use crate::composer::pat_type::PatTypeComposer;
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{ExpressionComposable, ToType};
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind};
use crate::presentation::{DocPresentation, FFIFullDictionaryPath, FFIFullPath, Name};


#[allow(unused)]
#[derive(ComposerBase)]
pub struct ModFnComposer<SPEC>
where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> ModFnComposer<SPEC>
where SPEC: Specification {
    #[allow(unused)]
    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::from(&ty_context), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<SPEC> DocsComposable for ModFnComposer<SPEC>
where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}


pub fn compose_mod_fn<SPEC>(
    path: &Path,
    signature_aspect: SignatureAspect<SPEC>,
    aspect: Aspect<SPEC::TYC>,
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
      VarComposer<SPEC>: SourceComposable<Source=ScopeContext, Output: ToType>
{
    let mut used_lifetimes = signature_aspect.1.clone();
    let Signature { output, inputs, asyncness, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (
            ReturnType::Default,
            SPEC::Expr::simple(Semi::default())
        ),
        ReturnType::Type(_, ty) => (
            ReturnType::Type(Default::default(), Box::new(VarComposer::<SPEC>::key_ref_in_composer_scope(ty).compose(source).to_type())),
            ConversionToComposer::<SPEC>::key_in_composer_scope(Name::obj(), ty).compose(source)
        )
    };
    let mut arguments = CommaPunctuatedArgKinds::<SPEC>::new();
    let mut argument_names = CommaPunctuatedTokens::new();
    let mut argument_conversions = CommaPunctuatedArgKinds::<SPEC>::new();
    for arg in inputs {
        match arg {
            FnArg::Receiver(..) => panic!("Receiver in regular fn"),
            FnArg::Typed(pat_type) => {
                let (lifetimes, tokenized_name, arg_kind, arg_conversion) = PatTypeComposer::new(pat_type).compose(source);
                used_lifetimes.extend(lifetimes);
                argument_names.push(tokenized_name);
                arguments.push(arg_kind);
                argument_conversions.push(arg_conversion);
            }
        }
    }
    let signature_aspect_ext = (signature_aspect.0, used_lifetimes, signature_aspect.2);
    BindingPresentableContext::RegFn(
        path.clone(),
        signature_aspect_ext.clone(),
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        SeqKind::FromUnnamedFields(((aspect, signature_aspect_ext, NameKind::Named), argument_conversions)),
        return_type_conversion
    )
}