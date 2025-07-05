use std::collections::HashMap;
use quote::quote;
use syn::{Attribute, parse_quote};
use crate::{Crate, error, print_phase};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::composable::CfgAttributes;
use crate::composer::{SourceComposable, GenericComposer, SourceAccessible, SourceFermentable};
use crate::context::ScopeContextLink;
use crate::conversion::expand_attributes;
use crate::ext::RefineUnrefined;
use crate::lang::RustSpecification;
use crate::presentable::TypeContext;
use crate::presentation::RustFermentate;
use crate::tree::{create_crate_root_scope_tree, create_generics_scope_tree, ScopeTree, ScopeTreeExportItem};

/// Main entry point for resulting expansion
#[derive(Clone, Debug)]
pub struct CrateTree {
    pub attrs: Vec<Attribute>,
    pub crates: Depunctuated<ScopeTree>,
    pub generics_tree: ScopeTree
}

impl SourceAccessible for CrateTree {
    fn context(&self) -> &ScopeContextLink {
        &self.generics_tree.scope_context
    }
}

impl CrateTree {
    pub fn new(current_crate: &Crate, current_tree: ScopeTreeExportItem, external_crates: HashMap<Crate, ScopeTreeExportItem>) -> Result<Self, error::Error> {
        match current_tree {
            ScopeTreeExportItem::Item(..) =>
                Err(error::Error::ExpansionError("Bad tree root")),
            ScopeTreeExportItem::Tree(
                scope_context,
                imported,
                exported,
                attrs) => {
                // print_phase!("PHASE 2: CRATE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
                let current_tree = create_crate_root_scope_tree(current_crate.ident(), scope_context, imported, exported, attrs);
                let mut crates = Depunctuated::from_iter(external_crates.into_iter()
                    .map(|(external_crate, export_item)| match export_item {
                        ScopeTreeExportItem::Item(..) =>
                            panic!("•• It should never happen ••"),
                        ScopeTreeExportItem::Tree(scope_context, imported, exported, attrs) =>
                            create_crate_root_scope_tree(external_crate.ident(), scope_context, imported, exported, attrs)
                    }));
                // print_phase!("PHASE 2: CURRENT CRATE TREE", "\n{:?}", current_tree);
                // print_phase!("PHASE 2: EXTERNAL CRATES TREE", "\n{:?}", external_crates);
                // current_tree.print_scope_tree_with_message("PHASE 2: CRATE TREE CONTEXT");
                let global_context = current_tree.scope_context.borrow().context.clone();
                print_phase!("PHASE 3: CRATE TREE REFINEMENT", "");
                global_context
                    .write()
                    .unwrap()
                    .refine();
                let generics_tree = create_generics_scope_tree(&current_tree.scope, global_context);
                current_tree.print_scope_tree_with_message("PHASE 3: CRATE TREE REFINED CONTEXT");
                let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unreachable_patterns, unused_braces, unused_imports, unused_parens, unused_qualifications, unused_unsafe, unused_variables)]);
                crates.push(current_tree);
                Ok(Self { crates, generics_tree, attrs: vec![parse_quote!(#directives)] })
            }
        }
    }
}

impl SourceFermentable<RustFermentate> for CrateTree {
    fn ferment(&self) -> RustFermentate {
        let Self { attrs, crates, generics_tree: ScopeTree { imported, .. }} = self;
        let source = self.source_ref();
        let reg_conversions = Depunctuated::from_iter(crates.iter().map(SourceFermentable::<RustFermentate>::ferment));
        let generic_imports = SemiPunctuated::from_iter(imported.iter().cloned());
        let generic_conversions = Depunctuated::from_iter(
            source.context
                .read()
                .unwrap()
                .refined_mixins
                .iter()
                .filter_map(|(mixin, attrs)| {
                    let attrs = expand_attributes(attrs);
                    let tyc = TypeContext::mixin(mixin, attrs.cfg_attributes());
                    GenericComposer::<RustSpecification>::new(mixin, attrs, tyc, self.context())
                })
                .flat_map(|composer|
                    composer.borrow().compose(&source)));

        RustFermentate::Root {
            mods: Depunctuated::from_iter([
                RustFermentate::types(attrs, reg_conversions),
                RustFermentate::generics(attrs, generic_imports, generic_conversions)
            ])
        }
    }
}
