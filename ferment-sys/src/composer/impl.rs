use std::cell::RefCell;
use std::rc::Rc;
use syn::{ImplItem, ItemImpl};
use ferment_macro::ComposerBase;
use crate::ast::Depunctuated;
use crate::composable::{AttrsModel, CfgAttributes, FnSignatureContext, GenModel};
use crate::composer::{AspectPresentable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceFermentable, BasicComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{DocPresentation, RustFermentate};

#[derive(ComposerBase)]
pub struct ImplComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    pub methods: Vec<SigComposerLink<LANG, SPEC>>,
}
impl<LANG, SPEC> ImplComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn from_item_impl(item_impl: &ItemImpl, ty_context: SPEC::TYC, scope: &ScopeChain, context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemImpl { attrs, generics, trait_, self_ty, items, ..  } = item_impl;
        let mut methods = Vec::new();
        let attrs_model = AttrsModel::from(attrs);
        items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Method(item) => {
                    let method_scope_context = Rc::new(RefCell::new(context.borrow().joined(item)));
                    let ty_context = ty_context.join_fn(
                        scope.joined_path_holder(&item.sig.ident).0,
                        FnSignatureContext::Impl(*self_ty.clone(), trait_.as_ref().map(|(_, path, _)| path.to_type()), item.sig.clone()),
                        item.attrs.cfg_attributes()
                    );
                    methods.push(SigComposer::from_impl_item_method(item, ty_context, &method_scope_context));
                },
                _ => {},
            }
        });
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                attrs_model,
                ty_context,
                GenModel::new(Some(generics.clone())),
                constants::composer_doc(),
                Rc::clone(context)),
            methods,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}

impl<LANG, SPEC> DocsComposable for ImplComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for ImplComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let mut items = Depunctuated::<RustFermentate>::new();
        self.methods.iter().for_each(|sig_composer| {
            let fermentate = sig_composer.borrow().ferment();
            items.push(fermentate);
        });
        RustFermentate::Impl { comment: DocPresentation::Empty, items }
    }
}

