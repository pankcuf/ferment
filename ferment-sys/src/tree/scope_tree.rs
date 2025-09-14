//! Scope tree data structures and construction utilities for Ferment's processing pipeline.
//!
//! This module defines the core `ScopeTree` structure that represents a hierarchical view of
//! Rust module scopes with their imported and exported items. ScopeTree is the fundamental
//! building block used throughout Ferment's multi-phase processing to track item visibility,
//! resolve imports, and manage scope relationships.
//!
//! ## Key Concepts
//!
//! ### Scope Hierarchy
//! ScopeTree represents a tree structure where each node contains:
//! - A scope chain identifying the module path
//! - Items imported into this scope (`use` statements)
//! - Items exported from this scope (structs, functions, etc.)
//! - Nested child scopes (submodules)
//!
//! ### Import Resolution
//! Each ScopeTree tracks `ItemUse` statements that bring external items into scope.
//! These imports are processed during the refinement phase to resolve fully qualified paths.
//!
//! ### Export Mapping
//! The `exported` field maps scope identifiers to either:
//! - `ScopeTreeItem::Item`: Individual exported items (functions, structs, etc.)
//! - `ScopeTreeItem::Tree`: Nested child scopes (submodules)
//!
//! ## Processing Pipeline Integration
//!
//! ScopeTree is constructed during Phase 2 (Scope Chain Construction) and consumed during
//! Phase 3 (Path Resolution & Refinement). The tree structure enables efficient scope
//! traversal for import resolution and cross-module type references.

use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use indexmap::IndexMap;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{Attribute, ItemUse, UseRename, UseTree};
use crate::composer::SourceAccessible;
use crate::context::{GlobalContext, Scope, ScopeContext, ScopeInfo};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::Join;
use crate::formatter::format_tree_item_dict;
use crate::tree::{ScopeTreeID, ScopeTreeItem};
use crate::tree::ScopeTreeExportItem;

/// Hierarchical representation of a Rust module scope with its imports, exports, and nested scopes.
///
/// ScopeTree is the core data structure that represents a single scope (module) within
/// Ferment's processing pipeline. Each ScopeTree contains all the information needed to
/// understand the scope's content, resolve imports, and navigate the module hierarchy.
///
/// ## Structure
///
/// - `scope`: The scope chain identifying this module's position in the crate hierarchy
/// - `imported`: All `use` statements that bring external items into this scope
/// - `exported`: Map of items and submodules exported from this scope
/// - `scope_context`: Shared context linking to the global type registry
/// - `attrs`: Attributes applied to this scope (e.g., `#[cfg(...)]` conditions)
///
/// ## Import Resolution Process
///
/// During refinement, the `imported` set is processed to resolve all import paths:
/// 1. Each `ItemUse` is analyzed to extract the imported path and alias
/// 2. Glob imports (`use mod::*`) are expanded by scanning target modules
/// 3. Import conflicts are resolved following Rust's scoping rules
/// 4. Final import mappings are stored in the global `ImportResolver`
///
/// ## Export Hierarchy
///
/// The `exported` field creates a tree structure where each entry can be:
/// - **Item Export**: A concrete item (struct, function, trait, etc.) marked for FFI
/// - **Nested Scope**: A child module containing its own imports/exports
///
/// This hierarchy enables efficient scope traversal during path resolution.
///
/// ## Usage in Pipeline
///
/// ScopeTree instances are:
/// 1. **Constructed** during Phase 2 from parsed Rust modules
/// 2. **Linked** together to form complete crate trees with cross-references
/// 3. **Traversed** during Phase 3 refinement for import and type resolution
/// 4. **Consumed** by code generators to produce FFI bindings
#[derive(Clone)]
pub struct ScopeTree {
    /// Attributes applied to this scope (visibility modifiers, cfg conditions, etc.)
    pub attrs: Vec<Attribute>,
    /// Scope chain identifying this module's path within the crate hierarchy
    pub scope: ScopeChain,
    /// All `use` statements importing external items into this scope
    pub imported: HashSet<ItemUse>,
    /// Map of exported items and nested submodules within this scope
    pub exported: IndexMap<ScopeTreeID, ScopeTreeItem>,
    /// Shared context providing access to the global type registry and import resolver
    pub scope_context: ScopeContextLink,
}
impl Debug for ScopeTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ScopeTree({})", format_tree_item_dict(&self.exported)))
    }
}

impl ScopeTree {
    pub fn print_scope_tree_with_message(&self, message: &str) {
        self.scope_context.borrow().print_with_message(message)
    }
}

impl SourceAccessible for ScopeTree {
    fn context(&self) -> &ScopeContextLink {
        &self.scope_context
    }
}


/// Creates a specialized scope tree for handling generic type parameters across all crates.
///
/// This function constructs a virtual scope tree that contains all generic type information
/// needed for FFI generation. The generics scope provides a centralized location for
/// resolving generic bounds, trait constraints, and type parameter relationships.
///
/// ## Purpose
///
/// Generic types in Rust cannot be directly represented in C FFI, so Ferment creates
/// a specialized scope to manage generic type information and generate appropriate
/// monomorphizations or trait objects for FFI consumption.
///
/// ## Structure
///
/// The generics scope tree:
/// - Creates a virtual `generics` module under the root crate
/// - Imports the crate root as an alias for easy access
/// - Provides a centralized registry for all generic type information
/// - Enables cross-crate generic type resolution
///
/// ## Parameters
///
/// - `root_scope_chain`: The root scope chain of the primary crate being processed
/// - `global_context`: Shared global context containing all type and import information
///
/// ## Returns
///
/// A ScopeTree representing the generics scope with appropriate imports and context linkage.
#[allow(unused)]
pub fn create_generics_scope_tree(root_scope_chain: &ScopeChain, global_context: Rc<RefCell<GlobalContext>>) -> ScopeTree {
    let rename =  root_scope_chain.crate_ident();
    let generics_scope_ident = format_ident!("generics");
    let generics_scope_chain = ScopeChain::r#mod(
        ScopeInfo::attr_less(&rename, Scope::empty(root_scope_chain.self_path_ref().joined(&generics_scope_ident))),
        root_scope_chain.clone());

    create_scope_tree(
        generics_scope_chain.clone(),
        ScopeContext::cell_with(generics_scope_chain, global_context),
        HashSet::from_iter([
            create_item_use_with_tree(UseTree::Rename(UseRename { ident: format_ident!("crate"), as_token: Default::default(), rename }))
        ]),
        IndexMap::new(),
        vec![]
    )
}

/// Constructs an `ItemUse` (use statement) from a `UseTree` with default tokens and visibility.
///
/// This utility function creates a syntactically valid `ItemUse` structure from a `UseTree`,
/// filling in all the required syntactic tokens with their default values. This is commonly
/// used when programmatically generating import statements during scope tree construction.
///
/// ## Parameters
///
/// - `tree`: The use tree structure defining what to import (path, glob, group, etc.)
///
/// ## Returns
///
/// A complete `ItemUse` with inherited visibility and default syntactic tokens.
pub fn create_item_use_with_tree(tree: UseTree) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree,
        semi_token: Default::default(),
    }
}


/// Creates a crate root scope tree from parsed crate information.
///
/// This function constructs the top-level ScopeTree for a crate, representing the crate's
/// root module with all its imports, exported items, and nested sub-modules. This is
/// typically called once per crate during the scope tree construction phase.
///
/// ## Parameters
///
/// - `crate_ident`: The identifier name of the crate
/// - `scope_context`: Shared context linking to the global type registry
/// - `imported`: Set of all `use` statements at the crate root level
/// - `exported`: Map of all items and sub-modules exported from the crate root
/// - `attrs`: Crate-level attributes (e.g., `#![cfg(...)]` conditions)
///
/// ## Returns
///
/// A ScopeTree representing the crate root with all its content properly structured.
#[allow(unused)]
pub fn create_crate_root_scope_tree(
    crate_ident: Ident,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: IndexMap<ScopeTreeID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    // print_phase!("PHASE 2: SCOPE TREE MORPHING", "\n{}", format_tree_exported_dict(&exported));
    create_scope_tree(ScopeChain::crate_root(crate_ident, attrs.clone()), scope_context, imported, exported, attrs)
}

/// Creates a ScopeTree from parsed module information, recursively constructing nested scopes.
///
/// This is the core function for building ScopeTree structures from parsed Rust modules.
/// It handles both individual items and nested sub-modules, creating a complete hierarchical
/// representation of the module structure.
///
/// ## Recursive Processing
///
/// For each exported item in the `exported` map:
/// - **Items**: Creates `ScopeTreeItem::Item` for individual exported items (structs, functions, etc.)
/// - **Sub-modules**: Recursively calls `create_scope_tree` to build nested ScopeTree structures
///
/// This recursive approach ensures that the entire module hierarchy is properly represented
/// with correct scope chain relationships and import visibility.
///
/// ## Parameters
///
/// - `scope`: The scope chain for this particular module
/// - `scope_context`: Shared context linking to global type registry and imports
/// - `imported`: Set of `use` statements bringing external items into this scope
/// - `exported`: Map of items and sub-modules exported from this scope
/// - `attrs`: Module-level attributes affecting compilation and visibility
///
/// ## Returns
///
/// A complete ScopeTree with all nested structures properly linked and initialized.
#[allow(unused)]
pub fn create_scope_tree(
    scope: ScopeChain,
    scope_context: ScopeContextLink,
    imported: HashSet<ItemUse>,
    exported: IndexMap<ScopeTreeID, ScopeTreeExportItem>,
    attrs: Vec<Attribute>
) -> ScopeTree {
    let exported = IndexMap::from_iter(exported.into_iter()
        .map(|(scope_id, scope_tree_export_item)| {
            let scope_tree_item = match scope_tree_export_item {
                ScopeTreeExportItem::Item(scope_context, item) =>
                    ScopeTreeItem::item(scope.joined(&item), item, scope_context),
                ScopeTreeExportItem::Tree(scope_context, imported, exported, attrs) =>
                    ScopeTreeItem::tree(create_scope_tree(scope_id.create_child_scope(&scope, attrs.clone()), scope_context, imported, exported, attrs))
            };
            (scope_id, scope_tree_item)
        }));
    ScopeTree {
        scope,
        imported,
        attrs,
        scope_context,
        exported,
    }
}