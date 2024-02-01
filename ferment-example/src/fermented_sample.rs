
pub mod some_mod {
    pub trait CanRetry {
        fn can_retry(&self) -> bool;
    }
    pub enum Status {
        Error,
        Success
    }
    impl CanRetry for Status {
        fn can_retry(&self) -> bool { true }
    }
    unsafe impl Send for Status {}
    pub struct Uri {
        pub(crate) scheme: String,
    }

    pub trait TransportClient: Send + Sized {
        type Error: CanRetry + Send;
        fn with_uri(uri: Uri) -> Self;
    }
    pub struct CoreGrpcClient {
        pub uri: Uri
    }
    impl CoreGrpcClient {
        pub fn new(uri: Uri) -> Self { Self { uri } }
    }

    impl TransportClient for CoreGrpcClient {
        type Error = Status;

        fn with_uri(uri: Uri) -> Self {
            CoreGrpcClient::new(uri)
        }
    }
}

#[allow(
clippy::let_and_return,
clippy::suspicious_else_formatting,
clippy::redundant_field_names,
dead_code,
non_camel_case_types,
non_snake_case,
non_upper_case_globals,
redundant_semicolons,
unused_braces,
unused_imports,
unused_unsafe,
unused_variables,
unused_qualifications
)]
pub mod fermented {
    pub mod generics {}

    pub mod vtable {
        pub mod r#crate {
            pub mod some_mod {

                static CanRetry_VTABLE: crate::fermented::types::some_mod::CanRetry = {
                    unsafe extern "C" fn can_retry(obj: *const ()) -> bool {
                        true
                    }
                    crate::fermented::types::some_mod::CanRetry { can_retry }
                };
                static SomeOtherTrait_VTABLE: crate::fermented::types::some_mod::SomeOtherTrait = {
                    unsafe extern "C" fn some_other_method(obj: *const ()) {}
                    crate::fermented::types::some_mod::SomeOtherTrait { some_other_method };
                };
                static TransportClient_VTABLE: crate::fermented::types::some_mod::TransportClient = {
                    unsafe extern "C" fn with_uri(uri: *const crate::fermented::types::some_mod::Uri) -> crate::fermented::types::some_mod::TransportClient {
                        crate::fermented::types::some_mod::TransportClient::with_uri(uri)
                    }
                    crate::fermented::types::some_mod::TransportClient { with_uri };
                };

            }
        }
    }
    pub mod types {
        pub mod r#crate {
            pub mod some_mod {
                #[repr(C)]
                pub struct CanRetry {
                    pub can_retry: extern "C" fn(*const ()) -> bool,
                }
                #[repr(C)]
                pub struct SomeOtherTrait {
                    pub some_other_method: extern "C" fn(*const ()),
                }

                #[repr(C)]
                pub enum Status {
                    Error,
                    Success
                }
                #[repr(C)]
                pub struct Uri {
                    pub(crate) scheme: *mut std::os::raw::c_char,
                }
            }
        }



        // pub trait CanRetry {
        //     fn can_retry(&self) -> bool;
        // }
        // pub trait SomeOtherTrait {
        //     fn some_other_method(&self);
        // }
        //
        // pub enum Status {
        //     Error,
        //     Success
        // }
        //
        // impl CanRetry for Status {
        //     fn can_retry(&self) -> bool { true }
        // }
        // impl SomeOtherTrait for Status {
        //     fn some_other_method(&self) {}
        // }
        //
        // unsafe impl Send for Status {}
        //
        // pub struct Uri {
        //     pub(crate) scheme: String,
        // }
        //
        //
        // pub trait TransportClient: Send + Sized {
        //     type Error: CanRetry + Send + SomeOtherTrait;
        //     fn with_uri(uri: Uri) -> Self;
        // }
        //
        // pub struct CoreGrpcClient {
        //     pub uri: Uri
        // }
        //
        // impl CoreGrpcClient {
        //     pub fn new(uri: Uri) -> Self { Self { uri } }
        // }
        //
        // impl TransportClient for CoreGrpcClient {
        //     type Error = Status;
        //
        //     fn with_uri(uri: Uri) -> Self {
        //         CoreGrpcClient::new(uri)
        //     }
        // }
    }
}

//     #[allow(non_snake_case, non_upper_case_globals)]
//     static DevnetType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
//         unsafe extern "C" fn DevnetType_name(
//             obj: *const (),
//         ) -> *mut std::os::raw::c_char {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: name (cast_obj ,) ;
//             ferment_interfaces::FFIConversion::ffi_to(obj)
//         }
//         unsafe extern "C" fn DevnetType_genesis_hash(
//             obj: *const (),
//         ) -> *mut crate::fermented::types::nested::HashID {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_hash (cast_obj ,) ;
//             ferment_interfaces::FFIConversion::ffi_to(obj)
//         }
//         unsafe extern "C" fn DevnetType_genesis_height(obj: *const ()) -> u32 {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_height (cast_obj ,) ;
//             obj
//         }
//         unsafe extern "C" fn DevnetType_has_genesis_hash(
//             obj: *const (),
//             hash: *mut crate::fermented::types::nested::HashID,
//         ) -> bool {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: has_genesis_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
//             obj
//         }
//         unsafe extern "C" fn DevnetType_get_hash_by_hash(
//             obj: *const (),
//             hash: *mut crate::fermented::types::nested::HashID,
//         ) -> *mut crate::fermented::types::nested::HashID {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: get_hash_by_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
//             ferment_interfaces::FFIConversion::ffi_to(obj)
//         }
//         unsafe extern "C" fn DevnetType_should_process_llmq_of_type(
//             obj: *const (),
//             llmq_type: u16,
//         ) -> bool {
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: should_process_llmq_of_type (cast_obj , llmq_type ,) ;
//             obj
//         }                    unsafe extern "C" fn DevnetType_find_masternode_list (obj : * const () , cached_mn_lists : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , unknown_mn_lists : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError{
//             let cast_obj =
//                 &(*(obj as *const crate::chain::common::chain_type::DevnetType));
//             let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: find_masternode_list (cast_obj , & ferment_interfaces :: FFIConversion :: ffi_from (cached_mn_lists) , & mut ferment_interfaces :: FFIConversion :: ffi_from (unknown_mn_lists) ,) ;
//             ferment_interfaces::FFIConversion::ffi_to(obj)
//         }
//         IHaveChainSettings_VTable {
//             name: DevnetType_name,
//             genesis_hash: DevnetType_genesis_hash,
//             genesis_height: DevnetType_genesis_height,
//             has_genesis_hash: DevnetType_has_genesis_hash,
//             get_hash_by_hash: DevnetType_get_hash_by_hash,
//             should_process_llmq_of_type: DevnetType_should_process_llmq_of_type,
//             find_masternode_list: DevnetType_find_masternode_list,
//         }
//     };
// }
