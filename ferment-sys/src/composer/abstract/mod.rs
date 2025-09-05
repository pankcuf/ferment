mod composable;
mod context;
mod iterative;
mod linked;
mod sequence;
mod sequence_mixer;
mod spec;
mod var_composable;

use syn::{Item, Meta, Path, Attribute};
use syn::parse::Parser;
use crate::ast::CommaPunctuated;
use crate::composer::ItemComposerWrapper;
use crate::context::{ScopeChain, ScopeContextLink};
use crate::kind::{MacroKind, ScopeItemKind};
use crate::ext::{ItemExtension, ToType};
use crate::lang::Specification;
use crate::presentable::{ScopeContextPresentable, Expression};
pub use self::composable::*;
pub use self::context::*;
pub use self::iterative::*;
pub use self::linked::*;
pub use self::sequence::*;
pub use self::sequence_mixer::*;
pub use self::spec::*;
pub use self::var_composable::*;

pub trait MaybeMacroLabeled {
    fn maybe_macro_labeled(&self) -> Option<MacroKind>;
    fn is_labeled_for_export(&self) -> bool {
        if let Some(MacroKind::Export) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }
    fn is_labeled_for_opaque_export(&self) -> bool {
        if let Some(MacroKind::Opaque) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }
    fn is_labeled_for_register(&self) -> bool {
        if let Some(MacroKind::Register(_)) = self.maybe_macro_labeled() {
            true
        } else {
            false
        }
    }

}

pub trait MaybeComposer<SPEC>
    where SPEC: Specification<Expr=Expression<SPEC>>,
          SPEC::Expr: ScopeContextPresentable {
    fn maybe_composer(&self, scope: &ScopeChain, scope_context: &ScopeContextLink) -> Option<ItemComposerWrapper<SPEC>>;
}

pub trait SourceComposable {
    type Source;
    type Output;
    fn compose(&self, source: &Self::Source) -> Self::Output;
}

impl MaybeMacroLabeled for Item {
    fn maybe_macro_labeled(&self) -> Option<MacroKind> {
        self.maybe_attrs()
            .and_then(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for ScopeItemKind {
    fn maybe_macro_labeled(&self) -> Option<MacroKind> {
        self.maybe_attrs().and_then(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for Vec<Attribute> {
    fn maybe_macro_labeled(&self) -> Option<MacroKind> {
        self.iter().find_map(MaybeMacroLabeled::maybe_macro_labeled)
    }
}

impl MaybeMacroLabeled for Attribute {
    fn maybe_macro_labeled(&self) -> Option<MacroKind> {
        let mut arguments = Vec::<Path>::new();
        let mut macro_name: Option<String> = None;

        match &self.meta {
            Meta::Path(path) => {
                // Handle simple case like `#[ferment_macro::export]`
                macro_name = path.segments.last().map(|s| s.ident.to_string());
            }
            Meta::List(meta_list) => {
                let path = &meta_list.path;
                let nested = CommaPunctuated::<Meta>::parse_terminated.parse2(meta_list.tokens.clone()).ok()?;

                if path.is_ident("cfg_attr") {
                    // #[cfg_attr(feature = "...", ferment_macro::export)]
                    for (i, meta_item) in nested.iter().enumerate() {
                        if i == 0 {
                            continue; // Skip cfg condition
                        }
                        match meta_item {
                            Meta::Path(inner_path) => {
                                macro_name = inner_path.segments.last().map(|s| s.ident.to_string());
                            }
                            Meta::List(inner_list) => {
                                macro_name = inner_list.path.segments.last().map(|s| s.ident.to_string());

                                let inner_nested = CommaPunctuated::<Meta>::parse_terminated
                                    .parse2(inner_list.tokens.clone())
                                    .ok()?;

                                for inner_meta in inner_nested {
                                    if let Meta::Path(arg_path) = inner_meta {
                                        arguments.push(arg_path.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    // #[ferment_macro::register(...)]
                    macro_name = path.segments.last().map(|s| s.ident.to_string());

                    for meta_item in nested {
                        if let Meta::Path(arg_path) = meta_item {
                            arguments.push(arg_path.clone());
                        }
                    }
                }
            }

            _ => {}
        }

        let detected_macro = macro_name?;

        match detected_macro.as_str() {
            "export" => Some(MacroKind::Export),
            "opaque" => Some(MacroKind::Opaque),
            "register" => {
                let first_argument = arguments.first()?;
                Some(MacroKind::Register(first_argument.to_type()))
            }
            _ => None,
        }
    }
}

