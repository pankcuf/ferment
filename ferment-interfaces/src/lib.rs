pub mod fermented;

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;
use std::hash::Hash;
use std::{mem, slice};
use std::os::raw::c_char;

/// We pass here main context of parent program

pub type OpaqueContext = *const std::os::raw::c_void;

pub type OpaqueContextMut = *mut std::os::raw::c_void;

// No Drop implementation for them


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
        (!ffi.is_null())
            .then(|| <Self as FFIConversion<T>>::ffi_from(ffi))
    }
    /// # Safety
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        if let Some(o) = obj {
            <Self as FFIConversion<T>>::ffi_to(o)
        } else {
            std::ptr::null_mut()
        }
    }
    /// # Safety
    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_any(ffi);
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
        V2: FFIConversion<V> {
    (0..count)
        .map(|i| FFIConversion::ffi_from(*vec.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn from_opt_complex_group<C, V, V2>(vec: *mut *mut V2, count: usize) -> C
    where
        C: FromIterator<Option<V>>,
        V2: FFIConversion<V> {
    (0..count)
        .map(|i| FFIConversion::ffi_from_opt(*vec.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn to_complex_group<T, U>(iter: impl Iterator<Item=T>) -> *mut *mut U
    where U: FFIConversion<T> {
    boxed_vec(iter.map(|o| <U as FFIConversion<T>>::ffi_to(o)).collect())
}
/// # Safety
pub unsafe fn to_opt_complex_group<T, U>(iter: impl Iterator<Item=Option<T>>) -> *mut *mut U
    where U: FFIConversion<T> {
    boxed_vec(iter.map(|o| <U as FFIConversion<T>>::ffi_to_opt(o)).collect())
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
        impl ferment_interfaces::FFIConversion<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
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
        impl ferment_interfaces::FFIConversion<$RustType> for $FFIType {
            unsafe fn ffi_from_const(ffi: *const Self) -> $RustType {
                <$RustType>::from(&*ffi)
            }
            unsafe fn ffi_to_const(obj: $RustType) -> *const Self {
                ferment_interfaces::boxed(<$FFIType>::from(&obj))
            }
        }
    };
}

// impl<T> FFIConversion<std::sync::Arc<T>> for T {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<T> {
//         std::sync::Arc::from_raw(ffi)
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::Arc<T>) -> *const Self {
//         std::sync::Arc::into_raw(obj)
//     }
// }

// pub struct Arc_u32 {
//     pub obj: u32
// }
// impl FFIConversion<std::sync::Arc<u32>> for Arc_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<u32> {
//         let ffi_ref = &*ffi;
//         Arc::new(ffi_ref.obj)
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::Arc<u32>) -> *const Self {
//         boxed(Self { obj: *obj } )
//     }
// }
//
// #[derive(Clone)]
// pub struct Arc_String {
//     pub value: *mut std::os::raw::c_char,
// }
// impl FFIConversion<std::sync::Arc<String>> for crate::Arc_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Arc<String> {
//         let ffi_ref = &*ffi;
//         Arc::new(FFIConversion::ffi_from(ffi_ref.value))
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::Arc<String>) -> *const Self {
//         boxed(Self { value: FFIConversion::ffi_to((*obj).clone()) })
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
// impl FFIConversion<std::sync::Mutex<u32>> for crate::Mutex_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Mutex<u32> {
//         let ffi_ref = &*ffi;
//         std::sync::Mutex::new(ffi_ref.obj)
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::Mutex<u32>) -> *const Self {
//         boxed(Self { obj: obj.into_inner().expect("Err") })
//     }
// }
// #[derive(Clone)]
// pub struct Mutex_String {
//     pub value: *mut std::os::raw::c_char,
// }
//
// impl FFIConversion<std::sync::Mutex<String>> for crate::Mutex_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::Mutex<String> {
//         let ffi_ref = &*ffi;
//         std::sync::Mutex::new(FFIConversion::ffi_from_const(ffi_ref.value))
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::Mutex<String>) -> *const Self {
//         boxed(Self { value: FFIConversion::ffi_to(obj.into_inner().expect("Err")) })
//     }
// }
//
// #[derive(Clone)]
// pub struct RefCell_String {
//     pub value: *mut std::os::raw::c_char,
// }
//
// impl FFIConversion<std::cell::RefCell<String>> for crate::RefCell_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::cell::RefCell<String> {
//         let ffi_ref = &*ffi;
//         std::cell::RefCell::new(FFIConversion::ffi_from_const(ffi_ref.value))
//     }
//
//     unsafe fn ffi_to_const(obj: std::cell::RefCell<String>) -> *const Self {
//         boxed(Self { value: FFIConversion::ffi_to(obj.into_inner()) })
//     }
// }


// impl FFIConversion<std::sync::RwLock<u32>> for crate::RwLock_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> std::sync::RwLock<u32> {
//         let ffi_ref = &*ffi;
//         std::sync::RwLock::new(ffi_ref.obj)
//     }
//
//     unsafe fn ffi_to_const(obj: std::sync::RwLock<u32>) -> *const Self {
//         boxed(Self { obj: obj.into_inner().expect("Err") })
//     }
// }

// impl<T, U> FFIConversion<Mutex<U>> for T where T: Clone,  {
//     unsafe fn ffi_from_const(ffi: *const Self) -> Mutex<T> {
//
//         Mutex::new(T::ffi_from_const(ffi))
//     }
//
//     unsafe fn ffi_to_const(obj: Mutex<T>) -> *const Self {
//         boxed(obj.try_lock().unwrap().clone())
//     }
//
//     // ... other methods ...
// }
// impl<T, FFI> FFIConversion<Mutex<T>> for FFI where T: Clone, FFI: FFIConversion<T> {
//    unsafe fn ffi_from_const(ffi: *const Self) -> Mutex<T> {
//        Mutex::new(<Self as FFIConversion<T>>::ffi_from_const(ffi))
//    }
//
//    unsafe fn ffi_to_const(obj: Mutex<T>) -> *const Self {
//        let lock = obj.try_lock().unwrap();
//        FFIConversion::ffi_to_const(lock.clone())
//    }
//}

// pub struct Slice_u32 {
//     pub values: *const u32,
//     pub count: usize,
// }
// impl<'a> FFIConversion<&'a [u32]> for Slice_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> &'a [u32] {
//         let ffi_ref = &*ffi;
//         slice::from_raw_parts(ffi_ref.values, ffi_ref.count)
//     }
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
// pub struct Array_u32 {
//     pub values: *mut u32,
//     pub count: usize,
// }
// impl<const N: usize> FFIConversion<[u32; N]> for Array_u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> [u32; N] {
//         let ffi_ref = &*ffi;
//         slice::from_raw_parts(ffi_ref.values, ffi_ref.count)
//             .try_into()
//             .expect("Array_u32 Length mismatch")
//     }
//     unsafe fn ffi_to_const(obj: [u32; N]) -> *const Self {
//         boxed(Self {
//             values: boxed_vec(obj.to_vec()),
//             count: N })
//     }
// }
//
// impl Drop for Array_u32 {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_vec_ptr(self.values, self.count);
//         }
//     }
// }
// // Doesn't work since returning &vec owned by fn
// pub struct Slice_String {
//     pub values: *const *const std::os::raw::c_char,
//     pub count: usize,
// }

// impl<'a> FFIConversion<&'a [String]> for Slice_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> &'a [String] {
//         let ffi_ref = &*ffi;
//         let vec = (0..ffi_ref.count).into_iter().map(|i| FFIConversion::ffi_from_const(*ffi_ref.values.add(i))).collect::<Vec<String>>();
//         vec.as_slice()
//     }
//     unsafe fn ffi_to_const(obj: &'a [String]) -> *const Self {
//         boxed(Self {
//             values: boxed_vec(obj.iter().map(|o| FFIConversion::ffi_to_const(o.clone())).collect()),
//             count: obj.len() })
//     }
// }
//
// impl Drop for Slice_String {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_any_vec_ptr(self.values as *mut *mut std::os::raw::c_char, self.count);
//         }
//     }
// }

// pub struct Array_String {
//     pub values: *mut *mut std::os::raw::c_char,
//     pub count: usize,
// }
//
// impl<const N: usize> FFIConversion<[String; N]> for Array_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> [String; N] {
//         let ffi_ref = &*ffi;
//         (0..ffi_ref.count)
//             .into_iter()
//             .map(|i| FFIConversion::ffi_from_const(*ffi_ref.values.add(i)))
//             .collect::<Vec<String>>()
//             .try_into()
//             .expect("Length mismatch")
//     }
//     unsafe fn ffi_to_const(obj: [String; N]) -> *const Self {
//         boxed(Self {
//             values: boxed_vec(obj.iter()
//                 .map(|o| FFIConversion::ffi_to(o.clone()))
//                 .collect()),
//             count: obj.len() })
//     }
// }
//
// impl Drop for Array_String {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_any_vec_ptr(self.values as *mut *mut std::os::raw::c_char, self.count);
//         }
//     }
// }


// impl<'a, const N: usize> FFIConversion<&'a [String; N]> for Slice_String {
//     unsafe fn ffi_from_const(ffi: *const Self) -> &'a [String; N] {
//         let ffi_ref = &*ffi;
//
//         let arr = (0..N)
//             .map(|i| FFIConversion::ffi_from_const(*ffi_ref.values.add(i)))
//             .collect();
//         &arr
//     }
//     unsafe fn ffi_to_const(obj: &'a [String; N]) -> *const Self {
//         boxed_slice(obj) as *const _
//     }
// }


// pub struct Array_u32 {
//     pub values: *mut u32,
//     pub count: usize,
// }

// impl<const N: usize> FFIConversion<[u32; N]> for u32 {
//     unsafe fn ffi_from_const(ffi: *const Self) -> [u32; N] {
//         let mut array = [0u32; N];
//         array.copy_from_slice(slice::from_raw_parts(ffi, N));
//         array
//     }
//     unsafe fn ffi_to_const(obj: [u32; N]) -> *const Self {
//         obj.as_ptr()
//     }
// }
//
// pub struct Slice_u32 {
//     pub values: *mut u32,
//     pub count: usize,
// }
//
// impl<'a> FFIConversion<&'a [u32]> for &'a [u32] {
//     unsafe fn ffi_from_const(ffi: *const Self) -> &'a [u32] {
//         slice::from_raw_parts(ffi.values, ffi.count)
//     }
//     unsafe fn ffi_to_const(obj: &'a [u32]) -> *const Self {
//         boxed_slice(obj)
//     }
// }
//

// impl<const N: usize, T, FFI> FFIConversion<[T; N]> for FFI where FFI: FFIConversion<T> {
//     unsafe fn ffi_from_const(ffi: *const Self) -> [T; N] {
//         // (0..N).into_iter().map()
//         let mut array = [FFI::ffi_from_const(ffi); N];
//         array.copy_from_slice(slice::from_raw_parts(ffi, N));
//         array
//     }
//     unsafe fn ffi_to_const(obj: [FFI; N]) -> *const Self {
//         obj.as_ptr()
//     }
// }

// impl ferment_interface/*s::FFIVecConversion for Vec_ferment_example_nested_FeatureVersion {
//     type Value = Vec<ferment_example::nested::FeatureVersion>;
//     unsafe fn decode(&self) -> Self::Value {
//         ferment_interfaces::from_complex_vec(self.values, self.count)
//     }
//     unsafe fn encode(obj: Self::Value) -> *mut Self {
//         ferment_interfaces::boxed(Self {
//             count: obj.len(),
//             values: ferment_interfaces::to_complex_vec(obj.into_iter()),
//         })
//     }
// }
// */








// // TODO: maybe refactor to this interface but not sure about nullability/optional since it'll not be a pointer anymore
// pub trait FFIConversion222<T> {
//     /// # Safety
//     unsafe fn ffi_from(ffi: Self) -> T;
//     /// # Safety
//     unsafe fn ffi_to(obj: T) -> Self;
//     // /// # Safety
//     // unsafe fn ffi_from_opt(ffi: Self) -> Option<T> where Self: Sized {
//     //     (!ffi.is_null())
//     //         .then(|| <Self as FFIConversion<T>>::ffi_from(ffi))
//     // }
//     // /// # Safety
//     // unsafe fn ffi_to_opt(obj: Option<T>) -> Self;
//     /// # Safety
//     unsafe fn destroy(_ffi: Self) where Self: Sized {}
// }

// impl FFIConversion222<u32> for u32 {
//     unsafe fn ffi_from(ffi: Self) -> u32 { ffi }
//     unsafe fn ffi_to(obj: u32) -> Self { obj }
// }
//
// impl FFIConversion222<String> for *mut c_char {
//     unsafe fn ffi_from(ffi: Self) -> String {
//         std::ffi::CStr::from_ptr(ffi)
//             .to_str()
//             .unwrap()
//             .to_string()
//     }
//     unsafe fn ffi_to(obj: String) -> Self {
//         CString::new(obj)
//             .unwrap()
//             .into_raw()
//     }
//
//     unsafe fn destroy(ffi: Self) {
//         if ffi.is_null() {
//             return;
//         }
//         unbox_string(ffi);
//     }
// }



// #[repr(C)]
// #[derive(Clone)]
// pub struct std_sync_Arc_std_sync_RwLock_String {
//     pub obj: *mut std_sync_RwLock_String
// }
// impl FFIConversion<std::sync::Arc<std::sync::RwLock<String>>>
// for std_sync_Arc_std_sync_RwLock_String {
//     unsafe fn ffi_from_const(ffi: *const std_sync_Arc_std_sync_RwLock_String) -> std::sync::Arc<std::sync::RwLock<String>> {
//         let ffi_ref = &*ffi;
//         std::sync::Arc::new(FFIConversion::ffi_from(ffi_ref.obj))
//     }
//     unsafe fn ffi_to_const(obj: std::sync::Arc<std::sync::RwLock<String>>) -> *const std_sync_Arc_std_sync_RwLock_String {
//         Box::into_raw(Box::new(Self { obj: FFIConversion::ffi_to(std::sync::RwLock::new(obj.read().expect("Poisoned").clone())) }))
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
// impl FFIConversion<std::sync::RwLock<String>>
// for std_sync_RwLock_String {
//     unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_String) -> std::sync::RwLock<String> {
//         let ffi_ref = &*ffi;
//         std::sync::RwLock::new(FFIConversion::ffi_from(ffi_ref.obj))
//     }
//     unsafe fn ffi_to_const(obj: std::sync::RwLock<String>) -> *const std_sync_RwLock_String {
//         Box::into_raw(Box::new(Self { obj: FFIConversion::ffi_to(obj.into_inner().expect("Err")) }))
//     }
// }
// impl Drop for std_sync_RwLock_String {
//     fn drop(&mut self) {
//         unsafe {
//             unbox_string(self.obj);
//         }
//     }
// }
