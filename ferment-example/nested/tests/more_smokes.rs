#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::{FFIConversionFrom, FFIConversionTo};

// Vectors (arrays) via platform_value::Value::Array
#[test]
fn platform_value_array_roundtrip() {
    use example_nested::fermented::types::platform_value::platform_value_Value as FFIValue;
    use platform_value::Value;

    let arr = vec![Value::U8(1), Value::U8(2), Value::U8(3)];
    let v = Value::Array(arr.clone());

    // SAFETY: Encodes Value into FFI enum and allocates it.
    let ffi_ptr = unsafe { <FFIValue as FFIConversionTo<Value>>::ffi_to_const(v.clone()) };
    assert!(!ffi_ptr.is_null());

    // SAFETY: Decodes back into Value.
    let decoded = unsafe { <FFIValue as FFIConversionFrom<Value>>::ffi_from_const(ffi_ptr) };
    match decoded {
        Value::Array(inner) => assert_eq!(inner, arr),
        other => panic!("expected Array, got: {:?}", other),
    }

    // SAFETY: Free the boxed FFI enum
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Maps via platform_value::value_map::ValueMap wrapper
#[test]
fn platform_value_value_map_roundtrip() {
    use example_nested::fermented::types::platform_value::value_map::platform_value_value_map_ValueMap as FFIValueMap;
    use platform_value::{value_map::ValueMap, Value};

    let mut map = ValueMap::new();
    map.push((Value::U8(1), Value::Text("one".into())));
    map.push((Value::U8(2), Value::Text("two".into())));

    // SAFETY: Encode the map into FFI wrapper and allocate it.
    let ffi_ptr = unsafe { <FFIValueMap as FFIConversionTo<ValueMap>>::ffi_to_const(map.clone()) };
    assert!(!ffi_ptr.is_null());

    // SAFETY: Decode the FFI wrapper back to a ValueMap
    let decoded = unsafe { <FFIValueMap as FFIConversionFrom<ValueMap>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, map);

    // SAFETY: Free the boxed FFI map
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

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

    // Also ensure passing through an exported FFI binding does not crash.
    unsafe { example_nested::fermented::types::example_nested::model::callback::example_nested_model_callback_lookup_block_hash_by_height((&*cb).clone()) };

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

// Generic Vec<u32> FFI wrapper roundtrip with cleanup
#[test]
fn generics_vec_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_u32 as FFIVecU32;

    let original: Vec<u32> = vec![10, 20, 30, 40];
    // SAFETY: allocate FFI vector
    let ffi_vec = unsafe { <FFIVecU32 as FFIConversionTo<Vec<u32>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_vec.is_null());

    // SAFETY: decode
    let decoded = unsafe { <FFIVecU32 as FFIConversionFrom<Vec<u32>>>::ffi_from_const(ffi_vec) };
    assert_eq!(decoded, original);

    // SAFETY: free; this also frees the inner values buffer via Drop
    unsafe { ferment::unbox_any(ffi_vec.cast_mut()) };
}

// Vec<String> wrapper roundtrip with inner C-string cleanup performed by Drop
#[test]
fn generics_vec_string_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_String as FFIVecString;

    let original: Vec<String> = vec!["alpha".into(), "beta".into(), "gamma".into()];
    let ffi_vec = unsafe { <FFIVecString as FFIConversionTo<Vec<String>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_vec.is_null());

    let decoded = unsafe { <FFIVecString as FFIConversionFrom<Vec<String>>>::ffi_from_const(ffi_vec) };
    assert_eq!(decoded, original);

    // SAFETY: Drop frees each inner c_char pointer via unbox_string
    unsafe { ferment::unbox_any(ffi_vec.cast_mut()) };
}

// Vec<platform_value::Value> FFI group wrapper roundtrip and cleanup
#[test]
fn generics_vec_platform_value_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_platform_value_Value as FFIVecVal;
    use platform_value::Value;

    let values: Vec<Value> = vec![Value::U8(5), Value::Bool(true), Value::Null];
    let ffi_vec = unsafe { <FFIVecVal as FFIConversionTo<Vec<Value>>>::ffi_to_const(values.clone()) };
    assert!(!ffi_vec.is_null());

    let decoded = unsafe { <FFIVecVal as FFIConversionFrom<Vec<Value>>>::ffi_from_const(ffi_vec) };
    assert_eq!(decoded, values);

    // SAFETY: Drop frees array of pointers to FFI enum elements.
    unsafe { ferment::unbox_any(ffi_vec.cast_mut()) };
}

// BTreeMap<u32, u32> wrapper roundtrip
#[test]
fn generics_btreemap_u32_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_Map_keys_u32_values_u32 as FFIMapU32U32;
    use std::collections::BTreeMap;

    let mut original = BTreeMap::new();
    original.insert(1, 10);
    original.insert(2, 20);

    let ffi_map = unsafe { <FFIMapU32U32 as FFIConversionTo<BTreeMap<u32, u32>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_map.is_null());

    let decoded = unsafe { <FFIMapU32U32 as FFIConversionFrom<BTreeMap<u32, u32>>>::ffi_from_const(ffi_map) };
    assert_eq!(decoded, original);

    // SAFETY: Drop frees keys and values buffers
    unsafe { ferment::unbox_any(ffi_map.cast_mut()) };
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

    // Exercise the exported binding for FnMut variant
    unsafe { example_nested::fermented::types::example_nested::model::callback::example_nested_model_callback_find_current_block_desc_mut((&*cb).clone()) };

    unsafe { FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(cb) };
}

// Result wrappers: primitive/primitive
#[test]
fn result_u32_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::Result_ok_u32_err_u32 as FFIResultU32U32;

    let cases = vec![Ok(123u32), Err(456u32)];
    for case in cases {
        let ffi_ptr = unsafe { <FFIResultU32U32 as FFIConversionTo<Result<u32, u32>>>::ffi_to_const(case.clone()) };
        assert!(!ffi_ptr.is_null());
        let decoded = unsafe { <FFIResultU32U32 as FFIConversionFrom<Result<u32, u32>>>::ffi_from_const(ffi_ptr) };
        assert_eq!(decoded, case);
        unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
    }
}

// Result wrappers: complex/optional-generic (String / Option<Vec<u8>>)
#[test]
fn result_string_opt_vecu8_roundtrip_and_free() {

    let cases = vec![
        Ok::<String, Option<Vec<u8>>>("hello".into()),
        Err::<String, Option<Vec<u8>>>(None),
        Err::<String, Option<Vec<u8>>>(Some(vec![1, 2, 3])),
    ];
    for case in cases {
        let ffi_ptr = unsafe { <example_nested::fermented::generics::Result_ok_String_err_Option_Vec_u8 as FFIConversionTo<Result<String, Option<Vec<u8>>>>>::ffi_to_const(case.clone()) };
        assert!(!ffi_ptr.is_null());
        let decoded = unsafe { <example_nested::fermented::generics::Result_ok_String_err_Option_Vec_u8 as FFIConversionFrom<Result<String, Option<Vec<u8>>>>>::ffi_from_const(ffi_ptr) };
        assert_eq!(decoded, case);
        // SAFETY: Drop frees optional inner allocations correctly
        unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
    }
}

// Result wrappers: complex/complex (String / String)
#[test]
fn result_string_string_roundtrip_and_free() {
    use example_nested::fermented::generics::Result_ok_String_err_String as FFIResultStrStr;

    let cases = vec![
        Ok::<String, String>("alpha".into()),
        Err::<String, String>("omega".into()),
    ];
    for case in cases {
        let ffi_ptr = unsafe { <FFIResultStrStr as FFIConversionTo<Result<String, String>>>::ffi_to_const(case.clone()) };
        assert!(!ffi_ptr.is_null());
        let decoded = unsafe { <FFIResultStrStr as FFIConversionFrom<Result<String, String>>>::ffi_from_const(ffi_ptr) };
        assert_eq!(decoded, case);
        unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
    }
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
