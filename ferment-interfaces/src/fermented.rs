pub mod types {
    use crate::{boxed, FFIConversion, OpaqueContext, OpaqueContextMut};

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
}