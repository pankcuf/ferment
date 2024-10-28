use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use syn::{ImplItem, ItemImpl};
use ferment_macro::ComposerBase;
use crate::ast::Depunctuated;
use crate::composable::{AttrsModel, CfgAttributes, FnSignatureContext, GenModel};
use crate::composer::{AspectPresentable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceFermentable, BasicComposerLink, VTableComposerLink, SourceAccessible};
use crate::composer::vtable::VTableComposer;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{DocPresentation, RustFermentate};

#[derive(ComposerBase)]
pub struct ImplComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    pub methods: Vec<SigComposerLink<LANG, SPEC>>,
    pub vtable: Option<VTableComposerLink<LANG, SPEC>>,
}
impl<LANG, SPEC> ImplComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn from_item_impl(item_impl: &ItemImpl, ty_context: SPEC::TYC, scope: &ScopeChain, scope_context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemImpl { attrs, generics, trait_, self_ty, items, ..  } = item_impl;
        let source = scope_context.borrow();
        let mut methods = Vec::new();
        let attrs_model = AttrsModel::from(attrs);
        items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Method(item) => {
                    let method_scope_context = Rc::new(RefCell::new(source.joined(item)));

                    let sig_context = FnSignatureContext::Impl(*self_ty.clone(), trait_.as_ref().map(|(_, path, _)| path.to_type()), item.sig.clone());
                    let ty_context = ty_context.join_fn(
                        scope.joined_path_holder(&item.sig.ident).0,
                        sig_context,
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
                ty_context.clone(),
                GenModel::new(Some(generics.clone())),
                constants::composer_doc(),
                Rc::clone(scope_context)),
            methods,
            vtable: trait_.as_ref().map(|(..)| VTableComposer::from_trait_path(ty_context, attrs, scope_context))
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
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for ImplComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let source = self.source_ref();
        // println!("ImplComposer::ferment: {}", self.base);
        let mut items = Depunctuated::<RustFermentate>::new();
        self.methods.iter().for_each(|sig_composer| {
            let fermentate = sig_composer.borrow().ferment();
            items.push(fermentate);
        });
        let vtable = self.vtable.as_ref().map(|composer| composer.borrow().compose(&source));
        RustFermentate::Impl { comment: DocPresentation::Empty, items, vtable }
    }
}

