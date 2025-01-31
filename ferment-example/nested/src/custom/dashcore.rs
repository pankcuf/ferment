use dashcore::hashes::Hash;

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::hash_types::Txid)]
pub struct dashcore_Txid(pub *mut [u8; dashcore::hash_types::Txid::LEN]);
#[no_mangle]
pub unsafe extern "C" fn dashcore_Txid_ctor(hash: *mut [u8; dashcore::hash_types::Txid::LEN]) -> dashcore_Txid {
    dashcore_Txid(hash)
}
#[no_mangle]
pub unsafe extern "C" fn dashcore_Txid_destroy(ptr: *mut dashcore_Txid) {
    ferment::unbox_any(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn dashcore_Txid_inner(ptr: *mut dashcore_Txid) -> *mut [u8; dashcore::hash_types::Txid::LEN] {
    (&*ptr).0
}
impl ferment::FFIConversionFrom<dashcore::hash_types::Txid> for dashcore_Txid {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::hash_types::Txid {
        dashcore::hash_types::Txid::from_slice(&*(&*ffi).0).expect("TxId error")
    }
}
impl ferment::FFIConversionTo<dashcore::hash_types::Txid> for dashcore_Txid {
    unsafe fn ffi_to_const(obj: dashcore::hash_types::Txid) -> *const Self {
        ferment::boxed(Self(ferment::boxed(obj.into())))
    }
}
impl Drop for dashcore_Txid {
    fn drop(&mut self) { unsafe { ferment::unbox_any(self.0); } }
}
