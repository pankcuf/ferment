use crate::context::ScopeSearchKey;
use syn::parse_quote;

#[test]
fn scope_search_key_ptr_flags() {
    // *const u8
    let key_const = ScopeSearchKey::maybe_from(parse_quote!(*const u8)).unwrap();
    assert!(!key_const.is_const_ptr());
    assert!(key_const.maybe_originally_is_const_ptr());
    assert!(!key_const.maybe_originally_is_mut_ptr());

    // *mut u8
    let key_mut = ScopeSearchKey::maybe_from(parse_quote!(*mut u8)).unwrap();
    assert!(!key_mut.is_mut_ptr());
    assert!(key_mut.maybe_originally_is_mut_ptr());
    assert!(!key_mut.maybe_originally_is_const_ptr());
}

#[test]
fn scope_search_key_ref_flags() {
    // &T
    let key_ref = ScopeSearchKey::maybe_from(parse_quote!(&T)).unwrap();
    assert!(!key_ref.is_ref());
    assert!(key_ref.maybe_originally_is_ref());
    assert!(!key_ref.maybe_originally_is_mut_ref());

    // &mut T
    let key_ref_mut = ScopeSearchKey::maybe_from(parse_quote!(&mut T)).unwrap();
    assert!(!key_ref_mut.is_mut_ref());
    assert!(key_ref_mut.maybe_originally_is_mut_ref());
}

#[test]
fn scope_search_key_dyn_flags() {
    let key = ScopeSearchKey::maybe_from(parse_quote!(dyn std::fmt::Debug)).unwrap();
    assert!(!key.is_dyn());
    assert!(key.maybe_originally_is_dyn());
}

#[test]
fn scope_search_key_plain_type_has_no_original_ptr_ref_dyn() {
    let key = ScopeSearchKey::maybe_from(parse_quote!(u32)).unwrap();
    assert!(!key.maybe_originally_is_ptr());
    assert!(!key.maybe_originally_is_const_ptr());
    assert!(!key.maybe_originally_is_mut_ptr());
    assert!(!key.maybe_originally_is_ref());
    assert!(!key.maybe_originally_is_mut_ref());
    assert!(!key.maybe_originally_is_dyn());
}
