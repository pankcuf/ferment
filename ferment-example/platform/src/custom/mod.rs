use ferment::{boxed, unbox_any, FFIConversionFrom, FFIConversionTo};

pub mod anyhow;
pub mod dashcore;
pub mod dpp;
pub mod jsonschema;
pub mod serde_json;
pub mod grovedb_version;
pub mod regex;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[ferment_macro::register(versioned_feature_core::FeatureVersion)]
pub struct versioned_feature_core_FeatureVersion {
    raw: *mut versioned_feature_core::FeatureVersion,
}
impl FFIConversionFrom<versioned_feature_core::FeatureVersion> for versioned_feature_core_FeatureVersion {
    unsafe fn ffi_from_const(ffi: *const Self) -> versioned_feature_core::FeatureVersion {
        FFIConversionFrom::ffi_from(ffi.cast_mut())
    }

    unsafe fn ffi_from(ffi: *mut Self) -> versioned_feature_core::FeatureVersion {
        *Box::from_raw((&*ffi).raw)
    }
}
impl FFIConversionTo<versioned_feature_core::FeatureVersion> for versioned_feature_core_FeatureVersion {
    unsafe fn ffi_to_const(obj: versioned_feature_core::FeatureVersion) -> *const Self {
        boxed(versioned_feature_core_FeatureVersion { raw: boxed(obj) })
    }
}

impl Drop for versioned_feature_core_FeatureVersion {
    fn drop(&mut self) {
        unsafe {
            unbox_any(self.raw);
        }
    }
}