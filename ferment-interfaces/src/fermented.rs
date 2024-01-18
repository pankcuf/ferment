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
        Self::ffi_to_const(obj) as *mut _
    }
    /// # Safety
    unsafe fn ffi_from_opt(ffi: *mut Self) -> Option<T> {
        (!ffi.is_null()).then_some(<Self as FFIConversion2<T>>::ffi_from(ffi))
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
    use std::os::raw::c_char;
    use crate::{boxed, FFIConversion, OpaqueContext, OpaqueContextMut, unbox_string};
    use crate::fermented::FFIConversion2;

    #[allow(non_camel_case_types)]
    pub type OpaqueContext_FFI = OpaqueContext;
    #[allow(non_camel_case_types)]
    pub type OpaqueContextMut_FFI = OpaqueContextMut;

    impl FFIConversion<OpaqueContext_FFI> for OpaqueContext {
        unsafe fn ffi_from_const(ffi: *const Self) -> OpaqueContext_FFI {
            *ffi
        }

        unsafe fn ffi_to_const(obj: OpaqueContext_FFI) -> *const Self {
            obj as *const _
        }

        unsafe fn ffi_from(ffi: *mut Self) -> OpaqueContext_FFI {
            *ffi
        }

        unsafe fn ffi_to(obj: OpaqueContext_FFI) -> *mut Self {
            // Converting a const pointer to a mut pointer and then writing to it can lead to undefined
            // behavior if the original memory location wasn't meant to be mutable
            obj as *mut _
        }

        unsafe fn destroy(_ffi: *mut Self) {
            // No destroy no ownership here
        }
    }

    impl FFIConversion<OpaqueContextMut_FFI> for OpaqueContextMut {
        unsafe fn ffi_from_const(ffi: *const Self) -> OpaqueContextMut_FFI {
            *ffi
        }

        unsafe fn ffi_to_const(obj: OpaqueContextMut_FFI) -> *const Self {
            obj as *const _
        }

        unsafe fn ffi_from(ffi: *mut Self) -> OpaqueContextMut_FFI {
            *ffi
        }

        unsafe fn ffi_to(obj: OpaqueContextMut_FFI) -> *mut Self {
            // Converting a const pointer to a mut pointer and then writing to it can lead to undefined
            // behavior if the original memory location wasn't meant to be mutable
            boxed(obj)
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

    impl<T, FFI> FFIConversion<Box<T>> for FFI where FFI: FFIConversion<T> {
        unsafe fn ffi_from_const(ffi: *const Self) -> Box<T> {
            Box::new(<Self as FFIConversion<T>>::ffi_from_const(ffi))
        }

        unsafe fn ffi_to_const(obj: Box<T>) -> *const Self {
            <Self as FFIConversion<T>>::ffi_to_const(*obj)
        }
    }

    impl<'a, T, FFI> FFIConversion2<'a, T> for FFI where FFI: From<&'a T> + 'a, T: From<&'a FFI> + 'a {
        unsafe fn ffi_from_const(ffi: *const Self) -> T {
            T::from(&*ffi)
        }

        unsafe fn ffi_to_const(obj: &'a T) -> *const Self {
            boxed(obj.into())
        }
    }

    // impl<E, FFI> FFIConversion<E> for FFI where E: std::error::Error {
    //     unsafe fn ffi_from_const(ffi: *const Self) -> E {
    //         <Self as FFIConversion<E>>::ffi_from_const(ffi)
    //     }
    //
    //     unsafe fn ffi_to_const(obj: E) -> *const Self {
    //         <Self as FFIConversion<E>>::ffi_to_const(*obj)
    //     }
    // }
}
