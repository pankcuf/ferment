use ferment_interfaces::{boxed, FFIConversion, unbox_any};

#[allow(non_camel_case_types)]
#[ferment_macro::register(jsonschema::ValidationError)]
pub struct jsonschema_ValidationError {
    raw: *mut jsonschema::ValidationError<'static>,
}
impl FFIConversion<jsonschema::ValidationError<'static>> for jsonschema_ValidationError {
    unsafe fn ffi_from_const(ffi: *const Self) -> jsonschema::ValidationError<'static> {
        FFIConversion::ffi_from(ffi as *mut Self)
    }

    unsafe fn ffi_from(ffi: *mut Self) -> jsonschema::ValidationError<'static> {
        *unbox_any((&*ffi).raw)
    }
    unsafe fn ffi_to_const(obj: jsonschema::ValidationError<'static>) -> *const Self {
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
