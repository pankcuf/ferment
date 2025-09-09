use ferment::FFIConversionTo;

// Callback: u32 -> Option<[u8; 32]>
#[test]
fn callback_u32_to_opt_hash_smoke() {
    use example_nested::fermented::generics::{
        Arr_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32, Fn_ARGS_u32_RTRN_Option_u8_32_ctor,
        Fn_ARGS_u32_RTRN_Option_u8_32_destroy,
    };

    // Caller returns None for odd numbers, Some([x;32]) for even numbers
    unsafe extern "C" fn caller(x: u32) -> *mut Arr_u8_32 {
        if x % 2 == 0 {
            let arr = [x as u8; 32];
            <Arr_u8_32 as FFIConversionTo<[u8; 32]>>::ffi_to(arr)
        } else {
            std::ptr::null_mut()
        }
    }

    // Destructor frees the optional pointer if non-null
    unsafe extern "C" fn destructor(ptr: *mut Arr_u8_32) {
        if !ptr.is_null() {
            ferment::unbox_any(ptr);
        }
    }

    // SAFETY: Construct the FFI callback wrapper
    let cb: *mut Fn_ARGS_u32_RTRN_Option_u8_32 = unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_ctor(caller, destructor) };
    assert!(!cb.is_null());

    // SAFETY: Exercise the call path directly
    let res_even = unsafe { (&*cb).call(10) };
    assert_eq!(res_even, Some([10u8; 32]));

    let res_odd = unsafe { (&*cb).call(11) };
    assert_eq!(res_odd, None);

    // No exported binding used here; direct wrapper call is sufficient

    // SAFETY: Destroy the wrapper
    unsafe { Fn_ARGS_u32_RTRN_Option_u8_32_destroy(cb) };
}

// Callback: (u32, [u8;32]) -> Option<String>, with allocation/deallocation assertions
#[test]
fn callback_two_args_to_opt_string_smoke() {
    use example_nested::fermented::generics::{
        Arr_u8_32, Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String,
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor,
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy,
    };
    use std::os::raw::c_char;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DESTRUCTOR_CALLS: AtomicUsize = AtomicUsize::new(0);
    static ARG_FREES: AtomicUsize = AtomicUsize::new(0);

    // Caller frees arg Arr_u8_32 and returns a C string for even x, otherwise null
    unsafe extern "C" fn caller(x: u32, arr: *mut Arr_u8_32) -> *mut c_char {
        if !arr.is_null() {
            // free the passed-in array to avoid leaks
            ferment::unbox_any(arr);
            ARG_FREES.fetch_add(1, Ordering::Relaxed);
        }
        if x % 2 == 0 {
            <c_char as FFIConversionTo<&str>>::ffi_to("ok")
        } else {
            std::ptr::null_mut()
        }
    }
    unsafe extern "C" fn destructor(ptr: *mut c_char) {
        if !ptr.is_null() {
            ferment::fermented::types::str_destroy(ptr);
            DESTRUCTOR_CALLS.fetch_add(1, Ordering::Relaxed);
        }
    }

    let cb: *mut Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String = unsafe {
        Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(caller, destructor)
    };
    assert!(!cb.is_null());

    // Even case -> Some("ok"), destructor must be called exactly once
    let res_even = unsafe { (&*cb).call(2, [7u8; 32]) };
    assert_eq!(res_even.as_deref(), Some("ok"));

    // Odd case -> None, destructor must NOT be called again
    let res_odd = unsafe { (&*cb).call(3, [9u8; 32]) };
    assert_eq!(res_odd, None);

    // Ensure arg frees happened for both calls
    assert_eq!(ARG_FREES.load(Ordering::Relaxed), 2);
    assert_eq!(DESTRUCTOR_CALLS.load(Ordering::Relaxed), 1);

    unsafe { Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(cb) };
}


// FnMut callback variant: (u32, [u8;32]) -> Option<String>
#[test]
fn callback_fnmut_two_args_to_opt_string_smoke() {
    use example_nested::fermented::generics::{
        Arr_u8_32, FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String,
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor,
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy,
    };
    use std::os::raw::c_char;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DESTRUCTOR_CALLS: AtomicUsize = AtomicUsize::new(0);
    static ARG_FREES: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" fn caller(x: u32, arr: *mut Arr_u8_32) -> *mut c_char {
        if !arr.is_null() {
            ferment::unbox_any(arr);
            ARG_FREES.fetch_add(1, Ordering::Relaxed);
        }
        if x % 2 == 1 {
            <c_char as FFIConversionTo<&str>>::ffi_to("odd")
        } else {
            std::ptr::null_mut()
        }
    }
    unsafe extern "C" fn destructor(ptr: *mut c_char) {
        if !ptr.is_null() {
            ferment::fermented::types::str_destroy(ptr);
            DESTRUCTOR_CALLS.fetch_add(1, Ordering::Relaxed);
        }
    }

    let cb: *mut FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String = unsafe {
        FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(caller, destructor)
    };
    assert!(!cb.is_null());

    // odd -> Some("odd")
    let s1 = unsafe { (&*cb).call(1, [0u8; 32]) };
    assert_eq!(s1.as_deref(), Some("odd"));

    // even -> None
    let s2 = unsafe { (&*cb).call(2, [0u8; 32]) };
    assert_eq!(s2, None);

    assert_eq!(ARG_FREES.load(Ordering::Relaxed), 2);
    assert_eq!(DESTRUCTOR_CALLS.load(Ordering::Relaxed), 1);

    // No exported binding used here; direct wrapper call is sufficient

    unsafe { FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(cb) };
}

// Callback using a Result return: ([u8;32],[u8;32]) -> Result<u32, ProtocolError>
#[test]
fn callback_args_to_result_u32_protocol_error_smoke() {
    use example_nested::fermented::generics::{
        Arr_u8_32,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as FnRes,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_ctor as FnResCtor,
        Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_destroy as FnResDestroy,
        Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as FFIResWrapper,
    };

    // We'll construct a ProtocolError value using the exported simple error type
    use example_simple::errors::protocol_error::ProtocolError;
    use example_simple::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError;

    unsafe extern "C" fn caller(_a: *mut Arr_u8_32, _b: *mut Arr_u8_32) -> *mut FFIResWrapper {
        // produce Ok(7)
        <FFIResWrapper as FFIConversionTo<Result<u32, ProtocolError>>>::ffi_to(Ok(7u32))
    }
    unsafe extern "C" fn destructor(ptr: *mut FFIResWrapper) {
        if !ptr.is_null() {
            ferment::unbox_any(ptr);
        }
    }

    let cb: *mut FnRes = unsafe { FnResCtor(caller, destructor) };
    assert!(!cb.is_null());

    // SAFETY: Exercise call; wrapper will convert to Result and call destructor
    let res = unsafe { (&*cb).call([0u8; 32], [1u8; 32]) };
    assert_eq!(res, Ok(7u32));

    // Now a variant that returns Err(…)
    unsafe extern "C" fn caller_err(_a: *mut Arr_u8_32, _b: *mut Arr_u8_32) -> *mut FFIResWrapper {
        // Build a valid ProtocolError variant
        let err = ProtocolError::InvalidPKT(InvalidIdentityPublicKeyTypeError { public_key_type: "boom".to_string() });
        <FFIResWrapper as FFIConversionTo<Result<u32, ProtocolError>>>::ffi_to(Err(err))
    }
    let cb_err: *mut FnRes = unsafe { FnResCtor(caller_err, destructor) };
    let res_err = unsafe { (&*cb_err).call([0u8; 32], [1u8; 32]) };
    assert!(res_err.is_err());

    unsafe {
        FnResDestroy(cb);
        FnResDestroy(cb_err);
    }
}
