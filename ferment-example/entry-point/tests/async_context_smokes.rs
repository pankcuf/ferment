#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use ferment::{FFIConversionFrom, FFIConversionTo};

// Shared test context: maps produced hashes to the originating height
static CONTEXT: OnceLock<Mutex<HashMap<[u8; 32], u32>>> = OnceLock::new();

fn ctx() -> &'static Mutex<HashMap<[u8; 32], u32>> {
    CONTEXT.get_or_init(|| Mutex::new(HashMap::new()))
}

// Advanced async chaining with context preservation between steps
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_chain_with_context_preserved() {
    use example_entry_point::fermented::generics::{
        Arr_u8_32,
        Fn_ARGS_u32_RTRN_Option_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32_ctor, Fn_ARGS_u32_RTRN_Option_u8_32_destroy,
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32, Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor, Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy,
    };

    // Step 1: height -> hash, store context (height) keyed by hash
    unsafe extern "C" fn h2hash_store(h: u32) -> *mut Arr_u8_32 {
        let hash = [h as u8; 32];
        let mut lock = ctx().lock().unwrap();
        lock.insert(hash, h);
        <Arr_u8_32 as FFIConversionTo<[u8; 32]>>::ffi_to(hash)
    }
    // Step 2: hash -> root, use stored context to build a root deterministically
    unsafe extern "C" fn hash2root_from_ctx(h_ptr: *mut Arr_u8_32) -> *mut Arr_u8_32 {
        let hash = <Arr_u8_32 as FFIConversionFrom<[u8; 32]>>::ffi_from(h_ptr);
        let lock = ctx().lock().unwrap();
        if let Some(height) = lock.get(&hash).copied() {
            // Build a root that encodes the height in the first byte to verify context
            let mut root = [0u8; 32];
            root[0] = (height & 0xFF) as u8;
            root[1..].copy_from_slice(&hash[1..]);
            <Arr_u8_32 as FFIConversionTo<[u8; 32]>>::ffi_to(root)
        } else {
            std::ptr::null_mut()
        }
    }
    unsafe extern "C" fn drop_arr(ptr: *mut Arr_u8_32) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb_h2hash: *mut Fn_ARGS_u32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_ctor(h2hash_store, drop_arr) };
    let cb_hash2root: *mut Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor(hash2root_from_ctx, drop_arr) };
    assert!(!cb_h2hash.is_null() && !cb_hash2root.is_null());

    // Run multiple chains concurrently and verify first byte of root matches original height (mod 256)
    let mut tasks = vec![];
    for h in 1u32..=8 {
        let a = unsafe { (&*cb_h2hash).clone() };
        let b = unsafe { (&*cb_hash2root).clone() };
        tasks.push(tokio::spawn(async move {
            let hash = unsafe { a.call(h) }.expect("hash");
            let root = unsafe { b.call(hash) }.expect("root");
            (h, root)
        }));
    }

    for t in tasks {
        let (h, root) = t.await.unwrap();
        assert_eq!(root[0], (h & 0xFF) as u8);
    }

    // Clean up wrappers
    unsafe {
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy(cb_h2hash);
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy(cb_hash2root);
    }
}

