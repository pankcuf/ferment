mod composable;
mod context;
mod iterative;
mod linked;
mod sequence;
mod sequence_mixer;
mod spec;
mod var_composable;

use syn::{Field, Item, Meta, NestedMeta, Path, Type, Visibility, VisPublic, MetaList, Attribute};
use syn::token::Pub;
use crate::ast::{CommaPunctuated, PathHolder, TypeHolder};
use crate::composable::CfgAttributes;
use crate::composer::{ItemComposerWrapper, SigComposer};
use crate::composer::type_alias::TypeAliasComposer;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::conversion::{MacroType, ScopeItemKind};
use crate::ext::{CrateExtension, ItemExtension, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{TypeContext, ScopeContextPresentable, Expression};
use crate::presentation::RustFermentate;
pub use self::composable::*;
pub use self::context::*;
pub use self::iterative::*;
pub use self::linked::*;
pub use self::sequence::*;
pub use self::sequence_mixer::*;
pub use self::spec::*;
pub use self::var_composable::*;

pub trait MaybeMacroLabeled {
    fn maybe_macro_labeled(&self) -> Option<MacroType>;
    fn is_labeled_for_export(&self) -> bool {
        if let Some(MacroType::Export) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }
    fn is_labeled_for_opaque_export(&self) -> bool {
        if let Some(MacroType::Opaque) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }
    fn is_labeled_for_register(&self) -> bool {
        if let Some(MacroType::Register(_)) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }

}

pub trait MaybeComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<LANG, SPEC>>;
}

pub trait SourceComposable {
    type Source;
    type Output;
    fn compose(&self, source: &Self::Source) -> Self::Output;
}

impl MaybeMacroLabeled for Item {
    fn maybe_macro_labeled(&self) -> Option<MacroType> {
        self.maybe_attrs()
            .and_then(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for ScopeItemKind {
    fn maybe_macro_labeled(&self) -> Option<MacroType> {
        self.maybe_attrs().and_then(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for Vec<Attribute> {
    fn maybe_macro_labeled(&self) -> Option<MacroType> {
        self.iter().find_map(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for Attribute {
    fn maybe_macro_labeled(&self) -> Option<MacroType> {
        let mut arguments = Vec::<Path>::new();
        let mut macro_name: Option<String> = None;

        match self.parse_meta() {
            Ok(Meta::Path(path)) => {
                // Handle simple case like `#[ferment_macro::export]`
                macro_name = Some(path.segments.last()?.ident.to_string());
            }
            Ok(Meta::List(MetaList { nested, path, .. })) => {
                if path.is_ident("cfg_attr") {
                    // Handle `#[cfg_attr(feature = "apple", ferment_macro::export)]`
                    for (i, meta) in nested.iter().enumerate() {
                        if i == 0 {
                            // Skip the first argument (feature flag)
                            continue;
                        }
                        match meta {
                            NestedMeta::Meta(Meta::Path(inner_path)) => {
                                macro_name = Some(inner_path.segments.last()?.ident.to_string());
                            }
                            NestedMeta::Meta(Meta::List(inner_list)) => {
                                macro_name = Some(inner_list.path.segments.last()?.ident.to_string());
                                // Extract arguments for `register(...)`
                                for inner_meta in &inner_list.nested {
                                    if let NestedMeta::Meta(Meta::Path(inner_path)) = inner_meta {
                                        arguments.push(inner_path.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    // Handle `#[ferment_macro::register(...)]`
                    macro_name = Some(path.segments.last()?.ident.to_string());

                    // Extract arguments
                    for meta in &nested {
                        if let NestedMeta::Meta(Meta::Path(inner_path)) = meta {
                            arguments.push(inner_path.clone());
                        }
                    }
                }
            }
            _ => {}
        }

        let detected_macro = macro_name?;

        match detected_macro.as_str() {
            "export" => Some(MacroType::Export),
            "opaque" => Some(MacroType::Opaque),
            "register" => {
                let first_argument = arguments.first()?;
                Some(MacroType::Register(TypeHolder(first_argument.to_type())))
            }
            _ => None,
        }
    }
}

impl<SPEC> MaybeComposer<RustFermentate, SPEC> for Item
    where SPEC: RustSpecification {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<RustFermentate, SPEC>> {
        self.maybe_macro_labeled()
            .and_then(|macro_type| {
                let source = scope_context.borrow();
                let crate_ident = source.scope.crate_ident_as_path();
                match (macro_type, self) {
                    (MacroType::Opaque, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::opaque_struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroType::Export, Item::Struct(item)) =>
                        Some(ItemComposerWrapper::r#struct(item, TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroType::Export, Item::Enum(item)) =>
                        Some(ItemComposerWrapper::r#enum(item, TypeContext::r#enum(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), scope_context)),
                    (MacroType::Export, Item::Type(item)) => match &*item.ty {
                        Type::BareFn(type_bare_fn) =>
                            Some(ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(TypeContext::callback(scope.self_path().crate_named(&scope.crate_ident_as_path()), &item.ident, type_bare_fn, &item.attrs.cfg_attributes()), &item.generics, &vec![], &item.attrs, scope_context))),
                        _ => {
                            let fields = CommaPunctuated::from_iter([Field {
                                vis: Visibility::Public(VisPublic { pub_token: Pub::default() }),
                                ty: *item.ty.clone(),
                                attrs: vec![],
                                ident: None,
                                colon_token: None,
                            }]);
                            Some(ItemComposerWrapper::TypeAlias(TypeAliasComposer::new(TypeContext::r#struct(&item.ident, item.attrs.cfg_attributes(), item.generics.clone()), &item.attrs, &item.generics, &vec![], &fields, scope_context)))
                        }
                    },
                    (MacroType::Export, Item::Fn(item)) =>
                        Some(ItemComposerWrapper::r#fn(item, TypeContext::mod_fn(scope.self_path().crate_named(&crate_ident), item), scope_context)),
                    (MacroType::Export, Item::Trait(item)) =>
                        Some(ItemComposerWrapper::r#trait(item, TypeContext::r#trait(item), scope, scope_context)),
                    (MacroType::Export, Item::Impl(item)) => {
                        let mut full_fn_path = scope.self_path_holder();
                        if full_fn_path.is_crate_based() {
                            full_fn_path.replace_first_with(&PathHolder::from(scope.crate_ident_ref().to_path()));
                        }
                        let trait_path = item.trait_.as_ref().map(|(_, trait_, _)| trait_.clone());
                        Some(ItemComposerWrapper::r#impl(item, TypeContext::r#impl(full_fn_path.0, trait_path, item.attrs.cfg_attributes()), scope, scope_context))
                    }
                    _ => None
                }
            })

    }
}
