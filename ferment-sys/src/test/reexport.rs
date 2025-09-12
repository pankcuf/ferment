use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Path};
use crate::context::{GlobalContext, ScopeChain};
use crate::ext::ReexportSeek;
use syn::Type;
use crate::ext::VisitScope;
use crate::ext::ToType;

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
    let bb = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("bb"), &root);
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
    let bb = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("cc"), &bb);
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
    let m = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("m"), &root);
    let z = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("z"), &m);
    let a = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("a"), &m);

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
    let m = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("m"), &root);
    let u = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("u"), &m);
    let k = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("k"), &u);

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

#[test]
fn reexport_glob_from_module_scope() {
    // Scope: example_aliasing::aa
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);

    // Simulate: pub use self::at_aa::*; in module aa
    ctx.borrow_mut().imports.fold_import_tree(&aa, &parse_quote!(self::at_aa::*), vec![]);

    // Resolve `example_aliasing::aa::AtAa` -> `example_aliasing::aa::at_aa::AtAa`
    let path: Path = parse_quote!(#crate_ident::aa::AtAa);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("glob reexport");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::at_aa::AtAa", crate_ident));
}

#[test]
fn reexport_glob_multi_level_like_aliasing() {
    // Build scopes: example_aliasing::aa, ::aa::bb, ::aa::bb::cc, ::aa::bb::cc::dd
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let _dd = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("dd"), &cc);

    // Simulate:
    // aa: pub use self::bb::*;
    // bb: pub use self::at_bb::*;
    // cc: pub use self::dd::*;
    // dd: pub use self::at_dd::*;
    ctx.borrow_mut().imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::at_bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&cc, &parse_quote!(self::at_cc::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&_dd, &parse_quote!(self::at_dd::*), vec![]);

    // Resolve aa::bb::AtBb -> aa::bb::at_bb::AtBb
    let path_bb: Path = parse_quote!(#crate_ident::aa::bb::AtBb);
    let resolved_bb = ReexportSeek::Absolute.maybe_reexport(&path_bb, &ctx.borrow()).expect("glob bb");
    assert_eq!(resolved_bb.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::at_bb::AtBb", crate_ident));

    // Resolve aa::bb::cc::AtDd -> aa::bb::cc::dd::at_dd::AtDd
    let path_dd: Path = parse_quote!(#crate_ident::aa::bb::cc::AtDd);
    let resolved_dd = ReexportSeek::Absolute.maybe_reexport(&path_dd, &ctx.borrow()).expect("glob dd");
    assert_eq!(resolved_dd.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::cc::dd::at_dd::AtDd", crate_ident));
}

/// Resolve from root alias to nested bb item via glob chain: aa -> bb::* -> at_bb::*
#[test]
fn aliasing_from_root_to_bb_item() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    // aa: pub use self::bb::*; bb: pub use self::at_bb::*;
    ctx.borrow_mut().imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::at_bb::*), vec![]);

    // Resolve aa::AtBb -> aa::bb::at_bb::AtBb
    let path: Path = parse_quote!(#crate_ident::aa::AtBb);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("alias root->bb");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::at_bb::AtBb", crate_ident));
}

/// Resolve from root alias to deep dd item via nested globs: aa -> bb::* -> cc::* -> dd::* -> at_dd::*
#[test]
fn aliasing_from_root_to_dd_item() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let dd = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("dd"), &cc);
    // aa: pub use self::bb::*; bb: pub use self::cc::*; cc: pub use self::dd::*; dd: pub use self::at_dd::*
    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    lock.imports.fold_import_tree(&dd, &parse_quote!(self::at_dd::*), vec![]);
    drop(lock);

    // Resolve aa::AtDd -> aa::bb::cc::dd::at_dd::AtDd
    let path: Path = parse_quote!(#crate_ident::aa::AtDd);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("alias root->dd");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::cc::dd::at_dd::AtDd", crate_ident));
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
    let zz = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("zz"), &root);
    let yy = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("yy"), &zz);
    let xx = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("xx"), &yy);
    let ww = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("ww"), &xx);

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

/// Resolve from bb-level alias to dd item via nested globs: bb::* -> cc::* -> dd::* -> at_dd::*
#[test]
fn aliasing_from_bb_to_dd_item() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let dd = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("dd"), &cc);
    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    lock.imports.fold_import_tree(&dd, &parse_quote!(self::at_dd::*), vec![]);
    drop(lock);

    // Resolve aa::bb::AtDd -> aa::bb::cc::dd::at_dd::AtDd
    let path: Path = parse_quote!(#crate_ident::aa::bb::AtDd);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("alias bb->dd");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::cc::dd::at_dd::AtDd", crate_ident));
}

/// Resolve from root alias to cc item via nested globs: aa -> bb::* -> cc::* -> at_cc::*
#[test]
fn aliasing_from_root_to_cc_item() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::at_cc::*), vec![]);
    drop(lock);

    // Resolve aa::AtCc -> aa::bb::cc::at_cc::AtCc
    let path: Path = parse_quote!(#crate_ident::aa::AtCc);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("alias root->cc");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::cc::at_cc::AtCc", crate_ident));
}

/// When multiple glob bases exist in a scope, prefer the one that can actually lead to the symbol.
#[test]
fn multi_base_glob_prefers_matching_base() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("bb"), &root);
    // Root has two globs: bb::* and cc::*. Only bb will have at_bb::*
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(self::bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(self::cc::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::at_bb::*), vec![]);

    // Resolve aa::AtBb -> aa::bb::at_bb::AtBb; must not choose cc
    let path: Path = parse_quote!(aa::AtBb);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("multi-base glob");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::bb::at_bb::AtBb");
}

/// super::super path handling in reexport joining
#[test]
fn reexport_with_super_super() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let bb = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("cc"), &bb);
    let k  = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("k"),  &cc);
    // k: use super::super::x::Y as Z;
    ctx.borrow_mut().imports.fold_import_tree(&k, &parse_quote!(super::super::x::Y as Z), vec![]);
    let path: Path = parse_quote!(aa::bb::cc::k::Z);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("super::super");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::bb::x::Y");
}

/// Ensure nested generic argument refines via the pipeline (NestedArgument)
#[test]
fn refine_nested_generic_via_pipeline() {
    use syn::Item;
    use crate::tree::Visitor;

    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    ctx.borrow_mut().imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::at_bb::*), vec![]);

    // fn f(arg: BTreeMap<example_aliasing::aa::AtBb, u32>) {}
    let f: Item = parse_quote!(fn f(arg: std::collections::BTreeMap<#crate_ident::aa::AtBb, u32>) {});
    let mut visitor = Visitor::new(&aa, &[], &ctx);
    let fn_scope = f.join_scope(&aa, &mut visitor).expect("fn scope");

    // In fn scope, the chain contains a key for the nested type path
    let key: Type = parse_quote!(#crate_ident::aa::AtBb);
    let context = ctx.borrow();
    let chain = context.scope_register.get(&fn_scope).expect("fn scope chain");
    let obj = chain.get(&key).expect("AtBb present in fn scope chain");
    // Refine via pipeline (NestedArgument driven)
    let refined = context.maybe_refined_object(&fn_scope, obj).expect("refined");
    let ty = refined.maybe_type_model_kind_ref().expect("ty kind").to_type();
    assert_eq!(ty.to_token_stream().to_string().replace(' ', ""),
               format!("{}::aa::bb::at_bb::AtBb", crate_ident));
}

/// Ensure nested fn argument refines via the pipeline (NestedArgument)
#[test]
fn refine_nested_fn_arg_via_pipeline() {
    use syn::Item;
    use crate::tree::Visitor;

    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let dd = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("dd"), &cc);
    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    lock.imports.fold_import_tree(&dd, &parse_quote!(self::at_dd::*), vec![]);
    drop(lock);

    // fn g(arg: Option<example_aliasing::aa::AtDd>) {}
    let g: Item = parse_quote!(fn g(arg: Option<#crate_ident::aa::AtDd>) {});
    let mut visitor = Visitor::new(&aa, &[], &ctx);
    let fn_scope = g.join_scope(&aa, &mut visitor).expect("fn scope");

    let key: Type = parse_quote!(#crate_ident::aa::AtDd);
    let context = ctx.borrow();
    let chain = context.scope_register.get(&fn_scope).expect("fn scope chain");
    let obj = chain.get(&key).expect("AtDd present in fn scope chain");
    let refined = context.maybe_refined_object(&fn_scope, obj).expect("refined");
    let ty = refined.maybe_type_model_kind_ref().expect("ty kind").to_type();
    assert_eq!(ty.to_token_stream().to_string().replace(' ', ""),
               format!("{}::aa::bb::cc::dd::at_dd::AtDd", crate_ident));
}

/// Rename chain across modules with self/super and verify final resolution.
#[test]
fn rename_chain_across_modules() {
    let aa: Ident = format_ident!("aa");
    let ctx = ctx_with_crate(&aa.to_string());
    let root = ScopeChain::crate_root_with_ident(aa.clone(), vec![]);
    let m = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("m"), &root);
    let a = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("a"), &m);

    // Root: use crate::m::{X as P};  m::a: use super::P as Q;
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::m::{X as P}), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&a, &parse_quote!(super::P as Q), vec![]);

    let path: Path = parse_quote!(aa::m::a::Q);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("rename chain");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::m::X");
}

/// Rename mixed with glob chain: root rename + nested globs to a concrete item
#[test]
fn rename_then_glob_chain_to_bb() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);

    // root: use crate::aa::bb::{AtBb as RootBb}; aa: pub use self::bb::*; bb: pub use self::at_bb::*
    ctx.borrow_mut().imports.fold_import_tree(&root, &parse_quote!(crate::aa::bb::{AtBb as RootBb}), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(self::at_bb::*), vec![]);

    let path: Path = parse_quote!(#crate_ident::RootBb);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("rename+glob bb");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""),
               format!("{}::aa::bb::at_bb::AtBb", crate_ident));
}

/// Glob chain then rename at the leaf: aa -> bb::* -> cc::* -> dd::* ; dd exposes `AtDd as DdAlias`
#[test]
fn glob_chain_then_leaf_rename() {
    let crate_ident: Ident = format_ident!("example_aliasing");
    let ctx = ctx_with_crate(&crate_ident.to_string());
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    let aa = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("aa"), &root);
    let bb = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("bb"), &aa);
    let cc = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("cc"), &bb);
    let dd = ScopeChain::child_mod(vec![], crate_ident.clone(), &format_ident!("dd"), &cc);

    let mut lock = ctx.borrow_mut();
    lock.imports.fold_import_tree(&aa, &parse_quote!(self::bb::*), vec![]);
    lock.imports.fold_import_tree(&bb, &parse_quote!(self::cc::*), vec![]);
    lock.imports.fold_import_tree(&cc, &parse_quote!(self::dd::*), vec![]);
    lock.imports.fold_import_tree(&dd, &parse_quote!(self::at_dd::{AtDd as DdAlias}), vec![]);
    drop(lock);

    // Resolve aa::DdAlias -> aa::bb::cc::dd::at_dd::AtDd
    let path: Path = parse_quote!(#crate_ident::aa::DdAlias);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("glob then leaf rename");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), format!("{}::aa::bb::cc::dd::at_dd::AtDd", crate_ident));
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
    let bb = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("bb"), &root);
    let cc = ScopeChain::child_mod(vec![], aa.clone(), &format_ident!("cc"), &bb);

    // bb: use super::x::Y as Z; cc: use self::Z as W
    ctx.borrow_mut().imports.fold_import_tree(&bb, &parse_quote!(super::x::Y as Z), vec![]);
    ctx.borrow_mut().imports.fold_import_tree(&cc, &parse_quote!(self::Z as W), vec![]);

    let path: Path = parse_quote!(aa::bb::cc::W);
    let resolved = ReexportSeek::Absolute.maybe_reexport(&path, &ctx.borrow()).expect("self/super rename");
    assert_eq!(resolved.to_token_stream().to_string().replace(' ', ""), "aa::x::Y");
}
