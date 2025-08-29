use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{FnArg, Generics, Lifetime, Path, ReturnType, Signature};
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::CommaPunctuatedTokens;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, BasicComposerLink, ComposerLink, DocComposer, DocsComposable, Linkable, SourceAccessible, SourceComposable, CommaPunctuatedArgKinds, VarComposer, ConversionToComposer, NameKind};
use crate::composer::pat_type::PatTypeComposer;
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::ToType;
use crate::lang::{FromDictionary, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ExpressionComposable, ScopeContextPresentable, SeqKind};
use crate::presentation::{DictionaryName, DocPresentation, FFIFullDictionaryPath, FFIFullPath, Name};


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
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
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
    aspect: Aspect<SPEC::TYC>,
    attrs: &SPEC::Attr,
    generics: SPEC::Gen,
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
    BindingPresentableContext::RegFn(
        path.clone(),
        attrs.clone(),
        used_lifetimes.clone(),
        generics.clone(),
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        SeqKind::FromUnnamedFields(((aspect, (attrs.clone(), used_lifetimes, generics), NameKind::Named), argument_conversions)),
        return_type_conversion
    )
}