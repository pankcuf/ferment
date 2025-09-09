#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::FFIConversionTo;

// Async: many concurrent calls to a simple u32 -> Option<[u8;32]> callback wrapper
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_many_height_to_hash_calls() {
    use example_entry_point::fermented::generics::{
        Arr_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32_ctor,
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy,
    };

    unsafe extern "C" fn caller(h: u32) -> *mut Arr_u8_32 {
        if h % 2 == 0 { <Arr_u8_32 as FFIConversionTo<[u8;32]>>::ffi_to([h as u8; 32]) } else { std::ptr::null_mut() }
    }
    unsafe extern "C" fn dtor(ptr: *mut Arr_u8_32) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb: *mut Fn_ARGS_u32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_ctor(caller, dtor) };
    assert!(!cb.is_null());

    let mut tasks = vec![];
    for h in 0u32..10 {
        let local = unsafe { (&*cb).clone() };
        tasks.push(tokio::spawn(async move {
            let r = unsafe { local.call(h) };
            (h, r)
        }));
    }

    for t in tasks {
        let (h, r) = t.await.unwrap();
        if h % 2 == 0 { assert_eq!(r, Some([h as u8; 32])); } else { assert!(r.is_none()); }
    }

    unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_destroy(cb) };
}

// Async: chain two wrappers in tasks: u32 -> hash, then hash -> root
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_chain_height_to_root() {
    use example_entry_point::fermented::generics::{
        Arr_u8_32,
        Fn_ARGS_u32_RTRN_Option_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32_ctor, Fn_ARGS_u32_RTRN_Option_u8_32_destroy,
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32, Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor, Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy,
    };

    unsafe extern "C" fn h2hash(h: u32) -> *mut Arr_u8_32 {
        if h > 0 { <Arr_u8_32 as FFIConversionTo<[u8;32]>>::ffi_to([h as u8; 32]) } else { std::ptr::null_mut() }
    }
    unsafe extern "C" fn hash2root(_h: *mut Arr_u8_32) -> *mut Arr_u8_32 { <Arr_u8_32 as FFIConversionTo<[u8;32]>>::ffi_to([0xAA; 32]) }
    unsafe extern "C" fn dtor(ptr: *mut Arr_u8_32) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb1: *mut Fn_ARGS_u32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_ctor(h2hash, dtor) };
    let cb2: *mut Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor(hash2root, dtor) };

    let mut tasks = vec![];
    for h in 1u32..6 {
        let a = unsafe { (&*cb1).clone() };
        let b = unsafe { (&*cb2).clone() };
        tasks.push(tokio::spawn(async move {
            let hash = unsafe { a.call(h) }.unwrap();
            let root = unsafe { b.call(hash) }.unwrap();
            (hash, root)
        }));
    }

    for t in tasks {
        let (_hash, root) = t.await.unwrap();
        assert_eq!(root, [0xAA; 32]);
    }

    unsafe {
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy(cb1);
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy(cb2);
    }
}

// Async: Result-returning callback used concurrently
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_result_callback() {
    use example_entry_point::fermented::generics::{
        Arr_u8_32,
        Result_ok_u32_err_String as FFIRes,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_String as FnRes,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_String_ctor as FnResCtor,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_String_destroy as FnResDestroy,
    };

    unsafe extern "C" fn ok_call(_a: *mut Arr_u8_32, _b: *mut Arr_u8_32) -> *mut FFIRes {
        <FFIRes as FFIConversionTo<Result<u32, String>>>::ffi_to(Ok(5u32))
    }
    unsafe extern "C" fn drop_res(ptr: *mut FFIRes) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb: *mut FnRes = unsafe { FnResCtor(ok_call, drop_res) };

    let mut tasks = vec![];
    for _ in 0..8 {
        let c = unsafe { (&*cb).clone() };
        tasks.push(tokio::spawn(async move { unsafe { c.call([1u8; 32], [2u8; 32]) } }));
    }

    for t in tasks { assert_eq!(t.await.unwrap(), Ok(5u32)); }

    unsafe { FnResDestroy(cb) };
}

