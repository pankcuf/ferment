use std::collections::BTreeMap;
use crate::nested::{BinaryData, Identifier, IdentifierBytes32, PlatformVersion, ProtocolError};

#[ferment_macro::export]
pub type KeyID = u32;
#[ferment_macro::export]
pub type TimestampMillis = u64;
#[ferment_macro::export]
pub type Revision = u64;

#[ferment_macro::export]
#[derive(Clone)]
pub enum Purpose {
    AUTHENTICATION = 0,
    ENCRYPTION = 1,
    DECRYPTION = 2,
    WITHDRAW = 3,
    SYSTEM = 4,
    VOTING = 5,
}
#[ferment_macro::export]
#[derive(Clone)]
pub enum SecurityLevel {
    MASTER = 0,
    CRITICAL = 1,
    HIGH = 2,
    MEDIUM = 3,
}

#[ferment_macro::export]
#[derive(Clone)]
pub enum Identity {
    V0(IdentityV0),
}

#[ferment_macro::export]
#[derive(Clone)]
pub enum IdentityPublicKey {
    V0(IdentityPublicKeyV0),
}


#[ferment_macro::export]
#[derive(Clone)]
pub enum ContractBounds {
    SingleContract { id: Identifier },
    SingleContractDocumentType  {
        id: Identifier,
        document_type_name: String,
    },
    // SingleContract { id: Identifier } = 0,
    // SingleContractDocumentType {
    //     id: Identifier,
    //     document_type_name: String,
    // } = 1,
}

#[allow(non_camel_case_types)]
#[ferment_macro::export]
#[derive(Clone)]
pub enum KeyType {
    ECDSA_SECP256K1 = 0,
    BLS12_381 = 1,
    ECDSA_HASH160 = 2,
    BIP13_SCRIPT_HASH = 3,
    EDDSA_25519_HASH160 = 4,
}

#[ferment_macro::export]
#[derive(Clone)]
pub struct IdentityPublicKeyV0 {
    pub id: KeyID,
    pub purpose: Purpose,
    pub security_level: SecurityLevel,
    pub contract_bounds: Option<ContractBounds>,
    pub key_type: KeyType,
    pub read_only: bool,
    pub data: BinaryData,
    pub disabled_at: Option<TimestampMillis>,
}


#[ferment_macro::export]
#[derive(Clone)]
pub struct IdentityV0 {
    pub id: Identifier,
    pub public_keys: BTreeMap<KeyID, IdentityPublicKey>,
    pub balance: u64,
    pub revision: Revision,
}

#[ferment_macro::export]
impl Identity {
    pub fn create_basic_identity(
        id: [u8; 32],
        _platform_version: &PlatformVersion,
    ) -> Result<Self, ProtocolError> {
        Ok(Self::create_basic_identity_v0(id))
    }
    pub fn create_basic_identity_v0(id: [u8; 32]) -> Self {
        Identity::V0(IdentityV0 {
            id: Identifier(IdentifierBytes32(id)),
            revision: 0,
            balance: 0,
            public_keys: BTreeMap::new(),
        })
    }

}