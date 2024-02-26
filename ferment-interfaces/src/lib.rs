pub mod fermented;

use std::collections::{BTreeMap, HashMap};
use std::ffi::CString;
use std::hash::Hash;
use std::mem;
use std::os::raw::c_char;
use std::sync::Arc;

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
        (!ffi.is_null()).then(|| <Self as FFIConversion<T>>::ffi_from(ffi))
    }
    /// # Safety
    unsafe fn ffi_to_opt(obj: Option<T>) -> *mut Self where Self: Sized {
        obj.map_or(std::ptr::null_mut(), |o| <Self as FFIConversion<T>>::ffi_to(o))
        // obj.map_or(NonNull::<Self>::dangling().as_ptr(), |o| <Self as FFIConversion<T>>::ffi_to(o))
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

/// # Safety
pub unsafe fn from_primitive_vec<T: Clone>(vec: *mut T, count: usize) -> Vec<T> {
    std::slice::from_raw_parts(vec, count).to_vec()
}

/// # Safety
pub unsafe fn from_complex_vec<V, V2: FFIConversion<V>>(vec: *mut *mut V2, count: usize) -> Vec<V> {
    (0..count)
        .map(|i| FFIConversion::ffi_from(*vec.add(i)))
        .collect()
}

/// # Safety
pub unsafe fn to_complex_vec<T, U>(iter: impl Iterator<Item=T>) -> *mut *mut U
    where U: FFIConversion<T> {
    boxed_vec(iter.map(|o| <U as FFIConversion<T>>::ffi_to(o)).collect())
}

/// # Safety
pub unsafe fn to_primitive_vec<T, U>(iter: impl Iterator<Item=T>) -> *mut U
    where Vec<U>: FromIterator<T> {
    boxed_vec(iter.collect())
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

// pub trait ResultFrom<T, E, TF, EF> {
//     fn result_from<RT>(from: &RT) -> Result<T, E> where RT: ResultTo<T, E>;
// }
// pub trait ResultTo<T, E> {
//     fn result_to(o: (T, E)) -> Self;
//     fn ok(&self) -> *mut T;
//     fn error(&self) -> *mut E;
// }

// impl<T, E, FFI> ResultTo<T, E> for FFI {
//    fn result_to(o: (T, E)) -> Self {
//        Self { ok: o.0, error: o.1 }
//    }
//
//     fn ok(&self) -> *mut T {
//         todo!()
//     }
//
//     fn error(&self) -> *mut E {
//         todo!()
//     }
// }

// impl<T, E, FT, FE, FFI> ResultFrom<T, E, FT, FE> for FFI
//    where
//        FT: FFIConversion<T>,
//        FE: FFIConversion<E> {
//
//    fn result_from<RT>(result: &RT) -> Result<T, E> where RT: ResultTo<T, E> {
//        unsafe {
//            if result.error().is_null() {
//                Ok(<FT as FFIConversion<T>>::ffi_from(result.ok()))
//            } else {
//                Err(<FE as FFIConversion<E>>::ffi_from(result.error()))
//            }
//        }
//    }
// }
//
// impl<T, E, FT, FE, FFIResult> FFIConversion<Result<T, E>> for FFIResult
//    where
//        FT: FFIConversion<T>,
//        FE: FFIConversion<E>,
//        FFIResult: ResultFrom<T, E, FT, FE> + ResultTo<T, E> {
//
//    unsafe fn ffi_from_const(ffi: *const Self) -> Result<T, E> {
//        FFIResult::result_from(&*ffi)
//    }
//
//    unsafe fn ffi_to_const(obj: Result<T, E>) -> *const Self {
//        boxed(FFIResult::result_to(match obj {
//            Ok(o) => (FFIConversion::ffi_to(o), std::ptr::null_mut()),
//            Err(o) => (std::ptr::null_mut(), FFIConversion::ffi_to(o)),
//        }))
//    }
// }


impl<T> FFIConversion<Arc<T>> for T {
    unsafe fn ffi_from_const(ffi: *const Self) -> Arc<T> {
        Arc::from_raw(ffi)
    }

    unsafe fn ffi_to_const(obj: Arc<T>) -> *const Self {
        Arc::into_raw(obj) as *mut T
    }
}

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
