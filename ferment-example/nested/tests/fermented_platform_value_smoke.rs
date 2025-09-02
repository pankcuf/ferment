#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::{FFIConversionFrom, FFIConversionTo};

// Smoke test round-trips for a subset of platform_value::Value variants
// using the generated FFI enum `platform_value_Value`.
//
// This exercises trait-based conversions defined in the generated module
// without constructing deep pointer graphs.

#[test]
fn platform_value_scalar_roundtrip() {
    use example_nested::fermented::types::platform_value::platform_value_Value as FFIValue;
    use platform_value::Value as Value;

    let cases = vec![
        Value::U8(42),
        Value::I16(-1234),
        Value::U32(17),
        Value::I64(-987654321),
        Value::U64(123456789),
        Value::Bool(true),
        Value::Null,
    ];

    for v in cases {
        // SAFETY: `ffi_to_const` allocates and returns a boxed FFI enum.
        let ffi_ptr = unsafe { <FFIValue as FFIConversionTo<Value>>::ffi_to_const(v.clone()) };
        assert!(!ffi_ptr.is_null());

        // SAFETY: `ffi_from_const` decodes from the FFI enum into a Value.
        let decoded = unsafe { <FFIValue as FFIConversionFrom<Value>>::ffi_from_const(ffi_ptr) };
        assert_eq!(decoded, v);

        // SAFETY: Free the boxed FFI enum that `ffi_to_const` allocated.
        unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
    }
}

