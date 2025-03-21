use std::ptr::null;
use std::sync::Arc;

#[derive(Clone)]
#[ferment_macro::export]
pub enum KeyChainKey {
    StandaloneInfoDictionaryLocationString { extended_public_key_identifier: String },
    StandaloneExtendedPublicKeyLocationString { extended_public_key_identifier: String },
    HasKnownBalanceUniqueIDString { reference: u32, unique_id: String },
    WalletBasedExtendedPrivateKeyLocationString { unique_id: String },
    WalletBasedExtendedPublicKeyLocationString { unique_id: String },
}

impl KeyChainKey {
    pub fn standalone_info_dictionary_location_string(extended_public_key_identifier: String) -> Self {
        Self::StandaloneInfoDictionaryLocationString { extended_public_key_identifier }
    }
    pub fn standalone_extended_public_key_location_string(extended_public_key_identifier: String) -> Self {
        Self::StandaloneExtendedPublicKeyLocationString { extended_public_key_identifier }
    }
    pub fn has_known_balance_unique_id_string(reference: u32, unique_id: String) -> Self {
        Self::HasKnownBalanceUniqueIDString { reference, unique_id }
    }
    pub fn wallet_based_extended_private_key_location_string(unique_id: String) -> Self {
        Self::WalletBasedExtendedPrivateKeyLocationString { unique_id }
    }
    pub fn wallet_based_extended_public_key_location_string(unique_id: String) -> Self {
        Self::WalletBasedExtendedPublicKeyLocationString { unique_id }
    }
}

#[derive(Clone)]
#[ferment_macro::export]
pub enum KeyChainValue {
    Bytes(Vec<u8>),
    Int64(i64),
    String(String),
}

#[derive(Clone)]
#[ferment_macro::export]
pub enum KeyChainError {
    OsStatusCode(i32)
}

#[derive(Clone)]
// #[ferment_macro::opaque]
pub struct KeychainController {
    pub get: Arc<dyn Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<KeyChainValue, KeyChainError> + Send + Sync>,
    pub set: Arc<dyn Fn(*const std::os::raw::c_void, KeyChainKey, KeyChainValue, bool) -> Result<bool, KeyChainError> + Send + Sync>,
    pub has: Arc<dyn Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<bool, KeyChainError> + Send + Sync>,
    pub delete: Arc<dyn Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<bool, KeyChainError> + Send + Sync>,
}

#[ferment_macro::export]
impl KeychainController {
    pub fn new<
        GET: Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<KeyChainValue, KeyChainError> + Send + Sync + 'static,
        SET: Fn(*const std::os::raw::c_void, KeyChainKey, KeyChainValue, bool) -> Result<bool, KeyChainError> + Send + Sync + 'static,
        HAS: Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<bool, KeyChainError> + Send + Sync + 'static,
        DEL: Fn(*const std::os::raw::c_void, KeyChainKey) -> Result<bool, KeyChainError> + Send + Sync + 'static,
    >(
        get: GET,
        set: SET,
        has: HAS,
        delete: DEL,
    ) -> Self {
        Self {
            get: Arc::new(get),
            set: Arc::new(set),
            has: Arc::new(has),
            delete: Arc::new(delete),
        }
    }
    pub fn get(&self, key: KeyChainKey) -> Result<KeyChainValue, KeyChainError> {
        (self.get)(null(), key)
    }
    pub fn set(&self, key: KeyChainKey, value: KeyChainValue, authenticated: bool) -> Result<bool, KeyChainError> {
        (self.set)(null(), key, value, authenticated)
    }
    pub fn has(&self, key: KeyChainKey) -> Result<bool, KeyChainError> {
        (self.has)(null(), key)
    }
    pub fn delete(&self, key: KeyChainKey) -> Result<bool, KeyChainError> {
        (self.delete)(null(), key)
    }

    pub fn mark_already_joined_queue_as_tried(&mut self, _dsq: &mut KeyChainKey) -> bool {
        true
    }
}