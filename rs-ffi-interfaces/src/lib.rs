use std::collections::{BTreeMap, HashMap};
use std::ffi::{CStr, CString};
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;
use std::ptr::null_mut;

// #[rs_ffi_macro_derive::impl_syn_extension]
pub trait FFIConversion<T> {
    unsafe fn ffi_from(ffi: *mut Self) -> T;
    unsafe fn ffi_to(obj: T) -> *mut Self;
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T>;
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self;

    unsafe fn destroy(ffi: *mut Self);
}

impl FFIConversion<String> for c_char {
    unsafe fn ffi_from(ffi: *mut Self) -> String {
        CStr::from_ptr(ffi).to_str().unwrap().to_string()
    }

    unsafe fn ffi_to(obj: String) -> *mut Self {
        CString::new(obj).unwrap().into_raw()
    }

    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<String> {
        (!ffi.is_null())
            .then_some(<Self as FFIConversion<String>>::ffi_from(ffi))
    }

    unsafe fn ffi_to_opt(obj: Option<String>) -> *mut Self {
        obj.map_or(null_mut(), |o| <Self as FFIConversion<String>>::ffi_to(o))
    }

    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        let _ = CString::from_raw(ffi);
    }
}

impl FFIConversion<&str> for c_char {
    unsafe fn ffi_from(ffi: *mut Self) -> &'static str {
        CStr::from_ptr(ffi).to_str().unwrap()
    }

    unsafe fn ffi_to(obj: &str) -> *mut Self {
        CString::new(obj).unwrap().into_raw()
    }

    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<&'static str> {
        (!ffi.is_null())
            .then_some(<Self as FFIConversion<&str>>::ffi_from(ffi))
    }

    unsafe fn ffi_to_opt(obj: Option<&str>) -> *mut Self {
        obj.map_or(null_mut(), |o| <Self as FFIConversion<&str>>::ffi_to(o))
    }

    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        let _ = CString::from_raw(ffi);
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
pub unsafe fn unbox_vec<T>(vec: Vec<*mut T>) -> Vec<Box<T>> {
    vec.iter().map(|&x| unbox_any(x)).collect()
}

/// # Safety
pub unsafe fn unbox_vec_ptr<T>(ptr: *mut T, count: usize) -> Vec<T> {
    Vec::from_raw_parts(ptr, count, count)
}

/// # Safety
pub unsafe fn unbox_simple_vec<T>(vec: VecFFI<*mut T>) {
    let mem = unbox_vec_ptr(vec.values, vec.count);
    drop(mem)
}

pub fn convert_vec_to_fixed_array<const N: usize>(data: &Vec<u8>) -> *mut [u8; N] {
    let mut fixed_array = [0u8; N];
    fixed_array.copy_from_slice(data);
    boxed(fixed_array)
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MapFFI<K, V> {
    pub count: usize,
    pub keys: *mut K,
    pub values: *mut V,
}

impl<K, V> MapFFI<K, V> where K: Copy, V: Copy  {
    pub unsafe fn fold_to_btree_map<K2: Ord, V2>(self, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> BTreeMap<K2, V2> {
        (0..self.count).fold(BTreeMap::new(), |mut acc, i| {
            let key = key_converter(*self.keys.add(i));
            let value = value_converter(*self.values.add(i));
            acc.insert(key, value);
            acc
        })
    }
}

impl<K, V> MapFFI<K, V> where K: Copy, V: Copy  {
    pub unsafe fn fold_to_hash_map<K2: Hash + PartialEq + Eq, V2>(self, key_converter: impl Fn(K) -> K2, value_converter: impl Fn(V) -> V2) -> HashMap<K2, V2> {
        (0..self.count).fold(HashMap::new(), |mut acc, i| {
            let key = key_converter(*self.keys.add(i));
            let value = value_converter(*self.values.add(i));
            acc.insert(key, value);
            acc
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VecFFI<V> {
    pub count: usize,
    pub values: *mut V,
}

impl<V> VecFFI<V> {
    pub fn new(vec: Vec<V>) -> VecFFI<V> {
        Self { count: vec.len(), values: boxed_vec(vec) }
    }
}
