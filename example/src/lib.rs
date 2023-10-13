mod chain;
mod example;
mod ffi_expansions;

extern crate rs_ffi_macro_derive;

use rs_ffi_macro_derive::ferment;

#[ferment]
pub struct RootStruct {
    pub name: String,
}
// pub struct RootStruct<'a> {
//     pub name: &'a str,
// }

pub mod ffi {
    use rs_ffi_macro_derive::ferment;
    use std::collections::BTreeMap;

    #[ferment]
    pub type KeyID = u32;

    #[ferment]
    pub type Hash160 = [u8; 20];

    #[ferment]
    pub type HashID = [u8; 32];

    #[ferment]
    pub type UsedKeyMatrix = Vec<bool>;

    #[ferment]
    pub type UsedStruct = HashID;

    #[ferment]
    pub type ArrayOfArraysOfHashes = Vec<Vec<HashID>>;

    #[ferment]
    pub type MapOfHashes = BTreeMap<HashID, HashID>;

    #[ferment]
    pub type MapOfVecHashes = BTreeMap<HashID, Vec<HashID>>;

    #[ferment]
    pub struct BinaryData(pub Vec<u8>);

    #[ferment]
    pub struct SimpleData(pub Vec<u32>);

    #[ferment]
    pub struct IdentifierBytes32(pub [u8; 32]);

    #[ferment]
    pub struct UnnamedPair(pub [u8; 32], pub u32);

    #[ferment]
    pub struct Identifier(pub IdentifierBytes32);

    #[ferment]
    pub enum TestEnum {
        Variant1(String),
        Variant2,
        Variant3(HashID, u32),
        Variant4(HashID, u32, String),
        Variant5(BTreeMap<String, HashID>, u32, String),
    }

    #[ferment]
    pub struct DataContractNotPresentError {
        pub data_contract_id: Identifier,
    }

    #[ferment]
    pub enum ProtocolError {
        IdentifierError(String),
        StringDecodeError(String),
        StringDecodeError2(String, u32),
        EmptyPublicKeyDataError,
        MaxEncodedBytesReachedError {
            max_size_kbytes: usize,
            size_hit: usize,
        },
        EncodingError(String),
        EncodingError2(&'static str),
        DataContractNotPresentError(DataContractNotPresentError),
    }

    #[ferment]
    pub type AddInsightCallback = fn(block_hash: HashID, context: rs_ffi_interfaces::OpaqueContext);

    #[ferment]
    pub type ShouldProcessDiffWithRangeCallback = fn(
        base_block_hash: HashID,
        block_hash: HashID,
        context: rs_ffi_interfaces::OpaqueContext,
    ) -> ProtocolError;

    // #[ferment]
    // pub fn address_from_hash(hash: HashID) -> String {
    //     format_args!("{0:?}", hash).to_string()
    // }
    //
    // #[ferment]
    // pub fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
    //     Some(format_args!("{0:?}", script).to_string())
    // }
    //
    // #[ferment]
    // pub fn address_from_hash160(hash: Hash160, pubkey: u8) -> String {
    //     let mut writer: Vec<u8> = Vec::new();
    //     writer.push(pubkey);
    //     writer.extend(hash);
    //     String::from_utf8(writer).unwrap()
    // }
    //
    // #[ferment]
    // pub fn address_from_hash160_2(
    //     chain_type: dash_spv_masternode_processor::chain::common::chain_type::ChainType,
    // ) -> String {
    //     chain_type.get_string()
    // }
    //
    #[ferment]
    pub fn find_hash_by_u32(key: u32, map: BTreeMap<u32, HashID>) -> Option<HashID> {
        map.get(&key).clone().copied()
    }

    #[ferment]
    pub struct TestStruct {
        pub vec_u8: Vec<u8>,
        pub vec_u32: Vec<u32>,
        pub vec_vec_u32: Vec<Vec<u32>>,
        pub map_key_simple_value_simple: BTreeMap<u32, u32>,
        pub map_key_simple_value_complex: BTreeMap<u32, HashID>,
        pub map_key_simple_value_vec_simple: BTreeMap<u32, Vec<u32>>,
        pub map_key_simple_value_vec_complex: BTreeMap<u32, Vec<HashID>>,
        pub map_key_simple_value_map_key_simple_value_simple: BTreeMap<u32, BTreeMap<u32, u32>>,
        pub map_key_simple_value_map_key_simple_value_complex: BTreeMap<u32, BTreeMap<u32, HashID>>,
        pub map_key_simple_value_map_key_simple_value_vec_simple:
            BTreeMap<u32, BTreeMap<u32, Vec<u32>>>,
        pub map_key_simple_value_map_key_simple_value_vec_complex:
            BTreeMap<u32, BTreeMap<u32, Vec<HashID>>>,

        pub map_key_complex_value_simple: BTreeMap<HashID, u32>,
        pub map_key_complex_value_complex: BTreeMap<HashID, HashID>,
        pub map_key_complex_value_vec_simple: BTreeMap<HashID, Vec<u32>>,
        pub map_key_complex_value_vec_complex: BTreeMap<HashID, Vec<HashID>>,

        pub map_key_complex_value_map_key_simple_value_vec_simple:
            BTreeMap<HashID, BTreeMap<u32, Vec<u32>>>,
        pub map_key_complex_value_map_key_simple_value_vec_complex:
            BTreeMap<HashID, BTreeMap<u32, Vec<HashID>>>,

        pub map_key_complex_value_map_key_simple_value_map_key_complex_value_complex:
            BTreeMap<HashID, BTreeMap<u32, BTreeMap<HashID, HashID>>>,
    }
}

// mod trait_bindings {
//     pub struct DashPlatformSdk {
//         dapi: tokio::sync::RwLock<crate::platform::PlatformClient>,
//         quorum_provider: Box<dyn QuorumInfoProvider>,
//     }
//
// // async fn fetch<Q: Query<Self::Request>>(api: &API, query: Q) -> Result<Option<Self>, Error> {
//
//     // DashPlatformSdk_FFI
//     pub trait Query<T: TransportRequest>: Send + Debug + Clone {
//         fn query(self) -> Result<T, Error>;
//     }
//
//     pub trait TransportRequest: Clone + Send + Sync + Debug {
//         type Client: TransportClient;
//         type Response: Clone + Send + Sync + Debug;
//         const SETTINGS_OVERRIDES: RequestSettings;
//         fn execute_transport<'c>(
//             self,
//             client: &'c mut Self::Client,
//             settings: &AppliedRequestSettings)
//             -> BoxFuture<'c, Result<Self::Response, <Self::Client as TransportClient>::Error>>;
//     }
//
//     pub trait TransportClient: Send {
//         type Error: CanRetry + Send + Debug;
//         fn with_uri(uri: Uri) -> Self;
//     }
//
//     pub trait Sdk: Send + Sync {
//         async fn platform_client<'a>(&self) -> RwLockWriteGuard<'a, crate::platform::PlatformClient>
//             where 'life0: 'a;
//         fn quorum_info_provider<'a>(&'a self) -> Result<&'a dyn QuorumInfoProvider, Error>;
//     }
//
//     impl Sdk for DashPlatformSdk {
//         async fn platform_client<'a>(&self) -> RwLockWriteGuard<'a, crate::platform::PlatformClient>
//             where
//                 'life0: 'a,
//         {
//             self.dapi.write().await
//         }
//         fn quorum_info_provider<'a>(&'a self) -> Result<&'a dyn QuorumInfoProvider, Error> {
//             let provider = self.quorum_provider.as_ref();
//             Ok(provider)
//         }
//     }
//     pub trait Fetch<API: Sdk>
//         where
//             Self: Sized + Debug + FromProof<Self::Request>,
//             <Self as FromProof<Self::Request>>::Response:
//             From<<Self::Request as TransportRequest>::Response>,
//     {
//         type Request: TransportRequest;
//         async fn fetch<Q: Query<Self::Request>>(api: &API, query: Q) -> Result<Option<Self>, Error> {
//             let request = query.query()?;
//             let mut client = api.platform_client().await;
//             let response = request
//                 .clone()
//                 .execute(&mut client, RequestSettings::default())
//                 .await?;
//             let response = response.into();
//             let object = Self::maybe_from_proof(&request, &response, api.quorum_info_provider()?)?;
//             match object {
//                 Some(item) => Ok(item.into()),
//                 None => Ok(None),
//             }
//         }
//     }
//
//     impl<API: Sdk> Fetch<API> for dpp::prelude::Identity {
//         type Request = platform_proto::GetIdentityRequest;
//     }
//
//     impl<API: Sdk> Fetch<API> for dpp::prelude::DataContract {
//         type Request = platform_proto::GetDataContractRequest;
//     }
// }
//
// mod expanded_trait {
//     use crate::trait_bindings::Fetch;
//     pub type Error = ();
//     pub struct Fetch_API_DashPlatformSdk_FFI;
//     pub struct Fetch_API_FFI;
//     pub struct Particular_Query_FFI;
//
//     pub struct DashPlatformSdk_FFI {
//         dapi: *mut tokio::sync::RwLock<crate::platform::PlatformClient>,
//         quorum_provider: dyn QuorumInfoProvider,
//     }
//
//
//     #[no_mangle]
//     pub unsafe extern "C" fn ffi_fetch_fetch(api: *mut DashPlatformSdk_FFI, query: *mut Particular_Query_FFI) -> Result<Option<Fetch_API_FFI>, Error> {
//
//     }
// // pub extern "C" fn address_with_script_pubkey(script: Vec<u8>) -> Option<String> {
// //
// // }
//
//
// }

