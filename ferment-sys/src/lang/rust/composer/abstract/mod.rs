use syn::{Item, Type};
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, MaybeComposer, MaybeMacroLabeled};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{CrateBased, ToPath};
use crate::kind::MacroKind;
use crate::lang::RustSpecification;
use crate::presentable::TypeContext;

impl MaybeComposer<RustSpecification> for Item {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<RustSpecification>> {
        self.maybe_macro_labeled()
            .and_then(|macro_type| match (macro_type, self) {
                (MacroKind::Opaque, Item::Struct(item)) =>
                    Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                (MacroKind::Export, Item::Struct(item)) =>
                    Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                (MacroKind::Export, Item::Enum(item)) =>
                    Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                (MacroKind::Export, Item::Type(item)) => Some(match &*item.ty {
                    Type::BareFn(type_bare_fn) =>
                        ItemComposerWrapper::callback(item, TypeContext::callback(scope.self_path_ref().crate_named(&scope.crate_ident_as_path()), &item.ident, type_bare_fn, &item.attrs.cfg_attributes()), scope_context),
                    _ =>
                        ItemComposerWrapper::type_alias(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)
                }),
                (MacroKind::Export, Item::Fn(item)) =>
                    Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path_ref().crate_named(&scope_context.borrow().scope.crate_ident_as_path()), item), scope_context)),
                (MacroKind::Export, Item::Trait(item)) =>
                    Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item), scope, scope_context)),
                (MacroKind::Export, Item::Impl(item)) =>
                    Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(scope.self_path_ref().crate_named(&scope.crate_ident_ref().to_path()), item.trait_.as_ref().map(|(_, trait_, _)| trait_.clone()), item.attrs.cfg_attributes()), scope, scope_context)),
                _ => None
            })

    }
}
