pub mod fermented;

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;

/// We pass here main context of parent program
///
pub trait FFIConversionFrom<T> {
    /// # Safety
    unsafe fn ffi_from_const(ffi: *const Self) -> T;
    /// # Safety
    unsafe fn ffi_from(ffi: *mut Self) -> T {
        Self::ffi_from_const(ffi.cast_const())
    }
    /// # Safety
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null())
            .then(|| Self::ffi_from(ffi))
    }
}

pub trait FFIConversionTo<T> {
    /// # Safety
    unsafe fn ffi_to_const(obj: T) -> *const Self;
    /// # Safety
    unsafe fn ffi_to(obj: T) -> *mut Self {
        Self::ffi_to_const(obj)
            .cast_mut()
    }
    /// # Safety
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        if let Some(o) = obj {
            Self::ffi_to(o)
        } else {
            std::ptr::null_mut()
        }
    }
}
pub fn black_hole<T>(_obj: T) {}
pub fn bypass<T>(obj: T) -> T { obj }

pub fn boxed<T>(obj: T) -> *mut T {
    Box::into_raw(Box::new(obj))
}
pub fn boxed_vec<T>(vec: Vec<T>) -> *mut T {
    let mut slice = vec.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    mem::forget(slice);
    ptr
}
pub fn boxed_arr<const N: usize, T: Clone>(arr: [T; N]) -> *mut T {
    boxed_vec(arr.to_vec())
}
pub fn boxed_slice<T: Clone>(slice: &[T]) -> *mut T {
    boxed_vec(slice.to_vec())
}

/// # Safety
pub unsafe fn unbox_any<T: ?Sized>(any: *mut T) {
    if !any.is_null() {
        let _ = Box::from_raw(any);
    }
}
pub unsafe fn unbox_any_opt<T: ?Sized>(any: *mut T) {
    if !any.is_null() {
        let _ = Box::from_raw(any);
    }
}

/// # Safety
pub unsafe fn unbox_string(data: *mut c_char) {
    if data.is_null() {
        return;
    }
    let _ = CString::from_raw(data);
}

/// # Safety
pub unsafe fn unbox_any_vec<T>(vec: Vec<*mut T>) {
    // TODO: that's wrong, need to make unbox_any composable of arbitrary type -> unbox_any_vec_composer
    if vec.is_empty() {
        return;
    }
    for &x in vec.iter() {
        unbox_any(x);
    }
}
/// # Safety
pub unsafe fn unbox_any_vec_composer<T, U: Fn(*mut T)>(vec: Vec<*mut T>, composer: U) {
    if vec.is_empty() {
        return;
    }
    for &x in vec.iter() {
        composer(x);
    }
}

/// # Safety
pub unsafe fn unbox_any_vec_ptr<T>(ptr: *mut *mut T, count: usize) {
    unbox_group(ptr, count, unbox_any);
}

/// # Safety
pub unsafe fn unbox_vec_ptr<T>(ptr: *mut T, count: usize) -> Vec<T> {
    Vec::from_raw_parts(ptr, count, count)
}

/// # Safety
/// Generalized function to free FFI-style arrays of either values or pointers.
/// You control how each element is freed via `item_destructor`.
pub unsafe fn unbox_group<T>(ptr: *mut T, count: usize, item_destructor: unsafe fn(T)) {
    if ptr.is_null() || count == 0 {
        return;
    }
    // SAFETY: We take ownership of the memory
    let vec_of_ptr = unbox_vec_ptr(ptr, count);
    if vec_of_ptr.is_empty() {
        return;
    }
    for item in vec_of_ptr {
        item_destructor(item);
    }
}


/// # Safety
pub unsafe fn from_opt_primitive<T: Copy>(ptr: *mut T) -> Option<T> {
    (!ptr.is_null()).then(|| *ptr)
}

/// # Safety
pub unsafe fn to_opt_primitive<T>(obj: Option<T>) -> *mut T {
    obj.map_or(std::ptr::null_mut(), |o| boxed(o))
}

/// # Safety
pub unsafe fn from_opt_opaque<T>(ptr: *mut T) -> Option<T> {
    (!ptr.is_null()).then(|| std::ptr::read(ptr))
}

/// # Safety
pub unsafe fn to_opt_opaque<T>(obj: Option<T>) -> *mut T {
    obj.map_or(std::ptr::null_mut(), |o| boxed(o))
}

/// # Safety
pub unsafe fn destroy_opt_primitive<T: Copy>(ptr: *mut T) {
    if !ptr.is_null() {
        unbox_any(ptr);
    }
}

pub trait FFIMapConversion {
    type Key;
    type Value;
    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Value);
    fn count(&self) -> usize;
}

impl<K: Ord, V> FFIMapConversion for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { BTreeMap::new() }
    fn insert(&mut self, key: K, value: V) { BTreeMap::insert(self, key, value); }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<K: Hash + Eq, V> FFIMapConversion for HashMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { HashMap::new() }
    fn insert(&mut self, key: K, value: V) { HashMap::insert(self, key, value); }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<K: Hash + Eq, V> FFIMapConversion for indexmap::IndexMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { indexmap::IndexMap::new() }
    fn insert(&mut self, key: K, value: V) { indexmap::IndexMap::insert(self, key, value); }

    fn count(&self) -> usize {
        self.len()
    }
}

impl FFIMapConversion for serde_json::Map<String, serde_json::Value> {
    type Key = String;
    type Value = serde_json::Value;

    fn new() -> Self {
        serde_json::Map::new()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) {
        serde_json::Map::insert(self, key, value);
    }

    fn count(&self) -> usize {
        self.len()
    }
}

/// # Safety
pub unsafe fn from_group<Iter, FFIType, TargetType>(
    count: usize,
    ptr: *mut FFIType,
    mut converter: impl FnMut(*mut FFIType) -> TargetType,
) -> Iter
where Iter: FromIterator<TargetType> {
    (0..count)
        .map(|i| converter(ptr.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn from_primitive_group<Iter, T: Copy>(ptr: *mut T, count: usize) -> Iter
where Iter: FromIterator<T> {
    from_group(count, ptr, |p| *p)
}
/// # Safety
pub unsafe fn from_opt_primitive_group<Iter, T: Copy>(ptr: *mut *mut T, count: usize) -> Iter
where Iter: FromIterator<Option<T>> {
    from_group(count, ptr, |p| {
        let inner = *p;
        (!inner.is_null()).then(|| *inner)
    })
}

/// # Safety
pub unsafe fn from_opt_opaque_group<Iter, T>(ptr: *mut *mut T, count: usize) -> Iter
where Iter: FromIterator<Option<T>> {
    from_group(count, ptr, |p| {
        let inner = *p;
        (!inner.is_null()).then(|| std::ptr::read(inner))
    })
}
/// # Safety
pub unsafe fn from_opaque_group<Iter, T>(ptr: *mut *mut T, count: usize) -> Iter
where Iter: FromIterator<T> {
    from_group(count, ptr, |p| std::ptr::read(*p))
}

/// # Safety
pub unsafe fn from_complex_group<Iter, T, FFI>(ptr: *mut *mut FFI, count: usize) -> Iter
where Iter: FromIterator<T>,
      FFI: FFIConversionFrom<T> {
    from_group(count, ptr, |p| FFIConversionFrom::ffi_from(*p))
}

/// # Safety
pub unsafe fn from_opt_complex_group<Iter, T, FFI>(ptr: *mut *mut FFI, count: usize) -> Iter
where
    Iter: FromIterator<Option<T>>,
    FFI: FFIConversionFrom<T>,
{
    from_group(count, ptr, |p| FFIConversionFrom::ffi_from_opt(*p))
}

/// # Safety
pub unsafe fn to_group<Iter, TargetType, FFIType>(iter: Iter, converter: fn(TargetType) -> FFIType) -> *mut FFIType
    where Iter: Iterator<Item=TargetType> {
    boxed_vec(iter.map(|o| converter(o)).collect())
}

/// # Safety
pub unsafe fn to_complex_group<Iter, TargetType, FFIType>(iter: Iter) -> *mut *mut FFIType
    where Iter: Iterator<Item=TargetType>,
          FFIType: FFIConversionTo<TargetType> {
    to_group(iter, |o| <FFIType as FFIConversionTo<TargetType>>::ffi_to(o))
}
/// # Safety
pub unsafe fn to_opt_complex_group<Iter, TargetType, FFIType>(iter: Iter) -> *mut *mut FFIType
    where Iter: Iterator<Item=Option<TargetType>>,
          FFIType: FFIConversionTo<TargetType> {
    to_group(iter, |o| <FFIType as FFIConversionTo<TargetType>>::ffi_to_opt(o))
}

/// # Safety
pub unsafe fn to_primitive_group<Iter, TargetType>(iter: Iter) -> *mut TargetType
    where Iter: Iterator<Item=TargetType>,
          TargetType: Copy {
    to_group(iter, |o| o)
}
/// # Safety
pub unsafe fn to_opt_primitive_group<Iter, TargetType>(iter: Iter) -> *mut *mut TargetType
    where Iter: Iterator<Item=Option<TargetType>>,
          Vec<*mut TargetType>: FromIterator<*mut TargetType> {
    to_group(iter, |o| o.map_or(std::ptr::null_mut(), |o| boxed(o)))
}
/// # Safety
pub unsafe fn to_opt_opaque_group<Iter, TargetType>(iter: Iter) -> *mut *mut TargetType
    where Iter: Iterator<Item=Option<TargetType>>,
          Vec<*mut TargetType>: FromIterator<*mut TargetType> {
    to_group(iter, |o| o.map_or(std::ptr::null_mut(), |o| boxed(o)))
}
/// # Safety
pub unsafe fn to_opaque_group<Iter, TargetType>(iter: Iter) -> *mut *mut TargetType
    where Iter: Iterator<Item=TargetType>,
          Vec<*mut TargetType>: FromIterator<*mut TargetType> {
    to_group(iter, |o| boxed(o))
}


pub fn to_map<TargetKey, TargetValue, FFIKey, FFIValue, Map, ToKey, ToValue>(obj: Map, key_converter: ToKey, value_converter: ToValue) -> (usize, *mut FFIKey, *mut FFIValue)
where Map: FFIMapConversion<Key=TargetKey, Value=TargetValue> + IntoIterator<Item= (TargetKey, TargetValue)>,
      ToKey: Fn(TargetKey) -> FFIKey,
      ToValue: Fn(TargetValue) -> FFIValue {
    let count = obj.count();
    let (keys, values) = obj.into_iter().map(|(key, value)| (key_converter(key), value_converter(value))).unzip();
    (count, boxed_vec(keys), boxed_vec(values))
}

/// # Safety
pub unsafe fn fold_to_map<M, K, V, K2, V2>(
    count: usize,
    keys: *mut K,
    values: *mut V,
    key_converter: impl Fn(K) -> K2,
    value_converter: impl Fn(V) -> V2) -> M
    where
        M: FFIMapConversion<Key=K2, Value=V2>,
        K: Copy,
        V: Copy {
    (0..count).fold(M::new(), |mut acc, i| {
        let key = key_converter(*keys.add(i));
        let value = value_converter(*values.add(i));
        acc.insert(key, value);
        acc
    })
}

/// # Safety
pub unsafe fn fold_to_vec<M, V: Copy, V2>(count: usize, values: *mut V, value_converter: impl Fn(V) -> V2) -> Vec<V2> {
    (0..count)
        .map(|i| value_converter(*values.add(i)))
        .collect()
}

/// # Safety
/// # Safety
/// Convert a pair of optional ok/error pointers into a `Result` using provided converters.
///
/// When both pointers are null, this prefers interpreting the value as `Err(error_converter(null))`.
/// This supports encodings like `Result<T, Option<E>>` where `Err(None)` is represented by both
/// pointers being null. If you need to prefer `Ok` on both-null (e.g. `Result<Option<T>, E>`),
/// use `fold_to_result_prefer_ok` instead.
pub unsafe fn fold_to_result<T, E, T2, E2>(
    ok: *mut T,
    ok_converter: impl Fn(*mut T) -> T2,
    error: *mut E,
    error_converter: impl Fn(*mut E) -> E2,
) -> Result<T2, E2> {
    if !error.is_null() {
        Err(error_converter(error))
    } else if !ok.is_null() {
        Ok(ok_converter(ok))
    } else {
        Err(error_converter(error))
    }
}

/// # Safety
/// Variant that prefers `Ok` when both pointers are null. Useful for
/// `Result<Option<T>, E>` encodings where `Ok(None)` is represented by a null ok pointer.
pub unsafe fn fold_to_result_prefer_ok<T, E, T2, E2>(
    ok: *mut T,
    ok_converter: impl Fn(*mut T) -> T2,
    error: *mut E,
    error_converter: impl Fn(*mut E) -> E2,
) -> Result<T2, E2> {
    if error.is_null() {
        Ok(ok_converter(ok))
    } else {
        Err(error_converter(error))
    }
}
/// # Safety-
pub unsafe fn to_result<T, E, T2, E2>(
    result: Result<T2, E2>,
    ok_converter: impl Fn(T2) -> *mut T,
    error_converter: impl Fn(E2) -> *mut E,
) -> (*mut T, *mut E) {
    match result {
        Ok(o) => (ok_converter(o), std::ptr::null_mut()),
        Err(o) => (std::ptr::null_mut(), error_converter(o))
    }
}

// /// # Safety
// pub unsafe fn ffi_from_cow<'a, T, E, T2, E2>(
//     value: *mut T, converter: impl Fn(*mut T) -> T2,
//     error: *mut E, error_converter: impl Fn(*mut E) -> E2) -> Cow<'a, E2> {
//     if error.is_null() {
//         Ok(ok_converter(ok))
//     } else {
//         Err(error_converter(error))
//     }
// }
// /// # Safety
// pub unsafe fn ffi_to_cow<T, E, T2, E2>(
//     result: Result<T2, E2>,
//     ok_converter: impl Fn(T2) -> *mut T,
//     error_converter: impl Fn(E2) -> *mut E,
// ) -> (*mut T, *mut E) {
//     match result {
//         Ok(o) => (ok_converter(o), std::ptr::null_mut()),
//         Err(o) => (std::ptr::null_mut(), error_converter(o))
//     }
// }

#[macro_export]
macro_rules! impl_custom_conversion {
    ($RustType:ty, $FFIType:ty, $from:expr, $to:expr) => {
        impl From<&$FFIType> for $RustType {
            fn from(value: &$FFIType) -> Self {
                $from(value)
            }
        }
        impl From<&$RustType> for $FFIType {
            fn from(value: &$RustType) -> Self {
                $to(value)
            }
        }

        impl ferment::FFIConversionFrom<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
        }
        impl ferment::FFIConversionTo<$RustType> for $FFIType {
            unsafe fn ffi_to_const(obj: $RustType) -> *const Self {
                ferment::boxed(<$FFIType>::from(&obj))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_custom_conversion2 {
    ($RustType:ty, $FFIType:ident { $($field_name:ident: $field_type:ty),* $(,)? }, $from:expr, $to:expr) => {
        #[allow(non_camel_case_types)]
        #[ferment_macro::register($RustType)]
        pub struct $FFIType {
            $(pub $field_name: $field_type),*
        }
        impl From<&$FFIType> for $RustType {
            fn from(value: &$FFIType) -> Self {
                $from(value)
            }
        }
        impl From<&$RustType> for $FFIType {
            fn from(value: &$RustType) -> Self {
                $to(value)
            }
        }
        impl ferment::FFIConversionFrom<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
        }
        impl ferment::FFIConversionTo<$RustType> for $FFIType {
            unsafe fn ffi_to_const(obj: $RustType) -> *const Self {
                ferment::boxed(<$FFIType>::from(&obj))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_cloneable_ferment {
    ($ty:path, $ffitype:ident) => {
        impl ferment::FFIConversionFrom<$ty> for $ffitype {
            unsafe fn ffi_from_const(ffi: *const Self) -> $ty {
                let ffi = &*ffi;
                let raw = &*ffi.0;
                raw.clone()
            }
        }
        impl ferment::FFIConversionTo<$ty> for $ffitype {
            unsafe fn ffi_to_const(obj: $ty) -> *const Self {
                ferment::boxed(Self(ferment::boxed(obj)))
            }
        }
        impl Drop for $ffitype {
            fn drop(&mut self) {
                unsafe {
                    ferment::unbox_any(self.0);
                }
            }
        }
    };
}

/// # Safety
pub unsafe fn to_arc<T: ?Sized>(obj: std::sync::Arc<T>) -> *mut T {
    std::sync::Arc::into_raw(obj).cast_mut()
}
/// # Safety
pub unsafe fn from_arc<T: ?Sized>(obj: *const T) -> std::sync::Arc<T> {
    std::sync::Arc::from_raw(obj)
}
