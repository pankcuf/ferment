use indexmap::IndexMap;
use syn::{parse_quote, Type};
use crate::composable::TypeModel;
use crate::context::TypeChain;
use crate::kind::{ObjectKind};
use quote::ToTokens;

#[test]
fn type_chain_replace_unknown_with_refined() {
    let mut chain = TypeChain::default();
    let key: Type = parse_quote!(T);

    // Start with unknown
    chain.add_one(key.clone(), ObjectKind::unknown_type(parse_quote!(UnknownTy)));
    assert_eq!(chain.get(&key).unwrap().maybe_type().unwrap(), parse_quote!(UnknownTy));

    // Replace with refined Object type
    let refined = ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(RefinedTy)));
    chain.add_one(key.clone(), refined);
    assert_eq!(chain.get(&key).unwrap().maybe_type().unwrap(), parse_quote!(RefinedTy));
}

#[test]
fn type_chain_keep_refined_over_non_bounds() {
    let mut chain = TypeChain::default();
    let key: Type = parse_quote!(K);

    // Start refined
    chain.add_one(key.clone(), ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(Refined1))));
    // Try to replace with unknown -> should keep refined
    chain.add_one(key.clone(), ObjectKind::unknown_type(parse_quote!(SomethingElse)));
    assert_eq!(chain.get(&key).unwrap().maybe_type().unwrap(), parse_quote!(Refined1));
}

#[test]
fn type_chain_replace_with_bounds() {
    use crate::composable::GenericBoundsModel;
    use crate::composer::CommaPunctuatedNestedArguments;

    let mut chain = TypeChain::default();
    let key: Type = parse_quote!(B);

    // Start refined
    chain.add_one(key.clone(), ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(Refined2))));

    // Candidate is bounds -> should replace refined
    let gb = GenericBoundsModel::new(&parse_quote!(T), IndexMap::new(), syn::Generics::default(), CommaPunctuatedNestedArguments::default());
    chain.add_one(key.clone(), ObjectKind::bounds(gb));
    // Now value should be a bounds kind (rendered type stays as generic T)
    assert_eq!(chain.get(&key).unwrap().maybe_type().unwrap(), parse_quote!(T));
}

#[test]
fn type_chain_replace_with_item_always() {
    use crate::kind::ScopeItemKind;

    let mut chain = TypeChain::default();
    let key: Type = parse_quote!(C);

    // Start refined
    chain.add_one(key.clone(), ObjectKind::object_model_type(TypeModel::new_default(parse_quote!(BaseTy))));

    // Build a minimal item for replacement
    let item: syn::ItemStruct = parse_quote!(struct S;);
    let scope: syn::Path = parse_quote!(my_crate);
    let item_kind = ScopeItemKind::item_struct(&item, &scope);
    let ty_kind = crate::kind::TypeModelKind::Object(TypeModel::new_default(parse_quote!(NewTy)));
    let candidate = ObjectKind::new_item(ty_kind, item_kind);

    chain.add_one(key.clone(), candidate);
    assert_eq!(chain.get(&key).unwrap().maybe_type().unwrap(), parse_quote!(NewTy));
}

#[test]
fn type_chain_selfless_filters_self_and_excluding_filters_non_generics() {
    use syn::{Generics};
    let mut chain = TypeChain::default();
    chain.add_one(parse_quote!(Self), ObjectKind::unknown_type(parse_quote!(Foo)));
    chain.add_one(parse_quote!(T), ObjectKind::unknown_type(parse_quote!(Foo)));
    chain.add_one(parse_quote!(U), ObjectKind::unknown_type(parse_quote!(Bar)));
    chain.add_one(parse_quote!(V), ObjectKind::unknown_type(parse_quote!(Baz)));

    let selfless = chain.selfless();
    assert!(selfless.inner.keys().all(|k| k.to_token_stream().to_string() != "Self"));

    let gens: Generics = parse_quote!(<T, U>);
    let filtered = chain.excluding_self_and_bounds(&gens);
    let keys_str = filtered.inner.keys().map(|k| k.to_token_stream().to_string()).collect::<Vec<_>>();
    // Excludes Self and method generics (T, U), keeps non-generics (V)
    assert!(keys_str.contains(&"V".to_string()));
    assert!(!keys_str.contains(&"Self".to_string()));
    assert!(!keys_str.contains(&"T".to_string()));
    assert!(!keys_str.contains(&"U".to_string()));
}
