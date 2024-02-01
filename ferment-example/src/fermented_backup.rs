#[allow(
clippy::let_and_return,
clippy::suspicious_else_formatting,
clippy::redundant_field_names,
dead_code,
non_camel_case_types,
non_snake_case,
redundant_semicolons,
unused_braces,
unused_imports,
unused_unsafe,
unused_variables,
unused_qualifications
)]
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_Empty_Error {}
    pub mod std {

        pub mod fmt {
            pub struct Result {
                pub object: *const (),
                pub vtable: *const Result_VTable,
            }
            pub struct Result_VTable {}
        }
    }
    pub struct Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry {
        pub ok: *mut crate::fermented::types::r#crate::asyn::query::TransportResponse,
        pub err: *mut crate::fermented::types::r#crate::asyn::query::CanRetry,
    }

    // pub struct Result<T, Box<dyn Error>>
    pub struct Result_ok_crate_asyn_query_TransportRequest_err_std_error_Error {
        pub ok: *mut crate::fermented::types::r#crate::asyn::query::TransportRequest_where_Client_is_TransportClient_where_Response_is_TransportResponse,
        pub err: *mut crate::fermented::types::std::error::Error,
    }

    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut *mut u8,
    }

    pub struct Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError {
        pub ok: *mut [u8; 48],
        pub err: *mut crate::fermented::types::r#crate::asyn::provider::ContextProviderError,
    }
    pub struct Result_ok_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError {
        pub ok: *mut [u8; 48],
        pub err: *mut crate::fermented::types::r#crate::asyn::provider::ContextProviderError,
    }

    pub struct Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
        pub ok: *mut crate::fermented::types::r#crate::asyn::proof::FromProof,
        pub err: *mut crate::fermented::types::r#crate::nested::ProtocolError,
    }
}

#[allow(
clippy::let_and_return,
clippy::suspicious_else_formatting,
clippy::redundant_field_names,
dead_code,
non_camel_case_types,
non_snake_case,
redundant_semicolons,
unused_braces,
unused_imports,
unused_unsafe,
unused_variables,
unused_qualifications
)]
pub mod types {
    pub mod std {
        pub mod any {
            pub struct TypeId {
                t: u128,
            }
        }
        pub mod convert {
            pub struct Into_where_T_is_FromProof_Request {
                pub object: *const (),
                pub vtable: *const Into_where_T_is_FromProof_Request_VTable,
            }
            pub struct Into_where_T_is_FromProof_Request_VTable {
                pub into: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::types::r#crate::asyn::proof::FromProof,
            }
            pub struct Into_where_T_is_FromProof_Response {
                pub object: *const (),
                pub vtable: *const Into_where_T_is_FromProof_Response_VTable,
            }
            pub struct Into_where_T_is_FromProof_Response_VTable {
                pub into: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::types::r#crate::asyn::proof::FromProof,
            }

        }
        pub mod error {
            pub mod private {
                pub struct Internal;
            }

            pub struct Erased {
                pub object: *const (),
                pub vtable: *const Erased_VTable,
            }

            pub struct Erased_VTable {
                pub tag_id: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::types::std::any::TypeId,
            }

            pub struct Request(*mut Erased);

            pub struct Error {
                pub object: *const (),
                pub vtable: *const Error_VTable,
                pub vtable_Debug: *const crate::fermented::types::std::fmt::Debug,
                pub vtable_Display: *const crate::fermented::types::std::fmt::Display,
            }

            pub struct Error_VTable {
                pub source: unsafe extern "C" fn(r#self: *const Self) -> *mut Self,
                pub type_id: unsafe extern "C" fn(r#self: *const Self, _: private::Internal) -> *mut crate::fermented::types::std::any::TypeId,
                pub description: unsafe extern "C" fn(r#self: *const Self) -> *mut std::os::raw::c_char,
                pub cause: unsafe extern "C" fn(r#self: *const Self) -> *mut Error,
                pub provide: unsafe extern "C" fn(r#self: *const Self, request: *mut Request),
            }
        }

        pub mod fmt {
            pub mod rt {
                pub enum Count {
                    Is(usize),
                    Param(usize),
                    Implied,
                }

                pub enum Alignment {
                    Left,
                    Right,
                    Center,
                    Unknown,
                }

                pub struct Placeholder {
                    pub position: usize,
                    pub fill: *mut std::os::raw::c_char,
                    pub align: *mut Alignment,
                    pub flags: u32,
                    pub precision: *mut Count,
                    pub width: *mut Count,
                }

                pub struct Argument {
                    value: *mut std::os::raw::c_void,
                    formatter: unsafe extern "C" fn(*const std::os::raw::c_void, *mut super::Formatter) -> super::Result,
                }
            }

            pub struct Error {}

            pub struct Result(*mut crate::fermented::generics::Result_Empty_Error);

            pub struct Arguments {
                // pieces: &'a [&'static str], => Not implemented yet
                pieces: *mut *mut std::os::raw::c_char,
                fmt: *mut *mut rt::Placeholder,
                args: *mut *mut rt::Placeholder,
            }

            pub struct Write_VTable {
                pub write_str: unsafe extern "C" fn(r#self: *mut Self, s: *mut std::os::raw::c_char) -> *mut Result,
                pub write_char: unsafe extern "C" fn(r#self: *mut Self, c: *mut std::os::raw::c_char) -> *mut Result,
                pub write_fmt: unsafe extern "C" fn(r#self: *mut Self, args: *mut Arguments) -> *mut Result,
            }

            pub struct Write {
                pub object: *const (),
                pub vtable: *const Write_VTable,
            }

            pub struct Formatter {
                flags: u32,
                fill: *mut std::os::raw::c_char,
                align: *mut rt::Alignment,
                width: *mut usize,
                precision: *mut usize,
                buf: *mut Write,
            }

            pub struct Debug {
                pub object: *const (),
                pub vtable: *const Debug_VTable,
            }

            pub struct Debug_VTable {
                pub fmt: unsafe extern "C" fn(*const Self, f: *mut Formatter) -> *mut crate::fermented::generics::std::fmt::Result,
            }

            pub struct Display {
                pub object: *const (),
                pub vtable: *const Display_VTable,
            }

            pub struct Display_VTable {
                pub fmt: unsafe extern "C" fn(*const Self, f: *mut Formatter) -> *mut crate::fermented::generics::std::fmt::Result,
            }
        }
        pub mod future {
            pub struct Future {
                pub object: *const (),
                pub vtable: *const Future_VTable,
            }
            pub struct Future_VTable {
                pub poll: unsafe extern "C" fn(
                    r#self: *mut Self,
                    cx: *mut crate::fermented::types::std::task::wake::Context
                ) -> *mut crate::fermented::types::std::task::poll::Poll_where_T_is_Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry,
            }

            pub struct Poll {
                pub object: *const (),
                pub vtable: *const Poll_VTable,
            }
            pub struct Poll_VTable {
                pub is_ready: unsafe extern "C" fn(r#self: *const Self) -> bool,
                pub is_pending: unsafe extern "C" fn(r#self: *const Self) -> bool,
            }

            pub struct Context {
                pub object: *const (),
                pub vtable: *const Context_VTable,
            }
            pub struct Context_VTable {
                pub waker: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::types::std::task::wake::Waker,
            }

            pub struct Waker {
                pub object: *const (),
                pub vtable: *const Waker_VTable,
            }
            pub struct Waker_VTable {
                pub wake: unsafe extern "C" fn(r#self: *const Self),
            }
        }

        pub mod marker {
            pub struct Sized {
                pub object: *const (),
                pub vtable: *const Sized_VTable,
            }

            pub struct Sized_VTable {}

            pub struct Send {
                pub object: *const (),
                pub vtable: *const Send_VTable,
            }

            pub struct Send_VTable {}

            pub struct Sync {
                pub object: *const (),
                pub vtable: *const Sync_VTable,
            }

            pub struct Sync_VTable {}

            pub struct Safe {
                pub object: *const (),
                pub vtable: *const Safe_VTable,
            }

            pub struct Safe_VTable {}

            pub struct Clone {
                pub object: *const (),
                pub vtable: *const Clone_VTable,
            }

            pub struct Clone_VTable {}
        }

        pub mod task {
            pub mod poll {
                pub enum Poll_where_T_is_Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry {
                    Ready(*mut crate::fermented::generics::Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry),
                    Pending,
                }
            }
            pub mod wake {
                pub struct Context {
                    waker: *mut Waker,
                    // pub object: *const (),
                    // pub vtable: *const Context_VTable,
                }
                pub struct Waker {

                    // waker: RawWaker,
                }

            }
        }

        pub mod time {
            pub struct Nanoseconds(u32);

            pub struct Duration {
                secs: u64,
                nanos: *mut Nanoseconds,
            }
        }
    }

    pub mod r#crate {
        pub mod asyn {
            pub mod dapi_client {
                pub struct Dapi {
                    pub object: *const (),
                    pub vtable: *const Dapi_VTable,

                }
                pub struct Dapi_VTable {
                    pub execute: unsafe extern "C" fn(
                        r#self: *const Self,
                        request: *const crate::fermented::types::r#crate::asyn::query::TransportRequest_where_Client_is_TransportClient_where_Response_is_TransportResponse,
                    ) -> *mut crate::fermented::generics::Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry,

                }
            }
            pub mod dapi_request {

                // pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
                pub struct BoxFuture_where_T_is_Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry {
                    pub object: *const (),
                    pub vtable: *const BoxFuture_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                }
                pub struct BoxFuture_VTable {

                }
                pub enum DapiClientError_where_TE_is_DapiRequest {
                    Transport(*mut DapiRequest),
                    NoAvailableAddresses,
                }
                pub enum DapiClientError_where_TE_is_CanRetry {
                    Transport(*mut crate::fermented::types::r#crate::asyn::query::CanRetry),
                    NoAvailableAddresses,
                }

                pub struct DapiRequest {
                    pub object: *const (),
                    pub vtable: *const DapiRequest_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                }
                pub struct DapiRequest_VTable {
                    pub execute: unsafe extern "C" fn(
                        r#self: *const Self,
                        dapi_client: *const crate::fermented::types::r#crate::asyn::dapi_client::Dapi,
                        settings: *const crate::fermented::types::r#crate::asyn::query::RequestSettings,
                    ) -> *mut BoxFuture_where_T_is_Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry
                }
            }
            pub mod query {
                pub struct AppliedRequestSettings {
                    pub timeout: *mut crate::fermented::types::std::time::Duration,
                    pub retries: usize,
                }

                pub struct RequestSettings {
                    pub timeout: *mut crate::fermented::types::std::time::Duration,
                    pub retries: *mut usize,
                }

                pub struct Uri {
                    pub scheme: *mut std::os::raw::c_char,
                }

                pub struct CanRetry {
                    pub object: *const (),
                    pub vtable: *const CanRetry_VTable,
                }

                pub struct CanRetry_VTable {
                    pub can_retry: unsafe extern "C" fn(*const Self) -> bool,
                }


                pub struct TransportClient {
                    pub object: *const (),
                    pub vtable: *const TransportClient_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                    pub vtable_Sized: *const crate::fermented::types::std::marker::Sized,
                }

                pub struct TransportClient_VTable {
                    pub with_uri: unsafe extern "C" fn(uri: *mut Uri) -> *mut Self,
                }

                pub struct TransportResponse {
                    pub object: *const (),
                    pub vtable: *const TransportResponse_VTable,
                    pub vtable_Clone: *const crate::fermented::types::std::marker::Clone,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                    pub vtable_Sync: *const crate::fermented::types::std::marker::Sync,
                }

                pub struct TransportResponse_VTable {}

                // pub trait TransportRequest: Clone + Send + Sync {
                //     type Client: TransportClient;
                //     type Response: TransportResponse;
                //     const SETTINGS_OVERRIDES: RequestSettings;
                //     fn execute_transport(self, client: &mut Self::Client, settings: &AppliedRequestSettings)
                //                          -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
                // }

                pub struct TransportRequest_where_Client_is_TransportClient_where_Response_is_TransportResponse {
                    pub object: *const (),
                    pub vtable: *const TransportRequest_VTable,
                }

                pub struct TransportRequest_VTable {
                    pub SETTINGS_OVERRIDES: *const RequestSettings,
                    pub execute_transport: unsafe extern "C" fn(
                        r#self: *const Self,
                        client: *mut TransportClient,
                        settings: *const AppliedRequestSettings)
                        -> *mut crate::fermented::generics::Result_ok_crate_asyn_query_TransportResponse_err_crate_asyn_query_CanRetry,
                }

                // pub trait Query<T: TransportRequest>: Send + Clone {
                //     fn query(self, prove: bool) -> Result<T, Box<dyn Error>>;
                // }


                pub struct Query_where_T_is_TransportRequest {
                    pub object: *const (),
                    pub vtable: *const Query_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                    pub vtable_Clone: *const crate::fermented::types::std::marker::Clone,
                }

                pub struct Query_VTable {
                    pub query: unsafe extern "C" fn(*const Self, prove: bool) -> *mut crate::fermented::generics::Result_ok_crate_asyn_query_TransportRequest_err_std_error_Error,
                }
            }
            pub mod mock {

                pub struct MockResponse {
                    pub object: *const (),
                    pub vtable: *const MockResponse_VTable,
                }
                pub struct MockResponse_VTable {
                }

                pub struct Mockable {
                    pub object: *const (),
                    pub vtable: *const Mockable_VTable,
                    pub vtable_Sized: *const crate::fermented::types::std::marker::Sized,
                }

                pub struct Mockable_VTable {
                    pub mock_serialize: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::generics::Vec_u8,
                    pub mock_deserialize: unsafe extern "C" fn(_data: *const crate::fermented::generics::Vec_u8) -> *mut Self,
                }

                pub struct MockRequest {
                    pub object: *const (),
                    pub vtable: *const MockRequest_VTable,
                    pub vtable_Sized: *const crate::fermented::types::std::marker::Sized,
                }
                pub struct MockRequest_VTable {
                    pub mock_key: unsafe extern "C" fn(r#self: *const Self) -> *mut crate::fermented::types::r#crate::nested::HashID,
                }
            }
            pub mod platform_version {
                pub struct PlatformVersion {
                    pub protocol_version: u32,
                }
            }
            pub mod proof {
                pub struct FromProof {
                    pub object: *const (),
                    pub vtable: *const FromProof_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                }

                pub struct FromProof_VTable {
                    pub maybe_from_proof: fn(
                        request: *mut crate::fermented::types::std::convert::Into_where_T_is_FromProof_Request,
                        response: *mut crate::fermented::types::std::convert::Into_where_T_is_FromProof_Response,
                        platform_version: *const crate::fermented::types::r#crate::asyn::platform_version::PlatformVersion,
                        provider: *const crate::fermented::types::r#crate::asyn::provider::ContextProvider,
                    ) -> *mut crate::fermented::generics::Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
                }
            }
            pub mod provider {
                pub struct DataContractV0 {
                    pub(crate) id: *mut crate::fermented::types::r#crate::nested::Identifier,
                    pub(crate) version: u32,
                }
                pub enum DataContract {
                    V0(*mut DataContractV0),
                }
                pub enum ContextProviderError {
                    Generic(*mut std::os::raw::c_char),
                    Config(*mut std::os::raw::c_char),
                    InvalidDataContract(*mut std::os::raw::c_char),
                    InvalidQuorum(*mut std::os::raw::c_char),
                }
                pub struct ContextProvider {
                    pub object: *const (),
                    pub vtable: *const ContextProvider_VTable,
                    pub vtable_Send: *const crate::fermented::types::std::marker::Send,
                    pub vtable_Sync: *const crate::fermented::types::std::marker::Sync,
                }
                pub struct ContextProvider_VTable {
                    pub get_quorum_public_key: unsafe extern "C" fn(
                        r#self: *const Self,
                        quorum_type: u32,
                        quorum_hash: *const [u8; 32],
                        core_chain_locked_height: u32
                    ) -> *mut crate::fermented::generics::Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError,
                    pub get_data_contract: unsafe extern "C" fn(
                        r#self: *const Self,
                        id: *const crate::fermented::types::r#crate::nested::Identifier,
                    ) -> *mut crate::fermented::generics::Result_ok_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError,
                }

            }
            pub mod sdk {
                pub struct Sdk {
                    pub proofs: bool,
                    pub context_provider: *mut crate::fermented::types::r#crate::asyn::provider::ContextProvider,
                }
            }
        }
        pub mod nested {
            pub struct HashID(pub *mut [u8; 32]);
            pub struct IdentifierBytes32(pub *mut [u8; 32]);
            pub struct Identifier(pub *mut IdentifierBytes32);
            pub struct DataContractNotPresentError {
                pub data_contract_id: *mut Identifier,
            }
            pub enum ProtocolError {
                IdentifierError(*mut std::os::raw::c_char),
                StringDecodeError(*mut std::os::raw::c_char),
                StringDecodeError2(*mut std::os::raw::c_char, u32),
                EmptyPublicKeyDataError,
                MaxEncodedBytesReachedError {
                    max_size_kbytes: usize,
                    size_hit: usize,
                },
                EncodingError(*mut std::os::raw::c_char),
                EncodingError2(*mut std::os::raw::c_char),
                DataContractNotPresentError(*mut DataContractNotPresentError),
            }
        }
    }
}