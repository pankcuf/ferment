use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Generics, ItemTrait, TraitItem, TraitItemFn, Lifetime};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, FnSignatureContext, GenModel, LifetimesModel, TraitTypeModel};
use crate::composer::{BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceAccessible, BasicComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, ToType};
use crate::lang::Specification;
use crate::presentable::NameTreeContext;
use crate::presentation::{DocComposer, DocPresentation};

#[derive(ComposerBase)]
pub struct TraitComposer<SPEC>
    where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
    pub methods: Vec<SigComposerLink<SPEC>>,
    #[allow(unused)]
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<SPEC> TraitComposer<SPEC>
    where SPEC: Specification {
    pub fn from_item_trait(
        item_trait: &ItemTrait,
        ty_context: SPEC::TYC,
        scope: &ScopeChain,
        scope_context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemTrait { attrs, generics, ident, items,  .. } = item_trait;
        let self_ty = ident.to_type();
        let source = scope_context.borrow();
        let mut methods = vec![];
        let mut types = HashMap::new();
        items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Fn(trait_item_method) => {
                    let TraitItemFn { sig, attrs, .. } = trait_item_method;
                    let sig_context = FnSignatureContext::TraitInner(self_ty.clone(), Some(self_ty.clone()), sig.clone());
                    let method_scope_context = Rc::new(RefCell::new(source.joined(trait_item_method)));
                    let ty_context = ty_context.join_fn(
                        scope.joined_path_holder(&sig.ident).0,
                        sig_context,
                        attrs.clone());
                    methods.push(SigComposer::from_trait_item_method(trait_item_method, ty_context, &method_scope_context));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                _ => {}
            });
        Self::new(
            methods,
            types,
            ty_context,
            Some(generics.clone()),
            vec![],
            AttrsModel::from(attrs),
            scope_context)
    }

    fn new(
        methods: Vec<SigComposerLink<SPEC>>,
        types: HashMap<Ident, TraitTypeModel>,
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics.clone()), LifetimesModel::new(lifetimes), Rc::clone(context)),
            methods,
            types,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<SPEC> DocsComposable for TraitComposer<SPEC>
    where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}

