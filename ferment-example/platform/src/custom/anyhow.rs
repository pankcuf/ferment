#[allow(non_camel_case_types)]
#[ferment_macro::register(anyhow::Error)]
#[derive(Clone)]
#[repr(C)]
pub struct anyhow_Error {
    raw_err: *mut anyhow::Error,
}
impl ferment::FFIConversionFrom<anyhow::Error> for anyhow_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> anyhow::Error {
        let ffi_ref = &*ffi;
        anyhow::Error::new(&**ffi_ref.raw_err)
    }
}
impl ferment::FFIConversionTo<anyhow::Error> for anyhow_Error {
    unsafe fn ffi_to_const(obj: anyhow::Error) -> *const Self {
        ferment::boxed(anyhow_Error { raw_err: ferment::boxed(obj) })
    }
}

impl Drop for anyhow_Error {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw_err);
        }
    }
}
