use std::collections::{BTreeMap, HashMap};
use std::ffi::{CStr, CString};
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;
use std::ptr::{NonNull, null_mut};

pub trait FFIConversion<T> {
    unsafe fn ffi_from(ffi: *mut Self) -> T;
    unsafe fn ffi_to(obj: T) -> *mut Self;
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null()).then_some(< Self as FFIConversion<T>>::ffi_from(ffi))
    }
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        obj.map_or(NonNull::<Self>::dangling().as_ptr(), |o| <Self as FFIConversion<T>>::ffi_to(o))
    }
    unsafe extern "C" fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_any(ffi);
    }
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

    unsafe extern "C" fn destroy(ffi: *mut Self) {
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

    unsafe extern "C" fn destroy(ffi: *mut Self) {
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

/// # Safety
pub unsafe fn unbox_simple_vec<T>(vec: VecFFI<T>) {
    let mem = unbox_vec_ptr(vec.values, vec.count);
    drop(mem)
}

pub unsafe fn unbox_vec_ffi<T>(vec: *mut VecFFI<T>) {
    let vec_ffi = unbox_any(vec);
    let _reconstructed_vec = unbox_vec_ptr(vec_ffi.values, vec_ffi.count);
}


pub fn convert_vec_to_fixed_array<const N: usize>(data: &Vec<u8>) -> *mut [u8; N] {
    let mut fixed_array = [0u8; N];
    fixed_array.copy_from_slice(data);
    boxed(fixed_array)
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MapFFI<K, V> {
    pub count: usize,
    pub keys: *mut K,
    pub values: *mut V,
}

impl<K, V> Drop for MapFFI<K, V> {
    fn drop(&mut self) {
        // TODO: Probably it needs to avoid drop for VecFFI and use chain of unbox_any_vec instead
        unsafe {
            // for complex maps:
            // unbox_any_vec(unbox_vec_ptr(self.keys, self.count));
            // unbox_any_vec(unbox_vec_ptr(self.values, self.count));
            // for simple maps:
            unbox_vec_ptr(self.keys, self.count);
            unbox_vec_ptr(self.values, self.count);
        }
    }
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
#[derive(Clone, Debug)]
pub struct VecFFI<V> {
    pub count: usize,
    pub values: *mut V,
}

impl<V> VecFFI<V> {
    pub fn new(vec: Vec<V>) -> VecFFI<V> {
        Self { count: vec.len(), values: boxed_vec(vec) }
    }
}

impl<V> Drop for VecFFI<V> {
    fn drop(&mut self) {
        // TODO: Probably it needs to avoid drop for VecFFI and use chain of unbox_any_vec instead
        unsafe {
            // for complex vecs:
            // unbox_any_vec(unbox_vec_ptr(self.values, self.count));
            // for simple vecs:
            unbox_vec_ptr(self.values, self.count);
        }
    }
}
