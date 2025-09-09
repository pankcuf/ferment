pub mod types {
    use std::borrow::Cow;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use crate::{boxed, FFIConversionFrom, FFIConversionTo, unbox_string};

    impl FFIConversionFrom<u128> for [u8; 16] {
        unsafe fn ffi_from_const(ffi: *const Self) -> u128 {
            let arr = &*ffi;
            u128::from_le_bytes(*arr)
        }
    }
    impl FFIConversionTo<u128> for [u8; 16] {
        unsafe fn ffi_to_const(obj: u128) -> *const Self {
            boxed(obj.to_le_bytes())
        }
    }
    impl FFIConversionFrom<i128> for [u8; 16] {
        unsafe fn ffi_from_const(ffi: *const Self) -> i128 {
            let arr = &*ffi;
            i128::from_le_bytes(*arr)
        }
    }
    impl FFIConversionTo<i128> for [u8; 16] {
        unsafe fn ffi_to_const(obj: i128) -> *const Self {
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
            Self::ffi_to(obj).cast_const()
        }
        unsafe fn ffi_to(obj: String) -> *mut Self {
            CString::new(obj).unwrap().into_raw()
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
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn str_destroy(str: *mut c_char) {
        unbox_string(str);
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

    impl<'a, T, FFI> FFIConversionFrom<Cow<'a, T>> for FFI where T: Clone, FFI: FFIConversionFrom<T> {
        unsafe fn ffi_from_const(ffi: *const Self) -> Cow<'a, T> {
            Cow::Owned(<FFI as FFIConversionFrom<T>>::ffi_from_const(ffi))
        }
    }

    impl<'a, T, FFI> FFIConversionTo<Cow<'a, T>> for FFI where T: Clone, FFI: FFIConversionTo<T> {
        unsafe fn ffi_to_const(obj: Cow<'a, T>) -> *const Self {
            match obj {
                Cow::Borrowed(v) => <FFI as FFIConversionTo<T>>::ffi_to_const(v.clone()),
                Cow::Owned(v) => <FFI as FFIConversionTo<T>>::ffi_to_const(v),
            }
        }
    }

}
