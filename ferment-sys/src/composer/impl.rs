use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};
use ferment_macro::ComposerBase;
use crate::ast::Depunctuated;
use crate::composable::{AttrsModel, CfgAttributes, FnSignatureContext, GenModel};
use crate::composer::{BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceFermentable, BasicComposerLink, VTableComposerLink, SourceAccessible};
use crate::composer::vtable::VTableComposer;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::NameTreeContext;
use crate::presentation::{DocComposer, DocPresentation, RustFermentate};

#[derive(ComposerBase)]
pub struct ImplComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
    pub methods: Vec<SigComposerLink<LANG, SPEC>>,
    pub vtable: Option<VTableComposerLink<LANG, SPEC>>,
}
impl<LANG, SPEC> ImplComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
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
                DocComposer::new(ty_context.to_token_stream()),
                attrs_model,
                ty_context.clone(),
                GenModel::new(Some(generics.clone())),
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
          SPEC: Specification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
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

