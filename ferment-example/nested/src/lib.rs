mod fermented;
mod model;
mod gen;
mod entry;
mod state_transition;


extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
    pub names: &'static str,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[ferment_macro::register(std::time::Duration)]
pub struct std_time_Duration2 {
    secs: u64,
    nanos: u32,
}
ferment::impl_custom_conversion!(std::time::Duration, std_time_Duration2,
    |value: &std_time_Duration2| std::time::Duration::new(value.secs, value.nanos),
    |value: &std::time::Duration| Self { secs: value.as_secs(), nanos: value.subsec_nanos() }
);

#[allow(non_camel_case_types)]
#[ferment_macro::register(regex::Regex)]
pub struct regex_Regex {
    raw: *mut regex::Regex,
}

impl ferment::FFIConversionFrom<regex::Regex> for regex_Regex {
    unsafe fn ffi_from_const(ffi: *const Self) -> regex::Regex {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
}
impl ferment::FFIConversionTo<regex::Regex> for regex_Regex {
    unsafe fn ffi_to_const(obj: regex::Regex) -> *const Self {
        ferment::boxed(Self { raw: ferment::boxed(obj) })
    }
}

impl Drop for regex_Regex {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(serde_json::Error)]
pub struct serde_json_Error {
    raw: *mut serde_json::Error,
}
impl ferment::FFIConversionFrom<serde_json::Error> for serde_json_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> serde_json::Error {
        ferment::FFIConversionFrom::ffi_from(ffi.cast_mut())
    }
    unsafe fn ffi_from(ffi: *mut Self) -> serde_json::Error {
        *ferment::unbox_any((&*ffi).raw)
    }
}
impl ferment::FFIConversionTo<serde_json::Error> for serde_json_Error {
    unsafe fn ffi_to_const(obj: serde_json::Error) -> *const Self {
        ferment::boxed(serde_json_Error { raw: ferment::boxed(obj) })
    }
}

impl Drop for serde_json_Error {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::consensus::encode::Error)]
pub struct dashcore_consensus_Error {
    pub raw: *mut dashcore::consensus::encode::Error,
}

impl ferment::FFIConversionFrom<dashcore::consensus::encode::Error> for dashcore_consensus_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::consensus::encode::Error {
        *ferment::unbox_any((&*ffi).raw)
    }
}
impl ferment::FFIConversionTo<dashcore::consensus::encode::Error> for dashcore_consensus_Error {
    unsafe fn ffi_to_const(obj: dashcore::consensus::encode::Error) -> *const Self {
        ferment::boxed(Self { raw: ferment::boxed(obj.into()) })
    }
}

impl Drop for dashcore_consensus_Error {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}

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

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[ferment_macro::register(versioned_feature_core::FeatureVersion)]
pub struct versioned_feature_core_FeatureVersion {
    raw: *mut versioned_feature_core::FeatureVersion,
}
impl ferment::FFIConversionFrom<versioned_feature_core::FeatureVersion> for versioned_feature_core_FeatureVersion {
    unsafe fn ffi_from_const(ffi: *const Self) -> versioned_feature_core::FeatureVersion {
        ferment::FFIConversionFrom::ffi_from(ffi.cast_mut())
    }

    unsafe fn ffi_from(ffi: *mut Self) -> versioned_feature_core::FeatureVersion {
        *ferment::unbox_any((&*ffi).raw)
    }
}
impl ferment::FFIConversionTo<versioned_feature_core::FeatureVersion> for versioned_feature_core_FeatureVersion {
    unsafe fn ffi_to_const(obj: versioned_feature_core::FeatureVersion) -> *const Self {
        ferment::boxed(versioned_feature_core_FeatureVersion { raw: ferment::boxed(obj) })
    }
}

impl Drop for versioned_feature_core_FeatureVersion {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[ferment_macro::register(serde_json::Value)]
pub struct serde_json_Value {
    raw: *mut serde_json::Value,
}
impl ferment::FFIConversionFrom<serde_json::Value> for serde_json_Value {
    unsafe fn ffi_from_const(ffi: *const Self) -> serde_json::Value {
        ferment::FFIConversionFrom::ffi_from(ffi.cast_mut())
    }

    unsafe fn ffi_from(ffi: *mut Self) -> serde_json::Value {
        *ferment::unbox_any((&*ffi).raw)
    }
}
impl ferment::FFIConversionTo<serde_json::Value> for serde_json_Value {
    unsafe fn ffi_to_const(obj: serde_json::Value) -> *const Self {
        ferment::boxed(serde_json_Value { raw: ferment::boxed(obj) })
    }
}

impl Drop for serde_json_Value {
    fn drop(&mut self) {
        unsafe {
            ferment::unbox_any(self.raw);
        }
    }
}
