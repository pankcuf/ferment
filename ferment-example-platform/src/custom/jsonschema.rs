use jsonschema::ValidationError;
use ferment_interfaces::{boxed, FFIConversion, unbox_any};

#[allow(non_camel_case_types)]
#[ferment_macro::register(jsonschema::ValidationError)]
pub struct jsonschema_ValidationError {
    raw: *mut ValidationError<'static>,
}
impl FFIConversion<ValidationError<'static>> for jsonschema_ValidationError {
    unsafe fn ffi_from_const(ffi: *const Self) -> ValidationError<'static> {
        FFIConversion::ffi_from(ffi as *mut Self)
    }

    unsafe fn ffi_from(ffi: *mut Self) -> ValidationError<'static> {
        *unbox_any((&*ffi).raw)
    }
    unsafe fn ffi_to_const(obj: ValidationError<'static>) -> *const Self {
        boxed(jsonschema_ValidationError { raw: boxed(obj) })
    }
}

impl Drop for jsonschema_ValidationError {
    fn drop(&mut self) {
        unsafe {
            unbox_any(self.raw);
        }
    }
}
