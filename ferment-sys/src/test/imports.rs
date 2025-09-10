use quote::ToTokens;
use syn::{parse_quote, UseTree};
use proc_macro2::Ident;
use crate::context::{ImportResolver, ScopeChain};

#[test]
fn fold_import_tree_simple_and_group() {
    // Scope: crate root `my_crate`
    let scope = ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![]);
    let mut resolver = ImportResolver::default();

    // use foo::bar::Baz;
    let use_tree: UseTree = parse_quote!(foo::bar::Baz);
    resolver.fold_import_tree(&scope, &use_tree, Vec::<Ident>::new());

    // Ensure `Baz` maps to full path `foo::bar::Baz`
    let map = resolver.maybe_scope_imports(&scope).expect("imports present");
    let key: syn::Path = parse_quote!(Baz);
    let full = map.get(&key).expect("Baz present");
    assert_eq!(full.to_token_stream().to_string(), "foo :: bar :: Baz");

    // use top::{A, B};
    let use_tree_group: UseTree = parse_quote!(top::{A, B});
    resolver.fold_import_tree(&scope, &use_tree_group, Vec::<Ident>::new());

    let map = resolver.maybe_scope_imports(&scope).unwrap();
    let a_key: syn::Path = parse_quote!(A);
    let b_key: syn::Path = parse_quote!(B);
    assert_eq!(map.get(&a_key).unwrap().to_token_stream().to_string(), "top :: A");
    assert_eq!(map.get(&b_key).unwrap().to_token_stream().to_string(), "top :: B");
}

#[test]
fn fold_import_tree_rename_and_ignore_underscore() {
    let scope = ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![]);
    let mut resolver = ImportResolver::default();

    // use pkg::Type as Renamed;
    let renamed: UseTree = parse_quote!(pkg::Type as Renamed);
    resolver.fold_import_tree(&scope, &renamed, Vec::<Ident>::new());

    let map = resolver.maybe_scope_imports(&scope).cloned().unwrap();
    let key: syn::Path = parse_quote!(Renamed);
    assert_eq!(map.get(&key).unwrap().to_token_stream().to_string(), "pkg :: Type");

    // use pkg as _; should be ignored
    let before_len = resolver.maybe_scope_imports(&scope).map(|m| m.len()).unwrap_or_default();
    let ignored: UseTree = parse_quote!(pkg as _);
    resolver.fold_import_tree(&scope, &ignored, Vec::<Ident>::new());
    let after_len = resolver.maybe_scope_imports(&scope).map(|m| m.len()).unwrap_or_default();
    assert_eq!(after_len, before_len);
}

#[test]
fn fold_import_tree_glob_is_ignored() {
    let scope = ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![]);
    let mut resolver = ImportResolver::default();
    // Prime with one import so map exists
    resolver.fold_import_tree(&scope, &parse_quote!(foo::Bar), Vec::<Ident>::new());
    let before_len = resolver.maybe_scope_imports(&scope).map(|m| m.len()).unwrap_or_default();

    // use foo::*; should be ignored by resolver
    let glob: UseTree = parse_quote!(foo::*);
    resolver.fold_import_tree(&scope, &glob, Vec::<Ident>::new());
    let after_len = resolver.maybe_scope_imports(&scope).map(|m| m.len()).unwrap_or_default();
    assert_eq!(after_len, before_len);
}
