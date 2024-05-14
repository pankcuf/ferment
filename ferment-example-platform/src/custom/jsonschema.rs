#[allow(non_camel_case_types)]
#[ferment_macro::register(jsonschema::ValidationError)]
pub struct jsonschema_ValidationError {
    raw: Box<jsonschema::ValidationError<'static>>,
}
impl ferment_interfaces::FFIConversion<jsonschema::ValidationError<'static>> for jsonschema_ValidationError {
    unsafe fn ffi_from_const(ffi: *const Self) -> jsonschema::ValidationError<'static> {
        let ffi = &*ffi;
        *(*ffi).raw
    }
    unsafe fn ffi_to_const(obj: jsonschema::ValidationError<'static>) -> *const Self {
        ferment_interfaces::boxed(jsonschema_ValidationError { raw: Box::new(obj) })
    }
}
