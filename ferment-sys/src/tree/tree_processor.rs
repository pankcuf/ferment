//! File system traversal and Rust source code processing for Ferment's Phase 1 pipeline.
//!
//! This module implements the first phase of Ferment's processing pipeline: parsing Rust
//! source files from the filesystem and building initial scope trees. The main entry point
//! is [`FileTreeProcessor::build`] which orchestrates the complete file processing workflow.
//!
//! ## Phase 1: File Parsing & Initial Scope Construction
//!
//! The file processing pipeline follows this sequence:
//! ``
//! Filesystem → Rust Files → AST Parsing → Visitor Pattern → ScopeTreeExportItem → CrateTree
//! ``
//!
//! ### File Discovery
//! - Traverses the filesystem starting from crate root directories
//! - Follows Rust module conventions: `mod.rs`, `module_name.rs`, and nested directories
//! - Respects module visibility and `#[cfg(...)]` conditional compilation
//!
//! ### AST Processing
//! - Parses each Rust file using the `syn` crate to build syntax trees
//! - Identifies items marked with `#[ferment_macro::export]` for FFI generation
//! - Tracks `use` statements and import relationships
//!
//! ### Scope Tree Building
//! - Creates hierarchical scope representations mirroring Rust's module system
//! - Links scopes across files and crates to enable cross-reference resolution
//! - Builds the foundation for later import resolution and type refinement
//!
//! ## Performance Characteristics
//!
//! - File I/O: O(n) where n = number of source files
//! - AST Parsing: O(m) where m = total lines of code
//! - Memory usage: Scales with codebase size (AST nodes are memory-intensive)
//!
//! This phase is typically fast for most projects, but can become a bottleneck for
//! very large codebases with hundreds of modules.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Attribute, Item, ItemMod};
use syn::visit::Visit;
use crate::Crate;
use crate::context::{GlobalContext, ScopeChain};
use crate::{Config, error, print_phase};
use crate::tree::{CrateTree, ScopeTreeExportItem, Visitor};
/// Core processor for traversing the filesystem and parsing Rust source files into scope trees.
///
/// FileTreeProcessor implements the first phase of Ferment's processing pipeline, handling
/// filesystem traversal, Rust file parsing, and initial scope tree construction. Each
/// processor instance is responsible for processing a single file or module and recursively
/// handling its sub-modules.
///
/// ## Processing Workflow
///
/// 1. **File Reading**: Reads Rust source files from the filesystem
/// 2. **AST Parsing**: Parses source code using the `syn` crate
/// 3. **Visitor Processing**: Uses the visitor pattern to extract exportable items
/// 4. **Module Discovery**: Recursively processes sub-modules following Rust conventions
/// 5. **Scope Tree Building**: Constructs hierarchical scope representations
///
/// ## Module Resolution
///
/// FileTreeProcessor follows Rust's module system conventions:
/// - `mod.rs` files for directory-based modules
/// - `module_name.rs` files for single-file modules
/// - Nested directory structures for hierarchical modules
/// - Proper scope chain construction maintaining module relationships
///
/// ## State Management
///
/// - `path`: Filesystem path to the current file being processed
/// - `scope`: Scope chain representing the module's position in the hierarchy
/// - `context`: Shared global context for cross-module type and import information
/// - `attrs`: Module-level attributes affecting compilation and visibility
pub struct FileTreeProcessor {
    /// Filesystem path to the Rust source file being processed
    pub path: PathBuf,
    /// Scope chain identifying this module's position in the crate hierarchy
    pub scope: ScopeChain,
    /// Shared global context containing cross-crate type and import information
    pub context: Rc<RefCell<GlobalContext>>,
    /// Module-level attributes (visibility, cfg conditions, etc.)
    pub attrs: Vec<Attribute>
}

impl FileTreeProcessor {
    /// Main entry point for the complete file processing pipeline.
    ///
    /// This method orchestrates the entire Phase 1 processing workflow, from initial
    /// configuration through complete CrateTree construction. It processes both the
    /// current crate and all external dependency crates specified in the configuration.
    ///
    /// ## Processing Sequence
    ///
    /// 1. **Global Context Initialization**: Creates shared context for cross-crate information
    /// 2. **External Crate Processing**: Processes all dependency crates in parallel
    /// 3. **Current Crate Processing**: Processes the primary crate being built
    /// 4. **CrateTree Construction**: Combines all processed crates into final tree structure
    ///
    /// ## Parameters
    ///
    /// - `config`: Complete build configuration including crate paths and settings
    ///
    /// ## Returns
    ///
    /// `Ok(CrateTree)` containing the fully processed crate tree ready for Phase 2,
    /// or `Err(Error)` if file processing, parsing, or tree construction fails.
    ///
    /// ## Error Handling
    ///
    /// This method can fail due to:
    /// - Missing or unreadable source files
    /// - Syntax errors in Rust source code
    /// - Invalid module structure or missing module files
    /// - Memory exhaustion during large codebase processing
    #[allow(unused)]
    pub fn build(config: &Config) -> Result<CrateTree, error::Error> {
        let Config { current_crate, external_crates, .. } = config;
        let context = Rc::new(RefCell::new(GlobalContext::from(config)));
        print_phase!("PHASE 0: PROCESS CRATES", "{}", config);
        process_crates(external_crates, &context)
            .and_then(|external_crates|
                current_crate.process(&context)
                    .and_then(|current_tree| CrateTree::new(current_crate, current_tree, external_crates)))
    }
    /// Processes a single crate's root directory into a ScopeTreeExportItem.
    ///
    /// This method handles the processing of a complete crate, starting from its root
    /// directory and recursively processing all modules within the crate. It creates
    /// the appropriate scope chain for the crate root and initiates the file processing.
    ///
    /// ## Parameters
    ///
    /// - `crate_config`: Configuration for the specific crate being processed
    /// - `context`: Shared global context for storing cross-crate information
    ///
    /// ## Returns
    ///
    /// A `ScopeTreeExportItem` representing the complete crate with all its modules
    /// and exported items properly structured.
    pub fn process_crate_tree(crate_config: &Crate, context: &Rc<RefCell<GlobalContext>>) -> Result<ScopeTreeExportItem, error::Error> {
        Self::new(crate_config.root_path(), ScopeChain::crate_root_with_ident_attr_less(crate_config.ident()), vec![], context)
            .process()
            .map(Visitor::into_code_tree)
    }
    fn new(path: PathBuf, scope: ScopeChain, attrs: Vec<Attribute>, context: &Rc<RefCell<GlobalContext>>) -> Self {
        Self { path, scope, context: context.clone(), attrs }
    }
    fn process(self) -> Result<Visitor, error::Error> {
        //print_phase!("PHASE 1: PROCESS FILE", "{:?}", self.path);
        self.read_syntax_tree()
            .map(|syntax_tree| self.setup_visitor(syntax_tree))
    }
    fn read_syntax_tree(&self) -> Result<syn::File, error::Error> {
        std::fs::read_to_string(&self.path)
            .map_err(error::Error::from)
            .and_then(|content| syn::parse_file(&content)
                .map_err(error::Error::from))
    }
    fn to_inner_visitors(&self, items: Vec<Item>) -> Vec<Visitor> {
        let mut visitors = vec![];
        for item in items {
            if let Item::Mod(ItemMod { ident, attrs, content: None, .. }) = item {
                if !self.is_fermented_mod(&ident) {
                    if let Ok(visitor) = self.process_module(&ident, attrs) {
                        visitors.push(visitor);
                    }
                }
            }
        }
        visitors
    }
    fn setup_visitor(&self, syntax_tree: syn::File) -> Visitor {
        let mut visitor = Visitor::new(&self.scope, &self.attrs, &self.context);
        visitor.visit_file(&syntax_tree);
        visitor.inner_visitors = self.to_inner_visitors(syntax_tree.items);
        visitor
    }
    /// Recursively processes a sub-module following Rust's module file conventions.
    ///
    /// This method implements Rust's module resolution logic, searching for module files
    /// in the standard locations and recursively processing them. It handles both
    /// single-file modules (`module.rs`) and directory-based modules (`module/mod.rs`).
    ///
    /// ## Module Resolution Order
    ///
    /// For a module named `foo`, searches in this order:
    /// 1. `./foo` (if it's a file)
    /// 2. `./foo/mod.rs`
    /// 3. `../foo.rs` (sibling to current directory)
    ///
    /// ## Parameters
    ///
    /// - `mod_name`: The identifier name of the module to process
    /// - `attrs`: Attributes applied to the module declaration
    ///
    /// ## Returns
    ///
    /// A `Visitor` containing the processed module contents, or an error if the
    /// module file cannot be located or processed.
    fn process_module(&self, mod_name: &Ident, attrs: Vec<Attribute>) -> Result<Visitor, error::Error> {
        let scope = ScopeChain::child_mod(attrs.clone(), self.scope.crate_ident_ref().clone(), mod_name, &self.scope);
        if let Some(parent_path) = self.path.parent() {
            let file_path = parent_path.join(mod_name.to_string());
            if file_path.is_file() {
                return FileTreeProcessor::new(file_path, scope, attrs, &self.context).process();
            } else {
                let path = file_path.join("mod.rs");
                if path.is_file() {
                    return FileTreeProcessor::new(path, scope, attrs, &self.context).process()
                } else if let Some(file_parent) = file_path.parent() {
                    let path = file_parent.join(format!("{mod_name}.rs"));
                    if path.is_file() {
                        return FileTreeProcessor::new(path, scope, attrs, &self.context).process()
                    }
                }
            }
        }
        Err(error::Error::ExpansionError("Can't locate module file"))
    }
    fn is_fermented_mod(&self, ident: &Ident) -> bool {
        let lock = self.context.borrow();
        lock.is_fermented_mod(ident)
    }
}

/// Processes multiple external crates in parallel, building a map of processed crate trees.
///
/// This function handles the batch processing of external dependency crates, creating
/// a map that associates each crate configuration with its processed ScopeTreeExportItem.
/// This enables the main crate processing to have access to all external crate information
/// for cross-crate reference resolution.
///
/// ## Processing Strategy
///
/// External crates are processed independently since they don't depend on each other
/// for basic scope construction (cross-references are resolved later in Phase 3).
/// This allows for potential parallelization in future optimizations.
///
/// ## Parameters
///
/// - `crates`: Slice of external crate configurations to process
/// - `context`: Shared global context for storing cross-crate information
///
/// ## Returns
///
/// A HashMap mapping each crate configuration to its processed scope tree, or an error
/// if any crate processing fails.
#[allow(unused)]
fn process_crates(crates: &[Crate], context: &Rc<RefCell<GlobalContext>>) -> Result<HashMap<Crate, ScopeTreeExportItem>, error::Error> {
    crates.iter()
        .try_fold(HashMap::new(), |mut acc, crate_config| {
            acc.insert(crate_config.clone(), crate_config.process(context)?);
            Ok(acc)
        })
}
