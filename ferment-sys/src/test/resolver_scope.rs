use quote::ToTokens;
use syn::{parse_quote, Path, Type};
use crate::context::{ScopeResolver, ScopeChain, ScopeInfo, Scope, ScopeSearchKey};
use crate::ext::ToType;
use crate::kind::ObjectKind;

fn root_scope() -> ScopeChain {
    ScopeChain::crate_root_with_ident(parse_quote!(my_crate), vec![])
}

fn make_info(path: Path) -> ScopeInfo {
    ScopeInfo::attr_less(parse_quote!(my_crate), Scope::empty(path))
}

#[test]
fn scope_resolver_finds_scope_by_path() {
    let mut resolver = ScopeResolver::default();
    let root = root_scope();
    let p: Path = parse_quote!(my_crate::module::Type);
    let s_mod = ScopeChain::r#mod(make_info(p.clone()), root.clone());
    let _ = resolver.type_chain_mut(&s_mod);

    let found = resolver.maybe_scope(&p).expect("scope found");
    assert_eq!(found.self_path_ref().to_token_stream().to_string(), p.to_token_stream().to_string());
}

#[test]
fn scope_resolver_prefers_object_variant_for_same_path() {
    let mut resolver = ScopeResolver::default();
    let root = root_scope();
    let p: Path = parse_quote!(my_crate::same::path);
    let s_mod = ScopeChain::r#mod(make_info(p.clone()), root.clone());
    let s_fn = ScopeChain::r#fn(make_info(p.clone()), root.clone());
    let s_obj = ScopeChain::object(make_info(p.clone()), root.clone());
    let _ = resolver.type_chain_mut(&s_mod);
    let _ = resolver.type_chain_mut(&s_fn);
    let _ = resolver.type_chain_mut(&s_obj);

    let found = resolver.maybe_first_obj_scope(&p).expect("scope found");
    // Highest priority is Object
    assert!(matches!(found, ScopeChain::Object { .. }));
}

#[test]
fn scope_resolver_object_lookup_by_key_in_scope() {
    let mut resolver = ScopeResolver::default();
    let root = root_scope();
    let p: Path = parse_quote!(my_crate::obj::scope);
    let scope = ScopeChain::object(make_info(p.clone()), root.clone());
    let chain = resolver.type_chain_mut(&scope);

    let key_ty: Type = parse_quote!(KeyTy);
    let val_ty: Type = parse_quote!(ValTy);
    chain.add_one(key_ty.clone(), ObjectKind::unknown_type(val_ty.clone()));

    let search = ScopeSearchKey::maybe_from(key_ty.clone()).unwrap();
    let found = resolver.maybe_object_ref_by_key_in_scope(search, &scope).unwrap();
    assert_eq!(found.maybe_type().unwrap().to_token_stream().to_string(), val_ty.to_token_stream().to_string());
}

#[test]
fn scope_resolver_object_lookup_by_value_across_scopes() {
    let mut resolver = ScopeResolver::default();
    let root = root_scope();
    let p1: Path = parse_quote!(my_crate::s1);
    let p2: Path = parse_quote!(my_crate::s2);
    let s1 = ScopeChain::r#fn(make_info(p1), root.clone());
    let s2 = ScopeChain::object(make_info(p2), root.clone());
    resolver.type_chain_mut(&s1).add_one(parse_quote!(A), ObjectKind::unknown_type(parse_quote!(Foo)));
    resolver.type_chain_mut(&s2).add_one(parse_quote!(B), ObjectKind::unknown_type(parse_quote!(Bar)));

    let found = resolver.maybe_object_ref_by_value(ScopeSearchKey::maybe_from(parse_quote!(Foo)).unwrap()).unwrap();
    assert_eq!(found.maybe_type().unwrap().to_token_stream().to_string(), "Foo");
}

#[test]
fn scope_resolver_key_type_for_path() {
    let mut resolver = ScopeResolver::default();
    let root = root_scope();
    let p: Path = parse_quote!(my_crate::foo);
    let s = ScopeChain::r#mod(make_info(p.clone()), root.clone());
    let chain = resolver.type_chain_mut(&s);

    let key_path: Path = parse_quote!(MyTy);
    chain.add_one(key_path.clone().to_type(), ObjectKind::unknown_type(parse_quote!(FullTy)));

    let resolved = resolver.scope_key_type_for_path(&key_path, &s).unwrap();
    assert_eq!(resolved.to_token_stream().to_string(), "FullTy");
}
