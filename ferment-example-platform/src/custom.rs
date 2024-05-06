use dashcore::hashes::Hash;
use dashcore::secp256k1::ThirtyTwoByteHash;
use dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator;

#[allow(non_camel_case_types)]
#[ferment_macro::register(serde_json::Value)]
#[derive(Clone)]
pub struct serde_json_JsonValue {
    raw_err: *mut serde_json::Value,
}
impl ferment_interfaces::FFIConversion<serde_json::Value> for serde_json_JsonValue {
    unsafe fn ffi_from_const(ffi: *const Self) -> serde_json::Value {
        let ffi = &*ffi;
        serde_json::Value::try_from(&*ffi.raw_err).expect("err")
    }
    unsafe fn ffi_to_const(obj: serde_json::Value) -> *const Self {
        ferment_interfaces::boxed(serde_json_JsonValue { raw_err: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for serde_json_JsonValue {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw_err);
        }
    }
}
#[allow(non_camel_case_types)]
#[ferment_macro::register(platform_value::Value)]
#[derive(Clone)]
pub struct platform_value_Value {
    raw_err: *mut platform_value::Value,
}
impl ferment_interfaces::FFIConversion<platform_value::Value> for platform_value_Value {
    unsafe fn ffi_from_const(ffi: *const Self) -> platform_value::Value {
        let ffi = &*ffi;
        platform_value::Value::try_from(&*ffi.raw_err).expect("err")
    }
    unsafe fn ffi_to_const(obj: platform_value::Value) -> *const Self {
        ferment_interfaces::boxed(platform_value_Value { raw_err: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for platform_value_Value {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw_err);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(jsonschema::ValidationError)]
#[derive(Clone)]
pub struct jsonschema_ValidationError {
    raw_err: *mut jsonschema::ValidationError<'static>,
}
impl ferment_interfaces::FFIConversion<jsonschema::ValidationError<'static>> for jsonschema_ValidationError {
    unsafe fn ffi_from_const(ffi: *const Self) -> jsonschema::ValidationError<'static> {
        let ffi = &*ffi;
        jsonschema::ValidationError::try_from(&*ffi.raw_err).expect("err")
    }
    unsafe fn ffi_to_const(obj: jsonschema::ValidationError<'static>) -> *const Self {
        ferment_interfaces::boxed(jsonschema_ValidationError { raw_err: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for jsonschema_ValidationError {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw_err);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(serde_json::Error)]
#[derive(Clone)]
pub struct serde_json_Error {
    raw_err: *mut serde_json::Error,
}
// #[allow(non_camel_case_types)]
// #[ferment_macro::register(serde_json::Map)]
// #[derive(Clone)]
// pub struct serde_json_Map {
//     raw_err: *mut serde_json::Map<>,
// }

impl ferment_interfaces::FFIConversion<serde_json::Error> for serde_json_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> serde_json::Error {
        let ffi = &*ffi;
        serde_json::Error::try_from(&*ffi.raw_err).expect("err")
    }
    unsafe fn ffi_to_const(obj: serde_json::Error) -> *const Self {
        ferment_interfaces::boxed(serde_json_Error { raw_err: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for serde_json_Error {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw_err);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(serde_json::Map)]
#[derive(Clone)]
pub struct serde_json_Map_keys_String_values_platform_value_Value {
    raw: *mut serde_json::Map<String, platform_value::Value>,
}

impl ferment_interfaces::FFIConversion<serde_json::Map<String, platform_value::Value>> for serde_json_Map_keys_String_values_platform_value_Value {
    unsafe fn ffi_from_const(ffi: *const Self) -> serde_json::Map<String, platform_value::Value> {
        let ffi = &*ffi;
        serde_json::Map::try_from(&*ffi.raw).expect("err")
    }
    unsafe fn ffi_to_const(obj: serde_json::Map<String, platform_value::Value>) -> *const Self {
        ferment_interfaces::boxed(serde_json_Map_keys_String_values_platform_value_Value { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for serde_json_Map_keys_String_values_platform_value_Value {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}

// serde_json::Map<String, Value>
#[allow(non_camel_case_types)]
#[ferment_macro::register(anyhow::Error)]
#[derive(Clone)]
pub struct anyhow_Error {
    raw_err: *mut anyhow::Error,
}
impl ferment_interfaces::FFIConversion<anyhow::Error> for anyhow_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> anyhow::Error {
        let ffi = &*ffi;
        anyhow::Error::try_from(&*ffi.raw_err).expect("err")
    }
    unsafe fn ffi_to_const(obj: anyhow::Error) -> *const Self {
        ferment_interfaces::boxed(anyhow_Error { raw_err: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for anyhow_Error {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw_err);
        }
    }
}
#[allow(non_camel_case_types)]
#[ferment_macro::register(dpp::validation::JsonSchemaValidator)]
#[derive(Clone)]
pub struct dpp_validation_JsonSchemaValidator {
    validator: *mut std::os::raw::c_void,
}
impl ferment_interfaces::FFIConversion<dpp::validation::JsonSchemaValidator> for dpp_validation_JsonSchemaValidator {
    unsafe fn ffi_from_const(ffi: *const Self) -> dpp::validation::JsonSchemaValidator {
        let ffi = &*ffi;
        // validator: RwLock<Option<JSONSchema>>
        dpp::validation::JsonSchemaValidator::try_from(&*ffi.validator).expect("err")
    }
    unsafe fn ffi_to_const(obj: dpp::validation::JsonSchemaValidator) -> *const Self {
        ferment_interfaces::boxed(dpp_validation_JsonSchemaValidator { validator: ferment_interfaces::boxed(obj) as *mut std::os::raw::c_void })
    }
}

impl Drop for dpp_validation_JsonSchemaValidator {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.validator);
        }
    }
}


/*#[allow(non_camel_case_types)]
#[ferment_macro::register(indexmap::IndexMap)]
#[derive(Clone)]
pub struct indexmap_IndexMap {

    raw: *mut std::os::raw::c_void,
}
impl<K, V> ferment_interfaces::FFIConversion<indexmap::IndexMap<K, V, RandomState>> for indexmap_IndexMap {
    unsafe fn ffi_from_const(ffi: *const Self) -> indexmap::IndexMap<K, V, RandomState> {
        let ffi = &*ffi;
        indexmap::IndexMap::try_from(&*ffi.raw).expect("err")
    }
    unsafe fn ffi_to_const(obj: indexmap::IndexMap<K, V, RandomState>) -> *const Self {
        ferment_interfaces::boxed(indexmap_IndexMap { raw: ferment_interfaces::boxed(obj) as *mut std::os::raw::c_void })
    }
}

impl Drop for indexmap_IndexMap {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}
*/
#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::blockdata::transaction::OutPoint)]
#[derive(Clone)]
pub struct OutPoint {
    pub txid: *mut [u8; 32],
    pub vout: u32,
}
impl ferment_interfaces::FFIConversion<dashcore::blockdata::transaction::OutPoint> for OutPoint {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::blockdata::transaction::OutPoint {
        let ffi = &*ffi;
        dashcore::blockdata::transaction::OutPoint::new(dashcore::hash_types::Txid::from_slice(&*ffi.txid).expect("err"), ffi.vout)
    }
    unsafe fn ffi_to_const(obj: dashcore::blockdata::transaction::OutPoint) -> *const Self {
        ferment_interfaces::boxed(OutPoint { txid: ferment_interfaces::boxed(obj.txid.to_raw_hash().into_32()), vout: obj.vout })
    }
}

impl Drop for OutPoint {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.txid);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::ephemerealdata::instant_lock::InstantLock)]
#[derive(Clone)]
pub struct InstantLock {
    pub raw: *mut dashcore::ephemerealdata::instant_lock::InstantLock,
}
impl ferment_interfaces::FFIConversion<dashcore::ephemerealdata::instant_lock::InstantLock> for InstantLock {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::ephemerealdata::instant_lock::InstantLock {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dashcore::ephemerealdata::instant_lock::InstantLock) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for InstantLock {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::blockdata::transaction::Transaction)]
#[derive(Clone)]
pub struct Transaction {
    pub raw: *mut dashcore::blockdata::transaction::Transaction,
}
impl ferment_interfaces::FFIConversion<dashcore::blockdata::transaction::Transaction> for Transaction {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::blockdata::transaction::Transaction {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dashcore::blockdata::transaction::Transaction) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for InstantLock {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(dpp::identity::core_script::CoreScript)]
#[derive(Clone)]
pub struct CoreScriptFFI {
    pub raw: *mut dpp::identity::core_script::CoreScript,
}
impl ferment_interfaces::FFIConversion<dpp::identity::core_script::CoreScript> for CoreScriptFFI {
    unsafe fn ffi_from_const(ffi: *const Self) -> dpp::identity::core_script::CoreScript {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dpp::identity::core_script::CoreScript) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for CoreScriptFFI {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}
#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::consensus::encode::Error)]
#[derive(Clone)]
pub struct dashcore_consensus_encode_Error {
    pub raw: *mut dashcore::consensus::encode::Error,
}
impl ferment_interfaces::FFIConversion<dashcore::consensus::encode::Error> for dashcore_consensus_encode_Error {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::consensus::encode::Error {
        &**ffi
    }
    unsafe fn ffi_to_const(obj: dashcore::consensus::encode::Error) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for dashcore_consensus_encode_Error {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}
#[allow(non_camel_case_types)]
#[ferment_macro::register(StatelessJsonSchemaLazyValidator)]
#[derive(Clone)]
pub struct StatelessJsonSchemaLazyValidatorFFI {
    pub raw: *mut StatelessJsonSchemaLazyValidator,
}
impl ferment_interfaces::FFIConversion<StatelessJsonSchemaLazyValidator> for StatelessJsonSchemaLazyValidatorFFI {
    unsafe fn ffi_from_const(ffi: *const Self) -> StatelessJsonSchemaLazyValidator {
        &**ffi
    }
    unsafe fn ffi_to_const(obj: StatelessJsonSchemaLazyValidator) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for StatelessJsonSchemaLazyValidatorFFI {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}


