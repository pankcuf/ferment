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
pub mod types {
    pub mod platform_value {
        pub mod types {
            pub mod identifier {
                #[doc = "FFI-representation of the [`platform_value::types::identifier::IdentifierBytes32`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_identifier_IdentifierBytes32(*mut [u8; 32]);
                impl
                    ferment_interfaces::FFIConversion<
                        platform_value::types::identifier::IdentifierBytes32,
                    > for platform_value_types_identifier_IdentifierBytes32
                {
                    unsafe fn ffi_from_const(
                        ffi: *const platform_value_types_identifier_IdentifierBytes32,
                    ) -> platform_value::types::identifier::IdentifierBytes32 {
                        let ffi_ref = &*ffi;
                        platform_value::types::identifier::IdentifierBytes32(*ffi_ref.0)
                    }
                    unsafe fn ffi_to_const(
                        obj: platform_value::types::identifier::IdentifierBytes32,
                    ) -> *const platform_value_types_identifier_IdentifierBytes32
                    {
                        ferment_interfaces::boxed(
                            platform_value_types_identifier_IdentifierBytes32(
                                ferment_interfaces::boxed(obj.0),
                            ),
                        )
                    }
                    unsafe fn destroy(ffi: *mut platform_value_types_identifier_IdentifierBytes32) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for platform_value_types_identifier_IdentifierBytes32 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_ctor(
                    o_0: *mut [u8; 32],
                ) -> *mut platform_value_types_identifier_IdentifierBytes32 {
                    ferment_interfaces::boxed(platform_value_types_identifier_IdentifierBytes32(
                        o_0,
                    ))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_destroy(
                    ffi: *mut platform_value_types_identifier_IdentifierBytes32,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_get_0(
                    obj: *const platform_value_types_identifier_IdentifierBytes32,
                ) -> *mut [u8; 32] {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_IdentifierBytes32_set_0(
                    obj: *mut platform_value_types_identifier_IdentifierBytes32,
                    value: *mut [u8; 32],
                ) {
                    (*obj).0 = value;
                }
                #[doc = "FFI-representation of the [`platform_value::types::identifier::Identifier`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_identifier_Identifier (* mut crate :: fermented :: types :: platform_value :: types :: identifier :: platform_value_types_identifier_IdentifierBytes32) ;
                impl
                    ferment_interfaces::FFIConversion<platform_value::types::identifier::Identifier>
                    for platform_value_types_identifier_Identifier
                {
                    unsafe fn ffi_from_const(
                        ffi: *const platform_value_types_identifier_Identifier,
                    ) -> platform_value::types::identifier::Identifier {
                        let ffi_ref = &*ffi;
                        platform_value::types::identifier::Identifier(
                            ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0),
                        )
                    }
                    unsafe fn ffi_to_const(
                        obj: platform_value::types::identifier::Identifier,
                    ) -> *const platform_value_types_identifier_Identifier {
                        ferment_interfaces::boxed(platform_value_types_identifier_Identifier(
                            ferment_interfaces::FFIConversion::ffi_to(obj.0),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut platform_value_types_identifier_Identifier) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for platform_value_types_identifier_Identifier {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_ctor(
                    o_0 : * mut crate :: fermented :: types :: platform_value :: types :: identifier :: platform_value_types_identifier_IdentifierBytes32,
                ) -> *mut platform_value_types_identifier_Identifier {
                    ferment_interfaces::boxed(platform_value_types_identifier_Identifier(o_0))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_destroy(
                    ffi: *mut platform_value_types_identifier_Identifier,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_get_0 (obj : * const platform_value_types_identifier_Identifier) -> * mut crate :: fermented :: types :: platform_value :: types :: identifier :: platform_value_types_identifier_IdentifierBytes32{
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_identifier_Identifier_set_0(
                    obj: *mut platform_value_types_identifier_Identifier,
                    value : * mut crate :: fermented :: types :: platform_value :: types :: identifier :: platform_value_types_identifier_IdentifierBytes32,
                ) {
                    (*obj).0 = value;
                }
            }
            pub mod binary_data {
                #[doc = "FFI-representation of the [`platform_value::types::binary_data::BinaryData`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct platform_value_types_binary_data_BinaryData(
                    *mut crate::fermented::generics::Vec_u8,
                );
                impl
                    ferment_interfaces::FFIConversion<
                        platform_value::types::binary_data::BinaryData,
                    > for platform_value_types_binary_data_BinaryData
                {
                    unsafe fn ffi_from_const(
                        ffi: *const platform_value_types_binary_data_BinaryData,
                    ) -> platform_value::types::binary_data::BinaryData {
                        let ffi_ref = &*ffi;
                        platform_value::types::binary_data::BinaryData(
                            ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0),
                        )
                    }
                    unsafe fn ffi_to_const(
                        obj: platform_value::types::binary_data::BinaryData,
                    ) -> *const platform_value_types_binary_data_BinaryData {
                        ferment_interfaces::boxed(platform_value_types_binary_data_BinaryData(
                            ferment_interfaces::FFIConversion::ffi_to(obj.0),
                        ))
                    }
                    unsafe fn destroy(ffi: *mut platform_value_types_binary_data_BinaryData) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for platform_value_types_binary_data_BinaryData {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.0);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_ctor(
                    o_0: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut platform_value_types_binary_data_BinaryData {
                    ferment_interfaces::boxed(platform_value_types_binary_data_BinaryData(o_0))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_destroy(
                    ffi: *mut platform_value_types_binary_data_BinaryData,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_get_0(
                    obj: *const platform_value_types_binary_data_BinaryData,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn platform_value_types_binary_data_BinaryData_set_0(
                    obj: *mut platform_value_types_binary_data_BinaryData,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).0 = value;
                }
            }
        }
    }
    pub mod dpp {}
    pub mod ferment_example_platform {
        use crate as ferment_example_platform;
        pub mod spv {
            use crate as ferment_example_platform;
            #[doc = "FFI-representation of the [`fetch_identity`]"]
            #[doc = r" # Safety"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_platform_spv_fetch_identity (identifier : * mut crate :: fermented :: types :: platform_value ::types::identifier::platform_value_types_identifier_Identifier) -> * mut crate :: fermented :: generics :: Result_ok_dpp_identity_Identity_err_dpp_ProtocolError{
                let obj = ferment_example_platform::spv::fetch_identity(
                    ferment_interfaces::FFIConversion::ffi_from(identifier),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
        #[doc = "FFI-representation of the [`ferment_example_platform::SPV`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_platform_SPV {
            pub version: u32,
        }
        impl ferment_interfaces::FFIConversion<ferment_example_platform::SPV>
            for ferment_example_platform_SPV
        {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_platform_SPV,
            ) -> ferment_example_platform::SPV {
                let ffi_ref = &*ffi;
                ferment_example_platform::SPV {
                    version: ffi_ref.version,
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example_platform::SPV,
            ) -> *const ferment_example_platform_SPV {
                ferment_interfaces::boxed(ferment_example_platform_SPV {
                    version: obj.version,
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_platform_SPV) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_platform_SPV {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_platform_SPV_ctor(
            version: u32,
        ) -> *mut ferment_example_platform_SPV {
            ferment_interfaces::boxed(ferment_example_platform_SPV { version })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_platform_SPV_destroy(
            ffi: *mut ferment_example_platform_SPV,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_platform_SPV_get_version(
            obj: *const ferment_example_platform_SPV,
        ) -> u32 {
            (*obj).version
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_platform_SPV_set_version(
            obj: *mut ferment_example_platform_SPV,
            value: u32,
        ) {
            (*obj).version = value;
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
pub mod generics {
    use crate as ferment_example_platform;
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
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_dpp_identity_Identity_err_dpp_ProtocolError {
        pub ok: *mut crate::fermented::types::dpp::identity::dpp_identity_Identity,
        pub error: *mut crate::fermented::types::dpp::dpp_ProtocolError,
    }
    impl ferment_interfaces::FFIConversion<Result<dpp::identity::Identity, dpp::ProtocolError>>
        for Result_ok_dpp_identity_Identity_err_dpp_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_dpp_identity_Identity_err_dpp_ProtocolError,
        ) -> Result<dpp::identity::Identity, dpp::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<dpp::identity::Identity, dpp::ProtocolError>,
        ) -> *const Result_ok_dpp_identity_Identity_err_dpp_ProtocolError {
            ferment_interfaces::boxed({
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
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_dpp_identity_Identity_err_dpp_ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_dpp_identity_Identity_err_dpp_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                };
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_identity_Identity_err_dpp_ProtocolError_ctor(
        ok: *mut crate::fermented::types::dpp::identity::dpp_identity_Identity,
        error: *mut crate::fermented::types::dpp::dpp_ProtocolError,
    ) -> *mut Result_ok_dpp_identity_Identity_err_dpp_ProtocolError {
        ferment_interfaces::boxed(Result_ok_dpp_identity_Identity_err_dpp_ProtocolError {
            ok,
            error,
        })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_dpp_identity_Identity_err_dpp_ProtocolError_destroy(
        ffi: *mut Result_ok_dpp_identity_Identity_err_dpp_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
