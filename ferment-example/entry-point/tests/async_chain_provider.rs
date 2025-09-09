#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

// Advanced async chained call with context preserved via Arc<FFIContext>
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_coinjoin_provider_chained_load() {
use example_entry_point::entry::{coinjoin::CoinJoinProvider, FFIContext};

    // Build provider with shared context
    let ctx = Arc::new(FFIContext {});
    let provider = CoinJoinProvider::new(ctx.clone());

    // Side context to verify we visited steps with access to the same Arc<FFIContext>
    static VISITS: std::sync::OnceLock<Mutex<HashMap<usize, usize>>> = std::sync::OnceLock::new();

    // Continue for first 5 iterations
    let should = |c: &FFIContext, idx: usize| {
        // Record the raw pointer for this context to ensure it is consistent
        let lock = VISITS.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = lock.lock().unwrap();
        let addr = (c as *const FFIContext) as usize;
        guard.insert(idx, addr);
        idx < 5
    };
    // Fetch produces bytes that encode the index; it also accesses the context
    let fetch = |_c: &FFIContext, idx: usize| vec![idx as u8, 0xCA, 0xFE];

    let acc = provider.load_smth_opaque(should, fetch, 10).await;
    assert_eq!(acc.len(), 5);
    for (i, item) in acc.iter().enumerate() {
        assert_eq!(item[0], i as u8);
    }

    // Ensure context pointer recorded for each iteration is the same as the provider's Arc
    let lock = VISITS.get().unwrap().lock().unwrap();
    for i in 0..5 {
        let addr = lock.get(&i).copied().unwrap();
        assert_eq!(addr, Arc::as_ptr(&ctx) as usize);
    }
}
