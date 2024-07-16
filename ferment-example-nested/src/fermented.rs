#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod types {
    pub mod ferment_example {
        use crate as ferment_example_nested;
        pub mod errors {
            use crate as ferment_example_nested;
            pub mod protocol_error {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: errors :: protocol_error :: ProtocolError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum ferment_example_errors_protocol_error_ProtocolError {
                    InvalidPKT (* mut crate :: fermented :: types :: ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::errors::protocol_error::ProtocolError,
                    > for ferment_example_errors_protocol_error_ProtocolError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_errors_protocol_error_ProtocolError,
                    ) -> ferment_example::errors::protocol_error::ProtocolError
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                o_0,
                            ) => {
                                ferment_example::errors::protocol_error::ProtocolError::InvalidPKT(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::errors::protocol_error::ProtocolError,
                    ) -> *const ferment_example_errors_protocol_error_ProtocolError
                    {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::errors::protocol_error::ProtocolError::InvalidPKT(
                                o_0,
                            ) => ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            ),
                            _ => unreachable!("Enum Variant unreachable"),
                        })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_errors_protocol_error_ProtocolError,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_errors_protocol_error_ProtocolError {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                    o_0,
                                ) => {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_protocol_error_ProtocolError_InvalidPKT_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                ) -> *mut ferment_example_errors_protocol_error_ProtocolError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_protocol_error_ProtocolError_destroy(
                    ffi: *mut ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod context {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: errors :: context :: ContextProviderError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum ferment_example_errors_context_ContextProviderError {
                    Generic(*mut std::os::raw::c_char),
                    Config(*mut std::os::raw::c_char),
                    InvalidDataContract(*mut std::os::raw::c_char),
                    InvalidQuorum(*mut std::os::raw::c_char),
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::errors::context::ContextProviderError,
                    > for ferment_example_errors_context_ContextProviderError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_errors_context_ContextProviderError,
                    ) -> ferment_example::errors::context::ContextProviderError
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_errors_context_ContextProviderError :: Generic (o_0) => ferment_example :: errors :: context :: ContextProviderError :: Generic (ferment_interfaces :: FFIConversion :: ffi_from (* o_0)) , ferment_example_errors_context_ContextProviderError :: Config (o_0) => ferment_example :: errors :: context :: ContextProviderError :: Config (ferment_interfaces :: FFIConversion :: ffi_from (* o_0)) , ferment_example_errors_context_ContextProviderError :: InvalidDataContract (o_0) => ferment_example :: errors :: context :: ContextProviderError :: InvalidDataContract (ferment_interfaces :: FFIConversion :: ffi_from (* o_0)) , ferment_example_errors_context_ContextProviderError :: InvalidQuorum (o_0) => ferment_example :: errors :: context :: ContextProviderError :: InvalidQuorum (ferment_interfaces :: FFIConversion :: ffi_from (* o_0)) }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::errors::context::ContextProviderError,
                    ) -> *const ferment_example_errors_context_ContextProviderError
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example :: errors :: context :: ContextProviderError :: Generic (o_0) => ferment_example_errors_context_ContextProviderError :: Generic (ferment_interfaces :: FFIConversion :: ffi_to (o_0)) , ferment_example :: errors :: context :: ContextProviderError :: Config (o_0) => ferment_example_errors_context_ContextProviderError :: Config (ferment_interfaces :: FFIConversion :: ffi_to (o_0)) , ferment_example :: errors :: context :: ContextProviderError :: InvalidDataContract (o_0) => ferment_example_errors_context_ContextProviderError :: InvalidDataContract (ferment_interfaces :: FFIConversion :: ffi_to (o_0)) , ferment_example :: errors :: context :: ContextProviderError :: InvalidQuorum (o_0) => ferment_example_errors_context_ContextProviderError :: InvalidQuorum (ferment_interfaces :: FFIConversion :: ffi_to (o_0)) , _ => unreachable ! ("Enum Variant unreachable") })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_errors_context_ContextProviderError,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_errors_context_ContextProviderError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_errors_context_ContextProviderError :: Generic (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_errors_context_ContextProviderError :: Config (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_errors_context_ContextProviderError :: InvalidDataContract (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_errors_context_ContextProviderError :: InvalidQuorum (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_context_ContextProviderError_Generic_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_errors_context_ContextProviderError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_context_ContextProviderError::Generic(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_context_ContextProviderError_Config_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_errors_context_ContextProviderError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_context_ContextProviderError::Config(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_context_ContextProviderError_InvalidDataContract_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_errors_context_ContextProviderError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_context_ContextProviderError::InvalidDataContract(
                            o_o_0,
                        ),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_context_ContextProviderError_InvalidQuorum_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_errors_context_ContextProviderError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_context_ContextProviderError::InvalidQuorum(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_context_ContextProviderError_destroy(
                    ffi: *mut ferment_example_errors_context_ContextProviderError,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
        }
        pub mod example {
            use crate as ferment_example_nested;
            pub mod custom_conversion {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example :: example :: custom_conversion :: StructUsesGenericWithCustom`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_example_custom_conversion_StructUsesGenericWithCustom { pub time : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::example::custom_conversion::StructUsesGenericWithCustom,
                    > for ferment_example_example_custom_conversion_StructUsesGenericWithCustom
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_example_custom_conversion_StructUsesGenericWithCustom,
                    ) -> ferment_example::example::custom_conversion::StructUsesGenericWithCustom
                    {
                        let ffi_ref = &*ffi;
                        ferment_example::example::custom_conversion::StructUsesGenericWithCustom {
                            time: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.time),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj : ferment_example :: example :: custom_conversion :: StructUsesGenericWithCustom,
                    ) -> *const ferment_example_example_custom_conversion_StructUsesGenericWithCustom
                    {
                        ferment_interfaces::boxed(
                            ferment_example_example_custom_conversion_StructUsesGenericWithCustom {
                                time: ferment_interfaces::FFIConversion::ffi_to(obj.time),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi : * mut ferment_example_example_custom_conversion_StructUsesGenericWithCustom,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_example_custom_conversion_StructUsesGenericWithCustom {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.time);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesGenericWithCustom_ctor(
                    time : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration,
                ) -> *mut ferment_example_example_custom_conversion_StructUsesGenericWithCustom
                {
                    ferment_interfaces::boxed(
                        ferment_example_example_custom_conversion_StructUsesGenericWithCustom {
                            time,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesGenericWithCustom_destroy(
                    ffi: *mut ferment_example_example_custom_conversion_StructUsesGenericWithCustom,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesGenericWithCustom_get_time < > (obj : * const ferment_example_example_custom_conversion_StructUsesGenericWithCustom) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration{
                    (*obj).time
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesGenericWithCustom_set_time(
                    obj: *mut ferment_example_example_custom_conversion_StructUsesGenericWithCustom,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration,
                ) {
                    (*obj).time = value;
                }
                #[doc = "FFI-representation of the [`ferment_example :: example :: custom_conversion :: StructUsesDurationTuple`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_example_custom_conversion_StructUsesDurationTuple {
                    pub time:
                        *mut crate::fermented::generics::Tuple_std_time_Duration_std_time_Duration,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::example::custom_conversion::StructUsesDurationTuple,
                    > for ferment_example_example_custom_conversion_StructUsesDurationTuple
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_example_custom_conversion_StructUsesDurationTuple,
                    ) -> ferment_example::example::custom_conversion::StructUsesDurationTuple
                    {
                        let ffi_ref = &*ffi;
                        ferment_example::example::custom_conversion::StructUsesDurationTuple {
                            time: {
                                let ffi_ref = &*ffi_ref.time;
                                (
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
                                )
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::example::custom_conversion::StructUsesDurationTuple,
                    ) -> *const ferment_example_example_custom_conversion_StructUsesDurationTuple
                    {
                        ferment_interfaces::boxed(
                            ferment_example_example_custom_conversion_StructUsesDurationTuple {
                                time: ferment_interfaces::FFIConversion::ffi_to(obj.time),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_example_custom_conversion_StructUsesDurationTuple,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_example_custom_conversion_StructUsesDurationTuple {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.time);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDurationTuple_ctor(
                    time : * mut crate :: fermented :: generics :: Tuple_std_time_Duration_std_time_Duration,
                ) -> *mut ferment_example_example_custom_conversion_StructUsesDurationTuple
                {
                    ferment_interfaces::boxed(
                        ferment_example_example_custom_conversion_StructUsesDurationTuple { time },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDurationTuple_destroy(
                    ffi: *mut ferment_example_example_custom_conversion_StructUsesDurationTuple,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDurationTuple_get_time(
                    obj: *const ferment_example_example_custom_conversion_StructUsesDurationTuple,
                ) -> *mut crate::fermented::generics::Tuple_std_time_Duration_std_time_Duration
                {
                    (*obj).time
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDurationTuple_set_time(
                    obj: *mut ferment_example_example_custom_conversion_StructUsesDurationTuple,
                    value : * mut crate :: fermented :: generics :: Tuple_std_time_Duration_std_time_Duration,
                ) {
                    (*obj).time = value;
                }
            }
            pub mod address {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example :: example :: address :: address_simple_result`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_address_address_simple_result(
                    script: *mut crate::fermented::generics::Vec_u32,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    let obj = ferment_example::example::address::address_simple_result(
                        ferment_interfaces::FFIConversion::ffi_from(script),
                    );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example :: example :: address :: address_with_script_pubkey`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_address_address_with_script_pubkey(
                    script: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut std::os::raw::c_char {
                    let obj = ferment_example::example::address::address_with_script_pubkey(
                        ferment_interfaces::FFIConversion::ffi_from(script),
                    );
                    ferment_interfaces::FFIConversion::ffi_to_opt(obj)
                }
            }
        }
        #[doc = "FFI-representation of the [`ferment_example :: get_root_struct`]"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_get_root_struct(
        ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct {
            let obj = ferment_example::get_root_struct();
            ferment_interfaces::FFIConversion::ffi_to(obj)
        }
        pub mod data_contract {
            use crate as ferment_example_nested;
            pub mod v0 {
                use crate as ferment_example_nested;
                pub mod data_contract {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example :: data_contract :: v0 :: data_contract :: DataContractV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_data_contract_v0_data_contract_DataContractV0 {}
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example::data_contract::v0::data_contract::DataContractV0,
                        > for ferment_example_data_contract_v0_data_contract_DataContractV0
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const ferment_example_data_contract_v0_data_contract_DataContractV0,
                        ) -> ferment_example::data_contract::v0::data_contract::DataContractV0
                        {
                            let ffi_ref = &*ffi;
                            ferment_example::data_contract::v0::data_contract::DataContractV0 {}
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example::data_contract::v0::data_contract::DataContractV0,
                        ) -> *const ferment_example_data_contract_v0_data_contract_DataContractV0
                        {
                            ferment_interfaces::boxed(
                                ferment_example_data_contract_v0_data_contract_DataContractV0 {},
                            )
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_data_contract_v0_data_contract_DataContractV0,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_data_contract_v0_data_contract_DataContractV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v0_data_contract_DataContractV0_ctor(
                    ) -> *mut ferment_example_data_contract_v0_data_contract_DataContractV0
                    {
                        ferment_interfaces::boxed(
                            ferment_example_data_contract_v0_data_contract_DataContractV0 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v0_data_contract_DataContractV0_destroy(
                        ffi: *mut ferment_example_data_contract_v0_data_contract_DataContractV0,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: data_contract :: DataContract`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum ferment_example_data_contract_DataContract {
                V0 (* mut crate :: fermented :: types :: ferment_example :: data_contract :: v0 :: data_contract :: ferment_example_data_contract_v0_data_contract_DataContractV0) , V1 (* mut crate :: fermented :: types :: ferment_example :: data_contract :: v1 :: data_contract :: ferment_example_data_contract_v1_data_contract_DataContractV1) , # [cfg (test)] Test }
            impl ferment_interfaces::FFIConversion<ferment_example::data_contract::DataContract>
                for ferment_example_data_contract_DataContract
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_data_contract_DataContract,
                ) -> ferment_example::data_contract::DataContract {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_data_contract_DataContract::V0(o_0) => {
                            ferment_example::data_contract::DataContract::V0(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_data_contract_DataContract::V1(o_0) => {
                            ferment_example::data_contract::DataContract::V1(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        #[cfg(test)]
                        ferment_example_data_contract_DataContract::Test => {
                            ferment_example::data_contract::DataContract::Test
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::data_contract::DataContract,
                ) -> *const ferment_example_data_contract_DataContract {
                    ferment_interfaces::boxed(match obj {
                        ferment_example::data_contract::DataContract::V0(o_0) => {
                            ferment_example_data_contract_DataContract::V0(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::data_contract::DataContract::V1(o_0) => {
                            ferment_example_data_contract_DataContract::V1(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        #[cfg(test)]
                        ferment_example::data_contract::DataContract::Test => {
                            ferment_example_data_contract_DataContract::Test
                        }
                        _ => unreachable!("Enum Variant unreachable"),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_data_contract_DataContract) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_data_contract_DataContract {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_data_contract_DataContract::V0(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                            ferment_example_data_contract_DataContract::V1(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                            #[cfg(test)]
                            ferment_example_data_contract_DataContract::Test => {}
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_V0_ctor(
                o_o_0 : * mut crate :: fermented :: types :: ferment_example :: data_contract :: v0 :: data_contract :: ferment_example_data_contract_v0_data_contract_DataContractV0,
            ) -> *mut ferment_example_data_contract_DataContract {
                ferment_interfaces::boxed(ferment_example_data_contract_DataContract::V0(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_V1_ctor(
                o_o_0 : * mut crate :: fermented :: types :: ferment_example :: data_contract :: v1 :: data_contract :: ferment_example_data_contract_v1_data_contract_DataContractV1,
            ) -> *mut ferment_example_data_contract_DataContract {
                ferment_interfaces::boxed(ferment_example_data_contract_DataContract::V1(o_o_0))
            }
            #[cfg(test)]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_Test_ctor(
            ) -> *mut ferment_example_data_contract_DataContract {
                ferment_interfaces::boxed(ferment_example_data_contract_DataContract::Test {})
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_destroy(
                ffi: *mut ferment_example_data_contract_DataContract,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod document_type {
                use crate as ferment_example_nested;
                pub mod v0 {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example :: data_contract :: document_type :: v0 :: DocumentTypeV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_data_contract_document_type_v0_DocumentTypeV0 {
                        pub name: *mut std::os::raw::c_char,
                        pub binary_paths:
                            *mut crate::fermented::generics::std_collections_BTreeSet_String,
                    }
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example::data_contract::document_type::v0::DocumentTypeV0,
                        > for ferment_example_data_contract_document_type_v0_DocumentTypeV0
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                        ) -> ferment_example::data_contract::document_type::v0::DocumentTypeV0
                        {
                            let ffi_ref = &*ffi;
                            ferment_example::data_contract::document_type::v0::DocumentTypeV0 {
                                name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                                binary_paths: ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.binary_paths,
                                ),
                            }
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example::data_contract::document_type::v0::DocumentTypeV0,
                        ) -> *const ferment_example_data_contract_document_type_v0_DocumentTypeV0
                        {
                            ferment_interfaces::boxed(
                                ferment_example_data_contract_document_type_v0_DocumentTypeV0 {
                                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                                    binary_paths: ferment_interfaces::FFIConversion::ffi_to(
                                        obj.binary_paths,
                                    ),
                                },
                            )
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_data_contract_document_type_v0_DocumentTypeV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(ffi_ref.name);
                                ferment_interfaces::unbox_any(ffi_ref.binary_paths);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_ctor(
                        name: *mut std::os::raw::c_char,
                        binary_paths : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String,
                    ) -> *mut ferment_example_data_contract_document_type_v0_DocumentTypeV0
                    {
                        ferment_interfaces::boxed(
                            ferment_example_data_contract_document_type_v0_DocumentTypeV0 {
                                name,
                                binary_paths,
                            },
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_destroy(
                        ffi: *mut ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_get_name(
                        obj: *const ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).name
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_get_binary_paths(
                        obj: *const ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                    {
                        (*obj).binary_paths
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_set_name(
                        obj: *mut ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                        value: *mut std::os::raw::c_char,
                    ) {
                        (*obj).name = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_document_type_v0_DocumentTypeV0_set_binary_paths(
                        obj: *mut ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                        value: *mut crate::fermented::generics::std_collections_BTreeSet_String,
                    ) {
                        (*obj).binary_paths = value;
                    }
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: data_contract :: document_type :: DocumentType`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum ferment_example_data_contract_document_type_DocumentType {
                    V0 (* mut crate :: fermented :: types :: ferment_example :: data_contract :: document_type :: v0 :: ferment_example_data_contract_document_type_v0_DocumentTypeV0) }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::data_contract::document_type::DocumentType,
                    > for ferment_example_data_contract_document_type_DocumentType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_data_contract_document_type_DocumentType,
                    ) -> ferment_example::data_contract::document_type::DocumentType
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_data_contract_document_type_DocumentType::V0(o_0) => {
                                ferment_example::data_contract::document_type::DocumentType::V0(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::data_contract::document_type::DocumentType,
                    ) -> *const ferment_example_data_contract_document_type_DocumentType
                    {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::data_contract::document_type::DocumentType::V0(
                                o_0,
                            ) => ferment_example_data_contract_document_type_DocumentType::V0(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            ),
                            _ => unreachable!("Enum Variant unreachable"),
                        })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_data_contract_document_type_DocumentType,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_data_contract_document_type_DocumentType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_data_contract_document_type_DocumentType::V0(
                                    o_0,
                                ) => {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_data_contract_document_type_DocumentType_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: ferment_example :: data_contract :: document_type :: v0 :: ferment_example_data_contract_document_type_v0_DocumentTypeV0,
                ) -> *mut ferment_example_data_contract_document_type_DocumentType {
                    ferment_interfaces::boxed(
                        ferment_example_data_contract_document_type_DocumentType::V0(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_data_contract_document_type_DocumentType_destroy(
                    ffi: *mut ferment_example_data_contract_document_type_DocumentType,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod v1 {
                use crate as ferment_example_nested;
                pub mod data_contract {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example :: data_contract :: v1 :: data_contract :: DataContractV1`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_data_contract_v1_data_contract_DataContractV1 {}
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example::data_contract::v1::data_contract::DataContractV1,
                        > for ferment_example_data_contract_v1_data_contract_DataContractV1
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const ferment_example_data_contract_v1_data_contract_DataContractV1,
                        ) -> ferment_example::data_contract::v1::data_contract::DataContractV1
                        {
                            let ffi_ref = &*ffi;
                            ferment_example::data_contract::v1::data_contract::DataContractV1 {}
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example::data_contract::v1::data_contract::DataContractV1,
                        ) -> *const ferment_example_data_contract_v1_data_contract_DataContractV1
                        {
                            ferment_interfaces::boxed(
                                ferment_example_data_contract_v1_data_contract_DataContractV1 {},
                            )
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_data_contract_v1_data_contract_DataContractV1,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_data_contract_v1_data_contract_DataContractV1 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v1_data_contract_DataContractV1_ctor(
                    ) -> *mut ferment_example_data_contract_v1_data_contract_DataContractV1
                    {
                        ferment_interfaces::boxed(
                            ferment_example_data_contract_v1_data_contract_DataContractV1 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v1_data_contract_DataContractV1_destroy(
                        ffi: *mut ferment_example_data_contract_v1_data_contract_DataContractV1,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
        }
        pub mod nested {
            use crate as ferment_example_nested;
            #[doc = "FFI-representation of the [`ferment_example :: nested :: get_root_struct_2`]"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_get_root_struct_2(
            ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
            {
                let obj = ferment_example::nested::get_root_struct_2();
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the [`ferment_example :: nested :: HashID`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_HashID(*mut crate::fermented::generics::Arr_u8_32);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::HashID>
                for ferment_example_nested_HashID
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_HashID,
                ) -> ferment_example::nested::HashID {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::HashID,
                ) -> *const ferment_example_nested_HashID {
                    ferment_interfaces::boxed(ferment_example_nested_HashID(
                        ferment_interfaces::FFIConversion::ffi_to(obj),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_HashID) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_HashID {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_ctor(
                o_0: *mut crate::fermented::generics::Arr_u8_32,
            ) -> *mut ferment_example_nested_HashID {
                ferment_interfaces::boxed(ferment_example_nested_HashID(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_destroy(
                ffi: *mut ferment_example_nested_HashID,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_get_0(
                obj: *const ferment_example_nested_HashID,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_set_0(
                obj: *mut ferment_example_nested_HashID,
                value: *mut crate::fermented::generics::Arr_u8_32,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`ferment_example :: nested :: RootUser`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_RootUser {
                pub root: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            }
            impl ferment_interfaces::FFIConversion<ferment_example::nested::RootUser>
                for ferment_example_nested_RootUser
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_RootUser,
                ) -> ferment_example::nested::RootUser {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::RootUser {
                        root: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.root),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::RootUser,
                ) -> *const ferment_example_nested_RootUser {
                    ferment_interfaces::boxed(ferment_example_nested_RootUser {
                        root: ferment_interfaces::FFIConversion::ffi_to(obj.root),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_RootUser) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_RootUser {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.root);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_ctor(
                root: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            ) -> *mut ferment_example_nested_RootUser {
                ferment_interfaces::boxed(ferment_example_nested_RootUser { root })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_destroy(
                ffi: *mut ferment_example_nested_RootUser,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_get_root(
                obj: *const ferment_example_nested_RootUser,
            ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
            {
                (*obj).root
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_set_root(
                obj: *mut ferment_example_nested_RootUser,
                value: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            ) {
                (*obj).root = value;
            }
            pub mod double_nested {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example :: nested :: double_nested :: get_root_struct_3`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_double_nested_get_root_struct_3(
                ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
                {
                    let obj = ferment_example::nested::double_nested::get_root_struct_3();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
        }
        pub mod document {
            use crate as ferment_example_nested;
            pub mod errors {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: document :: errors :: DocumentError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum ferment_example_document_errors_DocumentError {
                    InvalidActionError (u8) , InvalidInitialRevisionError { document : * mut crate :: fermented :: types :: ferment_example :: document :: ferment_example_document_Document } }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::document::errors::DocumentError,
                    > for ferment_example_document_errors_DocumentError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_document_errors_DocumentError,
                    ) -> ferment_example::document::errors::DocumentError {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_document_errors_DocumentError :: InvalidActionError (o_0) => ferment_example :: document :: errors :: DocumentError :: InvalidActionError (* o_0) , ferment_example_document_errors_DocumentError :: InvalidInitialRevisionError { document } => ferment_example :: document :: errors :: DocumentError :: InvalidInitialRevisionError { document : Box :: new (ferment_interfaces :: FFIConversion :: ffi_from (* document)) } }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::document::errors::DocumentError,
                    ) -> *const ferment_example_document_errors_DocumentError {
                        ferment_interfaces :: boxed (match obj { ferment_example :: document :: errors :: DocumentError :: InvalidActionError (o_0) => ferment_example_document_errors_DocumentError :: InvalidActionError (o_0) , ferment_example :: document :: errors :: DocumentError :: InvalidInitialRevisionError { document } => ferment_example_document_errors_DocumentError :: InvalidInitialRevisionError { document : ferment_interfaces :: FFIConversion :: ffi_to (document) } , _ => unreachable ! ("Enum Variant unreachable") })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_document_errors_DocumentError) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_document_errors_DocumentError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_document_errors_DocumentError :: InvalidActionError (o_0) => { } , ferment_example_document_errors_DocumentError :: InvalidInitialRevisionError { document } => { ferment_interfaces :: unbox_any (* document) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_document_errors_DocumentError_InvalidActionError_ctor(
                    o_o_0: u8,
                ) -> *mut ferment_example_document_errors_DocumentError {
                    ferment_interfaces::boxed(
                        ferment_example_document_errors_DocumentError::InvalidActionError(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_document_errors_DocumentError_InvalidInitialRevisionError_ctor(
                    document : * mut crate :: fermented :: types :: ferment_example :: document :: ferment_example_document_Document,
                ) -> *mut ferment_example_document_errors_DocumentError {
                    ferment_interfaces :: boxed (ferment_example_document_errors_DocumentError :: InvalidInitialRevisionError { document })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_document_errors_DocumentError_destroy(
                    ffi: *mut ferment_example_document_errors_DocumentError,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example :: document :: Document`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum ferment_example_document_Document {
                V0,
            }
            impl ferment_interfaces::FFIConversion<ferment_example::document::Document>
                for ferment_example_document_Document
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_document_Document,
                ) -> ferment_example::document::Document {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_document_Document::V0 => {
                            ferment_example::document::Document::V0
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::document::Document,
                ) -> *const ferment_example_document_Document {
                    ferment_interfaces::boxed(match obj {
                        ferment_example::document::Document::V0 => {
                            ferment_example_document_Document::V0
                        }
                        _ => unreachable!("Enum Variant unreachable"),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_document_Document) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_document_Document {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_document_Document::V0 => {}
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_document_Document_V0_ctor(
            ) -> *mut ferment_example_document_Document {
                ferment_interfaces::boxed(ferment_example_document_Document::V0 {})
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_document_Document_destroy(
                ffi: *mut ferment_example_document_Document,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        #[doc = "FFI-representation of the [`ferment_example :: RootStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_RootStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment_interfaces::FFIConversion<ferment_example::RootStruct> for ferment_example_RootStruct {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_RootStruct,
            ) -> ferment_example::RootStruct {
                let ffi_ref = &*ffi;
                ferment_example::RootStruct {
                    name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example::RootStruct,
            ) -> *const ferment_example_RootStruct {
                ferment_interfaces::boxed(ferment_example_RootStruct {
                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_RootStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_RootStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                        ffi_ref.name,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_RootStruct {
            ferment_interfaces::boxed(ferment_example_RootStruct { name })
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_destroy(
            ffi: *mut ferment_example_RootStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_get_name(
            obj: *const ferment_example_RootStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_set_name(
            obj: *mut ferment_example_RootStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
        }
        pub mod state_transition {
            use crate as ferment_example_nested;
            pub mod errors {
                use crate as ferment_example_nested;
                pub mod invalid_identity_public_key_type_error {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError
                    {
                        pub public_key_type: *mut std::os::raw::c_char,
                    }
                    impl ferment_interfaces :: FFIConversion < ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError > for ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { unsafe fn ffi_from_const (ffi : * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) -> ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError { let ffi_ref = & * ffi ; ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError { public_key_type : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . public_key_type) } } unsafe fn ffi_to_const (obj : ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError) -> * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { ferment_interfaces :: boxed (ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type : ferment_interfaces :: FFIConversion :: ffi_to (obj . public_key_type) }) } unsafe fn destroy (ffi : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) { ferment_interfaces :: unbox_any (ffi) ; } }
                    impl Drop for ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { fn drop (& mut self) { unsafe { let ffi_ref = self ; < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . public_key_type) ; } } }
                    #[no_mangle]                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_ctor < > (public_key_type : * mut std :: os :: raw :: c_char) -> * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError{
                        ferment_interfaces :: boxed (ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_destroy(
                        ffi : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_get_public_key_type(
                        obj : * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).public_key_type
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_set_public_key_type(
                        obj : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                        value: *mut std::os::raw::c_char,
                    ) {
                        (*obj).public_key_type = value;
                    }
                }
            }
        }
    }
    pub mod ferment_example_nested {
        use crate as ferment_example_nested;
        #[doc = "FFI-representation of the [`ferment_example_nested :: SomeStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_nested_SomeStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment_interfaces::FFIConversion<ferment_example_nested::SomeStruct>
            for ferment_example_nested_SomeStruct
        {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_nested_SomeStruct,
            ) -> ferment_example_nested::SomeStruct {
                let ffi_ref = &*ffi;
                ferment_example_nested::SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example_nested::SomeStruct,
            ) -> *const ferment_example_nested_SomeStruct {
                ferment_interfaces::boxed(ferment_example_nested_SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_nested_SomeStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_nested_SomeStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                        ffi_ref.name,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_nested_SomeStruct {
            ferment_interfaces::boxed(ferment_example_nested_SomeStruct { name })
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_destroy(
            ffi: *mut ferment_example_nested_SomeStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_get_name(
            obj: *const ferment_example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_set_name(
            obj: *mut ferment_example_nested_SomeStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
        }
        pub mod entry {
            use crate as ferment_example_nested;
            pub mod core {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: core :: DashSharedCore`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_core_DashSharedCore_ctor(
                    processor: *mut ferment_example_nested::entry::processor::Processor,
                    cache : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_String,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_nested::entry::core::DashSharedCore {
                    ferment_interfaces::boxed(ferment_example_nested::entry::core::DashSharedCore {
                        processor: processor,
                        cache: ferment_interfaces::FFIConversion::ffi_from(cache),
                        context: context,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_core_DashSharedCore_destroy(
                    ffi: *mut ferment_example_nested::entry::core::DashSharedCore,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: core :: DashSharedCore :: with_pointers`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_core_DashSharedCore_with_pointers(
                    block_hash_by_height: ferment_example_nested::entry::BlockHashByHeight,
                    model_by_height: ferment_example_nested::entry::ModelByHeight,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_nested::entry::core::DashSharedCore {
                    let obj = ferment_example_nested::entry::core::DashSharedCore::with_pointers(
                        block_hash_by_height,
                        model_by_height,
                        context,
                    );
                    ferment_interfaces::boxed(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: core :: DashSharedCore :: with_lambdas`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_core_DashSharedCore_with_lambdas(
                    block_hash_by_height : * mut crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Arr_u8_32,
                    model_by_height : * mut crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_ferment_example_nested_entry_SomeModel,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_nested::entry::core::DashSharedCore {
                    let obj = ferment_example_nested::entry::core::DashSharedCore::with_lambdas(
                        move |o_0| unsafe { (&*block_hash_by_height).call(o_0) },
                        move |o_0| unsafe { (&*model_by_height).call(o_0) },
                        context,
                    );
                    ferment_interfaces::boxed(obj)
                }
            }
            #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: SomeModel`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_entry_SomeModel {
                pub hash: *mut crate::fermented::generics::Arr_u8_32,
                pub desc: *mut std::os::raw::c_char,
            }
            impl ferment_interfaces::FFIConversion<ferment_example_nested::entry::SomeModel>
                for ferment_example_nested_entry_SomeModel
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_entry_SomeModel,
                ) -> ferment_example_nested::entry::SomeModel {
                    let ffi_ref = &*ffi;
                    ferment_example_nested::entry::SomeModel {
                        hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.hash),
                        desc: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.desc),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::entry::SomeModel,
                ) -> *const ferment_example_nested_entry_SomeModel {
                    ferment_interfaces::boxed(ferment_example_nested_entry_SomeModel {
                        hash: ferment_interfaces::FFIConversion::ffi_to(obj.hash),
                        desc: ferment_interfaces::FFIConversion::ffi_to(obj.desc),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_entry_SomeModel) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_entry_SomeModel {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.hash);
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . desc) ;
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_ctor(
                hash: *mut crate::fermented::generics::Arr_u8_32,
                desc: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_nested_entry_SomeModel {
                ferment_interfaces::boxed(ferment_example_nested_entry_SomeModel { hash, desc })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_destroy(
                ffi: *mut ferment_example_nested_entry_SomeModel,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_get_hash(
                obj: *const ferment_example_nested_entry_SomeModel,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).hash
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_get_desc(
                obj: *const ferment_example_nested_entry_SomeModel,
            ) -> *mut std::os::raw::c_char {
                (*obj).desc
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_set_hash(
                obj: *mut ferment_example_nested_entry_SomeModel,
                value: *mut crate::fermented::generics::Arr_u8_32,
            ) {
                (*obj).hash = value;
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_entry_SomeModel_set_desc(
                obj: *mut ferment_example_nested_entry_SomeModel,
                value: *mut std::os::raw::c_char,
            ) {
                (*obj).desc = value;
            }
            pub mod processor {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: processor :: Processor`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_processor_Processor_ctor(
                    chain_id: *mut dyn ferment_example_nested::entry::provider::CoreProvider,
                ) -> *mut ferment_example_nested::entry::processor::Processor {
                    ferment_interfaces::boxed(ferment_example_nested::entry::processor::Processor {
                        chain_id: Box::from_raw(chain_id),
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_processor_Processor_destroy(
                    ffi: *mut ferment_example_nested::entry::processor::Processor,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod provider {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: entry :: provider :: FFIPtrCoreProvider`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_provider_FFIPtrCoreProvider_ctor(
                    block_hash_by_height: ferment_example_nested::entry::BlockHashByHeight,
                    model_by_height: ferment_example_nested::entry::ModelByHeight,
                ) -> *mut ferment_example_nested::entry::provider::FFIPtrCoreProvider
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested::entry::provider::FFIPtrCoreProvider {
                            block_hash_by_height: block_hash_by_height,
                            model_by_height: model_by_height,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_entry_provider_FFIPtrCoreProvider_destroy(
                    ffi: *mut ferment_example_nested::entry::provider::FFIPtrCoreProvider,
                ) {
                    ferment_interfaces::unbox_any(ffi);
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
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod generics {
    use crate as ferment_example_nested;
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_32 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<[u8; 32]> for Arr_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_32) -> [u8; 32] {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
                .try_into()
                .unwrap()
        }
        unsafe fn ffi_to_const(obj: [u8; 32]) -> *const Arr_u8_32 {
            ferment_interfaces::FFIVecConversion::encode(obj.to_vec())
        }
        unsafe fn destroy(ffi: *mut Arr_u8_32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Arr_u8_32 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Arr_u8_32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_32 {
        ferment_interfaces::boxed(Arr_u8_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_destroy(ffi: *mut Arr_u8_32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_std_time_Duration_std_time_Duration {
        pub o_0: *mut ferment_example::std_time_Duration,
        pub o_1: *mut ferment_example::std_time_Duration,
    }
    impl ferment_interfaces::FFIConversion<(std::time::Duration, std::time::Duration)>
        for Tuple_std_time_Duration_std_time_Duration
    {
        unsafe fn ffi_from_const(
            ffi: *const Tuple_std_time_Duration_std_time_Duration,
        ) -> (std::time::Duration, std::time::Duration) {
            let ffi_ref = &*ffi;
            (
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
            )
        }
        unsafe fn ffi_to_const(
            obj: (std::time::Duration, std::time::Duration),
        ) -> *const Tuple_std_time_Duration_std_time_Duration {
            ferment_interfaces::boxed(Self {
                o_0: ferment_interfaces::FFIConversion::ffi_to(obj.0),
                o_1: ferment_interfaces::FFIConversion::ffi_to(obj.1),
            })
        }
        unsafe fn destroy(ffi: *mut Tuple_std_time_Duration_std_time_Duration) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_std_time_Duration_std_time_Duration {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.o_0);
                ferment_interfaces::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_std_time_Duration_std_time_Duration_ctor(
        o_0: *mut ferment_example::std_time_Duration,
        o_1: *mut ferment_example::std_time_Duration,
    ) -> *mut Tuple_std_time_Duration_std_time_Duration {
        ferment_interfaces::boxed(Tuple_std_time_Duration_std_time_Duration { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_std_time_Duration_std_time_Duration_destroy(
        ffi: *mut Tuple_std_time_Duration_std_time_Duration,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
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
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (o as *mut _, std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), o as *mut _),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_u32_err_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::destroy_opt_primitive(self.ok);
                ferment_interfaces::destroy_opt_primitive(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_ctor(
        ok: *mut u32,
        error: *mut u32,
    ) -> *mut Result_ok_u32_err_u32 {
        ferment_interfaces::boxed(Result_ok_u32_err_u32 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_destroy(ffi: *mut Result_ok_u32_err_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<String>>
        for std_collections_BTreeSet_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_String,
        ) -> std::collections::BTreeSet<String> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<String>,
        ) -> *const std_collections_BTreeSet_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_String {
        type Value = std::collections::BTreeSet<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_BTreeSet_String {
        ferment_interfaces::boxed(std_collections_BTreeSet_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_destroy(
        ffi: *mut std_collections_BTreeSet_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_String {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<String, String>>
        for std_collections_Map_keys_String_values_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_String,
        ) -> std::collections::BTreeMap<String, String> {
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
            obj: std::collections::BTreeMap<String, String>,
        ) -> *const std_collections_Map_keys_String_values_String {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_String_values_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_ctor(
        count: usize,
        keys: *mut *mut std::os::raw::c_char,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_String_values_String {
        ferment_interfaces::boxed(std_collections_Map_keys_String_values_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_destroy(
        ffi: *mut std_collections_Map_keys_String_values_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
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
            ferment_interfaces::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_primitive_group(obj.into_iter()),
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
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_ctor(count: usize, values: *mut u32) -> *mut Vec_u32 {
        ferment_interfaces::boxed(Vec_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_destroy(ffi: *mut Vec_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
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
            ferment_interfaces::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_primitive_group(obj.into_iter()),
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
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_ferment_example_nested_entry_SomeModel { pub context : * const std :: os :: raw :: c_void , caller : fn (u32) -> * mut crate :: fermented :: types :: ferment_example_nested :: entry :: ferment_example_nested_entry_SomeModel , destructor : fn (result : * mut crate :: fermented :: types :: ferment_example_nested :: entry :: ferment_example_nested_entry_SomeModel) , }
    impl Fn_ARGS_u32_RTRN_ferment_example_nested_entry_SomeModel {
        pub unsafe fn call(&self, o_0: u32) -> ferment_example_nested::entry::SomeModel {
            let ffi_result = (self.caller)(o_0);
            let result = < crate :: fermented :: types :: ferment_example_nested :: entry :: ferment_example_nested_entry_SomeModel as ferment_interfaces :: FFIConversion < ferment_example_nested :: entry :: SomeModel >> :: ffi_from (ffi_result) ;
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Arr_u8_32 {
        pub context: *const std::os::raw::c_void,
        caller: fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: fn(result: *mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_u32_RTRN_Arr_u8_32 {
        pub unsafe fn call(&self, o_0: u32) -> [u8; 32] {
            let ffi_result = (self.caller)(o_0);
            let result =
                <crate::fermented::generics::Arr_u8_32 as ferment_interfaces::FFIConversion<
                    [u8; 32],
                >>::ffi_from(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_std_time_Duration {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut ferment_example::std_time_Duration,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<String, std::time::Duration>>
        for std_collections_Map_keys_String_values_std_time_Duration
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_std_time_Duration,
        ) -> std::collections::BTreeMap<String, std::time::Duration> {
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
            obj: std::collections::BTreeMap<String, std::time::Duration>,
        ) -> *const std_collections_Map_keys_String_values_std_time_Duration {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_String_values_std_time_Duration) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_std_time_Duration {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_std_time_Duration_ctor(
        count: usize,
        keys: *mut *mut std::os::raw::c_char,
        values: *mut *mut ferment_example::std_time_Duration,
    ) -> *mut std_collections_Map_keys_String_values_std_time_Duration {
        ferment_interfaces::boxed(std_collections_Map_keys_String_values_std_time_Duration {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_std_time_Duration_destroy(
        ffi: *mut std_collections_Map_keys_String_values_std_time_Duration,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
