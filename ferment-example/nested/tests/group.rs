#![allow(clippy::not_unsafe_ptr_arg_deref)]

use ferment::{FFIConversionFrom, FFIConversionTo};

// Vec<Vec<u8>>
#[test]
fn vec_vec_u8_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Vec_u8 as FFIVecVecU8;

    let original: Vec<Vec<u8>> = vec![vec![1, 2], vec![3, 4, 5]];
    let ffi_ptr = unsafe { <FFIVecVecU8 as FFIConversionTo<Vec<Vec<u8>>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecVecU8 as FFIConversionFrom<Vec<Vec<u8>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<[u8; 32]>
#[test]
fn vec_arr_u8_32_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_u8_32 as FFIVecArr32;

    let original: Vec<[u8; 32]> = vec![[7u8; 32], [9u8; 32]];
    let ffi_ptr = unsafe { <FFIVecArr32 as FFIConversionTo<Vec<[u8; 32]>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecArr32 as FFIConversionFrom<Vec<[u8; 32]>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<(String, [u8; 32])>
#[test]
fn vec_tuple_string_arr32_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Tuple_String_Arr_u8_32 as FFIVecTuple;

    let original: Vec<(String, [u8; 32])> = vec![
        ("a".into(), [1u8; 32]),
        ("b".into(), [2u8; 32]),
    ];
    let ffi_ptr = unsafe { <FFIVecTuple as FFIConversionTo<Vec<(String, [u8; 32])>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecTuple as FFIConversionFrom<Vec<(String, [u8; 32])>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// BTreeSet<Option<u32>>
#[test]
fn btreeset_option_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_BTreeSet_Option_u32 as FFIBTreeSetOptU32;
    use std::collections::BTreeSet;

    let mut original = BTreeSet::new();
    original.insert(None);
    original.insert(Some(1));
    original.insert(Some(2));

    let ffi_ptr = unsafe { <FFIBTreeSetOptU32 as FFIConversionTo<BTreeSet<Option<u32>>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIBTreeSetOptU32 as FFIConversionFrom<BTreeSet<Option<u32>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// HashSet<u32>
#[test]
fn hashset_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_HashSet_u32 as FFIHashSetU32;
    use std::collections::HashSet;

    let mut original = HashSet::new();
    original.insert(10);
    original.insert(20);

    let ffi_ptr = unsafe { <FFIHashSetU32 as FFIConversionTo<HashSet<u32>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIHashSetU32 as FFIConversionFrom<HashSet<u32>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// HashSet<Option<Vec<u8>>>
#[test]
fn hashset_option_vec_u8_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_HashSet_Option_Vec_u8 as FFIHashSetOptVecU8;
    use std::collections::HashSet;

    let mut original: HashSet<Option<Vec<u8>>> = HashSet::new();
    original.insert(None);
    original.insert(Some(vec![1, 2, 3]));

    let ffi_ptr = unsafe { <FFIHashSetOptVecU8 as FFIConversionTo<HashSet<Option<Vec<u8>>>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIHashSetOptVecU8 as FFIConversionFrom<HashSet<Option<Vec<u8>>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<Option<u32>>
#[test]
fn vec_option_u32_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Option_u32 as FFIVecOptU32;

    let original: Vec<Option<u32>> = vec![None, Some(5), Some(10)];
    let ffi_ptr = unsafe { <FFIVecOptU32 as FFIConversionTo<Vec<Option<u32>>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecOptU32 as FFIConversionFrom<Vec<Option<u32>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<i32>
#[test]
fn vec_i32_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_i32 as FFIVecI32;

    let original: Vec<i32> = vec![1, -2, 3, -4];
    let ffi_ptr = unsafe { <FFIVecI32 as FFIConversionTo<Vec<i32>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecI32 as FFIConversionFrom<Vec<i32>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<u8>
#[test]
fn vec_u8_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_u8 as FFIVecU8;

    let original: Vec<u8> = vec![10, 20, 30, 40, 50];
    let ffi_ptr = unsafe { <FFIVecU8 as FFIConversionTo<Vec<u8>>>::ffi_to_const(original.clone()) };
    assert!(!ffi_ptr.is_null());
    let decoded = unsafe { <FFIVecU8 as FFIConversionFrom<Vec<u8>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<(String, [u8; 32])> empty
#[test]
fn vec_tuple_string_arr32_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Tuple_String_Arr_u8_32 as FFIVecTuple;
    let original: Vec<(String, [u8; 32])> = vec![];
    let ffi_ptr = unsafe { <FFIVecTuple as FFIConversionTo<Vec<(String, [u8; 32])>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIVecTuple as FFIConversionFrom<Vec<(String, [u8; 32])>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<platform_value::Value> empty
#[test]
fn vec_platform_value_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_platform_value_Value as FFIVecVal;
    use platform_value::Value;
    let original: Vec<Value> = vec![];
    let ffi_ptr = unsafe { <FFIVecVal as FFIConversionTo<Vec<Value>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIVecVal as FFIConversionFrom<Vec<Value>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// Vec<Option<u32>> all None
#[test]
fn vec_option_u32_all_none_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Option_u32 as FFIVecOptU32;
    let original: Vec<Option<u32>> = vec![None, None, None];
    let ffi_ptr = unsafe { <FFIVecOptU32 as FFIConversionTo<Vec<Option<u32>>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIVecOptU32 as FFIConversionFrom<Vec<Option<u32>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
}

// HashSet<Option<Vec<u8>>> empty
#[test]
fn hashset_option_vec_u8_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_HashSet_Option_Vec_u8 as FFIHashSetOptVecU8;
    use std::collections::HashSet;
    let original: HashSet<Option<Vec<u8>>> = HashSet::new();
    let ffi_ptr = unsafe { <FFIHashSetOptVecU8 as FFIConversionTo<HashSet<Option<Vec<u8>>>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIHashSetOptVecU8 as FFIConversionFrom<HashSet<Option<Vec<u8>>>>>::ffi_from_const(ffi_ptr) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi_ptr.cast_mut()) };
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

// Vec<String> empty
#[test]
fn vec_string_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_String as FFIVecString;
    let original: Vec<String> = vec![];
    let ffi = unsafe { <FFIVecString as FFIConversionTo<Vec<String>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIVecString as FFIConversionFrom<Vec<String>>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}

// HashSet<u32> empty
#[test]
fn hashset_u32_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_HashSet_u32 as FFIHashSetU32;
    use std::collections::HashSet;
    let original: HashSet<u32> = HashSet::new();
    let ffi = unsafe { <FFIHashSetU32 as FFIConversionTo<HashSet<u32>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIHashSetU32 as FFIConversionFrom<HashSet<u32>>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}

// BTreeSet<Option<u32>> empty
#[test]
fn btreeset_option_u32_empty_roundtrip_and_free() {
    use example_nested::fermented::generics::std_collections_BTreeSet_Option_u32 as FFIBTreeSetOptU32;
    use std::collections::BTreeSet;
    let original: BTreeSet<Option<u32>> = BTreeSet::new();
    let ffi = unsafe { <FFIBTreeSetOptU32 as FFIConversionTo<BTreeSet<Option<u32>>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIBTreeSetOptU32 as FFIConversionFrom<BTreeSet<Option<u32>>>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}


// Vec<Vec<u8>> empty inner vectors included
#[test]
fn vec_vec_u8_with_empty_inner_roundtrip_and_free() {
    use example_nested::fermented::generics::Vec_Vec_u8 as FFIVecVecU8;
    let original: Vec<Vec<u8>> = vec![vec![], vec![1, 2], vec![]];
    let ffi = unsafe { <FFIVecVecU8 as FFIConversionTo<Vec<Vec<u8>>>>::ffi_to_const(original.clone()) };
    let decoded = unsafe { <FFIVecVecU8 as FFIConversionFrom<Vec<Vec<u8>>>>::ffi_from_const(ffi) };
    assert_eq!(decoded, original);
    unsafe { ferment::unbox_any(ffi.cast_mut()) };
}

