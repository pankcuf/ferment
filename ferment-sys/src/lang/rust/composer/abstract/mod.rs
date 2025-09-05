use syn::{Field, FieldMutability, Item, Type, Visibility};
use syn::token::Pub;
use crate::ast::{CommaPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, MaybeComposer, MaybeMacroLabeled, SigComposer, TypeAliasComposer};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{CrateExtension, ToPath};
use crate::kind::MacroKind;
use crate::lang::RustSpecification;
use crate::presentable::TypeContext;

impl MaybeComposer<RustSpecification> for Item {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<RustSpecification>> {
        self.maybe_macro_labeled()
            .and_then(|macro_type| {
                let source = scope_context.borrow();
                let crate_ident = source.scope.crate_ident_as_path();
                match (macro_type, self) {
                    (MacroKind::Opaque, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroKind::Export, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroKind::Export, Item::Enum(item)) =>
                        Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroKind::Export, Item::Type(item)) => match &*item.ty {
                        Type::BareFn(type_bare_fn) =>
                            Some(ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(TypeContext::callback(scope.self_path_ref().crate_named(&scope.crate_ident_as_path()), &item.ident, type_bare_fn, &item.attrs.cfg_attributes()), &item.generics, &vec![], &item.attrs, scope_context))),
                        _ => {
                            let fields = CommaPunctuated::from_iter([Field {
                                vis: Visibility::Public(Pub::default()),
                                ty: *item.ty.clone(),
                                attrs: vec![],
                                ident: None,
                                colon_token: None,
                                mutability: FieldMutability::None,
                            }]);
                            Some(ItemComposerWrapper::TypeAlias(TypeAliasComposer::new(TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), &item.attrs, &vec![], &item.generics, &fields, scope_context)))
                        }
                    },
                    (MacroKind::Export, Item::Fn(item)) =>
                        Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path_ref().crate_named(&crate_ident), item), scope_context)),
                    (MacroKind::Export, Item::Trait(item)) =>
                        Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item), scope, scope_context)),
                    (MacroKind::Export, Item::Impl(item)) => {
                        let mut full_fn_path = scope.self_path();
                        if full_fn_path.is_crate_based() {
                            full_fn_path.replace_first_with(&scope.crate_ident_ref().to_path());
                        }
                        let trait_path = item.trait_.as_ref().map(|(_, trait_, _)| trait_.clone());
                        Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(full_fn_path, trait_path, item.attrs.cfg_attributes()), scope, scope_context))
                    }
                    _ => None
                }
            })

    }
}
