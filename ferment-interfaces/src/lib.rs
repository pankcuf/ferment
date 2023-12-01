pub mod fermented;

use std::collections::{BTreeMap, HashMap};
use std::ffi::{CStr, CString};
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;
use std::ptr::NonNull;

pub trait FFIConversion<T> {
    /// # Safety
    unsafe fn ffi_from_const(ffi: *const Self) -> T;
    /// # Safety
    unsafe fn ffi_to_const(obj: T) -> *const Self;
    /// # Safety
    unsafe fn ffi_from(ffi: *mut Self) -> T {
        Self::ffi_from_const(ffi)
    }
    /// # Safety
    unsafe fn ffi_to(obj: T) -> *mut Self {
        Self::ffi_to_const(obj) as *mut _
    }
    /// # Safety
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null()).then_some(<Self as FFIConversion<T>>::ffi_from(ffi))
    }
    /// # Safety
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        obj.map_or(NonNull::<Self>::dangling().as_ptr(), |o| <Self as FFIConversion<T>>::ffi_to(o))
    }
    /// # Safety
    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_any(ffi);
    }
}

impl FFIConversion<String> for c_char {
    unsafe fn ffi_from_const(ffi: *const Self) -> String {
        CStr::from_ptr(ffi).to_str().unwrap().to_string()
    }

    unsafe fn ffi_to_const(obj: String) -> *const Self {
        let s = CString::new(obj).unwrap();
        s.as_ptr()
    }

    unsafe fn ffi_from(ffi: *mut Self) -> String {
        Self::ffi_from_const(ffi as *const _)
    }

    unsafe fn ffi_to(obj: String) -> *mut Self {
        CString::new(obj).unwrap().into_raw()
    }

    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_string(ffi);
    }
}

impl FFIConversion<&str> for c_char {
    unsafe fn ffi_from_const(ffi: *const Self) -> &'static str {
        CStr::from_ptr(ffi).to_str().unwrap()
    }

    unsafe fn ffi_to_const(obj: &str) -> *const Self {
        let s = CString::new(obj).unwrap();
        s.as_ptr()
    }

    unsafe fn ffi_from(ffi: *mut Self) -> &'static str {
        Self::ffi_from_const(ffi)
    }

    unsafe fn ffi_to(obj: &str) -> *mut Self {
        CString::new(obj).unwrap().into_raw()
    }

    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_string(ffi);
    }
}

pub fn boxed<T>(obj: T) -> *mut T {
    Box::into_raw(Box::new(obj))
}

pub fn boxed_vec<T>(vec: Vec<T>) -> *mut T {
    let mut slice = vec.into_boxed_slice();
    let ptr = slice.as_mut_ptr();
    mem::forget(slice);
    ptr
}

/// # Safety
pub unsafe fn unbox_any<T: ?Sized>(any: *mut T) -> Box<T> {
    Box::from_raw(any)
}

/// # Safety
pub unsafe fn unbox_string(data: *mut c_char) {
    let _ = CString::from_raw(data);
}

/// # Safety
pub unsafe fn unbox_vec<T>(vec: Vec<*mut T>) -> Vec<Box<T>> {
    vec.iter().map(|&x| unbox_any(x)).collect()
}

/// # Safety
pub unsafe fn unbox_any_vec<T>(vec: Vec<*mut T>) {
    for &x in vec.iter() {
        unbox_any(x);
    }
}

/// # Safety
pub unsafe fn unbox_any_vec_ptr<T>(ptr: *mut *mut T, count: usize) {
    unbox_any_vec(unbox_vec_ptr(ptr, count));
}

/// # Safety
pub unsafe fn unbox_vec_ptr<T>(ptr: *mut T, count: usize) -> Vec<T> {
    Vec::from_raw_parts(ptr, count, count)
}

pub fn convert_vec_to_fixed_array<const N: usize>(data: &[u8]) -> *mut [u8; N] {
    let mut fixed_array = [0u8; N];
    fixed_array.copy_from_slice(data);
    boxed(fixed_array)
}


/// Vec conversions
/// # Safety
pub unsafe fn from_simple_vec<N: Clone>(vec: *const N, count: usize) -> Vec<N> {
    std::slice::from_raw_parts(vec, count).to_vec()
}

/// # Safety
pub unsafe fn from_complex_vec<N, V: FFIConversion<N>>(vec: *mut *mut V, count: usize) -> Vec<N> {
    (0..count)
        .map(|i| FFIConversion::ffi_from_const(*vec.add(i)))
        .collect()
}
/// # Safety
pub unsafe fn to_simple_vec<O>(obj: Vec<O>) -> *mut O
    where Vec<*mut O>: FromIterator<*mut O> {
    boxed_vec(obj)
}
/// # Safety
pub unsafe fn to_complex_vec<O, FFI>(obj: Vec<O>) -> *mut *mut FFI
    where
        FFI: FFIConversion<O>,
        Vec<*mut FFI>: FromIterator<*mut O> {
    boxed_vec(obj.into_iter()
        .map(|o| <FFI as FFIConversion<O>>::ffi_to(o))
        .collect::<Vec<*mut FFI>>())
}
// /// Result conversions
// /// # Safety
// pub unsafe fn from_simple_result<T, V>(ok: T, error: V) -> Result<T, V> {
//     fold_to_result(ok, error, |o| o, |o| o)
// }
//
// /// # Safety
// pub unsafe fn from_simple_complex_result<T, V, V2>(ok: T, error: *mut V2) -> Result<T, V>
//     where V2: FFIConversion<V> {
//     fold_to_result(ok, error, |o| o, |o| FFIConversion::ffi_from(o))
// }
// /// # Safety
// pub unsafe fn from_complex_simple_result<T, V, T2>(ok: *mut T2, error: V) -> Result<T, V>
//     where T2: FFIConversion<T> {
//     fold_to_result(ok, error, |o| FFIConversion::ffi_from(o), |o| o)
// }
// /// # Safety
// pub unsafe fn from_complex_result<T, V, T2, V2>(ok: *mut T2, error: *mut V2) -> Result<T, V>
//     where T2: FFIConversion<T>, V2: FFIConversion<V> {
//     fold_to_result(ok, error, |o| FFIConversion::ffi_from(o), |o| FFIConversion::ffi_from(o))
// }


/// Map conversions
/// # Safety
pub unsafe fn from_simple_map<M, K, V>(count: usize, keys: *mut K, values: *mut V) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Copy + Ord,
        V: Copy + Ord {
    fold_to_map(count, keys, values, |o| o, |o| o)
}

/// # Safety
pub unsafe fn from_simple_complex_map<M, K, V, V2>(count: usize, keys: *mut K, values: *mut *mut V2) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Copy + Ord,
        V: Ord,
        V2: FFIConversion<V> {
    fold_to_map(count, keys, values, |o| o, |o| FFIConversion::ffi_from(o))
}

/// # Safety
pub unsafe fn from_complex_simple_map<M, K, V, K2>(count: usize, keys: *mut *mut K2, values: *mut V) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        V: Copy,
        K2: FFIConversion<K> {
    fold_to_map(count, keys, values, |o| FFIConversion::ffi_from(o), |o| o)
}

/// # Safety
pub unsafe fn from_complex_map<M, K, V, K2, V2>(count: usize, keys: *mut *mut K2, values: *mut *mut V2) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Ord,
        V: Ord,
        K2: FFIConversion<K>,
        V2: FFIConversion<V> {
    fold_to_map(count, keys, values, |o| FFIConversion::ffi_from(o), |o| FFIConversion::ffi_from(o))
}

/// # Safety
pub unsafe fn fold_to_btree_map<K: Copy, V: Copy, K2: Ord, V2>(count: usize, keys: *mut K, values: *mut V, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> BTreeMap<K2, V2> {
    (0..count).fold(BTreeMap::new(), |mut acc, i| {
        let key = key_converter(*keys.add(i));
        let value = value_converter(*values.add(i));
        acc.insert(key, value);
        acc
    })
}
/// We pass here main context of parent program

pub type OpaqueContext = *const std::os::raw::c_void;

pub type OpaqueContextMut = *mut std::os::raw::c_void;

// No Drop implementation for them

pub trait FFIVecConversion {
    type Value;
    /// # Safety
    unsafe fn decode(&self) -> Vec<Self::Value>;
    /// # Safety
    unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self;
}


pub trait FFIMapConversion {
    type Key;
    type Value;
    fn new() -> Self;
    fn insert(&mut self, key: Self::Key, value: Self::Value);
}

impl<K: Ord, V> FFIMapConversion for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { BTreeMap::new() }
    fn insert(&mut self, key: K, value: V) { BTreeMap::insert(self, key, value); }
}

impl<K: Hash + Eq, V> FFIMapConversion for HashMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { HashMap::new() }
    fn insert(&mut self, key: K, value: V) { HashMap::insert(self, key, value); }
}

// pub trait FFIResultConversion {
//     type Ok;
//     type Error;
//     fn new(ok: Self::Ok, error: Self::Error) -> Self;
// }
// pub unsafe fn fold_to_result<T, V, T2, V2>(
//     ok: T,
//     error: V,
//     ok_converter: impl Fn(T) -> T2,
//     error_converter: impl Fn(V) -> V2)
//     -> Result<T2, V2> {
//     let t = ok_converter(ok);
//     let v = error_converter(error);
//
//     M::new(key_converter(ok), value_converter(error))
// }

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

pub fn simple_vec_iterator<T, F>(iter: impl Iterator<Item=T>) -> *mut T
    where F: Fn(T) -> T {
    boxed_vec(iter.collect::<Vec<_>>())
}

/// # Safety
pub unsafe fn complex_vec_iterator<T, U>(iter: impl Iterator<Item=T>) -> *mut *mut U
    where U: FFIConversion<T> {
    boxed_vec(iter.map(|o| <U as FFIConversion<T>>::ffi_to(o)).collect())
}

#[macro_export]
macro_rules! vec_ffi_struct {
    ($name:ident, $t:ty) => {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            pub count: usize,
            pub values: *mut $t,
        }
    }
}

#[macro_export]
macro_rules! map_ffi_struct {
    ($name:ident, $k:ty, $v:ty) => {
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            pub count: usize,
            pub keys: *mut $k,
            pub values: *mut $v,
        }
    }
}


#[macro_export]
macro_rules! ffi_conversion_interface_ffi_vec_conversion {
    ($name:ident, $t:ty) => {
        impl ferment_interfaces::FFIConversion<Vec<$t>> for $name {
            unsafe fn ffi_from_const(ffi: *const $name) -> Vec<$t> {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIVecConversion::decode(ffi_ref)
            }
            unsafe fn ffi_to_const(obj: Vec<$t>) -> *const $name {
                ferment_interfaces::FFIVecConversion::encode(obj)
            }
            unsafe fn destroy(ffi: *mut $name) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_conversion_interface_ffi_map_conversion {
    ($name:ident, $map:ty, $from:block, $to:block) => {
        impl ferment_interfaces::FFIConversion<$map> for $name {
            unsafe fn ffi_from_const(ffi: *const $name) -> $map {
                $from
            }
            unsafe fn ffi_to_const(obj: $map) -> *const $name {
                $to
            }
            unsafe fn destroy(ffi: *mut $name) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_conversion_interface_ffi_hash_map_conversion {
    ($name:ident, $k:ty, $v:ty, $from: block, $to: block) => {
        ffi_conversion_interface_ffi_map_conversion!($name, HashMap<$k, $v>, $from, $to);
    }
}

#[macro_export]
macro_rules! ffi_vec_conversion {
    ($name:ident, $t:ty, $decode:block, $encode:block) => {
        impl ferment_interfaces::FFIVecConversion for $name {
            type Value = $t;
            unsafe fn decode(&self) -> Vec<Self::Value> { $decode }
            unsafe fn encode(obj: Vec<Self::Value>) -> *mut Self { $encode }
        }
    }
}

#[macro_export]
macro_rules! ffi_vec_simple_conversion {
    ($name:ident, $t:ty) => {
        ffi_vec_conversion!(
            $name,
            $t,
            { ferment_interfaces::from_simple_vec(self.values as *const Self::Value, self.count) },
            { ferment_interfaces::boxed(Self { count: obj.len(), values: ferment_interfaces::boxed_vec(obj) }) }
        );
    }
}

#[macro_export]
macro_rules! ffi_vec_complex_conversion {
    ($name:ident, $t:ty) => {
        ffi_vec_conversion!(
            $name,
            $t, {
                let count = self.count;
                let values = self.values;
                (0..count)
                    .map(|i| ferment_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            }, {
                ferment_interfaces::boxed(Self { count: obj.len(), values: ferment_interfaces::complex_vec_iterator(obj.into_iter()) })
            }
        );
    }
}

#[macro_export]
macro_rules! ffi_drop_interface {
    ($name:ident, $drop_code:block) => {
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $drop_code
                }
            }
        }
    }
}


#[macro_export]
macro_rules! vec_ffi_simple_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! vec_ffi_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_simple_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_simple_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_complex_simple_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
        });
    }
}


#[macro_export]
macro_rules! vec_ffi_simple_expansion {
    ($name:ident, $t:ty) => {
        vec_ffi_struct!($name, $t);
        ffi_conversion_interface_ffi_vec_conversion!($name, $t);
        ffi_vec_simple_conversion!($name, $t);
        vec_ffi_simple_drop_interface!($name);
    }
}
#[macro_export]
macro_rules! vec_ffi_complex_expansion {
    ($name:ident, $t:ty) => {
        vec_ffi_struct!($name, *mut $t);
        ffi_conversion_interface_ffi_vec_conversion!($name, $t);
        ffi_vec_complex_conversion!($name, $t);
        vec_ffi_complex_drop_interface!($name);
    }
}

#[macro_export]
macro_rules! map_ffi_to {
    ($keys_method:ident, $values_method:ident) => {
        ferment_interfaces::boxed(Self {
            count: obj.len(),
            keys: ferment_interfaces::$keys_method(obj.into_keys()),
            values: ferment_interfaces::$values_method(obj.into_values())
        })
    }
}

#[macro_export]
macro_rules! map_ffi_from {
    ($method:ident) => {
        let ffi_ref = &*ffi;
        ferment_interfaces::$method(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
    }
}

#[macro_export]
macro_rules! map_ffi_simple_expansion {
    ($name:ident, $map:ty, $k:ty, $v:ty) => {
        map_ffi_struct!($name, $k, $v);
        map_ffi_simple_drop_interface!($name);
        ffi_conversion_interface_ffi_map_conversion!(
            $name,
            $map, {
                let ffi_ref = &*ffi;
                ferment_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            }, {
                map_ffi_to!(simple_vec_iterator, simple_vec_iterator)
            }
        );
    }
}



#[macro_export]
macro_rules! map_ffi_simple_complex_expansion {
    ($name:ident, $map:ty, $k:ty, $v:ty) => {
        map_ffi_struct!($name, $k, *mut $v);
        map_ffi_simple_complex_drop_interface!($name);
        ffi_conversion_interface_ffi_map_conversion!(
            $name,
            $map,
            {
                let ffi_ref = &*ffi;
                ferment_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            },
            map_ffi_to!(simple_vec_iterator, complex_vec_iterator)
        );
    }
}

#[macro_export]
macro_rules! map_ffi_complex_simple_expansion {
    ($name:ident, $map:ty, $k:ty, $v:ty) => {
        map_ffi_struct!($name, *mut $k, $v);
        map_ffi_complex_simple_drop_interface!($name);
        ffi_conversion_interface_ffi_map_conversion!(
            $name,
            $map,
            {
                let ffi_ref = &*ffi;
                ferment_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            },
            { map_ffi_to!(complex_vec_iterator, simple_vec_iterator) }
        );
    }
}

#[macro_export]
macro_rules! map_ffi_complex_expansion {
    ($name:ident, $map:ty, $k:ty, $v:ty) => {
        map_ffi_struct!($name, *mut $k, *mut $v);
        map_ffi_complex_drop_interface!($name);
        ffi_conversion_interface_ffi_map_conversion!(
            $name,
            $map,
            {
                let ffi_ref = &*ffi;
                ferment_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            },
            { map_ffi_to!(complex_vec_iterator, complex_vec_iterator) }
        );
    }
}
