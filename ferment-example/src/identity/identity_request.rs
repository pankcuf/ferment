#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct ResponseMetadata {
    pub height: u64,
    pub core_chain_locked_height: u32,
    pub epoch: u32,
    pub time_ms: u64,
    pub protocol_version: u32,
    pub chain_id: String,
}

#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct Proof {
    pub grovedb_proof: Vec<u8>,
    pub quorum_hash: Vec<u8>,
    pub signature: Vec<u8>,
    pub round: u32,
    pub block_id_hash: Vec<u8>,
    pub quorum_type: u32,
}

#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct GetIdentityRequest {
    pub version: Option<get_identity_request::Version>,
}
#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct GetIdentityResponse {
    pub version: Option<get_identity_response::Version>,
}

#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct GetIdentityByPublicKeyHashRequest {
    pub version: Option<get_identity_by_public_key_hash_request::Version>,
}
#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub struct GetIdentityByPublicKeyHashResponse {
    pub version: Option<get_identity_by_public_key_hash_response::Version>,
}

#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub enum IdentityRequest {
    GetIdentity(GetIdentityRequest),
    GetIdentityByPublicKeyHash(GetIdentityByPublicKeyHashRequest)
}

#[derive(Clone, PartialEq)]
#[ferment_macro::export]
pub enum IdentityResponse {
    Unknown,
    GetIdentity(GetIdentityResponse),
    GetIdentityByPublicKeyHash(GetIdentityByPublicKeyHashResponse)
}


pub mod get_identity_request {
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub struct GetIdentityRequestV0 {
        pub id: Vec<u8>,
        pub prove: bool,
    }
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub enum Version {
        V0(GetIdentityRequestV0),
    }
}
pub mod get_identity_response {
    use crate::identity::identity_request::ResponseMetadata;

    #[ferment_macro::export]
    pub struct GetIdentityResponseV0 {
        pub metadata: Option<ResponseMetadata>,
        pub result: Option<get_identity_response_v0::Result>,
    }


    pub mod get_identity_response_v0 {
        use crate::identity::identity_request::Proof;

        #[derive(Clone, PartialEq)]
        #[ferment_macro::export]
        pub enum Result {
            Identity(Vec<u8>),
            Proof(Proof),
        }
    }
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub enum Version {
        V0(GetIdentityResponseV0),
    }
}
pub mod get_identity_by_public_key_hash_request {
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub struct GetIdentityByPublicKeyHashRequestV0 {
        pub public_key_hash: Vec<u8>,
        pub prove: bool,
    }
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub enum Version {
        V0(GetIdentityByPublicKeyHashRequestV0),
    }
}

pub mod get_identity_by_public_key_hash_response {
    use crate::identity::identity_request::ResponseMetadata;

    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub struct GetIdentityByPublicKeyHashResponseV0 {
        pub metadata: Option<ResponseMetadata>,
        pub result: Option<get_identity_by_public_key_hash_response_v0::Result>,
    }
    pub mod get_identity_by_public_key_hash_response_v0 {
        use crate::identity::identity_request::Proof;

        #[derive(Clone, PartialEq)]
        #[ferment_macro::export]
        pub enum Result {
            Identity(Vec<u8>),
            Proof(Proof),
        }
    }
    #[derive(Clone, PartialEq)]
    #[ferment_macro::export]
    pub enum Version {
        V0(GetIdentityByPublicKeyHashResponseV0),
    }
}
