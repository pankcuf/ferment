use std::collections::HashMap;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Attribute, parse_quote};
use syn::punctuated::Punctuated;
use crate::{Crate, error, print_phase};
use crate::ast::{Depunctuated, SemiPunctuated};
use crate::conversion::{ObjectKind, TypeKind};
use crate::ext::{RefineUnrefined, ToType};
use crate::formatter::{format_generic_conversions, format_mixin_conversions};
use crate::presentation::Expansion;
use crate::tree::{create_crate_root_scope_tree, create_generics_scope_tree, ScopeTree, ScopeTreeExportItem};

#[derive(Clone, Debug)]
pub struct CrateTree {
    pub current_tree: ScopeTree,
    pub external_crates: HashMap<Crate, ScopeTree>,
    pub generics_tree: ScopeTree
}

// Main entry point for resulting expansion
impl ToTokens for CrateTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let directives = self.directives();
        let attrs: Vec<Attribute> = vec![parse_quote!(#directives)];
        Expansion::Mod {
            attrs: attrs.clone(),
            name: quote!(types),
            imports: Punctuated::new(),
            conversions: self.regular_conversions()
        }.to_tokens(tokens);
        Expansion::Mod {
            attrs,
            name: quote!(generics),
            imports: SemiPunctuated::from_iter(self.generics_tree.imported.iter().cloned()),
            conversions: self.generic_conversions(),
        }.to_tokens(tokens);
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
                let external_crates = external_crates.into_iter()
                    .map(|(external_crate, crate_root_tree_export_item)|
                        match crate_root_tree_export_item {
                            ScopeTreeExportItem::Item(..) =>
                                panic!("•• It should never happen ••"),
                            ScopeTreeExportItem::Tree(
                                scope_context,
                                imported,
                                exported,
                                attrs) => {
                                let crate_ident = external_crate.ident();
                                (external_crate, create_crate_root_scope_tree(crate_ident, scope_context, imported, exported, attrs))
                            }
                        })
                    .collect();
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
                Ok(Self { current_tree, external_crates, generics_tree })
            }
        }
    }
    fn generic_conversions(&self) -> Depunctuated<TokenStream2> {
        let source = self.current_tree.scope_context.borrow();
        let global = source.context.read().unwrap();

        let generics_source = &self.generics_tree.scope_context;

        print_phase!("PHASE 3: GENERICS TO EXPAND", "\t{}", format_generic_conversions(&global.refined_generics));
        let mut generics = Depunctuated::new();
        generics.extend(global.refined_generics.iter()
            .map(|(generic, attrs)| match &generic.object {
                ObjectKind::Type(model_kind) |
                ObjectKind::Item(model_kind, _) => match TypeKind::from(model_kind.to_type()) {
                    TypeKind::Generic(generic_kind) => generic_kind.expand(attrs, &generics_source),
                    otherwise => unimplemented!("non-generic GenericConversion: {:?}", otherwise)
                },
                ObjectKind::Empty => unimplemented!("expand: ObjectKind::Empty")
        }));
        print_phase!("PHASE 3: MIXINS TO EXPAND", "\t{}", format_mixin_conversions(&global.refined_generic_constraints));
        generics.extend(global.refined_generic_constraints.iter()
            .map(|(mixin, attrs)|
                mixin.expand(attrs, &generics_source)));
        generics
    }

    fn regular_conversions(&self) -> Depunctuated<TokenStream2> {
        let mut regular_conversions = self.external_crates
            .values()
            .map(ScopeTree::to_token_stream)
            .collect::<Depunctuated<_>>();
        regular_conversions.push(self.current_tree.to_token_stream());
        regular_conversions
    }
    fn directives(&self) -> TokenStream2 {
        quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unreachable_patterns, unused_braces, unused_imports, unused_parens, unused_qualifications, unused_unsafe, unused_variables)])
    }

    // fn generic_imports(&self) -> SemiPunctuated<ItemUse> {
    //     SemiPunctuated::from_iter([
    //         create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: self.current_tree.scope.crate_ident().clone() }))
    //     ])
    // }
    // fn generic_imports_map(&self) -> HashMap<ImportConversion, HashSet<ImportModel>> {
    //     SemiPunctuated::from_iter([
    //         Im
    //         create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename: self.current_tree.scope.crate_ident().clone() }))
    //     ])
    // }

}