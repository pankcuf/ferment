#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::FFIConversionTo;

// Chain two exported callback-based calls sequentially
#[test]
fn chain_lookup_then_merkle() {
    use example_nested::fermented::generics::{
        Arr_u8_32,
        Fn_ARGS_u32_RTRN_Option_u8_32_ctor,
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy,
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor,
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy,
    };

    // First callback: height -> Option<[u8;32]>
    unsafe extern "C" fn height_to_hash(h: u32) -> *mut Arr_u8_32 {
        if h == 0 { std::ptr::null_mut() } else { <Arr_u8_32 as FFIConversionTo<[u8;32]>>::ffi_to([h as u8; 32]) }
    }
    unsafe extern "C" fn drop_arr(ptr: *mut Arr_u8_32) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb1 = unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_ctor(height_to_hash, drop_arr) };
    assert!(!cb1.is_null());
    // Directly call the wrapper and chain to the next callback
    let first: Option<[u8; 32]> = unsafe { (&*cb1).call(10) };
    assert!(first.is_some());

    // Second callback: hash -> Option<[u8;32]>
    unsafe extern "C" fn hash_to_root(_h: *mut Arr_u8_32) -> *mut Arr_u8_32 {
        // Always return a constant root
        <Arr_u8_32 as FFIConversionTo<[u8;32]>>::ffi_to([0xAB; 32])
    }
    let cb2 = unsafe { Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_ctor(hash_to_root, drop_arr) };
    assert!(!cb2.is_null());
    if let Some(h) = first {
        let second = unsafe { (&*cb2).call(h) };
        assert!(second.is_some());
    }

    unsafe {
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy(cb1);
        Fn_ARGS_Arr_u8_32_RTRN_Option_u8_32_destroy(cb2);
    }
}

// Chain find_current_block_desc (Fn) then find_current_block_desc_mut (FnMut)
#[test]
fn chain_find_desc_then_desc_mut() {
    use example_nested::fermented::generics::{
        Arr_u8_32,
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String,
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor,
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy,
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String,
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor,
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy,
    };
    use std::os::raw::c_char;

    unsafe extern "C" fn cstr_drop(ptr: *mut c_char) { if !ptr.is_null() { ferment::fermented::types::str_destroy(ptr) } }

    // Fn variant
    unsafe extern "C" fn desc(_h: u32, _hash: *mut Arr_u8_32) -> *mut c_char {
        <c_char as FFIConversionTo<&str>>::ffi_to("desc")
    }
    let cb1: *mut Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String = unsafe { Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(desc, cstr_drop) };
    let r1 = unsafe { (&*cb1).call(1, [0u8; 32]) };
    assert_eq!(r1.as_deref(), Some("desc"));

    // FnMut variant
    unsafe extern "C" fn desc_mut(_h: u32, _hash: *mut Arr_u8_32) -> *mut c_char {
        <c_char as FFIConversionTo<&str>>::ffi_to("desc_mut")
    }
    let cb2: *mut FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String = unsafe { FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(desc_mut, cstr_drop) };
    let r2 = unsafe { (&*cb2).call(2, [1u8; 32]) };
    assert_eq!(r2.as_deref(), Some("desc_mut"));

    unsafe {
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(cb1);
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(cb2);
    }
}

// Call a Result-returning callback twice (reuse wrapper via Clone semantics)
#[test]
fn chain_should_process_diff_twice() {
    use example_nested::fermented::generics::{
        Arr_u8_32,
        Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as FFIRes,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as FnRes,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_ctor as FnResCtor,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_destroy as FnResDestroy,
    };
    use example_simple::errors::protocol_error::ProtocolError;

    unsafe extern "C" fn ok_caller(_a: *mut Arr_u8_32, _b: *mut Arr_u8_32) -> *mut FFIRes {
        <FFIRes as FFIConversionTo<Result<u32, ProtocolError>>>::ffi_to(Ok(9u32))
    }
    unsafe extern "C" fn res_drop(ptr: *mut FFIRes) { if !ptr.is_null() { ferment::unbox_any(ptr) } }

    let cb: *mut FnRes = unsafe { FnResCtor(ok_caller, res_drop) };
    // Invoke twice via direct wrapper method
    let a = [1u8; 32];
    let b = [2u8; 32];
    let r1 = unsafe { (&*cb).call(a, b) };
    let r2 = unsafe { (&*cb).call(a, b) };
    assert_eq!(r1, Ok(9u32));
    assert_eq!(r2, Ok(9u32));
    unsafe { FnResDestroy(cb) };
}
