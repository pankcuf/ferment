pub mod fermented;

pub use crate as ferment;

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;
use std::hash::Hash;
use std::{mem, slice};
use std::os::raw::c_char;

/// We pass here main context of parent program

pub type OpaqueContext = *const std::os::raw::c_void;

pub type OpaqueContextMut = *mut std::os::raw::c_void;

// No Drop implementation for them

// pub trait FFIOpaqueConversion {
//
// }

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
        Self::ffi_to_const(obj).cast_mut()
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


pub trait FFIConversionDestroy<T> {
    /// # Safety
    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        let _ = unbox_any(ffi);
    }
}

// pub trait FFIConversion<T>: FFIConversionFrom<T> + FFIConversionTo<T> + FFIConversionDestroy<T> {}

pub fn boxed<T>(obj: T) -> *mut T {
    Box::into_raw(Box::new(obj))
}

// /// # Safety
// pub unsafe fn from_opt_box<C, T>(vec: *mut *mut T, count: usize) -> C {
//
// }

pub fn clone_into_array<A, T>(slice: &[T]) -> A where A: Default + AsMut<[T]>, T: Clone {
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
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
pub unsafe fn unbox_any<T: ?Sized>(any: *mut T) -> Box<T> {
    Box::from_raw(any)
}
pub unsafe fn unbox_any_opt<T: ?Sized>(any: *mut T) {
    if !any.is_null() {
        unbox_any(any);
    }
}

/// # Safety
pub unsafe fn unbox_string(data: *mut c_char) {
    let _ = CString::from_raw(data);
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
pub unsafe fn from_opt_primitive<T: Copy>(ptr: *mut T) -> Option<T> {
    (!ptr.is_null()).then(|| *ptr)
}

/// # Safety
pub unsafe fn to_opt_primitive<T: Copy>(obj: Option<T>) -> *mut T {
    obj.map_or(std::ptr::null_mut(), |o| boxed(o))
}

/// # Safety
pub unsafe fn destroy_opt_primitive<T: Copy>(ptr: *mut T) {
    if !ptr.is_null() {
        unbox_any(ptr);
    }
}


pub trait FFIVecConversion {
    type Value: IntoIterator;
    /// # Safety
    unsafe fn decode(&self) -> Self::Value;
    /// # Safety
    unsafe fn encode(obj: Self::Value) -> *mut Self;
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

impl<K: Hash + Eq, V> FFIMapConversion for indexmap::IndexMap<K, V> {
    type Key = K;
    type Value = V;
    fn new() -> Self { indexmap::IndexMap::new() }
    fn insert(&mut self, key: K, value: V) { indexmap::IndexMap::insert(self, key, value); }
}

/// # Safety
pub unsafe fn from_primitive_group<C, T: Copy>(vec: *mut T, count: usize) -> C
    where
        C: FromIterator<T>,
        T: Clone {
    slice::from_raw_parts(vec, count).iter().cloned().collect()
}

/// # Safety
pub unsafe fn from_opt_primitive_group<C, T>(vec: *mut *mut T, count: usize) -> C
    where
        C: FromIterator<Option<T>>,
        T: Copy {
    (0..count)
        .map(|i| {
            let v = *vec.add(i);
            (!v.is_null()).then(|| *v)
        })
        .collect()
}


/// # Safety
pub unsafe fn from_complex_group<C, V, V2>(vec: *mut *mut V2, count: usize) -> C
    where
        C: FromIterator<V>,
        V2: FFIConversionFrom<V> {
    (0..count)
        .map(|i| FFIConversionFrom::ffi_from(*vec.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn from_opt_complex_group<C, V, V2>(vec: *mut *mut V2, count: usize) -> C
    where
        C: FromIterator<Option<V>>,
        V2: FFIConversionFrom<V> {
    (0..count)
        .map(|i| FFIConversionFrom::ffi_from_opt(*vec.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn to_complex_group<T, U>(iter: impl Iterator<Item=T>) -> *mut *mut U
    where U: FFIConversionTo<T> {
    boxed_vec(iter.map(|o| <U as FFIConversionTo<T>>::ffi_to(o)).collect())
}
/// # Safety
pub unsafe fn to_opt_complex_group<T, U>(iter: impl Iterator<Item=Option<T>>) -> *mut *mut U
    where U: FFIConversionTo<T> {
    boxed_vec(iter.map(|o| <U as FFIConversionTo<T>>::ffi_to_opt(o)).collect())
}

/// # Safety
pub unsafe fn to_primitive_group<T, U>(iter: impl Iterator<Item=T>) -> *mut U
    where Vec<U>: FromIterator<T> {
    boxed_vec(iter.collect())
}
/// # Safety
pub unsafe fn to_opt_primitive_group<T, U>(iter: impl Iterator<Item=Option<T>>) -> *mut *mut U
    where Vec<*mut U>: FromIterator<*mut T> {
    boxed_vec(iter.map(|t| t.map_or(std::ptr::null_mut(), |o| boxed(o))).collect())
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
pub unsafe fn fold_to_result<T, E, T2, E2>(
    ok: *mut T,
    error: *mut E,
    key_converter: impl Fn(*mut T) -> T2,
    value_converter: impl Fn(*mut E) -> E2) -> Result<T2, E2> {
    if error.is_null() {
        Ok(key_converter(ok))
    } else {
        Err(value_converter(error))
    }
}
/// # Safety
pub unsafe fn to_result<T, E, T2, E2>(
    result: Result<T2, E2>,
    key_converter: impl Fn(T2) -> *mut T,
    value_converter: impl Fn(E2) -> *mut E,

) -> (*mut T, *mut E) {
    match result {
        Ok(o) => (key_converter(o), std::ptr::null_mut()),
        Err(o) => (std::ptr::null_mut(), value_converter(o))
    }
}
//     ok: *mut T,
//     error: *mut E,
//     key_converter: impl Fn(*mut T) -> T2,
//     value_converter: impl Fn(*mut E) -> E2) -> Result<T2, E2> {
//     if error.is_null() {
//         Ok(key_converter(ok))
//     } else {
//         Err(value_converter(error))
//     }
// }

// pub trait FFICallback<I, O> {
//     // unsafe fn apply(&self, args: I) -> O;
//     unsafe fn get<T>(&self) -> T where T: Fn(I) -> O;
//     // unsafe fn get(&self) -> Box<dyn Fn(I) -> O>;
//     // unsafe fn get(&self) -> T;
//     // unsafe fn get(&self) -> Box<dyn Fn(I) -> O>;
//     // unsafe fn get<T: Fn(([u8; 32])) -> String>(&self) -> T
// }
// pub trait FFICallbackPtr<I, O> {
//     unsafe fn get(&self) -> (fn(*const std::os::raw::c_void, I) -> O, *const std::os::raw::c_void);
// }
// pub trait FFICallback2<I, O> {
//     unsafe fn apply(&self, args: I) -> O;
// }


/// # Safety
// pub unsafe fn callback_fn_ptr<T, I, O>(callback: T) -> unsafe fn(I) -> O where I: Sized, O: Sized {
//     unsafe fn _callback<I, O>(args: I) -> O {
//         callback(args)
//     }
//     _callback
// }
// pub unsafe fn callback<T, I, O, R>(callback: T) -> R where T: FFICallback<I, O>, R: Fn(I) -> O {
//     FFICallback::get(&callback).into()
// }
// pub unsafe fn callback_fn_mut<T, I, O, R>(callback: T) -> R where T: FFICallback<I, O>, R: FnMut(I) -> O {
//     FFICallback::get(&callback)
// }
// pub unsafe fn callback_fn_once<T, I, O, R>(callback: T) -> R where T: FFICallback<I, O>, R: FnOnce(I) -> O {
//     FFICallback::get(&callback)
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

        impl ferment_interfaces::FFIConversionFrom<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
        }
        impl ferment_interfaces::FFIConversionTo<$RustType> for $FFIType {
            unsafe fn ffi_to_const(obj: $RustType) -> *const Self {
                ferment_interfaces::boxed(<$FFIType>::from(&obj))
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
        impl ferment_interfaces::FFIConversionFrom<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
        }
        impl ferment_interfaces::FFIConversionTo<$RustType> for $FFIType {
            unsafe fn ffi_to_const(obj: $RustType) -> *const Self {
                ferment_interfaces::boxed(<$FFIType>::from(&obj))
            }
        }
    };
}

// impl<T> FFIConversionFrom<std::sync::Arc<T>> for T {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<T> {
//         std::sync::Arc::from_raw(ffi)
//     }
// }
// impl<T> FFIConversionTo<std::sync::Arc<T>> for T {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<T>) -> *const Self {
//         std::sync::Arc::into_raw(obj)
//     }
// }

// pub struct Arc_u32 {
//     pub obj: u32
// }
// impl FFIConversionFrom<std::sync::Arc<u32>> for Arc_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<u32> {
//         let ffi_ref = &*ffi;
//         Arc::new(ffi_ref.obj)
//     }
// }
// impl FFIConversionTo<std::sync::Arc<u32>> for Arc_u32 {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<u32>) -> *const Self {
//         boxed(Self { obj: *obj } )
//     }
// }
//
// #[derive(Clone)]
// pub struct Arc_String {
//     pub value: *mut std::os::raw::c_char,
// }
// impl FFIConversionFrom<std::sync::Arc<String>> for crate::Arc_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<String> {
//         let ffi_ref = &*ffi;
//         Arc::new(FFIConversionFrom::ffi_from(ffi_ref.value))
//     }
// }
// impl FFIConversionTo<std::sync::Arc<String>> for crate::Arc_String {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<String>) -> *const Self {
//         boxed(Self { value: FFIConversionTo::ffi_to((*obj).clone()) })
//     }
// }
//
//
//
//
// #[derive(Clone)]
// pub struct Mutex_u32 {
//     pub obj: u32,
// }
//
// impl FFIConversionFrom<std::sync::Mutex<u32>> for crate::Mutex_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Mutex<u32> {
//         let ffi_ref = &*ffi;
//         std::sync::Mutex::new(ffi_ref.obj)
//     }
// }
// impl FFIConversionTo<std::sync::Mutex<u32>> for crate::Mutex_u32 {
//     unsafe fn ffi_to_const(obj: std::sync::Mutex<u32>) -> *const Self {
//         boxed(Self { obj: obj.into_inner().expect("Err") })
//     }
// }
// #[derive(Clone)]
// pub struct Mutex_String {
//     pub value: *mut std::os::raw::c_char,
// }
//
// impl FFIConversionFrom<std::sync::Mutex<String>> for crate::Mutex_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Mutex<String> {
//         let ffi_ref = &*ffi;
//         std::sync::Mutex::new(FFIConversionFrom::ffi_from_const(ffi_ref.value))
//     }
// }
// impl FFIConversionTo<std::sync::Mutex<String>> for crate::Mutex_String {
//     unsafe fn ffi_to_const(obj: std::sync::Mutex<String>) -> *const Self {
//         boxed(Self { value: FFIConversionTo::ffi_to(obj.into_inner().expect("Err")) })
//     }
// }
//
// #[derive(Clone)]
// pub struct RefCell_String {
//     pub value: *mut std::os::raw::c_char,
// }
//
// impl FFIConversionFrom<std::cell::RefCell<String>> for crate::RefCell_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::cell::RefCell<String> {
//         let ffi_ref = &*ffi;
//         std::cell::RefCell::new(FFIConversionFrom::ffi_from_const(ffi_ref.value))
//     }
// }
// impl FFIConversionTo<std::cell::RefCell<String>> for crate::RefCell_String {
//     unsafe fn ffi_to_const(obj: std::cell::RefCell<String>) -> *const Self {
//         boxed(Self { value: FFIConversionTo::ffi_to(obj.into_inner()) })
//     }
// }


// impl FFIConversionFrom<std::sync::RwLock<u32>> for crate::RwLock_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::RwLock<u32> {
//         let ffi_ref = &*ffi;
//         std::sync::RwLock::new(ffi_ref.obj)
//     }
// }
// impl FFIConversionTo<std::sync::RwLock<u32>> for crate::RwLock_u32 {
//     unsafe fn ffi_to_const(obj: std::sync::RwLock<u32>) -> *const Self {
//         boxed(Self { obj: obj.into_inner().expect("Err") })
//     }
// }


// pub struct Slice_u32 {
//     pub values: *const u32,
//     pub count: usize,
// }
// impl<'a> FFIConversionFrom<&'a [u32]> for Slice_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> &'a [u32] {
//         let ffi_ref = &*ffi;
//         slice::from_raw_parts(ffi_ref.values, ffi_ref.count)
//     }
// }
// impl<'a> FFIConversionTo<&'a [u32]> for Slice_u32 {
//     unsafe fn ffi_to_const(obj: &'a [u32]) -> *const Self {
//         boxed(Self { values: boxed_vec(obj.to_vec()) as *const _, count: obj.len() })
//     }
// }
//
// impl Drop for Slice_u32 {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_vec_ptr(self.values as *mut u32, self.count);
//         }
//     }
// }
//
//
//
//
//



// // TODO: maybe refactor to this interface but not sure about nullability/optional since it'll not be a pointer anymore


// #[repr(C)]
// #[derive(Clone)]
// pub struct std_sync_Arc_std_sync_RwLock_String {
//     pub obj: *mut std_sync_RwLock_String
// }
// impl FFIConversionFrom<std::sync::Arc<std::sync::RwLock<String>>> for std_sync_Arc_std_sync_RwLock_String {
//     unsafe fn ffi_from_const(ffi: *const std_sync_Arc_std_sync_RwLock_String) -> std::sync::Arc<std::sync::RwLock<String>> {
//         let ffi_ref = &*ffi;
//         std::sync::Arc::new(FFIConversionFrom::ffi_from(ffi_ref.obj))
//     }
// }
// impl FFIConversionTo<std::sync::Arc<std::sync::RwLock<String>>> for std_sync_Arc_std_sync_RwLock_String {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<std::sync::RwLock<String>>) -> *const std_sync_Arc_std_sync_RwLock_String {
//         Box::into_raw(Box::new(Self { obj: FFIConversionTo::ffi_to(std::sync::RwLock::new(obj.read().expect("Poisoned").clone())) }))
//     }
// }
// impl Drop for std_sync_Arc_std_sync_RwLock_String {
//     fn drop(&mut self) {
//         unsafe {
//             Box::from_raw(self.obj);
//         }
//     }
// }
//
// #[repr(C)]
// #[derive(Clone)]
// pub struct std_sync_RwLock_String {
//     pub obj: *mut std::os::raw::c_char
// }
// impl FFIConversionFrom<std::sync::RwLock<String>> for std_sync_RwLock_String {
//     unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_String) -> std::sync::RwLock<String> {
//         let ffi_ref = &*ffi;
//         std::sync::RwLock::new(FFIConversionFrom::ffi_from(ffi_ref.obj))
//     }
// }
// impl FFIConversionTo<std::sync::RwLock<String>> for std_sync_RwLock_String {
//     unsafe fn ffi_to_const(obj: std::sync::RwLock<String>) -> *const std_sync_RwLock_String {
//         Box::into_raw(Box::new(Self { obj: FFIConversionTo::ffi_to(obj.into_inner().expect("Err")) }))
//     }
// }
// impl Drop for std_sync_RwLock_String {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_string(self.obj);
//         }
//     }
// }
// Arc with opaque pointer inside
// #[repr(C)]
// #[derive(Clone)]
// pub struct std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache {
//     pub obj: *mut dash_sdk::internal_cache::InternalSdkCache
// }
// impl ferment_interfaces::FFIConversionFrom<std::sync::Arc<dash_sdk::internal_cache::InternalSdkCache>> for std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache
// {
//     unsafe fn ffi_from_const(
//         ffi: *const std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache,
//     ) -> std::sync::Arc<dash_sdk::internal_cache::InternalSdkCache> {
//         let ffi_ref = &*ffi;
//         std::sync::Arc::from_raw(ffi_ref.obj)
//     }
// }
// impl ferment_interfaces::FFIConversionTo<std::sync::Arc<dash_sdk::internal_cache::InternalSdkCache>> for std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<dash_sdk::internal_cache::InternalSdkCache>) -> *const std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache {
//         ferment_interfaces::boxed(Self { obj: std::sync::Arc::into_raw(obj) as *mut _ })
//     }
// }
// impl ferment_interfaces::FFIConversionDestroy<std::sync::Arc<dash_sdk::internal_cache::InternalSdkCache>> for std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache {
//     unsafe fn destroy(ffi: *mut std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache) {
//         let _ = ferment_interfaces::unbox_any(ffi);
//     }
// }
// impl Drop for std_sync_Arc_dash_sdk_internal_cache_InternalSdkCache {
//     fn drop(&mut self) {
//         unsafe {
//             let _ = std::sync::Arc::from_raw(self.obj);
//         }
//     }
// }

/// # Safety
pub unsafe fn to_arc<T: ?Sized>(obj: std::sync::Arc<T>) -> *mut T {
    std::sync::Arc::into_raw(obj).cast_mut()
}
/// # Safety
pub unsafe fn from_arc<T: ?Sized>(obj: *const T) -> std::sync::Arc<T> {
    std::sync::Arc::from_raw(obj)
}

pub trait FFIComposer<T> {
    type From: Fn(*mut Self) -> T;
    type To: Fn(T) -> *mut Self;
    type Destroy: Fn(*mut Self);
    type Variable: Fn() -> T;
}

// #[repr(C)]
// pub struct FFIContext {
//     pub caller: std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String + Send + Sync>,
//
// }
// #[repr(C)]
// #[derive(Clone)]
// pub struct Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     pub obj: *mut Fn_ARGS_FFIContext_u32_String_u32_RTRN_String
// }
//
// #[repr(C)]
// #[derive(Clone)]
// pub struct Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     caller: unsafe extern "C" fn(context: *const FFIContext, quorum_type: u32, quorum_hash: *mut c_char, core_chain_locked_height: u32) -> *mut c_char,
//     destructor: unsafe extern "C" fn(result: *mut c_char),
// }
// impl Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     pub unsafe fn call(
//         &self,
//         context: *const FFIContext,
//         quorum_type: u32,
//         quorum_hash: String,
//         core_chain_locked_height: u32
//     ) -> String {
//         let ffi_result = (self.caller)(context, quorum_type, FFIConversionTo::ffi_to(quorum_hash), core_chain_locked_height);
//         let result = FFIConversionFrom::ffi_from(ffi_result);
//         (self.destructor)(ffi_result);
//         result
//     }
//     pub unsafe fn create(
//         caller: unsafe extern "C" fn(*const FFIContext, u32, *mut c_char, u32) -> *mut c_char,
//         destructor: unsafe extern "C" fn(*mut c_char)
//     ) -> Self {
//         Self { caller, destructor }
//     }
// }
//
// impl FFIConversionFrom<std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String>> for Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     unsafe fn ffi_from_const(ffi: *const Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String) -> std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String> {
//         let ffi_ref = &*ffi;
//         std::sync::Arc::new(|context , quorum_type, quorum_hash, core_chain_locked_height |
//             (&*ffi_ref.obj).call(context, quorum_type, quorum_hash, core_chain_locked_height))
//     }
// }
// impl FFIConversionTo<std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String>> for Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     unsafe fn ffi_to_const(obj: std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String>) -> *const Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//         boxed(Self {
//             obj: {
//                 unsafe extern "C" fn caller(context: *const FFIContext, quorum_type: u32, quorum_hash: *mut c_char, core_chain_locked_height: u32) -> *mut c_char {
//                     FFIConversionTo::ffi_to(((&*context).caller)(context, quorum_type, FFIConversionFrom::ffi_from(quorum_hash), core_chain_locked_height))
//                 }
//                 unsafe extern "C" fn destructor(result: *mut c_char) {
//                     unbox_any(result);
//                 }
//
//                 boxed(Fn_ARGS_FFIContext_u32_String_u32_RTRN_String { caller, destructor })
//             }
//         })
//     }
// }
//
// impl FFIConversionDestroy<std::sync::Arc<dyn Fn(*const FFIContext, u32, String, u32) -> String>> for Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String {
//     unsafe fn destroy(ffi: *mut Arc_Fn_ARGS_FFIContext_u32_String_u32_RTRN_String) {
//         unbox_any(ffi);
//     }
// }

