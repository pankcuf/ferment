use indexmap::IndexMap;
use quote::ToTokens;
use syn::{parse_quote, Path, Type};
use crate::context::{GenericResolver, ScopeChain};

fn root_scope() -> ScopeChain {
    ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![])
}

#[test]
fn generic_resolver_add_and_query_bounds() {
    let scope = root_scope();
    let mut resolver = GenericResolver::default();
    let mut map: IndexMap<Type, Vec<Path>> = IndexMap::new();
    map.insert(parse_quote!(T), vec![parse_quote!(Clone), parse_quote!(::std::fmt::Debug)]);
    resolver.extend_in_scope(&scope, map);

    let bounds = resolver.maybe_generic_bounds(&scope, &parse_quote!(T)).unwrap();
    let rendered = bounds.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>();
    assert_eq!(rendered, vec!["Clone", ":: std :: fmt :: Debug"]);

    let first = resolver.maybe_first_generic(&scope, &parse_quote!(T)).unwrap();
    assert_eq!(first.to_token_stream().to_string(), "Clone");
}

#[test]
fn generic_resolver_merge_extends_bounds() {
    let scope = root_scope();
    let mut resolver = GenericResolver::default();
    let mut m1: IndexMap<Type, Vec<Path>> = IndexMap::new();
    m1.insert(parse_quote!(T), vec![parse_quote!(Clone)]);
    resolver.extend_in_scope(&scope, m1);

    let mut m2: IndexMap<Type, Vec<Path>> = IndexMap::new();
    m2.insert(parse_quote!(T), vec![parse_quote!(Send), parse_quote!(Sync)]);
    resolver.extend_in_scope(&scope, m2);

    let bounds = resolver.maybe_generic_bounds(&scope, &parse_quote!(T)).unwrap();
    let rendered = bounds.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>();
    assert_eq!(rendered, vec!["Clone", "Send", "Sync"]);
}

#[test]
fn generic_resolver_unknown_generic_returns_none() {
    let scope = root_scope();
    let resolver = GenericResolver::default();
    assert!(resolver.maybe_generic_bounds(&scope, &parse_quote!(U)).is_none());
    assert!(resolver.maybe_first_generic(&scope, &parse_quote!(U)).is_none());
}

