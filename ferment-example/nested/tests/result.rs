use ferment::{FFIConversionFrom, FFIConversionTo};

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

// Result<String, Option<String>>
#[test]
fn result_string_opt_string_roundtrip_and_free() {
    use example_nested::fermented::generics::Result_ok_String_err_Option_String as FFIRes;

    let cases = vec![
        Ok::<String, Option<String>>("ok".into()),
        Err::<String, Option<String>>(None),
        Err::<String, Option<String>>(Some("err".into())),
    ];
    for case in cases {
        let ffi = unsafe { <FFIRes as FFIConversionTo<Result<String, Option<String>>>>::ffi_to_const(case.clone()) };
        let decoded = unsafe { <FFIRes as FFIConversionFrom<Result<String, Option<String>>>>::ffi_from_const(ffi) };
        assert_eq!(decoded, case);
        unsafe { ferment::unbox_any(ffi.cast_mut()) };
    }
}

// Result<String, Vec<u8>>
#[test]
fn result_string_vec_u8_roundtrip_and_free() {
    use example_nested::fermented::generics::Result_ok_String_err_Vec_u8 as FFIRes;

    let cases = vec![
        Ok::<String, Vec<u8>>("alpha".into()),
        Err::<String, Vec<u8>>(vec![1, 2, 3, 4]),
    ];
    for case in cases {
        let ffi = unsafe { <FFIRes as FFIConversionTo<Result<String, Vec<u8>>>>::ffi_to_const(case.clone()) };
        let decoded = unsafe { <FFIRes as FFIConversionFrom<Result<String, Vec<u8>>>>::ffi_from_const(ffi) };
        assert_eq!(decoded, case);
        unsafe { ferment::unbox_any(ffi.cast_mut()) };
    }
}

// Result<String, Option<u32>>
#[test]
fn result_string_opt_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::Result_ok_String_err_Option_u32 as FFIRes;

    let cases = vec![
        Ok::<String, Option<u32>>("ok".into()),
        Err::<String, Option<u32>>(None),
        Err::<String, Option<u32>>(Some(42)),
    ];
    for case in cases {
        let ffi = unsafe { <FFIRes as FFIConversionTo<Result<String, Option<u32>>>>::ffi_to_const(case.clone()) };
        let decoded = unsafe { <FFIRes as FFIConversionFrom<Result<String, Option<u32>>>>::ffi_from_const(ffi) };
        assert_eq!(decoded, case);
        unsafe { ferment::unbox_any(ffi.cast_mut()) };
    }
}