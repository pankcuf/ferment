#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod types {
    pub mod asyn {
        pub mod service {
            #[doc = "FFI-representation of the get_chain_type_string_async"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_get_chain_type_string_async(
                runtime: *mut std::os::raw::c_void,
                chain_type: *mut crate::fermented::types::chain::common::chain_type::ChainType,
            ) -> *mut std::os::raw::c_char {
                let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                let obj = rt.block_on(async {
                    crate::asyn::service::get_chain_type_string_async(
                        ferment_interfaces::FFIConversion::ffi_from(chain_type),
                    )
                    .await
                });
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
        pub mod query {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: query :: AppliedRequestSettings\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct AppliedRequestSettings {
                pub timeout: *mut crate::asyn::query::Duration_FFI,
                pub retries: usize,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::query::AppliedRequestSettings>
                for AppliedRequestSettings
            {
                unsafe fn ffi_from_const(
                    ffi: *const AppliedRequestSettings,
                ) -> crate::asyn::query::AppliedRequestSettings {
                    let ffi_ref = &*ffi;
                    crate::asyn::query::AppliedRequestSettings {
                        timeout: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.timeout),
                        retries: ffi_ref.retries,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::query::AppliedRequestSettings,
                ) -> *const AppliedRequestSettings {
                    ferment_interfaces::boxed(AppliedRequestSettings {
                        timeout: ferment_interfaces::FFIConversion::ffi_to(obj.timeout),
                        retries: obj.retries,
                    })
                }
                unsafe fn destroy(ffi: *mut AppliedRequestSettings) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for AppliedRequestSettings {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.timeout);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn AppliedRequestSettings_ctor(
                timeout: *mut crate::asyn::query::Duration_FFI,
                retries: usize,
            ) -> *mut AppliedRequestSettings {
                ferment_interfaces::boxed(AppliedRequestSettings { timeout, retries })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn AppliedRequestSettings_destroy(
                ffi: *mut AppliedRequestSettings,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CanRetry_VTable {
                pub can_retry: unsafe extern "C" fn(obj: *const ()) -> bool,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CanRetry_TraitObject {
                pub object: *const (),
                pub vtable: *const CanRetry_VTable,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportClient_VTable {
                pub with_uri: unsafe extern "C" fn(
                    uri: *mut crate::fermented::types::asyn::query::Uri,
                ) -> *mut Self,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportClient_TraitObject {
                pub object: *const (),
                pub vtable: *const TransportClient_VTable,
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: query :: RequestSettings\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct RequestSettings {
                pub timeout: *mut crate::asyn::query::Duration_FFI,
                pub retries: usize,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::query::RequestSettings> for RequestSettings {
                unsafe fn ffi_from_const(
                    ffi: *const RequestSettings,
                ) -> crate::asyn::query::RequestSettings {
                    let ffi_ref = &*ffi;
                    crate::asyn::query::RequestSettings {
                        timeout: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.timeout),
                        retries: (ffi_ref.retries > 0).then_some(ffi_ref.retries),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::query::RequestSettings,
                ) -> *const RequestSettings {
                    ferment_interfaces::boxed(RequestSettings {
                        timeout: ferment_interfaces::FFIConversion::ffi_to_opt(obj.timeout),
                        retries: obj.retries.unwrap_or(0),
                    })
                }
                unsafe fn destroy(ffi: *mut RequestSettings) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for RequestSettings {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.timeout.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.timeout);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn RequestSettings_ctor(
                timeout: *mut crate::asyn::query::Duration_FFI,
                retries: usize,
            ) -> *mut RequestSettings {
                ferment_interfaces::boxed(RequestSettings { timeout, retries })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn RequestSettings_destroy(ffi: *mut RequestSettings) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: query :: Uri\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Uri {
                pub scheme: *mut std::os::raw::c_char,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::query::Uri> for Uri {
                unsafe fn ffi_from_const(ffi: *const Uri) -> crate::asyn::query::Uri {
                    let ffi_ref = &*ffi;
                    crate::asyn::query::Uri {
                        scheme: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.scheme),
                    }
                }
                unsafe fn ffi_to_const(obj: crate::asyn::query::Uri) -> *const Uri {
                    ferment_interfaces::boxed(Uri {
                        scheme: ferment_interfaces::FFIConversion::ffi_to(obj.scheme),
                    })
                }
                unsafe fn destroy(ffi: *mut Uri) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Uri {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . scheme) ;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Uri_ctor(scheme: *mut std::os::raw::c_char) -> *mut Uri {
                ferment_interfaces::boxed(Uri { scheme })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Uri_destroy(ffi: *mut Uri) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod chain {
        pub mod common {
            pub mod chain_type {
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_VTable { pub name : unsafe extern "C" fn (obj : * const () ,) -> * mut std :: os :: raw :: c_char , pub genesis_hash : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: nested :: HashID , pub genesis_height : unsafe extern "C" fn (obj : * const () ,) -> u32 , pub has_genesis_hash : unsafe extern "C" fn (obj : * const () , hash : * mut crate :: fermented :: types :: nested :: HashID ,) -> bool , pub get_hash_by_hash : unsafe extern "C" fn (obj : * const () , hash : * mut crate :: fermented :: types :: nested :: HashID ,) -> * mut crate :: fermented :: types :: nested :: HashID , pub should_process_llmq_of_type : unsafe extern "C" fn (obj : * const () , llmq_type : u16 ,) -> bool , pub find_masternode_list : unsafe extern "C" fn (obj : * const () , cached_mn_lists : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , unknown_mn_lists : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError , }
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_TraitObject {
                    pub object: *const (),
                    pub vtable: *const IHaveChainSettings_VTable,
                }
                #[doc = "FFI-representation of the DevnetType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum DevnetType {
                    JackDaniels = 0,
                    Devnet333 = 1,
                    Chacha = 2,
                    Mojito = 3,
                    WhiteRussian = 4,
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::chain_type::DevnetType>
                    for DevnetType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const DevnetType,
                    ) -> crate::chain::common::chain_type::DevnetType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            DevnetType::JackDaniels => {
                                crate::chain::common::chain_type::DevnetType::JackDaniels
                            }
                            DevnetType::Devnet333 => {
                                crate::chain::common::chain_type::DevnetType::Devnet333
                            }
                            DevnetType::Chacha => {
                                crate::chain::common::chain_type::DevnetType::Chacha
                            }
                            DevnetType::Mojito => {
                                crate::chain::common::chain_type::DevnetType::Mojito
                            }
                            DevnetType::WhiteRussian => {
                                crate::chain::common::chain_type::DevnetType::WhiteRussian
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::chain_type::DevnetType,
                    ) -> *const DevnetType {
                        ferment_interfaces::boxed(match obj {
                            crate::chain::common::chain_type::DevnetType::JackDaniels => {
                                DevnetType::JackDaniels
                            }
                            crate::chain::common::chain_type::DevnetType::Devnet333 => {
                                DevnetType::Devnet333
                            }
                            crate::chain::common::chain_type::DevnetType::Chacha => {
                                DevnetType::Chacha
                            }
                            crate::chain::common::chain_type::DevnetType::Mojito => {
                                DevnetType::Mojito
                            }
                            crate::chain::common::chain_type::DevnetType::WhiteRussian => {
                                DevnetType::WhiteRussian
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut DevnetType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for DevnetType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                DevnetType::JackDaniels => {}
                                DevnetType::Devnet333 => {}
                                DevnetType::Chacha => {}
                                DevnetType::Mojito => {}
                                DevnetType::WhiteRussian => {}
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_JackDaniels_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::JackDaniels)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Devnet333_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Devnet333)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Chacha_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Chacha)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Mojito_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Mojito)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_WhiteRussian_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::WhiteRussian)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_destroy(ffi: *mut DevnetType) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[allow(non_snake_case, non_upper_case_globals)]
                static DevnetType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn DevnetType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: name (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_genesis_hash(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::nested::HashID {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_hash (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_height (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn DevnetType_has_genesis_hash(
                        obj: *const (),
                        hash: *mut crate::fermented::types::nested::HashID,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: has_genesis_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
                        obj
                    }
                    unsafe extern "C" fn DevnetType_get_hash_by_hash(
                        obj: *const (),
                        hash: *mut crate::fermented::types::nested::HashID,
                    ) -> *mut crate::fermented::types::nested::HashID {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: get_hash_by_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: u16,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: should_process_llmq_of_type (cast_obj , llmq_type ,) ;
                        obj
                    }                    unsafe extern "C" fn DevnetType_find_masternode_list (obj : * const () , cached_mn_lists : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , unknown_mn_lists : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError{
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: find_masternode_list (cast_obj , & ferment_interfaces :: FFIConversion :: ffi_from (cached_mn_lists) , & mut ferment_interfaces :: FFIConversion :: ffi_from (unknown_mn_lists) ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    IHaveChainSettings_VTable {
                        name: DevnetType_name,
                        genesis_hash: DevnetType_genesis_hash,
                        genesis_height: DevnetType_genesis_height,
                        has_genesis_hash: DevnetType_has_genesis_hash,
                        get_hash_by_hash: DevnetType_get_hash_by_hash,
                        should_process_llmq_of_type: DevnetType_should_process_llmq_of_type,
                        find_masternode_list: DevnetType_find_masternode_list,
                    }
                };
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub extern "C" fn DevnetType_as_IHaveChainSettings_TraitObject(
                    obj: *const crate::chain::common::chain_type::DevnetType,
                ) -> IHaveChainSettings_TraitObject {
                    IHaveChainSettings_TraitObject {
                        object: obj as *const (),
                        vtable: &DevnetType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_as_IHaveChainSettings_TraitObject_destroy(
                    obj: IHaveChainSettings_TraitObject,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::DevnetType,
                    );
                }
                #[doc = "FFI-representation of the ChainType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum ChainType {
                    MainNet,
                    TestNet,
                    DevNet(*mut crate::fermented::types::chain::common::chain_type::DevnetType),
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::chain_type::ChainType> for ChainType {
                    unsafe fn ffi_from_const(
                        ffi: *const ChainType,
                    ) -> crate::chain::common::chain_type::ChainType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ChainType::MainNet => {
                                crate::chain::common::chain_type::ChainType::MainNet
                            }
                            ChainType::TestNet => {
                                crate::chain::common::chain_type::ChainType::TestNet
                            }
                            ChainType::DevNet(o_0) => {
                                crate::chain::common::chain_type::ChainType::DevNet(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::chain_type::ChainType,
                    ) -> *const ChainType {
                        ferment_interfaces::boxed(match obj {
                            crate::chain::common::chain_type::ChainType::MainNet => {
                                ChainType::MainNet
                            }
                            crate::chain::common::chain_type::ChainType::TestNet => {
                                ChainType::TestNet
                            }
                            crate::chain::common::chain_type::ChainType::DevNet(o_0) => {
                                ChainType::DevNet(ferment_interfaces::FFIConversion::ffi_to(o_0))
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut ChainType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ChainType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ChainType::MainNet => {}
                                ChainType::TestNet => {}
                                ChainType::DevNet(o_0) => {
                                    ferment_interfaces::unbox_any(o_0.to_owned());
                                }
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_MainNet_ctor() -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::MainNet)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_TestNet_ctor() -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::TestNet)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_DevNet_ctor(
                    o_o_0: *mut crate::fermented::types::chain::common::chain_type::DevnetType,
                ) -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::DevNet(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_destroy(ffi: *mut ChainType) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[allow(non_snake_case, non_upper_case_globals)]
                static ChainType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn ChainType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: name (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_genesis_hash(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::nested::HashID {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_hash (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_height (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn ChainType_has_genesis_hash(
                        obj: *const (),
                        hash: *mut crate::fermented::types::nested::HashID,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: has_genesis_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
                        obj
                    }
                    unsafe extern "C" fn ChainType_get_hash_by_hash(
                        obj: *const (),
                        hash: *mut crate::fermented::types::nested::HashID,
                    ) -> *mut crate::fermented::types::nested::HashID {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: get_hash_by_hash (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (hash) ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: u16,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: should_process_llmq_of_type (cast_obj , llmq_type ,) ;
                        obj
                    }                    unsafe extern "C" fn ChainType_find_masternode_list (obj : * const () , cached_mn_lists : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , unknown_mn_lists : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError{
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: find_masternode_list (cast_obj , & ferment_interfaces :: FFIConversion :: ffi_from (cached_mn_lists) , & mut ferment_interfaces :: FFIConversion :: ffi_from (unknown_mn_lists) ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    IHaveChainSettings_VTable {
                        name: ChainType_name,
                        genesis_hash: ChainType_genesis_hash,
                        genesis_height: ChainType_genesis_height,
                        has_genesis_hash: ChainType_has_genesis_hash,
                        get_hash_by_hash: ChainType_get_hash_by_hash,
                        should_process_llmq_of_type: ChainType_should_process_llmq_of_type,
                        find_masternode_list: ChainType_find_masternode_list,
                    }
                };
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub extern "C" fn ChainType_as_IHaveChainSettings_TraitObject(
                    obj: *const crate::chain::common::chain_type::ChainType,
                ) -> IHaveChainSettings_TraitObject {
                    IHaveChainSettings_TraitObject {
                        object: obj as *const (),
                        vtable: &ChainType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_as_IHaveChainSettings_TraitObject_destroy(
                    obj: IHaveChainSettings_TraitObject,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::ChainType,
                    );
                }
            }
        }
    }
    pub mod nested {
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: UsedStruct\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UsedStruct(*mut crate::fermented::types::nested::HashID);
        impl ferment_interfaces::FFIConversion<crate::nested::UsedStruct> for UsedStruct {
            unsafe fn ffi_from_const(ffi: *const UsedStruct) -> crate::nested::UsedStruct {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: crate::nested::UsedStruct) -> *const UsedStruct {
                ferment_interfaces::boxed(UsedStruct(ferment_interfaces::FFIConversion::ffi_to(
                    obj,
                )))
            }
            unsafe fn destroy(ffi: *mut UsedStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UsedStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UsedStruct_ctor(
            o_0: *mut crate::fermented::types::nested::HashID,
        ) -> *mut UsedStruct {
            ferment_interfaces::boxed(UsedStruct(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UsedStruct_destroy(ffi: *mut UsedStruct) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: UnnamedPair\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UnnamedPair(*mut [u8; 32], u32);
        impl ferment_interfaces::FFIConversion<crate::nested::UnnamedPair> for UnnamedPair {
            unsafe fn ffi_from_const(ffi: *const UnnamedPair) -> crate::nested::UnnamedPair {
                let ffi_ref = &*ffi;
                crate::nested::UnnamedPair(*ffi_ref.0, ffi_ref.1)
            }
            unsafe fn ffi_to_const(obj: crate::nested::UnnamedPair) -> *const UnnamedPair {
                ferment_interfaces::boxed(UnnamedPair(ferment_interfaces::boxed(obj.0), obj.1))
            }
            unsafe fn destroy(ffi: *mut UnnamedPair) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UnnamedPair {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UnnamedPair_ctor(
            o_0: *mut [u8; 32],
            o_1: u32,
        ) -> *mut UnnamedPair {
            ferment_interfaces::boxed(UnnamedPair(o_0, o_1))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UnnamedPair_destroy(ffi: *mut UnnamedPair) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: TestStruct\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct TestStruct { pub vec_u8 : * mut crate :: fermented :: generics :: Vec_u8 , pub vec_u32 : * mut crate :: fermented :: generics :: Vec_u32 , pub vec_vec_u32 : * mut crate :: fermented :: generics :: Vec_Vec_u32 , pub map_key_simple_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32 , pub map_key_simple_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_crate_nested_HashID , pub map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u32 , pub map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_crate_nested_HashID , pub map_key_simple_value_map_key_simple_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32 , pub map_key_simple_value_map_key_simple_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID , pub map_key_simple_value_map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32 , pub map_key_simple_value_map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID , pub map_key_complex_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_u32 , pub map_key_complex_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , pub map_key_complex_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_u32 , pub map_key_complex_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID , pub map_key_complex_value_map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32 , pub map_key_complex_value_map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID , pub map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , pub opt_primitive : u8 , pub opt_string : * mut std :: os :: raw :: c_char , pub opt_vec_primitive : * mut crate :: fermented :: generics :: Vec_u8 , pub opt_vec_string : * mut crate :: fermented :: generics :: Vec_String , pub opt_vec_complex : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID , pub opt_vec_vec_complex : * mut crate :: fermented :: generics :: Vec_Vec_crate_nested_HashID , }
        impl ferment_interfaces::FFIConversion<crate::nested::TestStruct> for TestStruct {
            unsafe fn ffi_from_const(ffi: *const TestStruct) -> crate::nested::TestStruct {
                let ffi_ref = &*ffi;
                crate :: nested :: TestStruct { vec_u8 : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . vec_u8) , vec_u32 : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . vec_u32) , vec_vec_u32 : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . vec_vec_u32) , map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_simple) , map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_complex) , map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_vec_simple) , map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_vec_complex) , map_key_simple_value_map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_simple) , map_key_simple_value_map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_complex) , map_key_simple_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_vec_simple) , map_key_simple_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_simple_value_map_key_simple_value_vec_complex) , map_key_complex_value_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_simple) , map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_complex) , map_key_complex_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_vec_simple) , map_key_complex_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_vec_complex) , map_key_complex_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_vec_simple) , map_key_complex_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_vec_complex) , map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) , opt_primitive : (ffi_ref . opt_primitive > 0) . then_some (ffi_ref . opt_primitive) , opt_string : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . opt_string) , opt_vec_primitive : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . opt_vec_primitive) , opt_vec_string : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . opt_vec_string) , opt_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . opt_vec_complex) , opt_vec_vec_complex : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . opt_vec_vec_complex) , }
            }
            unsafe fn ffi_to_const(obj: crate::nested::TestStruct) -> *const TestStruct {
                ferment_interfaces :: boxed (TestStruct { vec_u8 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_u8) , vec_u32 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_u32) , vec_vec_u32 : ferment_interfaces :: FFIConversion :: ffi_to (obj . vec_vec_u32) , map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_simple) , map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_complex) , map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_vec_simple) , map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_vec_complex) , map_key_simple_value_map_key_simple_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_simple) , map_key_simple_value_map_key_simple_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_complex) , map_key_simple_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_vec_simple) , map_key_simple_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_simple_value_map_key_simple_value_vec_complex) , map_key_complex_value_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_simple) , map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_complex) , map_key_complex_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_vec_simple) , map_key_complex_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_vec_complex) , map_key_complex_value_map_key_simple_value_vec_simple : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_vec_simple) , map_key_complex_value_map_key_simple_value_vec_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_vec_complex) , map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : ferment_interfaces :: FFIConversion :: ffi_to (obj . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) , opt_primitive : obj . opt_primitive . unwrap_or (0) , opt_string : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . opt_string) , opt_vec_primitive : match obj . opt_vec_primitive { Some (vec) => ferment_interfaces :: FFIConversion :: ffi_to (vec) , None => std :: ptr :: null_mut () , } , opt_vec_string : match obj . opt_vec_string { Some (vec) => ferment_interfaces :: FFIConversion :: ffi_to (vec) , None => std :: ptr :: null_mut () , } , opt_vec_complex : match obj . opt_vec_complex { Some (vec) => ferment_interfaces :: FFIConversion :: ffi_to (vec) , None => std :: ptr :: null_mut () , } , opt_vec_vec_complex : match obj . opt_vec_vec_complex { Some (vec) => ferment_interfaces :: FFIConversion :: ffi_to (vec) , None => std :: ptr :: null_mut () , } , })
            }
            unsafe fn destroy(ffi: *mut TestStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for TestStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.vec_u8);
                    ferment_interfaces::unbox_any(ffi_ref.vec_u32);
                    ferment_interfaces::unbox_any(ffi_ref.vec_vec_u32);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_complex);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_vec_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_simple_value_vec_complex);
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_complex,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_vec_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_simple_value_map_key_simple_value_vec_complex,
                    );
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_complex);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_vec_simple);
                    ferment_interfaces::unbox_any(ffi_ref.map_key_complex_value_vec_complex);
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_complex_value_map_key_simple_value_vec_simple,
                    );
                    ferment_interfaces::unbox_any(
                        ffi_ref.map_key_complex_value_map_key_simple_value_vec_complex,
                    );
                    ferment_interfaces :: unbox_any (ffi_ref . map_key_complex_value_map_key_simple_value_map_key_complex_value_complex) ;
                    if !ffi_ref.opt_string.is_null() {
                        ferment_interfaces::unbox_any(ffi_ref.opt_string);
                    };
                    if !ffi_ref.opt_vec_primitive.is_null() {
                        ferment_interfaces::unbox_any(ffi_ref.opt_vec_primitive);
                    };
                    if !ffi_ref.opt_vec_string.is_null() {
                        ferment_interfaces::unbox_any(ffi_ref.opt_vec_string);
                    };
                    if !ffi_ref.opt_vec_complex.is_null() {
                        ferment_interfaces::unbox_any(ffi_ref.opt_vec_complex);
                    };
                    if !ffi_ref.opt_vec_vec_complex.is_null() {
                        ferment_interfaces::unbox_any(ffi_ref.opt_vec_vec_complex);
                    };
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestStruct_ctor(
            vec_u8: *mut crate::fermented::generics::Vec_u8,
            vec_u32: *mut crate::fermented::generics::Vec_u32,
            vec_vec_u32: *mut crate::fermented::generics::Vec_Vec_u32,
            map_key_simple_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32,
            map_key_simple_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_crate_nested_HashID,
            map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u32,
            map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_crate_nested_HashID,
            map_key_simple_value_map_key_simple_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32,
            map_key_simple_value_map_key_simple_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID,
            map_key_simple_value_map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32,
            map_key_simple_value_map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID,
            map_key_complex_value_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_u32,
            map_key_complex_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
            map_key_complex_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_u32,
            map_key_complex_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID,
            map_key_complex_value_map_key_simple_value_vec_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32,
            map_key_complex_value_map_key_simple_value_vec_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID,
            map_key_complex_value_map_key_simple_value_map_key_complex_value_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
            opt_primitive: u8,
            opt_string: *mut std::os::raw::c_char,
            opt_vec_primitive: *mut crate::fermented::generics::Vec_u8,
            opt_vec_string: *mut crate::fermented::generics::Vec_String,
            opt_vec_complex: *mut crate::fermented::generics::Vec_crate_nested_HashID,
            opt_vec_vec_complex: *mut crate::fermented::generics::Vec_Vec_crate_nested_HashID,
        ) -> *mut TestStruct {
            ferment_interfaces::boxed(TestStruct {
                vec_u8,
                vec_u32,
                vec_vec_u32,
                map_key_simple_value_simple,
                map_key_simple_value_complex,
                map_key_simple_value_vec_simple,
                map_key_simple_value_vec_complex,
                map_key_simple_value_map_key_simple_value_simple,
                map_key_simple_value_map_key_simple_value_complex,
                map_key_simple_value_map_key_simple_value_vec_simple,
                map_key_simple_value_map_key_simple_value_vec_complex,
                map_key_complex_value_simple,
                map_key_complex_value_complex,
                map_key_complex_value_vec_simple,
                map_key_complex_value_vec_complex,
                map_key_complex_value_map_key_simple_value_vec_simple,
                map_key_complex_value_map_key_simple_value_vec_complex,
                map_key_complex_value_map_key_simple_value_map_key_complex_value_complex,
                opt_primitive,
                opt_string,
                opt_vec_primitive,
                opt_vec_string,
                opt_vec_complex,
                opt_vec_vec_complex,
            })
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestStruct_destroy(ffi: *mut TestStruct) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the ProtocolError"]
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
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
            DataContractNotPresentError(
                *mut crate::fermented::types::nested::DataContractNotPresentError,
            ),
        }
        impl ferment_interfaces::FFIConversion<crate::nested::ProtocolError> for ProtocolError {
            unsafe fn ffi_from_const(ffi: *const ProtocolError) -> crate::nested::ProtocolError {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    ProtocolError::IdentifierError(o_0) => {
                        crate::nested::ProtocolError::IdentifierError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::StringDecodeError(o_0) => {
                        crate::nested::ProtocolError::StringDecodeError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::StringDecodeError2(o_0, o_1) => {
                        crate::nested::ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            *o_1,
                        )
                    }
                    ProtocolError::EmptyPublicKeyDataError => {
                        crate::nested::ProtocolError::EmptyPublicKeyDataError
                    }
                    ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => crate::nested::ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes: *max_size_kbytes,
                        size_hit: *size_hit,
                    },
                    ProtocolError::EncodingError(o_0) => {
                        crate::nested::ProtocolError::EncodingError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::EncodingError2(o_0) => {
                        crate::nested::ProtocolError::EncodingError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::DataContractNotPresentError(o_0) => {
                        crate::nested::ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::ProtocolError) -> *const ProtocolError {
                ferment_interfaces::boxed(match obj {
                    crate::nested::ProtocolError::IdentifierError(o_0) => {
                        ProtocolError::IdentifierError(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError(o_0) => {
                        ProtocolError::StringDecodeError(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError2(o_0, o_1) => {
                        ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                            o_1,
                        )
                    }
                    crate::nested::ProtocolError::EmptyPublicKeyDataError => {
                        ProtocolError::EmptyPublicKeyDataError
                    }
                    crate::nested::ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes: max_size_kbytes,
                        size_hit: size_hit,
                    },
                    crate::nested::ProtocolError::EncodingError(o_0) => {
                        ProtocolError::EncodingError(ferment_interfaces::FFIConversion::ffi_to(o_0))
                    }
                    crate::nested::ProtocolError::EncodingError2(o_0) => {
                        ProtocolError::EncodingError2(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::DataContractNotPresentError(o_0) => {
                        ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                        )
                    }
                })
            }
            unsafe fn destroy(ffi: *mut ProtocolError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ProtocolError {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        ProtocolError::IdentifierError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError::StringDecodeError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError::StringDecodeError2(o_0, o_1) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError::EmptyPublicKeyDataError => {}
                        ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => {}
                        ProtocolError::EncodingError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError::EncodingError2(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (o_0 . to_owned ()) ;
                        }
                        ProtocolError::DataContractNotPresentError(o_0) => {
                            ferment_interfaces::unbox_any(o_0.to_owned());
                        }
                    }
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_IdentifierError_ctor(
            o_o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::IdentifierError(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_StringDecodeError_ctor(
            o_o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::StringDecodeError(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_StringDecodeError2_ctor(
            o_o_0: *mut std::os::raw::c_char,
            o_o_1: u32,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::StringDecodeError2(o_o_0, o_o_1))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EmptyPublicKeyDataError_ctor() -> *mut ProtocolError
        {
            ferment_interfaces::boxed(ProtocolError::EmptyPublicKeyDataError)
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_MaxEncodedBytesReachedError_ctor(
            max_size_kbytes: usize,
            size_hit: usize,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::MaxEncodedBytesReachedError {
                max_size_kbytes,
                size_hit,
            })
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EncodingError_ctor(
            o_o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::EncodingError(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EncodingError2_ctor(
            o_o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::EncodingError2(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_DataContractNotPresentError_ctor(
            o_o_0: *mut crate::fermented::types::nested::DataContractNotPresentError,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::DataContractNotPresentError(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_destroy(ffi: *mut ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: KeyID\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct KeyID(u32);
        impl ferment_interfaces::FFIConversion<crate::nested::KeyID> for KeyID {
            unsafe fn ffi_from_const(ffi: *const KeyID) -> crate::nested::KeyID {
                let ffi_ref = &*ffi;
                ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: crate::nested::KeyID) -> *const KeyID {
                ferment_interfaces::boxed(KeyID(obj))
            }
            unsafe fn destroy(ffi: *mut KeyID) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for KeyID {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn KeyID_ctor(o_0: u32) -> *mut KeyID {
            ferment_interfaces::boxed(KeyID(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn KeyID_destroy(ffi: *mut KeyID) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: HashID\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct HashID(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<crate::nested::HashID> for HashID {
            unsafe fn ffi_from_const(ffi: *const HashID) -> crate::nested::HashID {
                let ffi_ref = &*ffi;
                *ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: crate::nested::HashID) -> *const HashID {
                ferment_interfaces::boxed(HashID(ferment_interfaces::boxed(obj)))
            }
            unsafe fn destroy(ffi: *mut HashID) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for HashID {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn HashID_ctor(o_0: *mut [u8; 32]) -> *mut HashID {
            ferment_interfaces::boxed(HashID(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn HashID_destroy(ffi: *mut HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the ShouldProcessDiffWithRangeCallback"]
        #[allow(non_camel_case_types)]
        pub type ShouldProcessDiffWithRangeCallback =
            unsafe extern "C" fn(
                base_block_hash: *mut crate::fermented::types::nested::HashID,
                block_hash: *mut crate::fermented::types::nested::HashID,
                context: *mut ferment_interfaces::fermented::types::OpaqueContext_FFI,
            )
                -> *mut crate::fermented::types::nested::ProtocolError;
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: DataContractNotPresentError\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct DataContractNotPresentError {
            pub data_contract_id: *mut crate::fermented::types::nested::Identifier,
        }
        impl ferment_interfaces::FFIConversion<crate::nested::DataContractNotPresentError>
            for DataContractNotPresentError
        {
            unsafe fn ffi_from_const(
                ffi: *const DataContractNotPresentError,
            ) -> crate::nested::DataContractNotPresentError {
                let ffi_ref = &*ffi;
                crate::nested::DataContractNotPresentError {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_from(
                        ffi_ref.data_contract_id,
                    ),
                }
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::DataContractNotPresentError,
            ) -> *const DataContractNotPresentError {
                ferment_interfaces::boxed(DataContractNotPresentError {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_to(
                        obj.data_contract_id,
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut DataContractNotPresentError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for DataContractNotPresentError {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.data_contract_id);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn DataContractNotPresentError_ctor(
            data_contract_id: *mut crate::fermented::types::nested::Identifier,
        ) -> *mut DataContractNotPresentError {
            ferment_interfaces::boxed(DataContractNotPresentError { data_contract_id })
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn DataContractNotPresentError_destroy(
            ffi: *mut DataContractNotPresentError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the find_hash_by_u32"]
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ffi_find_hash_by_u32(
            key: u32,
            map : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_crate_nested_HashID,
        ) -> *mut crate::fermented::types::nested::HashID {
            let obj = crate::nested::find_hash_by_u32(
                key,
                ferment_interfaces::FFIConversion::ffi_from(map),
            );
            ferment_interfaces::FFIConversion::ffi_to_opt(obj)
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: Hash160\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct Hash160(*mut [u8; 20]);
        impl ferment_interfaces::FFIConversion<crate::nested::Hash160> for Hash160 {
            unsafe fn ffi_from_const(ffi: *const Hash160) -> crate::nested::Hash160 {
                let ffi_ref = &*ffi;
                *ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: crate::nested::Hash160) -> *const Hash160 {
                ferment_interfaces::boxed(Hash160(ferment_interfaces::boxed(obj)))
            }
            unsafe fn destroy(ffi: *mut Hash160) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for Hash160 {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn Hash160_ctor(o_0: *mut [u8; 20]) -> *mut Hash160 {
            ferment_interfaces::boxed(Hash160(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn Hash160_destroy(ffi: *mut Hash160) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: MapOfHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct MapOfHashes (* mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID ,) ;
        impl ferment_interfaces::FFIConversion<crate::nested::MapOfHashes> for MapOfHashes {
            unsafe fn ffi_from_const(ffi: *const MapOfHashes) -> crate::nested::MapOfHashes {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: crate::nested::MapOfHashes) -> *const MapOfHashes {
                ferment_interfaces::boxed(MapOfHashes(ferment_interfaces::FFIConversion::ffi_to(
                    obj,
                )))
            }
            unsafe fn destroy(ffi: *mut MapOfHashes) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for MapOfHashes {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn MapOfHashes_ctor(
            o_0 : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
        ) -> *mut MapOfHashes {
            ferment_interfaces::boxed(MapOfHashes(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn MapOfHashes_destroy(ffi: *mut MapOfHashes) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: UsedKeyMatrix\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct UsedKeyMatrix(*mut crate::fermented::generics::Vec_bool);
        impl ferment_interfaces::FFIConversion<crate::nested::UsedKeyMatrix> for UsedKeyMatrix {
            unsafe fn ffi_from_const(ffi: *const UsedKeyMatrix) -> crate::nested::UsedKeyMatrix {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: crate::nested::UsedKeyMatrix) -> *const UsedKeyMatrix {
                ferment_interfaces::boxed(UsedKeyMatrix(ferment_interfaces::FFIConversion::ffi_to(
                    obj,
                )))
            }
            unsafe fn destroy(ffi: *mut UsedKeyMatrix) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for UsedKeyMatrix {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UsedKeyMatrix_ctor(
            o_0: *mut crate::fermented::generics::Vec_bool,
        ) -> *mut UsedKeyMatrix {
            ferment_interfaces::boxed(UsedKeyMatrix(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn UsedKeyMatrix_destroy(ffi: *mut UsedKeyMatrix) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the TestEnum"]
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum TestEnum {
            Variant1 (* mut std :: os :: raw :: c_char ,) , Variant2 , Variant3 (* mut crate :: fermented :: types :: nested :: HashID , u32 ,) , Variant4 (* mut crate :: fermented :: types :: nested :: HashID , u32 , * mut std :: os :: raw :: c_char ,) , Variant5 (* mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_crate_nested_HashID , u32 , * mut std :: os :: raw :: c_char ,) , }
        impl ferment_interfaces::FFIConversion<crate::nested::TestEnum> for TestEnum {
            unsafe fn ffi_from_const(ffi: *const TestEnum) -> crate::nested::TestEnum {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    TestEnum::Variant1(o_0) => crate::nested::TestEnum::Variant1(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                    ),
                    TestEnum::Variant2 => crate::nested::TestEnum::Variant2,
                    TestEnum::Variant3(o_0, o_1) => crate::nested::TestEnum::Variant3(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        *o_1,
                    ),
                    TestEnum::Variant4(o_0, o_1, o_2) => crate::nested::TestEnum::Variant4(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        *o_1,
                        ferment_interfaces::FFIConversion::ffi_from(*o_2),
                    ),
                    TestEnum::Variant5(o_0, o_1, o_2) => crate::nested::TestEnum::Variant5(
                        ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        *o_1,
                        ferment_interfaces::FFIConversion::ffi_from(*o_2),
                    ),
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::TestEnum) -> *const TestEnum {
                ferment_interfaces::boxed(match obj {
                    crate::nested::TestEnum::Variant1(o_0) => {
                        TestEnum::Variant1(ferment_interfaces::FFIConversion::ffi_to(o_0))
                    }
                    crate::nested::TestEnum::Variant2 => TestEnum::Variant2,
                    crate::nested::TestEnum::Variant3(o_0, o_1) => {
                        TestEnum::Variant3(ferment_interfaces::FFIConversion::ffi_to(o_0), o_1)
                    }
                    crate::nested::TestEnum::Variant4(o_0, o_1, o_2) => TestEnum::Variant4(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                        o_1,
                        ferment_interfaces::FFIConversion::ffi_to(o_2),
                    ),
                    crate::nested::TestEnum::Variant5(o_0, o_1, o_2) => TestEnum::Variant5(
                        ferment_interfaces::FFIConversion::ffi_to(o_0),
                        o_1,
                        ferment_interfaces::FFIConversion::ffi_to(o_2),
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut TestEnum) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for TestEnum {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        TestEnum::Variant1(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_0 . to_owned ()) ;
                        }
                        TestEnum::Variant2 => {}
                        TestEnum::Variant3(o_0, o_1) => {
                            ferment_interfaces::unbox_any(o_0.to_owned());
                        }
                        TestEnum::Variant4(o_0, o_1, o_2) => {
                            ferment_interfaces::unbox_any(o_0.to_owned());
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_2 . to_owned ()) ;
                        }
                        TestEnum::Variant5(o_0, o_1, o_2) => {
                            ferment_interfaces::unbox_any(o_0.to_owned());
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (o_2 . to_owned ()) ;
                        }
                    }
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_Variant1_ctor(
            o_o_0: *mut std::os::raw::c_char,
        ) -> *mut TestEnum {
            ferment_interfaces::boxed(TestEnum::Variant1(o_o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_Variant2_ctor() -> *mut TestEnum {
            ferment_interfaces::boxed(TestEnum::Variant2)
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_Variant3_ctor(
            o_o_0: *mut crate::fermented::types::nested::HashID,
            o_o_1: u32,
        ) -> *mut TestEnum {
            ferment_interfaces::boxed(TestEnum::Variant3(o_o_0, o_o_1))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_Variant4_ctor(
            o_o_0: *mut crate::fermented::types::nested::HashID,
            o_o_1: u32,
            o_o_2: *mut std::os::raw::c_char,
        ) -> *mut TestEnum {
            ferment_interfaces::boxed(TestEnum::Variant4(o_o_0, o_o_1, o_o_2))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_Variant5_ctor(
            o_o_0 : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_crate_nested_HashID,
            o_o_1: u32,
            o_o_2: *mut std::os::raw::c_char,
        ) -> *mut TestEnum {
            ferment_interfaces::boxed(TestEnum::Variant5(o_o_0, o_o_1, o_o_2))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn TestEnum_destroy(ffi: *mut TestEnum) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: IdentifierBytes32\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct IdentifierBytes32(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<crate::nested::IdentifierBytes32> for IdentifierBytes32 {
            unsafe fn ffi_from_const(
                ffi: *const IdentifierBytes32,
            ) -> crate::nested::IdentifierBytes32 {
                let ffi_ref = &*ffi;
                crate::nested::IdentifierBytes32(*ffi_ref.0)
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::IdentifierBytes32,
            ) -> *const IdentifierBytes32 {
                ferment_interfaces::boxed(IdentifierBytes32(ferment_interfaces::boxed(obj.0)))
            }
            unsafe fn destroy(ffi: *mut IdentifierBytes32) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for IdentifierBytes32 {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn IdentifierBytes32_ctor(
            o_0: *mut [u8; 32],
        ) -> *mut IdentifierBytes32 {
            ferment_interfaces::boxed(IdentifierBytes32(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn IdentifierBytes32_destroy(ffi: *mut IdentifierBytes32) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the AddInsightCallback"]
        #[allow(non_camel_case_types)]
        pub type AddInsightCallback = unsafe extern "C" fn(
            block_hash: *mut crate::fermented::types::nested::HashID,
            context: *mut ferment_interfaces::fermented::types::OpaqueContext_FFI,
        );
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: Identifier\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct Identifier(*mut crate::fermented::types::nested::IdentifierBytes32);
        impl ferment_interfaces::FFIConversion<crate::nested::Identifier> for Identifier {
            unsafe fn ffi_from_const(ffi: *const Identifier) -> crate::nested::Identifier {
                let ffi_ref = &*ffi;
                crate::nested::Identifier(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::Identifier) -> *const Identifier {
                ferment_interfaces::boxed(Identifier(ferment_interfaces::FFIConversion::ffi_to(
                    obj.0,
                )))
            }
            unsafe fn destroy(ffi: *mut Identifier) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for Identifier {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn Identifier_ctor(
            o_0: *mut crate::fermented::types::nested::IdentifierBytes32,
        ) -> *mut Identifier {
            ferment_interfaces::boxed(Identifier(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn Identifier_destroy(ffi: *mut Identifier) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: ArrayOfArraysOfHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct ArrayOfArraysOfHashes(
            *mut crate::fermented::generics::Vec_Vec_crate_nested_HashID,
        );
        impl ferment_interfaces::FFIConversion<crate::nested::ArrayOfArraysOfHashes>
            for ArrayOfArraysOfHashes
        {
            unsafe fn ffi_from_const(
                ffi: *const ArrayOfArraysOfHashes,
            ) -> crate::nested::ArrayOfArraysOfHashes {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::ArrayOfArraysOfHashes,
            ) -> *const ArrayOfArraysOfHashes {
                ferment_interfaces::boxed(ArrayOfArraysOfHashes(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut ArrayOfArraysOfHashes) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ArrayOfArraysOfHashes {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ArrayOfArraysOfHashes_ctor(
            o_0: *mut crate::fermented::generics::Vec_Vec_crate_nested_HashID,
        ) -> *mut ArrayOfArraysOfHashes {
            ferment_interfaces::boxed(ArrayOfArraysOfHashes(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ArrayOfArraysOfHashes_destroy(ffi: *mut ArrayOfArraysOfHashes) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: MapOfVecHashes\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct MapOfVecHashes (* mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID ,) ;
        impl ferment_interfaces::FFIConversion<crate::nested::MapOfVecHashes> for MapOfVecHashes {
            unsafe fn ffi_from_const(ffi: *const MapOfVecHashes) -> crate::nested::MapOfVecHashes {
                let ffi_ref = &*ffi;
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
            }
            unsafe fn ffi_to_const(obj: crate::nested::MapOfVecHashes) -> *const MapOfVecHashes {
                ferment_interfaces::boxed(MapOfVecHashes(
                    ferment_interfaces::FFIConversion::ffi_to(obj),
                ))
            }
            unsafe fn destroy(ffi: *mut MapOfVecHashes) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for MapOfVecHashes {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn MapOfVecHashes_ctor(
            o_0 : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID,
        ) -> *mut MapOfVecHashes {
            ferment_interfaces::boxed(MapOfVecHashes(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn MapOfVecHashes_destroy(ffi: *mut MapOfVecHashes) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: BinaryData\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct BinaryData(*mut crate::fermented::generics::Vec_u8);
        impl ferment_interfaces::FFIConversion<crate::nested::BinaryData> for BinaryData {
            unsafe fn ffi_from_const(ffi: *const BinaryData) -> crate::nested::BinaryData {
                let ffi_ref = &*ffi;
                crate::nested::BinaryData(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::BinaryData) -> *const BinaryData {
                ferment_interfaces::boxed(BinaryData(ferment_interfaces::FFIConversion::ffi_to(
                    obj.0,
                )))
            }
            unsafe fn destroy(ffi: *mut BinaryData) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for BinaryData {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn BinaryData_ctor(
            o_0: *mut crate::fermented::generics::Vec_u8,
        ) -> *mut BinaryData {
            ferment_interfaces::boxed(BinaryData(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn BinaryData_destroy(ffi: *mut BinaryData) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: SimpleData\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct SimpleData(*mut crate::fermented::generics::Vec_u32);
        impl ferment_interfaces::FFIConversion<crate::nested::SimpleData> for SimpleData {
            unsafe fn ffi_from_const(ffi: *const SimpleData) -> crate::nested::SimpleData {
                let ffi_ref = &*ffi;
                crate::nested::SimpleData(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::SimpleData) -> *const SimpleData {
                ferment_interfaces::boxed(SimpleData(ferment_interfaces::FFIConversion::ffi_to(
                    obj.0,
                )))
            }
            unsafe fn destroy(ffi: *mut SimpleData) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for SimpleData {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn SimpleData_ctor(
            o_0: *mut crate::fermented::generics::Vec_u32,
        ) -> *mut SimpleData {
            ferment_interfaces::boxed(SimpleData(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn SimpleData_destroy(ffi: *mut SimpleData) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    pub mod example {
        pub mod address {
            #[doc = "FFI-representation of the address_complex_simple_result"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_complex_simple_result(
                script: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut crate::fermented::generics::Result_ok_crate_nested_HashID_err_u32
            {
                let obj = crate::example::address::address_complex_simple_result(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the get_chain_hashes_by_map"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_get_chain_hashes_by_map(
                map : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID,
            ) -> *mut std::os::raw::c_char {
                let obj = crate::example::address::get_chain_hashes_by_map(
                    ferment_interfaces::FFIConversion::ffi_from(map),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the get_chain_type_string"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_get_chain_type_string(
                chain_type: *mut crate::fermented::types::chain::common::chain_type::ChainType,
            ) -> *mut std::os::raw::c_char {
                let obj = crate::example::address::get_chain_type_string(
                    ferment_interfaces::FFIConversion::ffi_from(chain_type),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the address_simple_result"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_simple_result(
                script: *mut crate::fermented::generics::Vec_u32,
            ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                let obj = crate::example::address::address_simple_result(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the address_with_script_pubkey"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_with_script_pubkey(
                script: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut std::os::raw::c_char {
                let obj = crate::example::address::address_with_script_pubkey(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to_opt(obj)
            }
            #[doc = "FFI-representation of the address_complex_result"]
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ffi_address_complex_result (script : * mut crate :: fermented :: generics :: Vec_u8 ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError{
                let obj = crate::example::address::address_complex_result(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the address_simple_complex_result"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_simple_complex_result(
                script: *mut crate::fermented::generics::Vec_u32,
            ) -> *mut crate::fermented::generics::Result_ok_u32_err_crate_nested_ProtocolError
            {
                let obj = crate::example::address::address_simple_complex_result(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
    }
    #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: RootStruct\"]"]
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct RootStruct {
        pub name: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<crate::RootStruct> for RootStruct {
        unsafe fn ffi_from_const(ffi: *const RootStruct) -> crate::RootStruct {
            let ffi_ref = &*ffi;
            crate::RootStruct {
                name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
            }
        }
        unsafe fn ffi_to_const(obj: crate::RootStruct) -> *const RootStruct {
            ferment_interfaces::boxed(RootStruct {
                name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
            })
        }
        unsafe fn destroy(ffi: *mut RootStruct) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for RootStruct {
        fn drop(&mut self) {
            unsafe {
                let ffi_ref = self;
                <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                    ffi_ref.name,
                );
            }
        }
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn RootStruct_ctor(name: *mut std::os::raw::c_char) -> *mut RootStruct {
        ferment_interfaces::boxed(RootStruct { name })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn RootStruct_destroy(ffi: *mut RootStruct) {
        ferment_interfaces::unbox_any(ffi);
    }
}
#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
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
    #[allow(non_camel_case_types)]
    pub struct Vec_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<Vec<String>> for Vec_String {
        unsafe fn ffi_from_const(ffi: *const Vec_String) -> Vec<String> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<String>) -> *const Vec_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_String {
        type Value = Vec<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_bool {
        pub count: usize,
        pub values: *mut bool,
    }
    impl ferment_interfaces::FFIConversion<Vec<bool>> for Vec_bool {
        unsafe fn ffi_from_const(ffi: *const Vec_bool) -> Vec<bool> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_bool) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_bool {
        type Value = Vec<bool>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_bool {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>,
        > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32,
        ) -> std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, u32>>,
        ) -> *const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_u32 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<crate::nested::HashID, u32>>
        for std_collections_Map_keys_crate_nested_HashID_values_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_u32,
        ) -> std::collections::BTreeMap<crate::nested::HashID, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| o,
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, u32>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_primitive_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID { pub count : usize , pub keys : * mut u32 , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_crate_nested_HashID , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | o , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , crate :: nested :: HashID > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_primitive_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::chain::common::chain_type::ChainType,
        pub values: *mut *mut crate::fermented::types::nested::HashID,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID > > for std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID) -> std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: chain :: common :: chain_type :: ChainType , crate :: nested :: HashID >) -> * const std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_chain_common_chain_type_ChainType_values_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_Vec_crate_nested_HashID {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_crate_nested_HashID,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, Vec<crate::nested::HashID>>,
        > for std_collections_Map_keys_u32_values_Vec_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_crate_nested_HashID,
        ) -> std::collections::BTreeMap<u32, Vec<crate::nested::HashID>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Vec<crate::nested::HashID>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_crate_nested_HashID {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Vec_crate_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values:
            *mut *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u32,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, Vec<u32>>>,
        > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32,
        ) -> std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, Vec<u32>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, std::collections::BTreeMap<u32, Vec<u32>>>,
        ) -> *const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u8 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_nested_HashID {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::nested::HashID,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::nested::HashID>> for Vec_crate_nested_HashID {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_nested_HashID,
        ) -> Vec<crate::nested::HashID> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<crate::nested::HashID>) -> *const Vec_crate_nested_HashID {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_nested_HashID {
        type Value = Vec<crate::nested::HashID>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_nested_HashID_err_u32 {
        pub ok: *mut crate::fermented::types::nested::HashID,
        pub error: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Result<crate::nested::HashID, u32>>
        for Result_ok_crate_nested_HashID_err_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_crate_nested_HashID_err_u32,
        ) -> Result<crate::nested::HashID, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| *o,
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<crate::nested::HashID, u32>,
        ) -> *const Result_ok_crate_nested_HashID_err_u32 {
            let (ok, error) = match obj {
                Ok(o) => (
                    ferment_interfaces::FFIConversion::ffi_to(o),
                    std::ptr::null_mut(),
                ),
                Err(o) => (std::ptr::null_mut(), o as *mut _),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(ffi: *mut Result_ok_crate_nested_HashID_err_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_crate_nested_HashID_err_u32 {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                }
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID,
        pub values:
            *mut *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u32,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32 { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < u32 > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32 { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_u32 { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID,
        pub values: *mut *mut crate::fermented::generics::Vec_crate_nested_HashID,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>,
        > for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID,
        ) -> std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, Vec<crate::nested::HashID>>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_String_values_crate_nested_HashID {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut crate::fermented::types::nested::HashID,
    }
    impl
        ferment_interfaces::FFIConversion<std::collections::BTreeMap<String, crate::nested::HashID>>
        for std_collections_Map_keys_String_values_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_crate_nested_HashID,
        ) -> std::collections::BTreeMap<String, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<String, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_String_values_crate_nested_HashID {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_String_values_crate_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError {
        pub ok: *mut crate::fermented::types::nested::HashID,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<crate::nested::HashID, crate::nested::ProtocolError>,
        > for Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError,
        ) -> Result<crate::nested::HashID, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<crate::nested::HashID, crate::nested::ProtocolError>,
        ) -> *const Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError {
            let (ok, error) = match obj {
                Ok(o) => (
                    ferment_interfaces::FFIConversion::ffi_to(o),
                    std::ptr::null_mut(),
                ),
                Err(o) => (
                    std::ptr::null_mut(),
                    ferment_interfaces::FFIConversion::ffi_to(o),
                ),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(ffi: *mut Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                }
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: nested :: HashID , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_crate_nested_HashID , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_Vec_u32 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u32,
    }
    impl ferment_interfaces::FFIConversion<Vec<Vec<u32>>> for Vec_Vec_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_Vec_u32) -> Vec<Vec<u32>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<Vec<u32>>) -> *const Vec_Vec_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Vec_u32 {
        type Value = Vec<Vec<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_u32_err_crate_nested_ProtocolError {
        pub ok: *mut u32,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl ferment_interfaces::FFIConversion<Result<u32, crate::nested::ProtocolError>>
        for Result_ok_u32_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_u32_err_crate_nested_ProtocolError,
        ) -> Result<u32, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| *o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<u32, crate::nested::ProtocolError>,
        ) -> *const Result_ok_u32_err_crate_nested_ProtocolError {
            let (ok, error) = match obj {
                Ok(o) => (o as *mut _, std::ptr::null_mut()),
                Err(o) => (
                    std::ptr::null_mut(),
                    ferment_interfaces::FFIConversion::ffi_to(o),
                ),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(ffi: *mut Result_ok_u32_err_crate_nested_ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_crate_nested_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                }
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_Vec_u32 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID,
        pub values: *mut *mut crate::fermented::generics::Vec_u32,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, Vec<u32>>,
        > for std_collections_Map_keys_crate_nested_HashID_values_Vec_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_Vec_u32,
        ) -> std::collections::BTreeMap<crate::nested::HashID, Vec<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, Vec<u32>>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_Vec_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_crate_nested_HashID {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::types::nested::HashID,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, crate::nested::HashID>>
        for std_collections_Map_keys_u32_values_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_crate_nested_HashID,
        ) -> std::collections::BTreeMap<u32, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_u32_values_crate_nested_HashID {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_crate_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::nested::HashID,
        pub values: *mut *mut crate::fermented::types::nested::HashID,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>,
        > for std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
        ) -> std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::nested::HashID, crate::nested::HashID>,
        ) -> *const std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID
        {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Vec<u32>> for Vec_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_u32) -> Vec<u32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<u32>) -> *const Vec_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u32 {
        type Value = Vec<u32>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_Vec_crate_nested_HashID {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_crate_nested_HashID,
    }
    impl ferment_interfaces::FFIConversion<Vec<Vec<crate::nested::HashID>>>
        for Vec_Vec_crate_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_Vec_crate_nested_HashID,
        ) -> Vec<Vec<crate::nested::HashID>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<Vec<crate::nested::HashID>>,
        ) -> *const Vec_Vec_crate_nested_HashID {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Vec_crate_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Vec_crate_nested_HashID {
        type Value = Vec<Vec<crate::nested::HashID>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Vec_crate_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { pub count : usize , pub keys : * mut u32 , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_crate_nested_HashID , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | o , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < u32 , Vec < crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_primitive_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_u32_values_Vec_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: nested :: HashID , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > > for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID) -> std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: nested :: HashID , std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > >) -> * const std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_nested_HashID_values_std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { pub count : usize , pub keys : * mut u32 , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > > for std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID) -> std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | o , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , std :: collections :: BTreeMap < crate :: nested :: HashID , crate :: nested :: HashID > >) -> * const std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_primitive_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_u32_values_std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, u32>>
        for std_collections_Map_keys_u32_values_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_u32,
        ) -> std::collections::BTreeMap<u32, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| o,
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, u32>,
        ) -> *const std_collections_Map_keys_u32_values_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_primitive_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_u32_values_Vec_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Vec<u32>>>
        for std_collections_Map_keys_u32_values_Vec_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_u32,
        ) -> std::collections::BTreeMap<u32, Vec<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Vec<u32>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_u32_err_u32 {
        pub ok: *mut u32,
        pub error: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Result<u32, u32>> for Result_ok_u32_err_u32 {
        unsafe fn ffi_from_const(ffi: *const Result_ok_u32_err_u32) -> Result<u32, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(ffi_ref.ok, ffi_ref.error, |o| *o, |o| *o)
        }
        unsafe fn ffi_to_const(obj: Result<u32, u32>) -> *const Result_ok_u32_err_u32 {
            let (ok, error) = match obj {
                Ok(o) => (o as *mut _, std::ptr::null_mut()),
                Err(o) => (std::ptr::null_mut(), o as *mut _),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(ffi: *mut Result_ok_u32_err_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_u32 {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                }
            }
        }
    }
}
