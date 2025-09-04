#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::sync::Arc;

// Call CoinJoinProvider::load_smth_opaque_blocking through the Rust method and verify behavior
// This test exercises the FFI-friendly facade (blocking over a supplied Runtime) using closures
// that would be wrapped via FFI in external consumers.
#[test]
fn ffi_async_coinjoin_blocking_facade() {
    use example_entry_point::entry::{coinjoin::CoinJoinProvider, FFIContext};

    // Runtime to drive async (not nested in test runtime)
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let mut rt = Box::new(rt);
    let rt_ptr: *mut tokio::runtime::Runtime = (&mut *rt) as *mut _;

    let ctx = Arc::new(FFIContext {});
    let provider = CoinJoinProvider::new(ctx);

    let should = |i: usize| i < 4;
    let fetch = |i: usize| vec![i as u8];
    let cancel = |_i: usize| false;

    let result = provider.load_smth_opaque_blocking(
        rt_ptr,
        should,
        fetch,
        cancel,
        10,
        1_000,
        1,
        2,
        1,
    );
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], vec![0]);
    assert_eq!(result[3], vec![3]);
}
