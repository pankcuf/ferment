use std::collections::{BTreeMap, HashMap};
// use std::collections::{BTreeMap, HashMap};
use std::ffi::{CStr, CString};
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;
use std::ptr::NonNull;

pub trait FFIConversion<T> {
    unsafe fn ffi_from_const(ffi: *const Self) -> T;
    unsafe fn ffi_to_const(obj: T) -> *const Self;
    unsafe fn ffi_from(ffi: *mut Self) -> T {
        Self::ffi_from_const(ffi)
    }
    unsafe fn ffi_to(obj: T) -> *mut Self {
        Self::ffi_to_const(obj) as *mut _
    }
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null()).then_some(<Self as FFIConversion<T>>::ffi_from(ffi))
    }
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        obj.map_or(NonNull::<Self>::dangling().as_ptr(), |o| <Self as FFIConversion<T>>::ffi_to(o))
    }
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

// pub fn boxed_vec_iterator<T, I: IntoIterator<Item = T>>(vec: I) -> *mut T {
//     vec.into_iter().into
//     let mut slice = vec.into_boxed_slice();
//     let ptr = slice.as_mut_ptr();
//     mem::forget(slice);
//     ptr
// }

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

// /// # Safety
// pub unsafe fn unbox_simple_vec<T>(vec: VecFFI<T>) {
//     let mem = unbox_vec_ptr(vec.values, vec.count);
//     drop(mem)
// }
//
// pub unsafe fn unbox_vec_ffi<T>(vec: *mut VecFFI<T>) {
//     let vec_ffi = unbox_any(vec);
//     let _reconstructed_vec = unbox_vec_ptr(vec_ffi.values, vec_ffi.count);
// }


pub fn convert_vec_to_fixed_array<const N: usize>(data: &Vec<u8>) -> *mut [u8; N] {
    let mut fixed_array = [0u8; N];
    fixed_array.copy_from_slice(data);
    boxed(fixed_array)
}

pub unsafe fn from_simple_vec<N: Clone>(vec: *const N, count: usize) -> Vec<N> {
    std::slice::from_raw_parts(vec, count).to_vec()
}

pub unsafe fn from_complex_vec<N, V: FFIConversion<N>>(vec: *mut *mut V, count: usize) -> Vec<N> {
    (0..count)
        .map(|i| FFIConversion::ffi_from_const(*vec.add(i)))
        .collect()
}

pub unsafe fn to_simple_vec<O>(obj: Vec<O>) -> *mut O
    where Vec<*mut O>: FromIterator<*mut O> {
    boxed_vec(obj)
}

pub unsafe fn to_complex_vec<O, FFI>(obj: Vec<O>) -> *mut *mut FFI
    where
        FFI: FFIConversion<O>,
        Vec<*mut FFI>: FromIterator<*mut O> {
    boxed_vec(obj.into_iter()
        .map(|o| <FFI as FFIConversion<O>>::ffi_to(o))
        .collect::<Vec<*mut FFI>>())
}



pub unsafe fn from_simple_map<M, K, V>(count: usize, keys: *mut K, values: *mut V) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Copy + Ord,
        V: Copy + Ord {
    fold_to_map(count, keys, values, |o| o, |o| o)
}

pub unsafe fn from_simple_complex_map<M, K, V, V2>(count: usize, keys: *mut K, values: *mut *mut V2) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Copy + Ord,
        V: Ord,
        V2: FFIConversion<V> {
    fold_to_map(count, keys, values, |o| o, |o| FFIConversion::ffi_from(o))
}

pub unsafe fn from_complex_simple_map<M, K, V, K2>(count: usize, keys: *mut *mut K2, values: *mut V) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        V: Copy,
        K2: FFIConversion<K> {
    fold_to_map(count, keys, values, |o| FFIConversion::ffi_from(o), |o| o)
}

pub unsafe fn from_complex_map<M, K, V, K2, V2>(count: usize, keys: *mut *mut K2, values: *mut *mut V2) -> M
    where
        M: FFIMapConversion<Key=K, Value=V>,
        K: Ord,
        V: Ord,
        K2: FFIConversion<K>,
        V2: FFIConversion<V> {
    fold_to_map(count, keys, values, |o| FFIConversion::ffi_from(o), |o| FFIConversion::ffi_from(o))
}

pub unsafe fn fold_to_btree_map<K: Copy, V: Copy, K2: Ord, V2>(count: usize, keys: *mut K, values: *mut V, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> BTreeMap<K2, V2> {
    (0..count).fold(BTreeMap::new(), |mut acc, i| {
        let key = key_converter(*keys.add(i));
        let value = value_converter(*values.add(i));
        acc.insert(key, value);
        acc
    })
}


// pub fn to_complex_vec<N, V: FFIConversion<N>>(obj: Vec<V>) -> *mut *mut V {
    // boxed(V::from { count: obj.len(), values: boxed_complex_vec(obj) })
// }

// pub fn boxed_complex_vec<N, V>(obj: Vec<N>) -> *mut *mut V where Vec<*mut V>: FromIterator<*mut N> {
//     // where
//         // V: FFIConversion<N>,
//         // Vec<*mut V>: FromIterator<*mut N> {
//
// }
// fn fold<B, F>(mut self, init: B, mut f: F) -> B
//     where
//         Self: Sized,
//         F: FnMut(B, Self::Item) -> B,
// {

// pub unsafe fn from_simple_btree_map<B, F, N, K, V>(init: B) -> std::collections::BTreeMap<K, V>
//     where F: FnMut(B, std::collections::BTreeMap::Item) -> B {
//     fol
// }
//
// #[repr(C)]
// #[derive(Clone, Debug)]
// pub struct MapFFI<K, V> {
//     pub count: usize,
//     pub keys: *mut K,
//     pub values: *mut V,
// }
//
// impl<K, V> Drop for MapFFI<K, V> {
//     fn drop(&mut self) {
//         // TODO: Probably it needs to avoid drop for VecFFI and use chain of unbox_any_vec instead
//         unsafe {
//             // for complex maps:
//             // unbox_any_vec(unbox_vec_ptr(self.keys, self.count));
//             // unbox_any_vec(unbox_vec_ptr(self.values, self.count));
//             // for simple maps:
//             unbox_vec_ptr(self.keys, self.count);
//             unbox_vec_ptr(self.values, self.count);
//         }
//     }
// }
//
// impl<K, V> MapFFI<K, V> where K: Copy, V: Copy  {
//     pub unsafe fn fold_to_btree_map<K2: Ord, V2>(self, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> std::collections::BTreeMap<K2, V2> {
//         (0..self.count).fold(std::collections::BTreeMap::new(), |mut acc, i| {
//             let key = key_converter(*self.keys.add(i));
//             let value = value_converter(*self.values.add(i));
//             acc.insert(key, value);
//             acc
//         })
//     }
// }
//
// impl<K, V> MapFFI<K, V> where K: Copy, V: Copy  {
//     pub unsafe fn fold_to_hash_map<K2: Hash + PartialEq + Eq, V2>(self, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> std::collections::HashMap<K2, V2> {
//         (0..self.count).fold(std::collections::HashMap::new(), |mut acc, i| {
//             let key = key_converter(*self.keys.add(i));
//             let value = value_converter(*self.values.add(i));
//             acc.insert(key, value);
//             acc
//         })
//     }
// }

// pub unsafe fn fold_to_btree_map<K, V, K2: Ord, V2>(count: usize, keys: *mut K, values: *mut V, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> BTreeMap<K2, V2> {
//     (0..count).fold(BTreeMap::new(), |mut acc, i| {
//         let key = key_converter(*keys.add(i));
//         let value = value_converter(*values.add(i));
//         acc.insert(key, value);
//         acc
//     })
// }


// #[repr(C)]
// #[derive(Clone, Debug)]
// pub struct VecFFI<V> {
//     pub count: usize,
//     pub values: *mut V,
// }
//
// impl<V> VecFFI<V> {
//     pub fn new(vec: Vec<V>) -> VecFFI<V> {
//         Self { count: vec.len(), values: boxed_vec(vec) }
//     }
// }
//
// impl<V> Drop for VecFFI<V> {
//     fn drop(&mut self) {
//         // TODO: Probably it needs to avoid drop for VecFFI and use chain of unbox_any_vec instead
//         unsafe {
//             // for complex vecs:
//             // unbox_any_vec(unbox_vec_ptr(self.values, self.count));
//             // for simple vecs:
//             unbox_vec_ptr(self.values, self.count);
//         }
//     }
// }


/// We pass here main context of parent program

pub type OpaqueContext = *const std::os::raw::c_void;
pub type OpaqueContextFFI = OpaqueContext;

pub type OpaqueContextMut = *mut std::os::raw::c_void;
pub type OpaqueContextMutFFI = OpaqueContextMut;

impl FFIConversion<OpaqueContextFFI> for OpaqueContext {
    unsafe fn ffi_from_const(ffi: *const Self) -> OpaqueContextFFI {
        *ffi
    }

    unsafe fn ffi_to_const(obj: OpaqueContextFFI) -> *const Self {
        obj as *const _
    }

    unsafe fn ffi_from(ffi: *mut Self) -> OpaqueContextFFI {
       *ffi
    }

    unsafe fn ffi_to(obj: OpaqueContextFFI) -> *mut Self {
        // Converting a const pointer to a mut pointer and then writing to it can lead to undefined
        // behavior if the original memory location wasn't meant to be mutable
        obj as *mut _
    }

    unsafe fn destroy(_ffi: *mut Self) {
        // No destroy no ownership here
    }
}

impl FFIConversion<OpaqueContextMutFFI> for OpaqueContextMut {
    unsafe fn ffi_from_const(ffi: *const Self) -> OpaqueContextMutFFI {
        *ffi
    }

    unsafe fn ffi_to_const(obj: OpaqueContextMutFFI) -> *const Self {
        obj as *const _
    }

    unsafe fn ffi_from(ffi: *mut Self) -> OpaqueContextMutFFI {
        *ffi
    }

    unsafe fn ffi_to(obj: OpaqueContextMutFFI) -> *mut Self {
        // Converting a const pointer to a mut pointer and then writing to it can lead to undefined
        // behavior if the original memory location wasn't meant to be mutable
        boxed(obj)
    }
}

// No Drop implementation for them

pub trait FFIVecConversion {
    type Value;
    unsafe fn decode(&self) -> Vec<Self::Value>;
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

// pub unsafe fn complex_vec_iterator<T: Vec<HashID>, U: Vec_HashID_FFI, F>(iter: impl Iterator<Item=T>) -> *mut *mut U
pub unsafe fn complex_vec_iterator<T, U>(iter: impl Iterator<Item=T>) -> *mut *mut U
    where
        // F: Fn(T) -> *mut U,
        U: FFIConversion<T> {
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
        impl rs_ffi_interfaces::FFIConversion<Vec<$t>> for $name {
            unsafe fn ffi_from_const(ffi: *const $name) -> Vec<$t> {
                let ffi_ref = &*ffi;
                rs_ffi_interfaces::FFIVecConversion::decode(ffi_ref)
            }
            unsafe fn ffi_to_const(obj: Vec<$t>) -> *const $name {
                rs_ffi_interfaces::FFIVecConversion::encode(obj)
            }
            unsafe fn destroy(ffi: *mut $name) {
                rs_ffi_interfaces::unbox_any(ffi);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_conversion_interface_ffi_map_conversion {
    ($name:ident, $map:ty, $from:block, $to:block) => {
        impl rs_ffi_interfaces::FFIConversion<$map> for $name {
            unsafe fn ffi_from_const(ffi: *const $name) -> $map {
                $from
            }
            unsafe fn ffi_to_const(obj: $map) -> *const $name {
                $to
            }
            unsafe fn destroy(ffi: *mut $name) {
                rs_ffi_interfaces::unbox_any(ffi);
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
        impl rs_ffi_interfaces::FFIVecConversion for $name {
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
            { rs_ffi_interfaces::from_simple_vec(self.values as *const Self::Value, self.count) },
            { rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::boxed_vec(obj) }) }
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
                    .map(|i| rs_ffi_interfaces::FFIConversion::ffi_from_const(*values.add(i)))
                    .collect()
            }, {
                rs_ffi_interfaces::boxed(Self { count: obj.len(), values: rs_ffi_interfaces::complex_vec_iterator(obj.into_iter()) })
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
            rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! vec_ffi_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_simple_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
            rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_simple_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            rs_ffi_interfaces::unbox_vec_ptr(self.keys, self.count);
            rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_complex_simple_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
            rs_ffi_interfaces::unbox_vec_ptr(self.values, self.count);
        });
    }
}

#[macro_export]
macro_rules! map_ffi_complex_drop_interface {
    ($name:ident) => {
        ffi_drop_interface!($name, {
            rs_ffi_interfaces::unbox_any_vec_ptr(self.keys, self.count);
            rs_ffi_interfaces::unbox_any_vec_ptr(self.values, self.count);
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
        rs_ffi_interfaces::boxed(Self {
            count: obj.len(),
            keys: rs_ffi_interfaces::$keys_method(obj.into_keys()),
            values: rs_ffi_interfaces::$values_method(obj.into_values())
        })
    }
}

#[macro_export]
macro_rules! map_ffi_from {
    ($method:ident) => {
        let ffi_ref = &*ffi;
        rs_ffi_interfaces::$method(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
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
                rs_ffi_interfaces::from_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
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
                rs_ffi_interfaces::from_simple_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
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
                rs_ffi_interfaces::from_complex_simple_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            },
            { map_ffi_to!(complex_vec_iterator, simple_vec_iterator) }
        );
    }
}

#[macro_export]
macro_rules! map_ffi_complex_expansion {
    ($name:ident, $map:ty, $k:ty, $v:ty) => {
        // let from = { let ffi_ref = &*ffi; rs_ffi_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values) };
        map_ffi_struct!($name, *mut $k, *mut $v);
        map_ffi_complex_drop_interface!($name);
        ffi_conversion_interface_ffi_map_conversion!(
            $name,
            $map,
            {
                let ffi_ref = &*ffi;
                rs_ffi_interfaces::from_complex_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values)
            },
            { map_ffi_to!(complex_vec_iterator, complex_vec_iterator) }
        );
    }
}

// pub enum