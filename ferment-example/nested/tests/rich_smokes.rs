#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::{FFIConversionFrom, FFIConversionTo};



// platform_value::types::bytes_32::Bytes32 wrapper
#[test]
fn platform_value_bytes32_wrapper_roundtrip() {
    use example_nested::fermented::types::platform_value::types::bytes_32::platform_value_types_bytes_32_Bytes32 as FFIBytes32;
    use platform_value::types::bytes_32::Bytes32;

    let original = Bytes32([9u8; 32]);
    let ffi = unsafe { <FFIBytes32 as FFIConversionTo<Bytes32>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIBytes32 as FFIConversionFrom<Bytes32>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}

// platform_value::types::identifier::{IdentifierBytes32, Identifier}
#[test]
fn platform_value_identifier_wrappers_roundtrip() {
    use example_nested::fermented::types::platform_value::types::identifier::platform_value_types_identifier_IdentifierBytes32 as FFIId32;
    use platform_value::types::identifier::IdentifierBytes32;

    let original = IdentifierBytes32([3u8; 32]);
    let ffi = unsafe { <FFIId32 as FFIConversionTo<IdentifierBytes32>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIId32 as FFIConversionFrom<IdentifierBytes32>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}


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


