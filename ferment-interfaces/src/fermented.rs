use std::ptr::NonNull;
use crate::unbox_any;

pub trait FFIConversion2<'a, T> {
    /// # Safety
    unsafe fn ffi_from_const(ffi: *const Self) -> T;
    /// # Safety
    unsafe fn ffi_to_const(obj: &'a T) -> *const Self;
    /// # Safety
    unsafe fn ffi_from(ffi: *mut Self) -> T {
        Self::ffi_from_const(ffi)
    }
    /// # Safety
    unsafe fn ffi_to(obj: &'a T) -> *mut Self {
        Self::ffi_to_const(obj).cast_mut()
    }
    /// # Safety
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null()).then(||<Self as FFIConversion2<T>>::ffi_from(ffi))
    }
    /// # Safety
    unsafe fn ffi_to_opt(obj: Option<&'a T>) -> *mut Self where Self: Sized {
        obj.map_or(NonNull::<Self>::dangling().as_ptr(), |o| <Self as FFIConversion2<T>>::ffi_to(o))
    }
    /// # Safety
    unsafe fn destroy(ffi: *mut Self) {
        if ffi.is_null() {
            return;
        }
        unbox_any(ffi);
    }
}

pub mod types {
    use std::ffi::{CStr, CString};
    use std::mem;
    use std::os::raw::c_char;
    use crate::{boxed, clone_into_array, FFIConversionDestroy, FFIConversionFrom, FFIConversionTo, unbox_string};
    use crate::fermented::FFIConversion2;

    impl FFIConversionFrom<u128> for [u8; 16] {
        unsafe fn ffi_from_const(ffi: *const Self) -> u128 {
            let arr = &*ffi;
            u128::from_le_bytes(*arr)
        }
    }
    impl FFIConversionTo<u128> for [u8; 16] {
        unsafe fn ffi_to_const(obj: u128) -> *const Self {
            // let bytes = obj.to_le_bytes();
            // let mut arr = [0u8; 16];
            // arr.copy_from_slice(&bytes);
            boxed(obj.to_le_bytes())
        }
    }

    impl FFIConversionFrom<String> for c_char {
        unsafe fn ffi_from_const(ffi: *const Self) -> String {
            CStr::from_ptr(ffi).to_str().unwrap().to_string()
        }
        unsafe fn ffi_from(ffi: *mut Self) -> String {
            Self::ffi_from_const(ffi.cast_const())
        }
    }

    impl FFIConversionTo<String> for c_char {
        unsafe fn ffi_to_const(obj: String) -> *const Self {
            let s = CString::new(obj).unwrap();
            s.as_ptr()
        }
        unsafe fn ffi_to(obj: String) -> *mut Self {
            CString::new(obj).unwrap().into_raw()
        }
    }

    impl FFIConversionDestroy<String> for c_char {
        unsafe fn destroy(ffi: *mut Self) {
            if ffi.is_null() {
                return;
            }
            unbox_string(ffi);
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct ByteArray {
        pub ptr: *const u8,
        pub len: usize,
    }


    impl FFIConversionFrom<&[u8]> for ByteArray {
        unsafe fn ffi_from_const(ffi: *const Self) -> &'static [u8] {
            let ffi_ref = &*ffi;
            std::slice::from_raw_parts(ffi_ref.ptr, ffi_ref.len)
        }
        unsafe fn ffi_from(ffi: *mut Self) -> &'static [u8] {
            Self::ffi_from_const(ffi)
        }
    }
    impl FFIConversionTo<&[u8]> for ByteArray {
        unsafe fn ffi_to_const(obj: &[u8]) -> *const Self {
            &Self { ptr: obj.as_ptr(), len: obj.len(), } as *const _
        }

        unsafe fn ffi_to(obj: &[u8]) -> *mut Self {
            Self::ffi_to_const(obj).cast_mut()
        }
    }
    impl FFIConversionDestroy<&[u8]> for ByteArray {
        unsafe fn destroy(_ffi: *mut Self) {
            // TODO: check
        }
    }

    impl FFIConversionFrom<&str> for c_char {
        unsafe fn ffi_from_const(ffi: *const Self) -> &'static str {
            CStr::from_ptr(ffi).to_str().unwrap()
        }
        unsafe fn ffi_from(ffi: *mut Self) -> &'static str {
            Self::ffi_from_const(ffi)
        }
    }
    impl FFIConversionTo<&str> for c_char {
        unsafe fn ffi_to_const(obj: &str) -> *const Self {
            let s = CString::new(obj).unwrap();
            s.as_ptr()
        }
        unsafe fn ffi_to(obj: &str) -> *mut Self {
            CString::new(obj).unwrap().into_raw()
        }
    }
    impl FFIConversionDestroy<&str> for c_char {
        unsafe fn destroy(ffi: *mut Self) {
            if ffi.is_null() {
                return;
            }
            unbox_string(ffi);
        }
    }

    impl<T, FFI> FFIConversionFrom<Box<T>> for FFI where FFI: FFIConversionFrom<T> {
        unsafe fn ffi_from_const(ffi: *const Self) -> Box<T> {
            Box::new(<Self as FFIConversionFrom<T>>::ffi_from_const(ffi))
        }
    }
    impl<T, FFI> FFIConversionTo<Box<T>> for FFI where FFI: FFIConversionTo<T> {
        unsafe fn ffi_to_const(obj: Box<T>) -> *const Self {
            <Self as FFIConversionTo<T>>::ffi_to_const(*obj)
        }
    }
    impl<T, FFI> FFIConversionDestroy<Box<T>> for FFI where FFI: FFIConversionDestroy<T> {}

    impl<'a, T, FFI> FFIConversion2<'a, T> for FFI where FFI: From<&'a T> + 'a, T: From<&'a FFI> + 'a {
        unsafe fn ffi_from_const(ffi: *const Self) -> T {
            T::from(&*ffi)
        }

        unsafe fn ffi_to_const(obj: &'a T) -> *const Self {
            boxed(obj.into())
        }
    }
}
