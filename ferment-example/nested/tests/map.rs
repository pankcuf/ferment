#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::{FFIConversionFrom, FFIConversionTo};

// BTreeMap<String, Duration> empty
#[test]
fn btreemap_string_duration_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_Map_keys_String_values_std_time_Duration as FFIMapStrDur;
    use std::collections::BTreeMap;
    use std::time::Duration;
    let original: BTreeMap<String, Duration> = BTreeMap::new();
    let ffi_ptr = unsafe { <FFIMapStrDur as FFIConversionTo<BTreeMap<String, Duration>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIMapStrDur as FFIConversionFrom<BTreeMap<String, Duration>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// BTreeMap<String, platform_value::Value> empty
#[test]
fn btreemap_string_platform_value_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as FFIMapStrVal;
    use std::collections::BTreeMap;
    use platform_value::Value;
    let original: BTreeMap<String, Value> = BTreeMap::new();
    let ffi_ptr = unsafe { <FFIMapStrVal as FFIConversionTo<BTreeMap<String, Value>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIMapStrVal as FFIConversionFrom<BTreeMap<String, Value>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// IndexMap<String, String> with unicode content
#[test]
fn indexmap_string_string_unicode_roundtrip_and_free() {
    use example_nested::fermented::generics::indexmap_IndexMap_String_String as FFIIndexMapStrStr;
    use indexmap::IndexMap;
    let mut original = IndexMap::new();
    original.insert("ключ".to_string(), "значение".to_string());
    original.insert("emoji".to_string(), "🙂🚀".to_string());
    let ffi_ptr = unsafe { <FFIIndexMapStrStr as FFIConversionTo<IndexMap<String, String>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIIndexMapStrStr as FFIConversionFrom<IndexMap<String, String>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// BTreeMap<String, std::time::Duration>
#[test]
fn btreemap_string_duration_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_Map_keys_String_values_std_time_Duration as FFIMapStrDur;
    use std::collections::BTreeMap;
    use std::time::Duration;

    let mut original = BTreeMap::new();
    original.insert("short".to_string(), Duration::new(1, 0));
    original.insert("long".to_string(), Duration::new(2, 500));

    let ffi_ptr = unsafe { <FFIMapStrDur as FFIConversionTo<BTreeMap<String, Duration>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIMapStrDur as FFIConversionFrom<BTreeMap<String, Duration>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// indexmap::IndexMap<String, String>
#[test]
fn indexmap_string_string_roundtrip_and_free() {
    use example_nested::fermented::generics::indexmap_IndexMap_String_String as FFIIndexMapStrStr;
    use indexmap::IndexMap;

    let mut original = IndexMap::new();
    original.insert("k1".to_string(), "v1".to_string());
    original.insert("k2".to_string(), "v2".to_string());

    let ffi_ptr = unsafe { <FFIIndexMapStrStr as FFIConversionTo<IndexMap<String, String>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIIndexMapStrStr as FFIConversionFrom<IndexMap<String, String>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// BTreeMap<String, platform_value::Value> (values may be nested e.g. Array)
#[test]
fn btreemap_string_platform_value_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_Map_keys_String_values_platform_value_Value as FFIMapStrVal;
    use std::collections::BTreeMap;
    use platform_value::Value;

    let mut original = BTreeMap::new();
    original.insert("a".to_string(), Value::U8(1));
    original.insert("b".to_string(), Value::Text("txt".into()));
    original.insert("c".to_string(), Value::Array(vec![Value::Bool(true), Value::Null]));

    let ffi_ptr = unsafe { <FFIMapStrVal as FFIConversionTo<BTreeMap<String, Value>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());

    let decoded = unsafe { <FFIMapStrVal as FFIConversionFrom<BTreeMap<String, Value>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);

    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
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


// IndexMap<String, String> empty
#[test]
fn indexmap_string_string_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::indexmap_IndexMap_String_String as FFIIndexMapStrStr;
    use indexmap::IndexMap;
    let original: IndexMap<String, String> = IndexMap::new();
    let ffi = unsafe { <FFIIndexMapStrStr as FFIConversionTo<IndexMap<String, String>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIIndexMapStrStr as FFIConversionFrom<IndexMap<String, String>>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}
