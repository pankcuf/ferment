use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, CfgAttributes, FnSignatureContext, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocComposer, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceAccessible, SourceComposable, VTableComposerLink};
use crate::composer::vtable::VTableComposer;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, ToType};
use crate::lang::Specification;
use crate::presentable::NameTreeContext;
use crate::presentation::DocPresentation;

#[derive(ComposerBase)]
pub struct ImplComposer<SPEC>
    where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
    pub methods: Vec<SigComposerLink<SPEC>>,
    pub vtable: Option<VTableComposerLink<SPEC>>,
}
impl<SPEC> ImplComposer<SPEC>
    where SPEC: Specification {
    pub fn from_item_impl(item_impl: &ItemImpl, ty_context: SPEC::TYC, scope: &ScopeChain, scope_context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemImpl { attrs, generics, trait_, self_ty, items, ..  } = item_impl;
        let source = scope_context.borrow();
        let mut methods = Vec::new();
        let mut vtable_method_composers = Vec::new();
        let attrs_model = AttrsModel::from(attrs);
        items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Fn(item) => {
                    let method_scope_context = Rc::new(RefCell::new(source.joined(item)));
                    // TMP strategy to provide both trait vtable based and implementor based bindings
                    match trait_.as_ref() {
                        Some((_, path, _)) => {

                            let trait_ty_context = ty_context.join_fn(
                                scope.joined_path(&item.sig.ident),
                                FnSignatureContext::TraitImpl(item.sig.clone(), *self_ty.clone(), path.to_type()),
                                item.attrs.cfg_attributes()
                            );
                            let composer = SigComposer::from_impl_item_method(item, trait_ty_context, &method_scope_context);
                            methods.push(composer.clone());
                            vtable_method_composers.push(composer);

                            let impl_ty_context = ty_context.join_fn(
                                scope.joined_path(&item.sig.ident),
                                FnSignatureContext::TraitAsType(item.sig.clone(), *self_ty.clone(), path.to_type()),
                                item.attrs.cfg_attributes()
                            );
                            methods.push(SigComposer::from_impl_item_method(item, impl_ty_context, &method_scope_context));
                        }
                        None => {
                            let sig_context = FnSignatureContext::Impl(item.sig.clone(), *self_ty.clone());
                            let ty_context = ty_context.join_fn(
                                scope.joined_path(&item.sig.ident),
                                sig_context,
                                item.attrs.cfg_attributes()
                            );
                            methods.push(SigComposer::from_impl_item_method(item, ty_context, &method_scope_context));
                        }
                    }
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
                LifetimesModel::new(vec![]),
                Rc::clone(scope_context)),
            methods: methods.clone(),
            vtable: trait_.as_ref().map(|(..)| VTableComposer::from_trait_path(ty_context, attrs, vtable_method_composers, Rc::clone(scope_context)))
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}

impl<SPEC> DocsComposable for ImplComposer<SPEC>
    where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}

