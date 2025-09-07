use syn::{Item, ItemType, Type};
use crate::ast::CommaPunctuated;
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, MaybeComposer, MaybeMacroLabeled, SigComposer, TypeAliasComposer};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::kind::MacroKind;
use crate::ext::{CrateBased, ToPath};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::presentable::TypeContext;

impl MaybeComposer<ObjCSpecification> for Item {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<ObjCSpecification>> {
        let macro_type = self.maybe_macro_labeled()?;
        let source = scope_context.borrow();
        let global = source.context.read().unwrap();
        let config = global.config.maybe_objc_config().expect("ObjC config must be present");
        let prefix = config.class_prefix();
        let crate_ident = source.scope.crate_ident_as_path();
        match (macro_type, self) {
            (MacroKind::Opaque, Item::Struct(item)) =>
                Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
            (MacroKind::Export, Item::Struct(item)) =>
                Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
            (MacroKind::Export, Item::Enum(item)) =>
                Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, prefix, item.attrs.cfg_attributes()), scope_context)),
            (MacroKind::Export, Item::Type(ItemType { attrs, ident, generics, ty, .. })) => match &**ty {
                Type::BareFn(type_bare_fn) =>
                    Some(ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(TypeContext::callback(scope.self_path_ref().crate_named(&scope.crate_ident_as_path()), ident, prefix, type_bare_fn, &attrs.cfg_attributes()), generics, &vec![], attrs, scope_context))),
                _ => {
                    let fields = CommaPunctuated::from_iter([crate::ast::pub_unnamed_field(*ty.clone())]);
                    Some(ItemComposerWrapper::TypeAlias(TypeAliasComposer::new(TypeContext::r#struct(ident, prefix, attrs.cfg_attributes()), attrs, &vec![], generics, &fields, scope_context)))
                }
            },
            (MacroKind::Export, Item::Fn(item)) =>
                Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path_ref().crate_named(&crate_ident), prefix, item), scope_context)),
            (MacroKind::Export, Item::Trait(item)) =>
                Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item, prefix), scope, scope_context)),
            (MacroKind::Export, Item::Impl(item)) =>
                Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(scope.self_path_ref().crate_named(&scope.crate_ident_ref().to_path()), prefix, item.attrs.cfg_attributes()), scope, scope_context)),
            _ => None
        }
    }
}
