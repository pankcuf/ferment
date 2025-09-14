use quote::ToTokens;
use syn::{parse_quote, Ident};
use crate::context::ScopeChain;

#[test]
fn scope_chain_child_paths_and_display() {
    // Create crate root scope
    let crate_ident: Ident = parse_quote!(my_crate);
    let root = ScopeChain::crate_root_with_ident(crate_ident.clone(), vec![]);
    // Add child module
    let child_ident: Ident = parse_quote!(utils);
    let child = ScopeChain::child_mod_attr_less(crate_ident, &child_ident, &root);

    // Self paths format as tokens
    assert_eq!(root.self_path_ref().to_token_stream().to_string(), "my_crate");
    assert_eq!(child.self_path_ref().to_token_stream().to_string(), "my_crate :: utils");

    // Joined path adds a function ident correctly
    let fn_ident: Ident = parse_quote!(do_work);
    let joined = child.joined_path(&fn_ident);
    assert_eq!(joined.to_token_stream().to_string(), "my_crate :: utils :: do_work");

    // Formatting helpers should not panic and contain crate name
    let mid = child.fmt_mid();
    assert!(mid.contains("utils"));
}
