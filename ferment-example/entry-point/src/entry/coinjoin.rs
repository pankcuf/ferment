use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::entry::FFIContext;

#[derive(Clone)]
#[ferment_macro::opaque]
pub struct CoinJoinProvider {
    pub context: Arc<FFIContext>,
}

#[ferment_macro::export]
impl CoinJoinProvider {
    pub fn new(context: Arc<FFIContext>) -> Self {
        Self { context }
    }

    // Recursively (loop-based) load opaque data while preserving context across iterations.
    // The function awaits between requests to simulate async IO.
    pub async fn load_smth_opaque<
        // Should,
        // Fetch,
        Should: Fn(&FFIContext, usize) -> bool + Send + Sync + 'static,
        Fetch: Fn(&FFIContext, usize) -> Vec<u8> + Send + Sync + 'static,
    >(
        &self,
        should_continue: Should,
        fetch: Fetch,
        max_iters: usize,
    ) -> Vec<Vec<u8>>
    // where
    //     Should: Fn(&FFIContext, usize) -> bool + Send + Sync + 'static,
    //     Fetch: Fn(&FFIContext, usize) -> Vec<u8> + Send + Sync + 'static,
    {
        let mut acc: Vec<Vec<u8>> = Vec::new();
        let mut i = 0usize;
        // Loop while continuation predicate holds and we have not exceeded max_iters
        while i < max_iters && should_continue(&self.context, i) {
            let chunk = fetch(&self.context, i);
            acc.push(chunk);
            i += 1;
            // Simulate async network/request delay
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        acc
    }

    pub async fn load_smth_opaque_with_cancel<
        Should: Fn(&FFIContext, usize) -> bool + Send + Sync + 'static,
        Fetch: Fn(&FFIContext, usize) -> Vec<u8> + Send + Sync + 'static,
        Cancel: Fn(usize) -> bool + Send + Sync + 'static,
    >(
        &self,
        should_continue: Should,
        fetch: Fetch,
        cancel: Cancel,
        max_iters: usize,
        total_timeout_ms: u64,
        initial_backoff_ms: u64,
        backoff_factor_num: u32,
        backoff_factor_den: u32,
    ) -> Vec<Vec<u8>> {
        let mut acc: Vec<Vec<u8>> = Vec::new();
        let mut i = 0usize;
        let start = Instant::now();
        let deadline = start + Duration::from_millis(total_timeout_ms);
        let mut backoff = Duration::from_millis(initial_backoff_ms);

        while i < max_iters {
            if Instant::now() >= deadline { break; }
            if cancel(i) { break; }
            if !should_continue(&self.context, i) { break; }

            let chunk = fetch(&self.context, i);
            acc.push(chunk);
            i += 1;

            if backoff.as_millis() > 0 {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() { break; }
                let sleep_dur = backoff.min(remaining);
                tokio::time::sleep(sleep_dur).await;
                // next backoff
                let next_us = (backoff.as_micros() as u128)
                    .saturating_mul(backoff_factor_num as u128)
                    .saturating_div(backoff_factor_den as u128) as u64;
                backoff = Duration::from_micros(next_us.max(1));
            }
        }
        acc
    }

    /// Blocking facade for FFI usage. Runs async logic inside provided Tokio runtime.
    pub fn load_smth_opaque_blocking<
        Should: Fn(usize) -> bool + Send + Sync + 'static,
        Fetch: Fn(usize) -> Vec<u8> + Send + Sync + 'static,
        Cancel: Fn(usize) -> bool + Send + Sync + 'static,
    >(
        &self,
        runtime: *mut tokio::runtime::Runtime,
        should: Should,
        fetch: Fetch,
        cancel: Cancel,
        max_iters: usize,
        total_timeout_ms: u64,
        initial_backoff_ms: u64,
        backoff_factor_num: u32,
        backoff_factor_den: u32,
    ) -> Vec<Vec<u8>> {
        let rt = unsafe { &mut *runtime };
        rt.block_on(self.load_smth_opaque_with_cancel(
            move |_ctx, i| should(i),
            move |_ctx, i| fetch(i),
            move |i| cancel(i),
            max_iters,
            total_timeout_ms,
            initial_backoff_ms,
            backoff_factor_num,
            backoff_factor_den,
        ))
    }
}
