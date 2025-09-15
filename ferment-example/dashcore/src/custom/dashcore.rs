use dashcore::hashes::Hash;

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::hash_types::Txid)]
pub struct dashcore_Txid {
    pub raw: *mut [u8; 32],
}

impl ferment::FFIConversionFrom<dashcore::hash_types::Txid> for dashcore_Txid {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::hash_types::Txid {
        let ffi_ref = &*ffi;
        dashcore::hash_types::Txid::from_slice(&*ffi_ref.raw).expect("TxId error")
    }
}
impl ferment::FFIConversionTo<dashcore::hash_types::Txid> for dashcore_Txid {
    unsafe fn ffi_to_const(obj: dashcore::hash_types::Txid) -> *const Self {
        ferment::boxed(Self { raw: ferment::boxed(obj.into()) })
    }
}

impl Drop for dashcore_Txid {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}
