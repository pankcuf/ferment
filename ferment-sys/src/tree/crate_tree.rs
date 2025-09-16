//! Core data structures and processing pipeline for Ferment's crate tree construction.
//!
//! This module implements the heart of Ferment's multi-phase processing pipeline that
//! transforms Rust source code into FFI-ready representations. The main entry point is
//! [`CrateTree::new`] which executes the complete 3-phase transformation process.
//!
//! ## Architecture Overview
//!
//! The processing pipeline follows a sequential transformation approach:
//!
//! ``
//! Raw Rust Files → Parsed Scope Trees → CrateTree → FFI Bindings
//! ``
//!
//! ### Phase 1 (External): File Parsing
//! - Handled by `FileTreeProcessor::build()` in tree module
//! - Parses Rust files using `syn` crate
//! - Identifies `#[ferment_macro::export]` marked items
//!
//! ### Phase 2: Scope Chain Construction
//! - Builds comprehensive scope trees for all crates
//! - Creates global context with cross-crate visibility
//! - Maps import statements to fully qualified paths
//!
//! ### Phase 3: Path Resolution & Refinement
//! - Resolves all type references through import chains
//! - Handles complex cases: re-exports, glob imports, trait bounds
//! - Transforms types into FFI-compatible representations
//!
//! ## Performance Characteristics
//!
//! - Phase 2: O(n) where n = number of items across all crates
//! - Phase 3: O(n×m) where m = average import chain depth
//! - Memory usage: Scales with codebase size and import complexity
//!
//! For large codebases with complex import graphs, Phase 3 can be the bottleneck,
//! taking several seconds for comprehensive path resolution.

use std::collections::HashMap;
use quote::quote;
use syn::parse_quote;
use syn::Attribute;
use crate::{Crate, error, print_phase};
use crate::ast::Depunctuated;
use crate::composer::SourceAccessible;
use crate::context::ScopeContextLink;
use crate::tree::ScopeTree;
use crate::tree::{create_crate_root_scope_tree, create_generics_scope_tree, ScopeTreeExportItem};

/// Core data structure representing the complete processed crate tree after Ferment's multi-phase analysis.
///
/// CrateTree is the final output of Ferment's processing pipeline, containing all crates
/// (current + external dependencies) with their scope trees fully constructed, import paths
/// resolved, and types refined for FFI generation.
///
/// ## Processing Pipeline Overview
///
/// CrateTree is constructed through a 3-phase process:
///
/// ### Phase 1: File Parsing & Scope Tree Construction
/// - Parses Rust source files into syntax trees using the `syn` crate
/// - Identifies items marked with `#[ferment_macro::export]` for FFI generation
/// - Builds initial scope trees mapping module paths to their contents
///
/// ### Phase 2: Scope Chain Construction
/// - Creates comprehensive scope registry tracking all types, functions, traits, imports
/// - Builds global context with cross-crate visibility
/// - Constructs import resolver mapping use statements to fully qualified paths
///
/// ### Phase 3: Path Resolution & Refinement
/// - Resolves all path references by following import chains and re-exports
/// - Handles glob imports (`use mod::*`) through scope traversal
/// - Refines type models into FFI-compatible representations
/// - Generates vtables for traits and conversion functions
///
/// ## Structure
///
/// - `crates`: All processed crates (current + external dependencies) as scope trees
/// - `generics_tree`: Specialized scope tree for handling generic type parameters
/// - `attrs`: Global attributes applied to generated FFI code (compiler directives)
///
/// ## Usage
///
/// CrateTree serves as input to the final code generation phase, where:
/// - C bindings are generated via cbindgen
/// - Language-specific bindings (Objective-C, Java) are created
/// - Constructor/destructor functions are generated with `_ctor`/`_destroy` suffixes
/// - Automatic `From`/`To` trait implementations enable seamless type conversions
#[derive(Clone, Debug)]
pub struct CrateTree {
    /// Global compiler directives applied to all generated FFI code
    pub attrs: Vec<Attribute>,
    /// All processed crates (current + external dependencies) with their scope trees
    pub crates: Depunctuated<ScopeTree>,
    /// Specialized scope tree for handling generic type parameters across all crates
    pub generics_tree: ScopeTree
}

impl SourceAccessible for CrateTree {
    fn context(&self) -> &ScopeContextLink {
        &self.generics_tree.scope_context
    }
}

#[allow(unused)]
impl CrateTree {
    /// Constructs a complete CrateTree by executing Ferment's 3-phase processing pipeline.
    ///
    /// This is the main entry point that transforms raw parsed scope trees into a fully
    /// refined CrateTree ready for FFI code generation.
    ///
    /// ## Parameters
    ///
    /// - `current_crate`: The primary crate being processed
    /// - `current_tree`: Parsed scope tree for the current crate
    /// - `external_crates`: Map of external dependency crates to their parsed scope trees
    ///
    /// ## Processing Phases
    ///
    /// ### Phase 2: Scope Chain Construction
    /// - Builds complete scope trees for current crate and all external dependencies
    /// - Creates global context with cross-crate type and import visibility
    /// - Constructs comprehensive import resolver for path resolution
    ///
    /// ### Phase 3: Path Resolution & Refinement
    /// - Executes `context.refine()` to resolve all path references
    /// - Follows import chains, re-exports, and glob imports
    /// - Transforms Rust types into FFI-compatible representations
    /// - Generates trait vtables and type conversion functions
    ///
    /// ### Finalization
    /// - Creates generics scope tree for handling generic type parameters
    /// - Applies global compiler directives for clean FFI code generation
    /// - Returns complete CrateTree ready for code generation
    ///
    /// ## Returns
    ///
    /// `Ok(CrateTree)` on successful processing, or `Err(Error::ExpansionError)`
    /// if the root scope tree is malformed.
    ///
    /// ## Performance Note
    ///
    /// Phase 3 refinement is the most expensive operation, as it performs path resolution
    /// across all scopes. For complex projects with many cross-crate references, this
    /// phase can take several seconds.
    pub fn new(current_crate: &Crate, current_tree: ScopeTreeExportItem, external_crates: HashMap<Crate, ScopeTreeExportItem>) -> Result<Self, error::Error> {
        match current_tree {
            ScopeTreeExportItem::Item(..) =>
                Err(error::Error::ExpansionError("Bad tree root")),
            ScopeTreeExportItem::Tree(scope_context, imported, exported, attrs) => {
                // print_phase!("PHASE 2: CRATE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
                let current_tree = create_crate_root_scope_tree(current_crate.ident(), scope_context, imported, exported, attrs);
                let mut crates = Depunctuated::from_iter(external_crates.into_iter()
                    .filter_map(|(external_crate, export_item)| match export_item {
                        ScopeTreeExportItem::Item(..) =>
                            None,
                        ScopeTreeExportItem::Tree(scope_context, imported, exported, attrs) =>
                            Some(create_crate_root_scope_tree(external_crate.ident(), scope_context, imported, exported, attrs))
                    }));
                //current_tree.print_scope_tree_with_message("PHASE 222: CRATE TREE UNREFINED CONTEXT");
                // print_phase!("PHASE 2: CURRENT CRATE TREE", "\n{:?}", current_tree);
                // print_phase!("PHASE 2: EXTERNAL CRATES TREE", "\n{:?}", external_crates);
                // current_tree.print_scope_tree_with_message("PHASE 2: CRATE TREE CONTEXT");
                let global_context = current_tree.scope_context.borrow().context.clone();
                print_phase!("PHASE 3: CRATE TREE REFINEMENT", "");
                {
                    let mut context = global_context.borrow_mut();
                    context.refine();
                }
                let generics_tree = create_generics_scope_tree(&current_tree.scope, global_context);
                current_tree.print_scope_tree_with_message("PHASE 3: CRATE TREE REFINED CONTEXT");
                let directives = quote!(#[allow(clippy::let_and_return, clippy::suspicious_else_formatting, clippy::redundant_field_names, dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, redundant_semicolons, unreachable_patterns, unused_braces, unused_imports, unused_parens, unused_qualifications, unused_unsafe, unused_variables)]);
                crates.push(current_tree);
                Ok(Self { crates, generics_tree, attrs: vec![parse_quote!(#directives)] })
            }
        }
    }
}

