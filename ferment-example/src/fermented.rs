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
    pub mod nested {
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
                crate::nested::IdentifierBytes32(*ffi_ref.o_0)
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::IdentifierBytes32,
            ) -> *const IdentifierBytes32 {
                ferment_interfaces::boxed(IdentifierBytes32(ferment_interfaces::boxed(obj.o_0)))
            }
            unsafe fn destroy(ffi: *mut IdentifierBytes32) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for IdentifierBytes32 {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.o_0);
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
                    ProtocolError::IdentifierError(o_o_0) => {
                        crate::nested::ProtocolError::IdentifierError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        )
                    }
                    ProtocolError::StringDecodeError(o_o_0) => {
                        crate::nested::ProtocolError::StringDecodeError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        )
                    }
                    ProtocolError::StringDecodeError2(o_o_0, o_o_1) => {
                        crate::nested::ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                            *o_o_1,
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
                    ProtocolError::EncodingError(o_o_0) => {
                        crate::nested::ProtocolError::EncodingError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        )
                    }
                    ProtocolError::EncodingError2(o_o_0) => {
                        crate::nested::ProtocolError::EncodingError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        )
                    }
                    ProtocolError::DataContractNotPresentError(o_o_0) => {
                        crate::nested::ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        )
                    }
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::ProtocolError) -> *const ProtocolError {
                ferment_interfaces::boxed(match obj {
                    crate::nested::ProtocolError::IdentifierError(o_o_0) => {
                        ProtocolError::IdentifierError(ferment_interfaces::FFIConversion::ffi_to(
                            o_o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError(o_o_0) => {
                        ProtocolError::StringDecodeError(ferment_interfaces::FFIConversion::ffi_to(
                            o_o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError2(o_o_0, o_o_1) => {
                        ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_to(o_o_0),
                            o_o_1,
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
                    crate::nested::ProtocolError::EncodingError(o_o_0) => {
                        ProtocolError::EncodingError(ferment_interfaces::FFIConversion::ffi_to(
                            o_o_0,
                        ))
                    }
                    crate::nested::ProtocolError::EncodingError2(o_o_0) => {
                        ProtocolError::EncodingError2(ferment_interfaces::FFIConversion::ffi_to(
                            o_o_0,
                        ))
                    }
                    crate::nested::ProtocolError::DataContractNotPresentError(o_o_0) => {
                        ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_to(o_o_0),
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
                        ProtocolError::IdentifierError(o_o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_o_0) ;
                        }
                        ProtocolError::StringDecodeError(o_o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_o_0) ;
                        }
                        ProtocolError::StringDecodeError2(o_o_0, o_o_1) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_o_0) ;
                        }
                        ProtocolError::EmptyPublicKeyDataError => {}
                        ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => {}
                        ProtocolError::EncodingError(o_o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_o_0) ;
                        }
                        ProtocolError::EncodingError2(o_o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (* o_o_0) ;
                        }
                        ProtocolError::DataContractNotPresentError(o_o_0) => {
                            ferment_interfaces::unbox_any(*o_o_0);
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
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: BinaryData\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct BinaryData(*mut crate::fermented::generics::Vec_u8);
        impl ferment_interfaces::FFIConversion<crate::nested::BinaryData> for BinaryData {
            unsafe fn ffi_from_const(ffi: *const BinaryData) -> crate::nested::BinaryData {
                let ffi_ref = &*ffi;
                crate::nested::BinaryData(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::BinaryData) -> *const BinaryData {
                ferment_interfaces::boxed(BinaryData(ferment_interfaces::FFIConversion::ffi_to(
                    obj.o_0,
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
                    ferment_interfaces::unbox_any(ffi_ref.o_0);
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
                    ferment_interfaces::unbox_any(ffi_ref.o_0);
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
        #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: nested :: Identifier\"]"]
        #[repr(C)]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct Identifier(*mut crate::fermented::types::nested::IdentifierBytes32);
        impl ferment_interfaces::FFIConversion<crate::nested::Identifier> for Identifier {
            unsafe fn ffi_from_const(ffi: *const Identifier) -> crate::nested::Identifier {
                let ffi_ref = &*ffi;
                crate::nested::Identifier(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::Identifier) -> *const Identifier {
                ferment_interfaces::boxed(Identifier(ferment_interfaces::FFIConversion::ffi_to(
                    obj.o_0,
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
                    ferment_interfaces::unbox_any(ffi_ref.o_0);
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
    }
    pub mod asyn {
        pub mod sdk {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: sdk :: Sdk\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Sdk {
                pub proofs: bool,
                pub context_provider: *mut Box,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::sdk::Sdk> for Sdk {
                unsafe fn ffi_from_const(ffi: *const Sdk) -> crate::asyn::sdk::Sdk {
                    let ffi_ref = &*ffi;
                    crate::asyn::sdk::Sdk {
                        proofs: ffi_ref.proofs,
                        context_provider: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.context_provider,
                        ),
                    }
                }
                unsafe fn ffi_to_const(obj: crate::asyn::sdk::Sdk) -> *const Sdk {
                    ferment_interfaces::boxed(Sdk {
                        proofs: obj.proofs,
                        context_provider: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.context_provider,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut Sdk) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Sdk {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.context_provider.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.context_provider);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Sdk_ctor(
                proofs: bool,
                context_provider: *mut Box,
            ) -> *mut Sdk {
                ferment_interfaces::boxed(Sdk {
                    proofs,
                    context_provider,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Sdk_destroy(ffi: *mut Sdk) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod platform_version {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: platform_version :: PlatformVersion\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct PlatformVersion {
                pub protocol_version: u32,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::platform_version::PlatformVersion>
                for PlatformVersion
            {
                unsafe fn ffi_from_const(
                    ffi: *const PlatformVersion,
                ) -> crate::asyn::platform_version::PlatformVersion {
                    let ffi_ref = &*ffi;
                    crate::asyn::platform_version::PlatformVersion {
                        protocol_version: ffi_ref.protocol_version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::platform_version::PlatformVersion,
                ) -> *const PlatformVersion {
                    ferment_interfaces::boxed(PlatformVersion {
                        protocol_version: obj.protocol_version,
                    })
                }
                unsafe fn destroy(ffi: *mut PlatformVersion) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for PlatformVersion {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn PlatformVersion_ctor(
                protocol_version: u32,
            ) -> *mut PlatformVersion {
                ferment_interfaces::boxed(PlatformVersion { protocol_version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn PlatformVersion_destroy(ffi: *mut PlatformVersion) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod provider {
            #[doc = "FFI-representation of the DataContract"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum DataContract {
                V0(*mut crate::fermented::types::asyn::provider::DataContractV0),
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::provider::DataContract> for DataContract {
                unsafe fn ffi_from_const(
                    ffi: *const DataContract,
                ) -> crate::asyn::provider::DataContract {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        DataContract::V0(o_o_0) => crate::asyn::provider::DataContract::V0(
                            ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::provider::DataContract,
                ) -> *const DataContract {
                    ferment_interfaces::boxed(match obj {
                        crate::asyn::provider::DataContract::V0(o_o_0) => {
                            DataContract::V0(ferment_interfaces::FFIConversion::ffi_to(o_o_0))
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut DataContract) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for DataContract {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            DataContract::V0(o_o_0) => {
                                ferment_interfaces::unbox_any(*o_o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DataContract_V0_ctor(
                o_o_0: *mut crate::fermented::types::asyn::provider::DataContractV0,
            ) -> *mut DataContract {
                ferment_interfaces::boxed(DataContract::V0(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DataContract_destroy(ffi: *mut DataContract) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct ContextProvider_VTable { pub get_quorum_public_key : unsafe extern "C" fn (obj : * const () , quorum_type : u32 , quorum_hash : * mut [u8 ; 32] , core_chain_locked_height : u32 ,) -> * mut crate :: fermented :: generics :: Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError , pub get_data_contract : unsafe extern "C" fn (obj : * const () , id : * mut crate :: fermented :: types :: nested :: Identifier ,) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct ContextProvider {
                pub object: *const (),
                pub vtable: *const ContextProvider_VTable,
            }
            #[doc = "FFI-representation of the ContextProviderError"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum ContextProviderError {
                Generic(*mut std::os::raw::c_char),
                Config(*mut std::os::raw::c_char),
                InvalidDataContract(*mut std::os::raw::c_char),
                InvalidQuorum(*mut std::os::raw::c_char),
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::provider::ContextProviderError>
                for ContextProviderError
            {
                unsafe fn ffi_from_const(
                    ffi: *const ContextProviderError,
                ) -> crate::asyn::provider::ContextProviderError {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ContextProviderError::Generic(o_o_0) => {
                            crate::asyn::provider::ContextProviderError::Generic(
                                ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                            )
                        }
                        ContextProviderError::Config(o_o_0) => {
                            crate::asyn::provider::ContextProviderError::Config(
                                ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                            )
                        }
                        ContextProviderError::InvalidDataContract(o_o_0) => {
                            crate::asyn::provider::ContextProviderError::InvalidDataContract(
                                ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                            )
                        }
                        ContextProviderError::InvalidQuorum(o_o_0) => {
                            crate::asyn::provider::ContextProviderError::InvalidQuorum(
                                ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::provider::ContextProviderError,
                ) -> *const ContextProviderError {
                    ferment_interfaces::boxed(match obj {
                        crate::asyn::provider::ContextProviderError::Generic(o_o_0) => {
                            ContextProviderError::Generic(
                                ferment_interfaces::FFIConversion::ffi_to(o_o_0),
                            )
                        }
                        crate::asyn::provider::ContextProviderError::Config(o_o_0) => {
                            ContextProviderError::Config(ferment_interfaces::FFIConversion::ffi_to(
                                o_o_0,
                            ))
                        }
                        crate::asyn::provider::ContextProviderError::InvalidDataContract(o_o_0) => {
                            ContextProviderError::InvalidDataContract(
                                ferment_interfaces::FFIConversion::ffi_to(o_o_0),
                            )
                        }
                        crate::asyn::provider::ContextProviderError::InvalidQuorum(o_o_0) => {
                            ContextProviderError::InvalidQuorum(
                                ferment_interfaces::FFIConversion::ffi_to(o_o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ContextProviderError) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ContextProviderError {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ContextProviderError::Generic(o_o_0) => {
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*o_o_0);
                            }
                            ContextProviderError::Config(o_o_0) => {
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*o_o_0);
                            }
                            ContextProviderError::InvalidDataContract(o_o_0) => {
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*o_o_0);
                            }
                            ContextProviderError::InvalidQuorum(o_o_0) => {
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*o_o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ContextProviderError_Generic_ctor(
                o_o_0: *mut std::os::raw::c_char,
            ) -> *mut ContextProviderError {
                ferment_interfaces::boxed(ContextProviderError::Generic(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ContextProviderError_Config_ctor(
                o_o_0: *mut std::os::raw::c_char,
            ) -> *mut ContextProviderError {
                ferment_interfaces::boxed(ContextProviderError::Config(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ContextProviderError_InvalidDataContract_ctor(
                o_o_0: *mut std::os::raw::c_char,
            ) -> *mut ContextProviderError {
                ferment_interfaces::boxed(ContextProviderError::InvalidDataContract(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ContextProviderError_InvalidQuorum_ctor(
                o_o_0: *mut std::os::raw::c_char,
            ) -> *mut ContextProviderError {
                ferment_interfaces::boxed(ContextProviderError::InvalidQuorum(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ContextProviderError_destroy(ffi: *mut ContextProviderError) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: provider :: DataContractV0\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct DataContractV0 {
                pub id: *mut crate::fermented::types::nested::Identifier,
                pub version: u32,
            }
            impl ferment_interfaces::FFIConversion<crate::asyn::provider::DataContractV0> for DataContractV0 {
                unsafe fn ffi_from_const(
                    ffi: *const DataContractV0,
                ) -> crate::asyn::provider::DataContractV0 {
                    let ffi_ref = &*ffi;
                    crate::asyn::provider::DataContractV0 {
                        id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.id),
                        version: ffi_ref.version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::provider::DataContractV0,
                ) -> *const DataContractV0 {
                    ferment_interfaces::boxed(DataContractV0 {
                        id: ferment_interfaces::FFIConversion::ffi_to(obj.id),
                        version: obj.version,
                    })
                }
                unsafe fn destroy(ffi: *mut DataContractV0) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for DataContractV0 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.id);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DataContractV0_ctor(
                id: *mut crate::fermented::types::nested::Identifier,
                version: u32,
            ) -> *mut DataContractV0 {
                ferment_interfaces::boxed(DataContractV0 { id, version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DataContractV0_destroy(ffi: *mut DataContractV0) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod dapi_request {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct DapiRequest_VTable {
                pub execute : unsafe extern "C" fn (
                    obj : * const () ,
                    dapi_client : * mut crate :: fermented :: types :: asyn :: dapi_request :: D ,
                    settings : * mut crate :: fermented :: types :: asyn :: query :: RequestSettings ,)
                    -> * mut crate :: fermented :: types :: asyn :: dapi_request :: BoxFuture , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct DapiRequest {
                pub object: *const (),
                pub vtable: *const DapiRequest_VTable,
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: dapi_request :: BoxFuture\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct BoxFuture(*mut std::pin::Pin);
            impl ferment_interfaces::FFIConversion<crate::asyn::dapi_request::BoxFuture> for BoxFuture {
                unsafe fn ffi_from_const(
                    ffi: *const BoxFuture,
                ) -> crate::asyn::dapi_request::BoxFuture {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: crate::asyn::dapi_request::BoxFuture,
                ) -> *const BoxFuture {
                    ferment_interfaces::boxed(BoxFuture(ferment_interfaces::FFIConversion::ffi_to(
                        obj,
                    )))
                }
                unsafe fn destroy(ffi: *mut BoxFuture) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for BoxFuture {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.o_0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn BoxFuture_ctor(o_0: *mut std::pin::Pin) -> *mut BoxFuture {
                ferment_interfaces::boxed(BoxFuture(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn BoxFuture_destroy(ffi: *mut BoxFuture) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod proof {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct FromProof_VTable { pub maybe_from_proof : unsafe extern "C" fn (request : * mut crate :: fermented :: types :: asyn :: proof :: I , response : * mut crate :: fermented :: types :: asyn :: proof :: O , platform_version : * mut crate :: fermented :: types :: asyn :: platform_version :: PlatformVersion , provider : * mut ContextProvider ,) -> * mut crate :: fermented :: generics :: Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError , pub from_proof : unsafe extern "C" fn (request : * mut crate :: fermented :: types :: asyn :: proof :: I , response : * mut crate :: fermented :: types :: asyn :: proof :: O , platform_version : * mut crate :: fermented :: types :: asyn :: platform_version :: PlatformVersion , provider : * mut ContextProvider ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct FromProof {
                pub object: *const (),
                pub vtable: *const FromProof_VTable,
            }
        }
        pub mod dapi_client {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Dapi_VTable { pub execute : unsafe extern "C" fn (obj : * const () , request : * mut crate :: fermented :: types :: asyn :: dapi_client :: R , settings : * mut crate :: fermented :: types :: asyn :: query :: RequestSettings ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Dapi {
                pub object: *const (),
                pub vtable: *const Dapi_VTable,
            }
        }
        pub mod query {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportClient_VTable { pub with_uri : unsafe extern "C" fn (uri : * mut crate :: fermented :: types :: asyn :: query :: Uri ,) -> * mut crate :: fermented :: types :: asyn :: query :: TransportClient , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportClient {
                pub object: *const (),
                pub vtable: *const TransportClient_VTable,
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: query :: AppliedRequestSettings\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct AppliedRequestSettings {
                pub timeout: *mut crate::Duration_FFI,
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
                timeout: *mut crate::Duration_FFI,
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
            pub struct Query_VTable { pub query : unsafe extern "C" fn (obj : * const () , prove : bool ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Query {
                pub object: *const (),
                pub vtable: *const Query_VTable,
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
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CanRetry_VTable {
                pub can_retry: unsafe extern "C" fn(obj: *const ()) -> bool,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CanRetry {
                pub object: *const (),
                pub vtable: *const CanRetry_VTable,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportRequest_VTable { pub execute_transport : unsafe extern "C" fn (obj : * const () , client : * mut crate :: fermented :: types :: asyn :: query :: TransportClient , settings : * mut crate :: fermented :: types :: asyn :: query :: AppliedRequestSettings ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportRequest {
                pub object: *const (),
                pub vtable: *const TransportRequest_VTable,
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: asyn :: query :: RequestSettings\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct RequestSettings {
                pub timeout: *mut crate::Duration_FFI,
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
                timeout: *mut crate::Duration_FFI,
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
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportResponse_VTable {}
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransportResponse {
                pub object: *const (),
                pub vtable: *const TransportResponse_VTable,
            }
        }
        pub mod fetch {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Fetch_VTable { pub fetch : unsafe extern "C" fn (sdk : * mut crate :: fermented :: types :: asyn :: sdk :: Sdk , query : * mut crate :: fermented :: types :: asyn :: fetch :: Q ,) -> * mut crate :: fermented :: generics :: Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError , pub fetch_by_identifier : unsafe extern "C" fn (sdk : * mut crate :: fermented :: types :: asyn :: sdk :: Sdk , id : * mut crate :: fermented :: types :: nested :: Identifier ,) -> * mut crate :: fermented :: generics :: Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError , }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Fetch {
                pub object: *const (),
                pub vtable: *const Fetch_VTable,
            }
        }
        pub mod mock {
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MockResponse_VTable {}
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MockResponse {
                pub object: *const (),
                pub vtable: *const MockResponse_VTable,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MockRequest_VTable {
                pub mock_key: unsafe extern "C" fn(
                    obj: *const (),
                )
                    -> *mut crate::fermented::types::nested::HashID,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MockRequest {
                pub object: *const (),
                pub vtable: *const MockRequest_VTable,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Mockable_VTable {
                pub mock_serialize:
                    unsafe extern "C" fn(obj: *const ()) -> *mut crate::fermented::generics::Vec_u8,
                pub mock_deserialize:
                    unsafe extern "C" fn(
                        _data: u8,
                    )
                        -> *mut crate::fermented::types::asyn::mock::Mockable,
            }
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Mockable {
                pub object: *const (),
                pub vtable: *const Mockable_VTable,
            }
        }
    }
    pub mod identity {
        pub mod identity_request {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityByPublicKeyHashRequest\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetIdentityByPublicKeyHashRequest { pub version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::GetIdentityByPublicKeyHashRequest,
                > for GetIdentityByPublicKeyHashRequest
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetIdentityByPublicKeyHashRequest,
                ) -> crate::identity::identity_request::GetIdentityByPublicKeyHashRequest
                {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::GetIdentityByPublicKeyHashRequest {
                        version: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.version),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::GetIdentityByPublicKeyHashRequest,
                ) -> *const GetIdentityByPublicKeyHashRequest {
                    ferment_interfaces::boxed(GetIdentityByPublicKeyHashRequest {
                        version: ferment_interfaces::FFIConversion::ffi_to_opt(obj.version),
                    })
                }
                unsafe fn destroy(ffi: *mut GetIdentityByPublicKeyHashRequest) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetIdentityByPublicKeyHashRequest {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.version.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.version);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityByPublicKeyHashRequest_ctor(
                version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version,
            ) -> *mut GetIdentityByPublicKeyHashRequest {
                ferment_interfaces::boxed(GetIdentityByPublicKeyHashRequest { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityByPublicKeyHashRequest_destroy(
                ffi: *mut GetIdentityByPublicKeyHashRequest,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod get_identity_response {
                #[doc = "FFI-representation of the Version"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum Version {
                    V0 (* mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0 ,) , }
                impl
                    ferment_interfaces::FFIConversion<
                        crate::identity::identity_request::get_identity_response::Version,
                    > for Version
                {
                    unsafe fn ffi_from_const(
                        ffi: *const Version,
                    ) -> crate::identity::identity_request::get_identity_response::Version
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { Version :: V0 (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_response :: Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::identity::identity_request::get_identity_response::Version,
                    ) -> *const Version {
                        ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_response :: Version :: V0 (o_o_0 ,) => Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , })
                    }
                    unsafe fn destroy(ffi: *mut Version) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for Version {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                Version::V0(o_o_0) => {
                                    ferment_interfaces::unbox_any(*o_o_0);
                                }
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0,
                ) -> *mut Version {
                    ferment_interfaces::boxed(Version::V0(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_destroy(ffi: *mut Version) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct GetIdentityResponseV0 { pub metadata : * mut crate :: fermented :: types :: identity :: identity_request :: ResponseMetadata , pub result : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result , }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0 > for GetIdentityResponseV0 { unsafe fn ffi_from_const (ffi : * const GetIdentityResponseV0) -> crate :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0 { let ffi_ref = & * ffi ; crate :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0 { metadata : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . metadata) , result : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . result) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_response :: GetIdentityResponseV0) -> * const GetIdentityResponseV0 { ferment_interfaces :: boxed (GetIdentityResponseV0 { metadata : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . metadata) , result : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . result) , }) } unsafe fn destroy (ffi : * mut GetIdentityResponseV0) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for GetIdentityResponseV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            if !ffi_ref.metadata.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.metadata);
                            };
                            if !ffi_ref.result.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.result);
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityResponseV0_ctor(
                    metadata : * mut crate :: fermented :: types :: identity :: identity_request :: ResponseMetadata,
                    result : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result,
                ) -> *mut GetIdentityResponseV0 {
                    ferment_interfaces::boxed(GetIdentityResponseV0 { metadata, result })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityResponseV0_destroy(
                    ffi: *mut GetIdentityResponseV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                pub mod get_identity_response_v0 {
                    #[doc = "FFI-representation of the Result"]
                    #[repr(C)]
                    #[allow(non_camel_case_types)]
                    #[derive(Clone)]
                    pub enum Result {
                        Identity(*mut crate::fermented::generics::Vec_u8),
                        Proof(*mut crate::fermented::types::identity::identity_request::Proof),
                    }
                    impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result > for Result { unsafe fn ffi_from_const (ffi : * const Result) -> crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result { let ffi_ref = & * ffi ; match ffi_ref { Result :: Identity (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result :: Identity (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , Result :: Proof (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result :: Proof (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result) -> * const Result { ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result :: Identity (o_o_0 ,) => Result :: Identity (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , crate :: identity :: identity_request :: get_identity_response :: get_identity_response_v0 :: Result :: Proof (o_o_0 ,) => Result :: Proof (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , }) } unsafe fn destroy (ffi : * mut Result) { ferment_interfaces :: unbox_any (ffi) ; } }
                    impl Drop for Result {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    Result::Identity(o_o_0) => {
                                        ferment_interfaces::unbox_any(*o_o_0);
                                    }
                                    Result::Proof(o_o_0) => {
                                        ferment_interfaces::unbox_any(*o_o_0);
                                    }
                                }
                            }
                        }
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_Identity_ctor(
                        o_o_0: *mut crate::fermented::generics::Vec_u8,
                    ) -> *mut Result {
                        ferment_interfaces::boxed(Result::Identity(o_o_0))
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_Proof_ctor(
                        o_o_0: *mut crate::fermented::types::identity::identity_request::Proof,
                    ) -> *mut Result {
                        ferment_interfaces::boxed(Result::Proof(o_o_0))
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_destroy(ffi: *mut Result) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
            #[doc = "FFI-representation of the IdentityRequest"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum IdentityRequest {
                GetIdentity (* mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityRequest ,) , GetIdentityByPublicKeyHash (* mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityByPublicKeyHashRequest ,) , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::IdentityRequest,
                > for IdentityRequest
            {
                unsafe fn ffi_from_const(
                    ffi: *const IdentityRequest,
                ) -> crate::identity::identity_request::IdentityRequest {
                    let ffi_ref = &*ffi;
                    match ffi_ref { IdentityRequest :: GetIdentity (o_o_0 ,) => crate :: identity :: identity_request :: IdentityRequest :: GetIdentity (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , IdentityRequest :: GetIdentityByPublicKeyHash (o_o_0 ,) => crate :: identity :: identity_request :: IdentityRequest :: GetIdentityByPublicKeyHash (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::IdentityRequest,
                ) -> *const IdentityRequest {
                    ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: IdentityRequest :: GetIdentity (o_o_0 ,) => IdentityRequest :: GetIdentity (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , crate :: identity :: identity_request :: IdentityRequest :: GetIdentityByPublicKeyHash (o_o_0 ,) => IdentityRequest :: GetIdentityByPublicKeyHash (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , })
                }
                unsafe fn destroy(ffi: *mut IdentityRequest) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for IdentityRequest {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            IdentityRequest::GetIdentity(o_o_0) => {
                                ferment_interfaces::unbox_any(*o_o_0);
                            }
                            IdentityRequest::GetIdentityByPublicKeyHash(o_o_0) => {
                                ferment_interfaces::unbox_any(*o_o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityRequest_GetIdentity_ctor(
                o_o_0: *mut crate::fermented::types::identity::identity_request::GetIdentityRequest,
            ) -> *mut IdentityRequest {
                ferment_interfaces::boxed(IdentityRequest::GetIdentity(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityRequest_GetIdentityByPublicKeyHash_ctor(
                o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityByPublicKeyHashRequest,
            ) -> *mut IdentityRequest {
                ferment_interfaces::boxed(IdentityRequest::GetIdentityByPublicKeyHash(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityRequest_destroy(ffi: *mut IdentityRequest) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: ResponseMetadata\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct ResponseMetadata {
                pub height: u64,
                pub core_chain_locked_height: u32,
                pub epoch: u32,
                pub time_ms: u64,
                pub protocol_version: u32,
                pub chain_id: *mut std::os::raw::c_char,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::ResponseMetadata,
                > for ResponseMetadata
            {
                unsafe fn ffi_from_const(
                    ffi: *const ResponseMetadata,
                ) -> crate::identity::identity_request::ResponseMetadata {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::ResponseMetadata {
                        height: ffi_ref.height,
                        core_chain_locked_height: ffi_ref.core_chain_locked_height,
                        epoch: ffi_ref.epoch,
                        time_ms: ffi_ref.time_ms,
                        protocol_version: ffi_ref.protocol_version,
                        chain_id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.chain_id),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::ResponseMetadata,
                ) -> *const ResponseMetadata {
                    ferment_interfaces::boxed(ResponseMetadata {
                        height: obj.height,
                        core_chain_locked_height: obj.core_chain_locked_height,
                        epoch: obj.epoch,
                        time_ms: obj.time_ms,
                        protocol_version: obj.protocol_version,
                        chain_id: ferment_interfaces::FFIConversion::ffi_to(obj.chain_id),
                    })
                }
                unsafe fn destroy(ffi: *mut ResponseMetadata) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ResponseMetadata {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . chain_id) ;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ResponseMetadata_ctor(
                height: u64,
                core_chain_locked_height: u32,
                epoch: u32,
                time_ms: u64,
                protocol_version: u32,
                chain_id: *mut std::os::raw::c_char,
            ) -> *mut ResponseMetadata {
                ferment_interfaces::boxed(ResponseMetadata {
                    height,
                    core_chain_locked_height,
                    epoch,
                    time_ms,
                    protocol_version,
                    chain_id,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ResponseMetadata_destroy(ffi: *mut ResponseMetadata) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: Proof\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Proof {
                pub grovedb_proof: *mut crate::fermented::generics::Vec_u8,
                pub quorum_hash: *mut crate::fermented::generics::Vec_u8,
                pub signature: *mut crate::fermented::generics::Vec_u8,
                pub round: u32,
                pub block_id_hash: *mut crate::fermented::generics::Vec_u8,
                pub quorum_type: u32,
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity_request::Proof> for Proof {
                unsafe fn ffi_from_const(
                    ffi: *const Proof,
                ) -> crate::identity::identity_request::Proof {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::Proof {
                        grovedb_proof: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.grovedb_proof,
                        ),
                        quorum_hash: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.quorum_hash,
                        ),
                        signature: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.signature),
                        round: ffi_ref.round,
                        block_id_hash: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.block_id_hash,
                        ),
                        quorum_type: ffi_ref.quorum_type,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::Proof,
                ) -> *const Proof {
                    ferment_interfaces::boxed(Proof {
                        grovedb_proof: ferment_interfaces::FFIConversion::ffi_to(obj.grovedb_proof),
                        quorum_hash: ferment_interfaces::FFIConversion::ffi_to(obj.quorum_hash),
                        signature: ferment_interfaces::FFIConversion::ffi_to(obj.signature),
                        round: obj.round,
                        block_id_hash: ferment_interfaces::FFIConversion::ffi_to(obj.block_id_hash),
                        quorum_type: obj.quorum_type,
                    })
                }
                unsafe fn destroy(ffi: *mut Proof) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Proof {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.grovedb_proof);
                        ferment_interfaces::unbox_any(ffi_ref.quorum_hash);
                        ferment_interfaces::unbox_any(ffi_ref.signature);
                        ferment_interfaces::unbox_any(ffi_ref.block_id_hash);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Proof_ctor(
                grovedb_proof: *mut crate::fermented::generics::Vec_u8,
                quorum_hash: *mut crate::fermented::generics::Vec_u8,
                signature: *mut crate::fermented::generics::Vec_u8,
                round: u32,
                block_id_hash: *mut crate::fermented::generics::Vec_u8,
                quorum_type: u32,
            ) -> *mut Proof {
                ferment_interfaces::boxed(Proof {
                    grovedb_proof,
                    quorum_hash,
                    signature,
                    round,
                    block_id_hash,
                    quorum_type,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Proof_destroy(ffi: *mut Proof) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityRequest\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetIdentityRequest { pub version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_request :: Version , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::GetIdentityRequest,
                > for GetIdentityRequest
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetIdentityRequest,
                ) -> crate::identity::identity_request::GetIdentityRequest {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::GetIdentityRequest {
                        version: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.version),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::GetIdentityRequest,
                ) -> *const GetIdentityRequest {
                    ferment_interfaces::boxed(GetIdentityRequest {
                        version: ferment_interfaces::FFIConversion::ffi_to_opt(obj.version),
                    })
                }
                unsafe fn destroy(ffi: *mut GetIdentityRequest) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetIdentityRequest {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.version.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.version);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityRequest_ctor(
                version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_request :: Version,
            ) -> *mut GetIdentityRequest {
                ferment_interfaces::boxed(GetIdentityRequest { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityRequest_destroy(ffi: *mut GetIdentityRequest) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod get_identity_by_public_key_hash_response {
                #[doc = "FFI-representation of the Version"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum Version {
                    V0 (* mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0 ,) , }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version > for Version { unsafe fn ffi_from_const (ffi : * const Version) -> crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version { let ffi_ref = & * ffi ; match ffi_ref { Version :: V0 (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version) -> * const Version { ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version :: V0 (o_o_0 ,) => Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , }) } unsafe fn destroy (ffi : * mut Version) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for Version {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                Version::V0(o_o_0) => {
                                    ferment_interfaces::unbox_any(*o_o_0);
                                }
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0,
                ) -> *mut Version {
                    ferment_interfaces::boxed(Version::V0(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_destroy(ffi: *mut Version) {
                    ferment_interfaces::unbox_any(ffi);
                }
                pub mod get_identity_by_public_key_hash_response_v0 {
                    #[doc = "FFI-representation of the Result"]
                    #[repr(C)]
                    #[allow(non_camel_case_types)]
                    #[derive(Clone)]
                    pub enum Result {
                        Identity(*mut crate::fermented::generics::Vec_u8),
                        Proof(*mut crate::fermented::types::identity::identity_request::Proof),
                    }
                    impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result > for Result { unsafe fn ffi_from_const (ffi : * const Result) -> crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result { let ffi_ref = & * ffi ; match ffi_ref { Result :: Identity (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result :: Identity (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , Result :: Proof (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result :: Proof (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result) -> * const Result { ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result :: Identity (o_o_0 ,) => Result :: Identity (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result :: Proof (o_o_0 ,) => Result :: Proof (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , }) } unsafe fn destroy (ffi : * mut Result) { ferment_interfaces :: unbox_any (ffi) ; } }
                    impl Drop for Result {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    Result::Identity(o_o_0) => {
                                        ferment_interfaces::unbox_any(*o_o_0);
                                    }
                                    Result::Proof(o_o_0) => {
                                        ferment_interfaces::unbox_any(*o_o_0);
                                    }
                                }
                            }
                        }
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_Identity_ctor(
                        o_o_0: *mut crate::fermented::generics::Vec_u8,
                    ) -> *mut Result {
                        ferment_interfaces::boxed(Result::Identity(o_o_0))
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_Proof_ctor(
                        o_o_0: *mut crate::fermented::types::identity::identity_request::Proof,
                    ) -> *mut Result {
                        ferment_interfaces::boxed(Result::Proof(o_o_0))
                    }
                    #[doc = r" # Safety"]
                    #[allow(non_snake_case)]
                    #[no_mangle]
                    pub unsafe extern "C" fn Result_destroy(ffi: *mut Result) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct GetIdentityByPublicKeyHashResponseV0 { pub metadata : * mut crate :: fermented :: types :: identity :: identity_request :: ResponseMetadata , pub result : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result , }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0 > for GetIdentityByPublicKeyHashResponseV0 { unsafe fn ffi_from_const (ffi : * const GetIdentityByPublicKeyHashResponseV0) -> crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0 { let ffi_ref = & * ffi ; crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0 { metadata : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . metadata) , result : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . result) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_by_public_key_hash_response :: GetIdentityByPublicKeyHashResponseV0) -> * const GetIdentityByPublicKeyHashResponseV0 { ferment_interfaces :: boxed (GetIdentityByPublicKeyHashResponseV0 { metadata : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . metadata) , result : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . result) , }) } unsafe fn destroy (ffi : * mut GetIdentityByPublicKeyHashResponseV0) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for GetIdentityByPublicKeyHashResponseV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            if !ffi_ref.metadata.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.metadata);
                            };
                            if !ffi_ref.result.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.result);
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityByPublicKeyHashResponseV0_ctor(
                    metadata : * mut crate :: fermented :: types :: identity :: identity_request :: ResponseMetadata,
                    result : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: get_identity_by_public_key_hash_response_v0 :: Result,
                ) -> *mut GetIdentityByPublicKeyHashResponseV0 {
                    ferment_interfaces::boxed(GetIdentityByPublicKeyHashResponseV0 {
                        metadata,
                        result,
                    })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityByPublicKeyHashResponseV0_destroy(
                    ffi: *mut GetIdentityByPublicKeyHashResponseV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityByPublicKeyHashResponse\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetIdentityByPublicKeyHashResponse { pub version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::GetIdentityByPublicKeyHashResponse,
                > for GetIdentityByPublicKeyHashResponse
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetIdentityByPublicKeyHashResponse,
                ) -> crate::identity::identity_request::GetIdentityByPublicKeyHashResponse
                {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::GetIdentityByPublicKeyHashResponse {
                        version: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.version),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::GetIdentityByPublicKeyHashResponse,
                ) -> *const GetIdentityByPublicKeyHashResponse {
                    ferment_interfaces::boxed(GetIdentityByPublicKeyHashResponse {
                        version: ferment_interfaces::FFIConversion::ffi_to_opt(obj.version),
                    })
                }
                unsafe fn destroy(ffi: *mut GetIdentityByPublicKeyHashResponse) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetIdentityByPublicKeyHashResponse {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.version.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.version);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityByPublicKeyHashResponse_ctor(
                version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_response :: Version,
            ) -> *mut GetIdentityByPublicKeyHashResponse {
                ferment_interfaces::boxed(GetIdentityByPublicKeyHashResponse { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityByPublicKeyHashResponse_destroy(
                ffi: *mut GetIdentityByPublicKeyHashResponse,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: GetIdentityResponse\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetIdentityResponse { pub version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: Version , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::GetIdentityResponse,
                > for GetIdentityResponse
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetIdentityResponse,
                ) -> crate::identity::identity_request::GetIdentityResponse {
                    let ffi_ref = &*ffi;
                    crate::identity::identity_request::GetIdentityResponse {
                        version: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.version),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::GetIdentityResponse,
                ) -> *const GetIdentityResponse {
                    ferment_interfaces::boxed(GetIdentityResponse {
                        version: ferment_interfaces::FFIConversion::ffi_to_opt(obj.version),
                    })
                }
                unsafe fn destroy(ffi: *mut GetIdentityResponse) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetIdentityResponse {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.version.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.version);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityResponse_ctor(
                version : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_response :: Version,
            ) -> *mut GetIdentityResponse {
                ferment_interfaces::boxed(GetIdentityResponse { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetIdentityResponse_destroy(ffi: *mut GetIdentityResponse) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod get_identity_by_public_key_hash_request {
                #[doc = "FFI-representation of the Version"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum Version {
                    V0 (* mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0 ,) , }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version > for Version { unsafe fn ffi_from_const (ffi : * const Version) -> crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version { let ffi_ref = & * ffi ; match ffi_ref { Version :: V0 (o_o_0 ,) => crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version) -> * const Version { ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: Version :: V0 (o_o_0 ,) => Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , }) } unsafe fn destroy (ffi : * mut Version) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for Version {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                Version::V0(o_o_0) => {
                                    ferment_interfaces::unbox_any(*o_o_0);
                                }
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0,
                ) -> *mut Version {
                    ferment_interfaces::boxed(Version::V0(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_destroy(ffi: *mut Version) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct GetIdentityByPublicKeyHashRequestV0 {
                    pub public_key_hash: *mut crate::fermented::generics::Vec_u8,
                    pub prove: bool,
                }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0 > for GetIdentityByPublicKeyHashRequestV0 { unsafe fn ffi_from_const (ffi : * const GetIdentityByPublicKeyHashRequestV0) -> crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0 { let ffi_ref = & * ffi ; crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0 { public_key_hash : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . public_key_hash) , prove : ffi_ref . prove , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_by_public_key_hash_request :: GetIdentityByPublicKeyHashRequestV0) -> * const GetIdentityByPublicKeyHashRequestV0 { ferment_interfaces :: boxed (GetIdentityByPublicKeyHashRequestV0 { public_key_hash : ferment_interfaces :: FFIConversion :: ffi_to (obj . public_key_hash) , prove : obj . prove , }) } unsafe fn destroy (ffi : * mut GetIdentityByPublicKeyHashRequestV0) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for GetIdentityByPublicKeyHashRequestV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.public_key_hash);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityByPublicKeyHashRequestV0_ctor(
                    public_key_hash: *mut crate::fermented::generics::Vec_u8,
                    prove: bool,
                ) -> *mut GetIdentityByPublicKeyHashRequestV0 {
                    ferment_interfaces::boxed(GetIdentityByPublicKeyHashRequestV0 {
                        public_key_hash,
                        prove,
                    })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityByPublicKeyHashRequestV0_destroy(
                    ffi: *mut GetIdentityByPublicKeyHashRequestV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod get_identity_request {
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct GetIdentityRequestV0 {
                    pub id: *mut crate::fermented::generics::Vec_u8,
                    pub prove: bool,
                }
                impl ferment_interfaces :: FFIConversion < crate :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0 > for GetIdentityRequestV0 { unsafe fn ffi_from_const (ffi : * const GetIdentityRequestV0) -> crate :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0 { let ffi_ref = & * ffi ; crate :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0 { id : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . id) , prove : ffi_ref . prove , } } unsafe fn ffi_to_const (obj : crate :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0) -> * const GetIdentityRequestV0 { ferment_interfaces :: boxed (GetIdentityRequestV0 { id : ferment_interfaces :: FFIConversion :: ffi_to (obj . id) , prove : obj . prove , }) } unsafe fn destroy (ffi : * mut GetIdentityRequestV0) { ferment_interfaces :: unbox_any (ffi) ; } }
                impl Drop for GetIdentityRequestV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.id);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityRequestV0_ctor(
                    id: *mut crate::fermented::generics::Vec_u8,
                    prove: bool,
                ) -> *mut GetIdentityRequestV0 {
                    ferment_interfaces::boxed(GetIdentityRequestV0 { id, prove })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn GetIdentityRequestV0_destroy(
                    ffi: *mut GetIdentityRequestV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the Version"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum Version {
                    V0 (* mut crate :: fermented :: types :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0 ,) , }
                impl
                    ferment_interfaces::FFIConversion<
                        crate::identity::identity_request::get_identity_request::Version,
                    > for Version
                {
                    unsafe fn ffi_from_const(
                        ffi: *const Version,
                    ) -> crate::identity::identity_request::get_identity_request::Version
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            Version::V0(o_o_0) => {
                                crate::identity::identity_request::get_identity_request::Version::V0(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::identity::identity_request::get_identity_request::Version,
                    ) -> *const Version {
                        ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: get_identity_request :: Version :: V0 (o_o_0 ,) => Version :: V0 (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , })
                    }
                    unsafe fn destroy(ffi: *mut Version) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for Version {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                Version::V0(o_o_0) => {
                                    ferment_interfaces::unbox_any(*o_o_0);
                                }
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: get_identity_request :: GetIdentityRequestV0,
                ) -> *mut Version {
                    ferment_interfaces::boxed(Version::V0(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn Version_destroy(ffi: *mut Version) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            #[doc = "FFI-representation of the IdentityResponse"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum IdentityResponse {
                Unknown , GetIdentity (* mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityResponse ,) , GetIdentityByPublicKeyHash (* mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityByPublicKeyHashResponse ,) , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::identity::identity_request::IdentityResponse,
                > for IdentityResponse
            {
                unsafe fn ffi_from_const(
                    ffi: *const IdentityResponse,
                ) -> crate::identity::identity_request::IdentityResponse {
                    let ffi_ref = &*ffi;
                    match ffi_ref { IdentityResponse :: Unknown => crate :: identity :: identity_request :: IdentityResponse :: Unknown , IdentityResponse :: GetIdentity (o_o_0 ,) => crate :: identity :: identity_request :: IdentityResponse :: GetIdentity (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , IdentityResponse :: GetIdentityByPublicKeyHash (o_o_0 ,) => crate :: identity :: identity_request :: IdentityResponse :: GetIdentityByPublicKeyHash (ferment_interfaces :: FFIConversion :: ffi_from (* o_o_0) ,) , }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity_request::IdentityResponse,
                ) -> *const IdentityResponse {
                    ferment_interfaces :: boxed (match obj { crate :: identity :: identity_request :: IdentityResponse :: Unknown => IdentityResponse :: Unknown , crate :: identity :: identity_request :: IdentityResponse :: GetIdentity (o_o_0 ,) => IdentityResponse :: GetIdentity (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , crate :: identity :: identity_request :: IdentityResponse :: GetIdentityByPublicKeyHash (o_o_0 ,) => IdentityResponse :: GetIdentityByPublicKeyHash (ferment_interfaces :: FFIConversion :: ffi_to (o_o_0) ,) , })
                }
                unsafe fn destroy(ffi: *mut IdentityResponse) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for IdentityResponse {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            IdentityResponse::Unknown => {}
                            IdentityResponse::GetIdentity(o_o_0) => {
                                ferment_interfaces::unbox_any(*o_o_0);
                            }
                            IdentityResponse::GetIdentityByPublicKeyHash(o_o_0) => {
                                ferment_interfaces::unbox_any(*o_o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityResponse_Unknown_ctor() -> *mut IdentityResponse {
                ferment_interfaces::boxed(IdentityResponse::Unknown)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityResponse_GetIdentity_ctor(
                o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityResponse,
            ) -> *mut IdentityResponse {
                ferment_interfaces::boxed(IdentityResponse::GetIdentity(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityResponse_GetIdentityByPublicKeyHash_ctor(
                o_o_0 : * mut crate :: fermented :: types :: identity :: identity_request :: GetIdentityByPublicKeyHashResponse,
            ) -> *mut IdentityResponse {
                ferment_interfaces::boxed(IdentityResponse::GetIdentityByPublicKeyHash(o_o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityResponse_destroy(ffi: *mut IdentityResponse) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod chain {
        pub mod common {
            pub mod chain_type {
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
                pub extern "C" fn DevnetType_as_IHaveChainSettings(
                    obj: *const crate::chain::common::chain_type::DevnetType,
                ) -> IHaveChainSettings {
                    IHaveChainSettings {
                        object: obj as *const (),
                        vtable: &DevnetType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_as_IHaveChainSettings_destroy(
                    obj: IHaveChainSettings,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::DevnetType,
                    );
                }
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_VTable { pub name : unsafe extern "C" fn (obj : * const () ,) -> * mut std :: os :: raw :: c_char , pub genesis_hash : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: nested :: HashID , pub genesis_height : unsafe extern "C" fn (obj : * const () ,) -> u32 , pub has_genesis_hash : unsafe extern "C" fn (obj : * const () , hash : * mut crate :: fermented :: types :: nested :: HashID ,) -> bool , pub get_hash_by_hash : unsafe extern "C" fn (obj : * const () , hash : * mut crate :: fermented :: types :: nested :: HashID ,) -> * mut crate :: fermented :: types :: nested :: HashID , pub should_process_llmq_of_type : unsafe extern "C" fn (obj : * const () , llmq_type : u16 ,) -> bool , pub find_masternode_list : unsafe extern "C" fn (obj : * const () , cached_mn_lists : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID , unknown_mn_lists : * mut crate :: fermented :: generics :: Vec_crate_nested_HashID ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError , }
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings {
                    pub object: *const (),
                    pub vtable: *const IHaveChainSettings_VTable,
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
                            ChainType::DevNet(o_o_0) => {
                                crate::chain::common::chain_type::ChainType::DevNet(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_o_0),
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
                            crate::chain::common::chain_type::ChainType::DevNet(o_o_0) => {
                                ChainType::DevNet(ferment_interfaces::FFIConversion::ffi_to(o_o_0))
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
                                ChainType::DevNet(o_o_0) => {
                                    ferment_interfaces::unbox_any(*o_o_0);
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
                pub extern "C" fn ChainType_as_IHaveChainSettings(
                    obj: *const crate::chain::common::chain_type::ChainType,
                ) -> IHaveChainSettings {
                    IHaveChainSettings {
                        object: obj as *const (),
                        vtable: &ChainType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_as_IHaveChainSettings_destroy(
                    obj: IHaveChainSettings,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::ChainType,
                    );
                }
            }
        }
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
    pub struct Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError
    {
        pub ok: *mut Option<std::sync::Arc<crate::asyn::provider::DataContract>>,
        pub error: *mut crate::fermented::types::asyn::provider::ContextProviderError,
    }
    impl ferment_interfaces :: FFIConversion < Result < Option < std :: sync :: Arc < crate :: asyn :: provider :: DataContract > > , crate :: asyn :: provider :: ContextProviderError > > for Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError) -> Result < Option < std :: sync :: Arc < crate :: asyn :: provider :: DataContract > > , crate :: asyn :: provider :: ContextProviderError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < Option < std :: sync :: Arc < crate :: asyn :: provider :: DataContract > > , crate :: asyn :: provider :: ContextProviderError >) -> * const Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } } } }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError_ctor (ok : * mut Option < std :: sync :: Arc < crate :: asyn :: provider :: DataContract > > , error : * mut crate :: fermented :: types :: asyn :: provider :: ContextProviderError) -> * mut Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError{
        ferment_interfaces :: boxed (Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError { ok , error })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError_destroy(
        ffi : * mut Result_ok_Option_std_sync_Arc_crate_asyn_provider_DataContract_err_crate_asyn_provider_ContextProviderError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error
    {
        pub ok: *mut crate::fermented::types::asyn::query::TransportResponse,
        pub error: *mut crate::fermented::types::asyn::query::CanRetry,
    }
    impl ferment_interfaces :: FFIConversion < Result < crate :: asyn :: query :: TransportRequest :: Response , < crate :: asyn :: query :: TransportRequest :: Client as crate :: asyn :: query :: TransportClient > :: Error > > for Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error { unsafe fn ffi_from_const (ffi : * const Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error) -> Result < crate :: asyn :: query :: TransportRequest :: Response , < crate :: asyn :: query :: TransportRequest :: Client as crate :: asyn :: query :: TransportClient > :: Error > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < crate :: asyn :: query :: TransportRequest :: Response , < crate :: asyn :: query :: TransportRequest :: Client as crate :: asyn :: query :: TransportClient > :: Error >) -> * const Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } } } }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error_ctor (ok : * mut crate :: fermented :: types :: asyn :: query :: TransportResponse , error : * mut crate :: fermented :: types :: asyn :: query :: CanRetry) -> * mut Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error{
        ferment_interfaces :: boxed (Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error { ok , error })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error_destroy(
        ffi : * mut Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError {
        pub ok: *mut Option<crate::asyn::fetch::Fetch>,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<Option<crate::asyn::fetch::Fetch>, crate::nested::ProtocolError>,
        > for Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError,
        ) -> Result<Option<crate::asyn::fetch::Fetch>, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<Option<crate::asyn::fetch::Fetch>, crate::nested::ProtocolError>,
        ) -> *const Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError {
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
        unsafe fn destroy(
            ffi: *mut Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError {
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError_ctor(
        ok: *mut Option<crate::asyn::fetch::Fetch>,
        error: *mut crate::fermented::types::nested::ProtocolError,
    ) -> *mut Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError_destroy(
        ffi: *mut Result_ok_Option_crate_asyn_fetch_Fetch_err_crate_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
        pub ok: *mut crate::fermented::types::asyn::proof::FromProof,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<crate::asyn::proof::FromProof, crate::nested::ProtocolError>,
        > for Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
        ) -> Result<crate::asyn::proof::FromProof, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<crate::asyn::proof::FromProof, crate::nested::ProtocolError>,
        ) -> *const Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
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
        unsafe fn destroy(
            ffi: *mut Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError_ctor(
        ok: *mut crate::fermented::types::asyn::proof::FromProof,
        error: *mut crate::fermented::types::nested::ProtocolError,
    ) -> *mut Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError_destroy(
        ffi: *mut Result_ok_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_ctor(
        keys: *mut *mut crate::fermented::types::nested::HashID,
        values: *mut *mut crate::fermented::types::nested::HashID,
        count: usize,
    ) -> *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID {
        ferment_interfaces::boxed(
            std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID {
                count,
                keys,
                values,
            },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID_destroy(
        ffi: *mut std_collections_Map_keys_crate_nested_HashID_values_crate_nested_HashID,
    ) {
        ferment_interfaces::unbox_any(ffi);
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_crate_nested_HashID_ctor(
        values: *mut *mut crate::fermented::types::nested::HashID,
        count: usize,
    ) -> *mut Vec_crate_nested_HashID {
        ferment_interfaces::boxed(Vec_crate_nested_HashID { count, values })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_crate_nested_HashID_destroy(ffi: *mut Vec_crate_nested_HashID) {
        ferment_interfaces::unbox_any(ffi);
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(values: *mut u8, count: usize) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
        pub ok: *mut Option<crate::asyn::proof::FromProof>,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<Option<crate::asyn::proof::FromProof>, crate::nested::ProtocolError>,
        > for Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
        ) -> Result<Option<crate::asyn::proof::FromProof>, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<Option<crate::asyn::proof::FromProof>, crate::nested::ProtocolError>,
        ) -> *const Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError
        {
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
        unsafe fn destroy(
            ffi: *mut Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError_ctor(
        ok: *mut Option<crate::asyn::proof::FromProof>,
        error: *mut crate::fermented::types::nested::ProtocolError,
    ) -> *mut Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError {
                ok,
                error,
            },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError_destroy(
        ffi: *mut Result_ok_Option_crate_asyn_proof_FromProof_err_crate_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError {
        pub ok: *mut u8,
        pub error: *mut crate::fermented::types::asyn::provider::ContextProviderError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<[u8; 48], crate::asyn::provider::ContextProviderError>,
        > for Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError,
        ) -> Result<[u8; 48], crate::asyn::provider::ContextProviderError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| *o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<[u8; 48], crate::asyn::provider::ContextProviderError>,
        ) -> *const Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError {
            let (ok, error) = match obj {
                Ok(o) => (o as *mut _, std::ptr::null_mut()),
                Err(o) => (
                    std::ptr::null_mut(),
                    ferment_interfaces::FFIConversion::ffi_to(o),
                ),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(
            ffi: *mut Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError {
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError_ctor(
        ok: *mut u8,
        error: *mut crate::fermented::types::asyn::provider::ContextProviderError,
    ) -> *mut Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError {
        ferment_interfaces::boxed(
            Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError_destroy(
        ffi: *mut Result_ok_arr_u8_48_err_crate_asyn_provider_ContextProviderError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
        pub ok: *mut crate::fermented::types::asyn::query::TransportRequest,
        pub error: *mut Box<dyn std::error::Error>,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<crate::asyn::query::TransportRequest, Box<dyn std::error::Error>>,
        > for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error
    {
        unsafe fn ffi_from_const(
            ffi : * const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error,
        ) -> Result<crate::asyn::query::TransportRequest, Box<dyn std::error::Error>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<crate::asyn::query::TransportRequest, Box<dyn std::error::Error>>,
        ) -> *const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error
        {
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
        unsafe fn destroy(
            ffi: *mut Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error_ctor(
        ok: *mut crate::fermented::types::asyn::query::TransportRequest,
        error: *mut Box<dyn std::error::Error>,
    ) -> *mut Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
        ferment_interfaces::boxed(
            Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
                ok,
                error,
            },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error_destroy(
        ffi: *mut Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError
    {
        pub ok: *mut crate::fermented::types::asyn::dapi_request::DapiRequest::Response,
        pub error: *mut crate::asyn::dapi_request::DapiClientError<
            crate::asyn::dapi_request::DapiRequest::TransportError,
        >,
    }
    impl ferment_interfaces :: FFIConversion < Result < crate :: asyn :: dapi_request :: DapiRequest :: Response , crate :: asyn :: dapi_request :: DapiClientError < crate :: asyn :: dapi_request :: DapiRequest :: TransportError > > > for Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError { unsafe fn ffi_from_const (ffi : * const Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError) -> Result < crate :: asyn :: dapi_request :: DapiRequest :: Response , crate :: asyn :: dapi_request :: DapiClientError < crate :: asyn :: dapi_request :: DapiRequest :: TransportError > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < crate :: asyn :: dapi_request :: DapiRequest :: Response , crate :: asyn :: dapi_request :: DapiClientError < crate :: asyn :: dapi_request :: DapiRequest :: TransportError > >) -> * const Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } } } }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError_ctor (ok : * mut crate :: fermented :: types :: asyn :: dapi_request :: DapiRequest :: Response , error : * mut crate :: asyn :: dapi_request :: DapiClientError < crate :: asyn :: dapi_request :: DapiRequest :: TransportError >) -> * mut Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError{
        ferment_interfaces :: boxed (Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError { ok , error })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError_destroy(
        ffi : * mut Result_ok_crate_asyn_dapi_request_DapiRequest_Response_err_crate_asyn_dapi_request_DapiClientError_crate_asyn_dapi_request_DapiRequest_TransportError,
    ) {
        ferment_interfaces::unbox_any(ffi);
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
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError_ctor(
        ok: *mut crate::fermented::types::nested::HashID,
        error: *mut crate::fermented::types::nested::ProtocolError,
    ) -> *mut Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError_destroy(
        ffi: *mut Result_ok_crate_nested_HashID_err_crate_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient {
        pub ok : * mut crate :: fermented :: types :: asyn :: dapi_client :: R :: Response ,
        pub error : * mut crate :: asyn :: dapi_request :: DapiClientError < < crate :: asyn :: dapi_client :: R :: Client as TransportClient > :: crate :: asyn :: query :: TransportClient > , }
    impl ferment_interfaces :: FFIConversion < Result < crate :: asyn :: dapi_client :: R :: Response , crate :: asyn :: dapi_request :: DapiClientError < < crate :: asyn :: dapi_client :: R :: Client as TransportClient > :: crate :: asyn :: query :: TransportClient > > > for Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient { unsafe fn ffi_from_const (ffi : * const Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient) -> Result < crate :: asyn :: dapi_client :: R :: Response , crate :: asyn :: dapi_request :: DapiClientError < < crate :: asyn :: dapi_client :: R :: Client as TransportClient > :: crate :: asyn :: query :: TransportClient > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < crate :: asyn :: dapi_client :: R :: Response , crate :: asyn :: dapi_request :: DapiClientError < < crate :: asyn :: dapi_client :: R :: Client as TransportClient > :: crate :: asyn :: query :: TransportClient > >) -> * const Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } } } }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient_ctor (ok : * mut crate :: fermented :: types :: asyn :: dapi_client :: R :: Response , error : * mut crate :: asyn :: dapi_request :: DapiClientError < < crate :: asyn :: dapi_client :: R :: Client as TransportClient > :: crate :: asyn :: query :: TransportClient >) -> * mut Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient{
        ferment_interfaces :: boxed (Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient { ok , error })
    }
    #[doc = r" # Safety"]
    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient_destroy(
        ffi : * mut Result_ok_crate_asyn_dapi_client_R_Response_err_crate_asyn_dapi_request_DapiClientError_TransportClient_crate_asyn_query_TransportClient,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
