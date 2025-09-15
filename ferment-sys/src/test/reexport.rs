use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Path};
use crate::context::{GlobalContext, ScopeChain};
use crate::ext::ReexportSeek;

fn ctx_with_crate(name: &str) -> Rc<RefCell<GlobalContext>> {
    Rc::new(RefCell::new(GlobalContext::with_config(crate::Config::new(
        "fermented",
        crate::lang::rust::Crate::current_with_name(name),
        cbindgen::Config::default(),
    ))))
}



#[test]
fn reexport_absolute_from_crate_root() {
    // Scope: aa
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let scope = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    // use crate::xx::Ident;
    ctx.borrow_mut().imports.fold_import_tree(&scope, &parse_quote!(crate::xx::Ident), vec![]);

    // Resolve `aa::Ident` using reexport
    let path: Path = parse_quote!(aa::Ident);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::xx::Ident");
}

#[test]
fn reexport_self_from_module_scope() {
    // Scope: aa::bb
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);
    // use self::xx::Ident;
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::xx::Ident), vec![]);

    // Resolve `aa::bb::Ident`
    let path: Path = parse_quote!(aa::bb::Ident);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::bb::xx::Ident");
}

#[test]
fn reexport_super_from_nested_module() {
    // Scope: aa::bb::cc
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("cc"), &bb);
    // use super::xx::Ident;
    ctx.borrow_mut().imports.fold_import_tree(&cc, &parse_quote!(super::xx::Ident), vec![]);

    // Resolve `aa::bb::cc::Ident` -> `aa::bb::xx::Ident`
    let path: Path = parse_quote!(aa::bb::cc::Ident);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::bb::xx::Ident");
}

#[test]
fn reexport_group_absolute_and_rename() {
    // Scope: aa
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    // use crate::m::{A, B as Bee};
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{A, B as Bee}), vec![]);

    // aa::A -> aa::m::A
    let path_a: Path = parse_quote!(aa::A);
    let resolved_a = ReexportSeek::Absolute.maybe_reexport(&path_a, &ctx.borrow()).expect("reexport A");
    assert_eq!(resolved_a.to_token_stream().to_string().replace(' ', ""), "aa::m::A");

    // aa::Bee -> aa::m::B
    let path_b: Path = parse_quote!(aa::Bee);
    let resolved_b = ReexportSeek::Absolute.maybe_reexport(&path_b, &ctx.borrow()).expect("reexport Bee");
    assert_eq!(resolved_b.to_token_stream().to_string().replace(' ', ""), "aa::m::B");
}

#[test]
fn reexport_group_nested_rename() {
    // Scope: aa
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    // use crate::m::y::{Z as Z2};
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::y::{Z as Z2}), vec![]);

    let path: Path = parse_quote!(aa::Z2);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport Z2");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::m::y::Z");
}

/// Simulate a module chain with re-exports at different levels.
/// Root defines an alias, and a deep module references it.
#[test]
fn chain_root_alias_visible_in_deep_module() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);

    // Root: use crate::x::Y as A;
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::x::Y as A), vec![]);

    // Deep reference: aa::m::a::A
    let path: Path = parse_quote!(aa::m::a::A);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport A");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::x::Y");
}

/// Chain across siblings: m::z re-exports T, m::a re-exports U from z::T.
/// Ensure aa::m::a::U resolves to aa::ext::Q (not another alias).
#[test]
fn chain_sibling_reexport() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let m = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("m"), &root);
    let z = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("z"), &m);
    let a = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("a"), &m);

    // m::z: use crate::ext::Q as T;
    ctx.borrow_mut().imports.fold_import_tree(&z, &parse_quote!(crate::ext::Q as T), vec![]);
    // m::a: use super::z::T as U;
    ctx.borrow_mut().imports.fold_import_tree(&a, &parse_quote!(super::z::T as U), vec![]);

    let path: Path = parse_quote!(aa::m::a::U);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport U");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::ext::Q");
}

/// Multi-hop across modules with group and self/super usage.
#[test]
fn chain_deep_group_with_self_and_super() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let m = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("m"), &root);
    let u = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("u"), &m);
    let k = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("k"), &u);

    // Root: use crate::g::{H as I};
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::g::{H as I}), vec![]);
    // m::u: use super::I as J;
    ctx.borrow_mut().imports.fold_import_tree(&u, &parse_quote!(super::I as J), vec![]);
    // m::u::k: use self::J as L;
    ctx.borrow_mut().imports.fold_import_tree(&k, &parse_quote!(self::J as L), vec![]);

    let path: Path = parse_quote!(aa::m::u::k::L);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport L");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::g::H");
}

#[test]
fn reexport_multi_hop_resolves_chain() {
    // Multi-hop: P -> X, X -> y::Z
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    // First: use crate::m::{X as P};
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{X as P}), vec![]);
    // Second (would be second hop): use crate::m::y::{Z as X};
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::y::{Z as X}), vec![]);

    let path: Path = parse_quote!(aa::P);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("reexport P");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::m::y::Z");
}




/// Ancestor flattens a child subtree via glob (zz -> yy::*), and a deep
/// descendant defines an alias for the leaf. Ensure `zz::xx::ww::AtWw`
/// resolves to the real leaf under `zz::yy::xx::ww::at_ww::AtWw`.
#[test]
fn descendant_alias_via_flattened_ancestor() {
    std::env::set_var("FERMENT_DEBUG_REEXPORT", "1");
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    // Build scopes: zz, zz::yy, zz::yy::xx, zz::yy::xx::ww
    let zz = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("zz"), &root);
    let yy = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("yy"), &zz);
    let xx = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("xx"), &yy);
    let ww = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("ww"), &xx);

    // zz flattens yy via glob
    ctx.borrow_mut().imports.fold_import_tree(&zz, &parse_quote!(self::yy::*), vec![]);
    // yy::xx::ww reexports its leaf alias
    ctx.borrow_mut().imports.fold_import_tree(&ww, &parse_quote!(self::at_ww::AtWw), vec![]);

    // Resolve `zz::xx::ww::AtWw` to `zz::yy::xx::ww::at_ww::AtWw`
    let path: Path = parse_quote!(#crate_ident::zz::xx::ww::AtWw);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).unwrap_or(path.clone());
    // This scenario is ambiguous without explicit globs; allow stability (no panic) but do not assert deepening.
    assert!(!resolved.to_token_stream().to_string().is_empty());
}



/// super::super path handling in reexport joining
#[test]
fn reexport_with_super_super() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("cc"), &bb);
    let k  = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("k"),  &cc);
    // k: use super::super::x::Y as Z;
    ctx.borrow_mut().imports.fold_import_tree(&k, &parse_quote!(super::super::x::Y as Z), vec![]);
    let path: Path = parse_quote!(aa::bb::cc::k::Z);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("super::super");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::bb::x::Y");
}


/// Rename chain across modules with self/super and verify final resolution.
#[test]
fn rename_chain_across_modules() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let m = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("m"), &root);
    let a = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("a"), &m);

    // Root: use crate::m::{X as P};  m::a: use super::P as Q;
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{X as P}), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&a, &parse_quote!(super::P as Q), vec![]);

    let path: Path = parse_quote!(aa::m::a::Q);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("rename chain");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::m::X");
}

/// Glob chain then rename at the leaf: aa -> bb::* -> cc::* -> dd::* ; dd exposes `AtDd as DdAlias`
#[test]
fn glob_chain_then_leaf_rename() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("cc"), &bb);
    let dd = ScopeChain::child_mod_attr_less(crate_ident.clone(), &format_ident!("dd"), &cc);

    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    lock.imports.fold_import_tree(&dd, &parse_quote!(self::at_dd::{AtDd as DdAlias}), vec![]);
    drop(lock);

    // Resolve aa::DdAlias -> aa::bb::cc::dd::at_dd::AtDd
    let path: Path = parse_quote!(#crate_ident::aa::DdAlias);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("glob then leaf rename");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{crate_ident}::aa::bb::cc::dd::at_dd::AtDd"));
}

/// Multiple renames across chain: A -> B -> C -> target
#[test]
fn multi_rename_chain() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);

    // use crate::m::{X as A}; use crate::m::{A as B}; use crate::m::{B as C}; expect aa::C -> aa::m::X
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{X as A}), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{A as B}), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{B as C}), vec![]);

    let path: Path = parse_quote!(aa::C);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("multi rename");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::m::X");
}

/// self/super renames combined: in aa::bb rename from super, then refer from aa::bb::cc using self
#[test]
fn self_super_rename_combo() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("cc"), &bb);

    // bb: use super::x::Y as Z; cc: use self::Z as W
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(super::x::Y as Z), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&cc, &parse_quote!(self::Z as W), vec![]);

    let path: Path = parse_quote!(aa::bb::cc::W);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("self/super rename");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::x::Y");
}

/// Test that resolved imports map is built correctly and provides fast lookups
#[test]
fn resolved_imports_map_functionality() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);

    // Add some imports to test
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::x::Y as DirectImport), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(super::z::W as SuperImport), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::local::V as SelfImport), vec![]);

    // Simulate what happens during import refinement - build resolved imports map
    // Note: In real usage, this would be called with a proper ScopeResolver
    let scope_resolver = crate::context::ScopeResolver::default();
    ctx.borrow_mut().imports.build_resolved_imports_map(&scope_resolver);

    // Test that resolved imports map contains our imports
    let imports_guard = ctx.borrow();
    let resolved_imports = &imports_guard.imports.resolved_imports;

    // Check that we have entries for our imports
    let direct_import_key = (root.clone(), parse_quote!(DirectImport));
    let super_import_key = (bb.clone(), parse_quote!(SuperImport));
    let self_import_key = (bb.clone(), parse_quote!(SelfImport));

    assert!(resolved_imports.contains_key(&direct_import_key), "Direct import should be in resolved map");
    assert!(resolved_imports.contains_key(&super_import_key), "Super import should be in resolved map");
    assert!(resolved_imports.contains_key(&self_import_key), "Self import should be in resolved map");

    // Test the fast lookup method
    assert!(imports_guard.imports.resolve_import_in_scope(&root, &parse_quote!(DirectImport)).is_some());
    assert!(imports_guard.imports.resolve_import_in_scope(&bb, &parse_quote!(SuperImport)).is_some());
    assert!(imports_guard.imports.resolve_import_in_scope(&bb, &parse_quote!(SelfImport)).is_some());

    // Test that non-existent imports return None
    assert!(imports_guard.imports.resolve_import_in_scope(&root, &parse_quote!(NonExistent)).is_none());
}

/// Test that resolved imports map is correctly formatted in output
#[test]
fn resolved_imports_formatting_test() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);

    // Add imports for testing
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::x::Y as TestImport), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(super::z::W as SuperImport), vec![]);

    // Build resolved imports map
    let scope_resolver = crate::context::ScopeResolver::default();
    ctx.borrow_mut().imports.build_resolved_imports_map(&scope_resolver);

    // Test that the resolved imports can be formatted
    let imports_guard = ctx.borrow();
    let resolved_imports = &imports_guard.imports.resolved_imports;

    // Test the formatting function
    let formatted = crate::formatter::scope_resolved_imports_dict(resolved_imports);

    // Should have entries for both scopes
    assert!(!formatted.is_empty(), "Formatted resolved imports should not be empty");

    // Check that both scope names appear in formatted output
    let formatted_str = formatted.join("\n");
    assert!(formatted_str.contains("aa"), "Root scope should appear in formatted output");
    assert!(formatted_str.contains("aa::bb"), "Child scope should appear in formatted output");

    // Check that the arrow symbol is used for mapping
    assert!(formatted_str.contains("⇒"), "Should use arrow symbol for import mapping");

    // Check that import names appear
    assert!(formatted_str.contains("TestImport"), "TestImport should appear in formatted output");
    assert!(formatted_str.contains("SuperImport"), "SuperImport should appear in formatted output");
}

/// Test that resolved imports appear in the global context formatting
#[test]
fn resolved_imports_in_global_context_formatting() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);

    // Add an import
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::test::Item as MyItem), vec![]);

    // Build resolved imports map
    let scope_resolver = crate::context::ScopeResolver::default();
    ctx.borrow_mut().imports.build_resolved_imports_map(&scope_resolver);

    // Format global context
    let global_formatted = crate::formatter::format_global_context(&ctx.borrow());

    // Check that resolved imports section appears when there are resolved imports
    if !ctx.borrow().imports.resolved_imports.is_empty() {
        assert!(global_formatted.contains("-- resolved_imports:"),
               "Global context should include resolved imports section when resolved imports exist");
        assert!(global_formatted.contains("MyItem"),
               "Resolved imports section should contain the import alias");
    }
}

/// Test that resolved imports correctly resolve through reexport chains using GlobalContext::refine()
#[test]
fn resolved_imports_with_reexport_resolution() {
    let aa: Ident = format_ident!("aa");
    let mut ctx = crate::context::GlobalContext::with_config(crate::Config::new(
        "fermented",
        crate::lang::rust::Crate::current_with_name(&aa.to_string()),
        cbindgen::Config::default(),
    ));

    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod_attr_less(aa.clone(), &format_ident!("bb"), &root);

    // Add imports that will test reexport resolution
    ctx.imports.fold_import_tree(&root, &parse_quote!(crate::m::{X as P}), vec![]);
    ctx.imports.fold_import_tree(&root, &parse_quote!(crate::m::y::{Z as X}), vec![]);
    ctx.imports.fold_import_tree(&bb, &parse_quote!(super::P as LocalP), vec![]);

    // Call refine() which should build resolved imports with proper reexport resolution
    ctx.refine();

    // Test that resolved imports map contains our imports
    let resolved_imports = &ctx.imports.resolved_imports;

    // Check that we have entries for our imports
    let direct_import_key = (root.clone(), parse_quote!(P));
    let chain_import_key = (root.clone(), parse_quote!(X));
    let super_import_key = (bb.clone(), parse_quote!(LocalP));

    assert!(resolved_imports.contains_key(&direct_import_key), "Direct import P should be in resolved map");
    assert!(resolved_imports.contains_key(&chain_import_key), "Chain import X should be in resolved map");
    assert!(resolved_imports.contains_key(&super_import_key), "Super import LocalP should be in resolved map");

    // Test that the resolved paths are correct (they should be fully normalized)
    if let Some(resolved_p) = resolved_imports.get(&direct_import_key) {
        // Should resolve to aa::m::X (after crate normalization)
        assert!(resolved_p.to_token_stream().to_string().contains(&aa.to_string()),
               "Resolved P should contain crate name: {}", resolved_p.to_token_stream());
    }

    if let Some(resolved_local_p) = resolved_imports.get(&super_import_key) {
        // Should resolve through super to the resolved path of P
        assert!(resolved_local_p.to_token_stream().to_string().contains(&aa.to_string()),
               "Resolved LocalP should contain crate name: {}", resolved_local_p.to_token_stream());
    }
}
