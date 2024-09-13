use std::cell::RefCell;
use std::rc::Rc;
use syn::{Attribute, Generics, ImplItem, ItemImpl};
use crate::ast::{Depunctuated, PathHolder};
use crate::composable::{AttrsModel, CfgAttributes, GenModel};
use crate::composer::{BasicComposer, BasicComposerOwner, Composer, constants, DocsComposable, ImplComposerLink, Linkable, ComposerLink, SigComposer, SigComposerLink, SourceFermentable2, NameContext, SourceAccessible, AttrComposable};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::{CrateExtension, Join, ToPath, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DocPresentation, RustFermentate};

// #[derive(BasicComposerOwner)]
pub struct ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static {
    pub base: BasicComposer<ImplComposerLink<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
    pub methods: Vec<SigComposerLink<LANG, SPEC, Gen>>,
}
impl<LANG, SPEC, Gen> ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn from_item_impl(item_impl: &ItemImpl, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> ImplComposerLink<LANG, SPEC, Gen> {
        let ItemImpl { attrs, generics, trait_, self_ty, items, ..  } = item_impl;
        let mut full_fn_path = scope.self_path_holder();
        if full_fn_path.is_crate_based() {
            full_fn_path.replace_first_with(&PathHolder::from(scope.crate_ident_ref().to_path()));
        }
        let mut methods = Vec::new();
        let attrs_model = AttrsModel::from(attrs);
        items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Method(item) => {
                    let method_scope_context = Rc::new(RefCell::new(context.borrow().joined(item)));
                    methods.push(SigComposer::from_impl_item_method(
                        item,
                        self_ty,
                        trait_.as_ref().map(|(_, path, _)| path.to_type()),
                        scope,
                        &method_scope_context
                    ));
                },
                _ => {},
            }
        });
        let ty_context = Context::Impl { path: full_fn_path.0, attrs: attrs.cfg_attributes() };

        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(attrs_model, ty_context, GenModel::new(Some(generics.clone())), constants::composer_doc(), Rc::clone(context)),
            methods,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}
impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<LANG, SPEC, Gen> AttrComposable<SPEC> for ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}

impl<LANG, SPEC, Gen> DocsComposable for ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}

impl<LANG, SPEC, Gen> SourceAccessible for ImplComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}


impl SourceFermentable2<RustFermentate> for ImplComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    fn ferment(&self) -> Depunctuated<RustFermentate> {
        let mut items = Depunctuated::<RustFermentate>::new();
        self.methods.iter().for_each(|sig_composer| {
            let fermentate = sig_composer.borrow().ferment();
            items.extend(fermentate);
        });
        Depunctuated::from_iter([
            RustFermentate::Impl { comment: DocPresentation::Empty, items }
        ])
    }
}