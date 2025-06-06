use syn::{Field, Item, Type, Visibility, VisPublic};
use syn::token::Pub;
use crate::ast::{CommaPunctuated, PathHolder};
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, MaybeComposer, MaybeMacroLabeled, SigComposer, TypeAliasComposer};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::conversion::MacroType;
use crate::ext::{CrateExtension, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::presentable::TypeContext;

impl<SPEC> MaybeComposer<ObjCFermentate, SPEC> for Item
    where SPEC: ObjCSpecification {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<ObjCFermentate, SPEC>> {
        self.maybe_macro_labeled()
            .and_then(|macro_type| {
                let source = scope_context.borrow();
                let global = source.context.read().unwrap();
                let config = global.config.maybe_objc_config().unwrap();
                let prefix = config.class_prefix();

                let crate_ident = source.scope.crate_ident_as_path();
                match (macro_type, self) {
                    (MacroType::Opaque, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Enum(item)) =>
                        Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
                    (MacroType::Export, Item::Type(item)) => match &*item.ty {
                        Type::BareFn(type_bare_fn) =>
                            Some(ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(TypeContext::callback(scope.self_path().crate_named(&scope.crate_ident_as_path()), &item.ident, prefix, type_bare_fn, &item.attrs.cfg_attributes()), &item.generics, &vec![], &item.attrs, scope_context))),
                        _ => {
                            let fields = CommaPunctuated::from_iter([Field {
                                vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
                                ty: *item.ty.clone(),
                                attrs: vec![],
                                ident: None,
                                colon_token: None,
                            }]);
                            Some(ItemComposerWrapper::TypeAlias(TypeAliasComposer::new(TypeContext::r#struct(&item.ident, prefix, item.attrs.cfg_attributes()), &item.attrs, &item.generics, &vec![], &fields, scope_context)))
                        }
                    },
                    (MacroType::Export, Item::Fn(item)) =>
                        Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path().crate_named(&crate_ident), prefix, item), scope_context)),
                    (MacroType::Export, Item::Trait(item)) =>
                        Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item, prefix), scope, scope_context)),
                    (MacroType::Export, Item::Impl(item)) => {
                        let mut full_fn_path = scope.self_path_holder();
                        if full_fn_path.is_crate_based() {
                            full_fn_path.replace_first_with(&PathHolder::from(scope.crate_ident_ref().to_path()));
                        }
                        Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(full_fn_path.0, prefix, item.attrs.cfg_attributes()), scope, scope_context))
                    }
                    _ => None
                }
            })

    }
}
