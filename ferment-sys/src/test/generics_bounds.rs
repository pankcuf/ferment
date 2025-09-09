use quote::ToTokens;
use syn::parse_quote;
use crate::ext::create_generics_chain;

#[test]
fn collect_trait_requirements_orders_and_dedups() {
    // Parse via a dummy function so syn accepts the generics + where-clause together
    let item: syn::ItemFn = parse_quote! {
        fn dummy<T: Clone + ::std::fmt::Debug, U>()
        where
            U: Send + Sync,
            ::std::vec::Vec<U>: ::std::fmt::Display,
            T: Clone // duplicate on purpose
        {}
    };
    let chain = create_generics_chain(&item.sig.generics);

    // Ensure stable ordering: by bounded type then trait path
    let keys: Vec<(String, String)> = chain.inner.iter()
        .map(|(bounded_ty, trait_paths)| (
            bounded_ty.to_token_stream().to_string(),
            trait_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + ")
        )
        )
        .collect();

    // Expected unique pairs (stringified)
    let expected = vec![
        ("T".to_string(), "Clone + :: std :: fmt :: Debug".to_string()),
        ("U".to_string(), "Send + Sync".to_string()),
        (":: std :: vec :: Vec < U >".to_string(), ":: std :: fmt :: Display".to_string()),
    ];

    assert_eq!(keys, expected);
}

#[test]
fn collect_trait_requirements_orders_and_dedups2() {
    use quote::ToTokens;
    let item: syn::ItemFn = parse_quote! {
        fn dummy<T: ?Sized + Clone, U>()
        where
            T: ?Sized + ::std::fmt::Debug,
            U: Send,
        {}
    };
    let chain = create_generics_chain(&item.sig.generics);
    let keys: Vec<(String, String)> = chain.inner
        .iter()
        .map(|(bounded_ty, trait_paths)| (
            bounded_ty.to_token_stream().to_string(),
            trait_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + "),
        ))
        .collect();
    // ?Sized bounds must be ignored; only restrictive trait bounds remain
    let expected = vec![
        ("T".to_string(), "Clone + :: std :: fmt :: Debug".to_string()),
        ("U".to_string(), "Send".to_string()),
    ];
    assert_eq!(keys, expected);
}


#[test]
fn collect_trait_requirements_orders_and_dedups3() {
    // Parse via a dummy function so syn accepts the generics + where-clause together
    let item: syn::ItemFn = parse_quote! {
        fn dummy<T, U>()
        where
            U: Send + Sync,
            ::std::vec::Vec<U>: ::std::fmt::Display,
            T: Clone + ::std::fmt::Debug
        {}
    };
    let chain = create_generics_chain(&item.sig.generics);
    // Ensure stable ordering: by bounded type then trait path
    let keys: Vec<(String, String)> = chain.inner.iter().map(|(bounded_ty, trait_paths)| (bounded_ty.to_token_stream().to_string(), trait_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + "))).collect();

    // Expected unique pairs (stringified)
    let expected = vec![
        ("T".to_string(), "Clone + :: std :: fmt :: Debug".to_string()),
        ("U".to_string(), "Send + Sync".to_string()),
        (":: std :: vec :: Vec < U >".to_string(), ":: std :: fmt :: Display".to_string()),
    ];

    assert_eq!(keys, expected);
}

#[test]
fn collect_trait_requirements_orders_and_dedups4() {
    let item: syn::ItemFn = parse_quote! {
        pub(crate) fn parse_proof<R, O: FromProof<R> + MockResponse>(request: O::Request) -> Result<Option<O>, ProtocolError>
        where O::Request: MockRequest {
            unimplemented!("request: {:?}, response: {:?}", request, response)
        }
    };
    let chain = create_generics_chain(&item.sig.generics);
    let keys: Vec<(String, String)> = chain.inner.iter()
        .map(|(bounded_ty, trait_paths)| (
            bounded_ty.to_token_stream().to_string(),
            (!trait_paths.is_empty())
                .then(|| trait_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + "))
                .unwrap_or_else(|| "<unlimited>".to_string())
        ))
        .collect();
    let expected = vec![
        ("O".to_string(), "FromProof < R > + MockResponse".to_string()),
        ("R".to_string(), "<unlimited>".to_string()),
        ("O :: Request".to_string(), "MockRequest".to_string()),
    ];

    assert_eq!(keys, expected);
}
