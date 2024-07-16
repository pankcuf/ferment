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
        pub mod nested {
            use crate as ferment_example_nested;
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
            }
        }
        #[doc = "FFI-representation of the [`ferment_example :: get_root_struct`]"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_get_root_struct(
        ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct {
            let obj = ferment_example::get_root_struct();
            ferment_interfaces::FFIConversion::ffi_to(obj)
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
        pub mod document {
            use crate as ferment_example_nested;
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
        }
        pub mod data_contract {
            use crate as ferment_example_nested;
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
        }
    }
    pub mod ferment_example_nested {
        use crate as ferment_example_nested;
        pub mod model {
            use crate as ferment_example_nested;
            pub mod snapshot {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    NoSkipping = 0,
                    SkipFirst = 1,
                    SkipExcept = 2,
                    SkipAll = 3,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    > for ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) -> ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    ) -> *const ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll , _ => unreachable ! ("Enum Variant unreachable") })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => { } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_NoSkipping_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipFirst_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipExcept_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipAll_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: snapshot :: LLMQSnapshot`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_model_snapshot_LLMQSnapshot { pub member_list : * mut crate :: fermented :: generics :: Vec_u8 , pub skip_list : * mut crate :: fermented :: generics :: Vec_i32 , pub skip_list_mode : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode , pub option_vec : * mut crate :: fermented :: generics :: Vec_u8 }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::model::snapshot::LLMQSnapshot,
                    > for ferment_example_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                    ) -> ferment_example_nested::model::snapshot::LLMQSnapshot {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::model::snapshot::LLMQSnapshot {
                            member_list: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.member_list,
                            ),
                            skip_list: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.skip_list,
                            ),
                            skip_list_mode: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.skip_list_mode,
                            ),
                            option_vec: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.option_vec,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::model::snapshot::LLMQSnapshot,
                    ) -> *const ferment_example_nested_model_snapshot_LLMQSnapshot
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_snapshot_LLMQSnapshot {
                                member_list: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.member_list,
                                ),
                                skip_list: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list),
                                skip_list_mode: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.skip_list_mode,
                                ),
                                option_vec: match obj.option_vec {
                                    Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                    None => std::ptr::null_mut(),
                                },
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_model_snapshot_LLMQSnapshot {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.member_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list_mode);
                            if (!(ffi_ref.option_vec).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.option_vec);
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
                    member_list: *mut crate::fermented::generics::Vec_u8,
                    skip_list: *mut crate::fermented::generics::Vec_i32,
                    skip_list_mode : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    option_vec: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshot {
                    ferment_interfaces::boxed(ferment_example_nested_model_snapshot_LLMQSnapshot {
                        member_list,
                        skip_list,
                        skip_list_mode,
                        option_vec,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_member_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list_mode < > (obj : * const ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_option_vec(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_member_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).member_list = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_i32,
                ) {
                    (*obj).skip_list = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list_mode(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    (*obj).skip_list_mode = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_option_vec(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).option_vec = value;
                }
            }
            pub mod ferment_example {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: ferment_example :: get_crazy_case`]"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_ferment_example_get_crazy_case () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                    let obj = ferment_example_nested::model::ferment_example::get_crazy_case();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: ferment_example :: get_rotated_quorum`]"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_ferment_example_get_rotated_quorum () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                    let obj = ferment_example_nested::model::ferment_example::get_rotated_quorum();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
            pub mod quorum {
                use crate as ferment_example_nested;
                pub mod quorum_type {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum ferment_example_nested_model_quorum_quorum_type_QuorumType {
                        Normal,
                        Rotated,
                    }
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example_nested::model::quorum::quorum_type::QuorumType,
                        > for ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const ferment_example_nested_model_quorum_quorum_type_QuorumType,
                        ) -> ferment_example_nested::model::quorum::quorum_type::QuorumType
                        {
                            let ffi_ref = &*ffi;
                            match ffi_ref { ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal => ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Normal , ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated => ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Rotated }
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example_nested::model::quorum::quorum_type::QuorumType,
                        ) -> *const ferment_example_nested_model_quorum_quorum_type_QuorumType
                        {
                            ferment_interfaces :: boxed (match obj { ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Normal => ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal , ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Rotated => ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated , _ => unreachable ! ("Enum Variant unreachable") })
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_nested_model_quorum_quorum_type_QuorumType,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_nested_model_quorum_quorum_type_QuorumType {
                        fn drop(&mut self) {
                            unsafe {
                                match self { ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal => { } , ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated => { } , _ => unreachable ! ("This is unreachable") } ;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_Normal_ctor(
                    ) -> *mut ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_quorum_quorum_type_QuorumType::Normal {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_Rotated_ctor(
                    ) -> *mut ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_quorum_quorum_type_QuorumType::Rotated {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_destroy(
                        ffi: *mut ferment_example_nested_model_quorum_quorum_type_QuorumType,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
            pub mod tuples {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: tuples :: get_tuple_simple_complex`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_get_tuple_simple_complex(
                ) -> *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID
                {
                    let obj = ferment_example_nested::model::tuples::get_tuple_simple_complex();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: tuples :: get_tuple_complex_complex`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_get_tuple_complex_complex(
                    tuple: *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID,
                ) -> u32 {
                    let obj = ferment_example_nested::model::tuples::get_tuple_complex_complex(
                        ferment_interfaces::FFIConversion::ffi_from(tuple),
                    );
                    obj
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: tuples :: get_hash_id_form_snapshot`]"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_tuples_get_hash_id_form_snapshot (_snapshot : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_HashID{
                    let obj = ferment_example_nested::model::tuples::get_hash_id_form_snapshot(
                        ferment_interfaces::FFIConversion::ffi_from(_snapshot),
                    );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: tuples :: get_tuple_simple`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_get_tuple_simple(
                ) -> *mut crate::fermented::generics::Tuple_u32_u32 {
                    let obj = ferment_example_nested::model::tuples::get_tuple_simple();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: tuples :: StructWithTuple`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_model_tuples_StructWithTuple {
                    pub tuple:
                        *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::model::tuples::StructWithTuple,
                    > for ferment_example_nested_model_tuples_StructWithTuple
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_model_tuples_StructWithTuple,
                    ) -> ferment_example_nested::model::tuples::StructWithTuple
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::model::tuples::StructWithTuple {
                            tuple: {
                                let ffi_ref = &*ffi_ref.tuple;
                                (
                                    ffi_ref.o_0,
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
                                )
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::model::tuples::StructWithTuple,
                    ) -> *const ferment_example_nested_model_tuples_StructWithTuple
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_tuples_StructWithTuple {
                                tuple: ferment_interfaces::FFIConversion::ffi_to(obj.tuple),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_model_tuples_StructWithTuple,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_model_tuples_StructWithTuple {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.tuple);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_StructWithTuple_ctor(
                    tuple: *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID,
                ) -> *mut ferment_example_nested_model_tuples_StructWithTuple {
                    ferment_interfaces::boxed(ferment_example_nested_model_tuples_StructWithTuple {
                        tuple,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_StructWithTuple_destroy(
                    ffi: *mut ferment_example_nested_model_tuples_StructWithTuple,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_StructWithTuple_get_tuple(
                    obj: *const ferment_example_nested_model_tuples_StructWithTuple,
                ) -> *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID
                {
                    (*obj).tuple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_tuples_StructWithTuple_set_tuple(
                    obj: *mut ferment_example_nested_model_tuples_StructWithTuple,
                    value: *mut crate::fermented::generics::Tuple_u32_ferment_example_nested_HashID,
                ) {
                    (*obj).tuple = value;
                }
            }
            pub mod some_inner_2 {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: some_inner_2 :: get_normal_quorum`]"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_some_inner_2_get_normal_quorum () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                    let obj = ferment_example_nested::model::some_inner_2::get_normal_quorum();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
            pub mod callback {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: should_process_diff_in_range2`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_should_process_diff_in_range2(
                    _callback : * mut crate :: fermented :: generics :: Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    let obj =
                        ferment_example_nested::model::callback::should_process_diff_in_range2(
                            move |o_0, o_1| unsafe { (&*_callback).call(o_0, o_1) },
                        );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: lookup_block_hash_by_height`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_lookup_block_hash_by_height(
                    _callback: *mut crate::fermented::generics::Fn_ARGS_u32_RTRN_Option_u8,
                ) {
                    let obj = ferment_example_nested::model::callback::lookup_block_hash_by_height(
                        move |o_0| unsafe { (&*_callback).call(o_0) },
                    );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: setup_two_callbacks`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_setup_two_callbacks(
                    _callback1 : * mut crate :: fermented :: generics :: Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
                    _callback2 : * mut crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    let obj = ferment_example_nested::model::callback::setup_two_callbacks(
                        move |o_0, o_1| unsafe { (&*_callback1).call(o_0, o_1) },
                        move |o_0| unsafe { (&*_callback2).call(o_0) },
                    );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: find_current_block_classic`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_find_current_block_classic(
                    _callback: ferment_example_nested::model::callback::ClassicCallback,
                ) {
                    let obj = ferment_example_nested::model::callback::find_current_block_classic(
                        _callback,
                    );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: find_current_block_desc_mut`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_find_current_block_desc_mut(
                    _callback : * mut crate :: fermented :: generics :: FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String,
                ) {
                    let obj = ferment_example_nested::model::callback::find_current_block_desc_mut(
                        move |o_0, o_1| unsafe { (&*_callback).call(o_0, o_1) },
                    );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: lookup_merkle_root_by_hash`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_lookup_merkle_root_by_hash(
                    _callback: *mut crate::fermented::generics::Fn_ARGS_Arr_u8_32_RTRN_Option_u8,
                ) {
                    let obj = ferment_example_nested::model::callback::lookup_merkle_root_by_hash(
                        move |o_0| unsafe { (&*_callback).call(o_0) },
                    );
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: callback :: find_current_block_desc`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_callback_find_current_block_desc(
                    _callback : * mut crate :: fermented :: generics :: Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String,
                ) {
                    let obj = ferment_example_nested::model::callback::find_current_block_desc(
                        move |o_0, o_1| unsafe { (&*_callback).call(o_0, o_1) },
                    );
                }
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested :: model :: TestModLevelSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum ferment_example_nested_model_TestModLevelSnapshot {
                VO (* mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelSnapshot,
                > for ferment_example_nested_model_TestModLevelSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelSnapshot,
                ) -> ferment_example_nested::model::TestModLevelSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        _ => unreachable!("Enum Variant unreachable"),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
            ) -> *mut ferment_example_nested_model_TestModLevelSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelSnapshot::VO(
                    o_o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod some_inner {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: model :: some_inner :: get_normal_quorum`]"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_some_inner_get_normal_quorum () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                    let obj = ferment_example_nested::model::some_inner::get_normal_quorum();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
            #[doc = "FFI-representation of the [`ferment_example_nested :: model :: LLMQParams`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_model_LLMQParams {
                pub r#type: *mut std::os::raw::c_char,
                pub known_confirmed_at_height: *mut u32,
            }
            impl ferment_interfaces::FFIConversion<ferment_example_nested::model::LLMQParams>
                for ferment_example_nested_model_LLMQParams
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_LLMQParams,
                ) -> ferment_example_nested::model::LLMQParams {
                    let ffi_ref = &*ffi;
                    ferment_example_nested::model::LLMQParams {
                        r#type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.r#type),
                        known_confirmed_at_height: ferment_interfaces::from_opt_primitive(
                            ffi_ref.known_confirmed_at_height,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::LLMQParams,
                ) -> *const ferment_example_nested_model_LLMQParams {
                    ferment_interfaces::boxed(ferment_example_nested_model_LLMQParams {
                        r#type: ferment_interfaces::FFIConversion::ffi_to(obj.r#type),
                        known_confirmed_at_height: ferment_interfaces::to_opt_primitive(
                            obj.known_confirmed_at_height,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_LLMQParams) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_LLMQParams {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . r#type) ;
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_ctor(
                r#type: *mut std::os::raw::c_char,
                known_confirmed_at_height: *mut u32,
            ) -> *mut ferment_example_nested_model_LLMQParams {
                ferment_interfaces::boxed(ferment_example_nested_model_LLMQParams {
                    r#type,
                    known_confirmed_at_height,
                })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_destroy(
                ffi: *mut ferment_example_nested_model_LLMQParams,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_get_type(
                obj: *const ferment_example_nested_model_LLMQParams,
            ) -> *mut std::os::raw::c_char {
                (*obj).r#type
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_get_known_confirmed_at_height(
                obj: *const ferment_example_nested_model_LLMQParams,
            ) -> *mut u32 {
                (*obj).known_confirmed_at_height
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_set_type(
                obj: *mut ferment_example_nested_model_LLMQParams,
                value: *mut std::os::raw::c_char,
            ) {
                (*obj).r#type = value;
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_LLMQParams_set_known_confirmed_at_height(
                obj: *mut ferment_example_nested_model_LLMQParams,
                value: *mut u32,
            ) {
                (*obj).known_confirmed_at_height = value;
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested :: model :: TestModLevelOptSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum ferment_example_nested_model_TestModLevelOptSnapshot {
                VO (* mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelOptSnapshot,
                > for ferment_example_nested_model_TestModLevelOptSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelOptSnapshot,
                ) -> ferment_example_nested::model::TestModLevelOptSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelOptSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from_opt(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelOptSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelOptSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelOptSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelOptSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to_opt(o_0),
                            )
                        }
                        _ => unreachable!("Enum Variant unreachable"),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelOptSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelOptSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                                if (!(*o_0).is_null()) {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelOptSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut ferment_example_nested_model_TestModLevelOptSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelOptSnapshot::VO(
                    o_o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelOptSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelOptSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested :: model :: TestModLevelVecSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum ferment_example_nested_model_TestModLevelVecSnapshot {
                VO (* mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelVecSnapshot,
                > for ferment_example_nested_model_TestModLevelVecSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelVecSnapshot,
                ) -> ferment_example_nested::model::TestModLevelVecSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelVecSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelVecSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelVecSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelVecSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelVecSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        _ => unreachable!("Enum Variant unreachable"),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelVecSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelVecSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelVecSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut ferment_example_nested_model_TestModLevelVecSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelVecSnapshot::VO(
                    o_o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelVecSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelVecSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`ferment_example_nested :: model :: Quorum`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_model_Quorum { pub llmq_type : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType }
            impl ferment_interfaces::FFIConversion<ferment_example_nested::model::Quorum>
                for ferment_example_nested_model_Quorum
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_Quorum,
                ) -> ferment_example_nested::model::Quorum {
                    let ffi_ref = &*ffi;
                    ferment_example_nested::model::Quorum {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.llmq_type),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::Quorum,
                ) -> *const ferment_example_nested_model_Quorum {
                    ferment_interfaces::boxed(ferment_example_nested_model_Quorum {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_to(obj.llmq_type),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_Quorum) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_Quorum {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.llmq_type);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_ctor(
                llmq_type : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType,
            ) -> *mut ferment_example_nested_model_Quorum {
                ferment_interfaces::boxed(ferment_example_nested_model_Quorum { llmq_type })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_destroy(
                ffi: *mut ferment_example_nested_model_Quorum,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_get_llmq_type < > (obj : * const ferment_example_nested_model_Quorum) -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType{
                (*obj).llmq_type
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_set_llmq_type(
                obj: *mut ferment_example_nested_model_Quorum,
                value : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType,
            ) {
                (*obj).llmq_type = value;
            }
        }
        pub mod entry {
            use crate as ferment_example_nested;
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
        }
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
        pub mod gen {
            use crate as ferment_example_nested;
            pub mod dict {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllArcExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllArcExamples { pub arc_simple : * mut crate :: fermented :: generics :: std_sync_Arc_u32 , pub arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot , pub arc_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Vec_u8 , pub arc_opt_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot , pub opt_arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_Option_String , pub crazy_type1 : * mut crate :: fermented :: generics :: Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError , pub crazy_type2 : * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllArcExamples,
                    > for ferment_example_nested_gen_dict_AllArcExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllArcExamples,
                    ) -> ferment_example_nested::gen::dict::AllArcExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllArcExamples {
                            arc_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_simple,
                            ),
                            arc_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_complex,
                            ),
                            arc_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_generic,
                            ),
                            arc_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_opt_generic,
                            ),
                            opt_arc_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_arc_complex,
                            ),
                            crazy_type1: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.crazy_type1,
                            ),
                            crazy_type2: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.crazy_type2,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllArcExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllArcExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllArcExamples {
                            arc_simple: ferment_interfaces::FFIConversion::ffi_to(obj.arc_simple),
                            arc_complex: ferment_interfaces::FFIConversion::ffi_to(obj.arc_complex),
                            arc_generic: ferment_interfaces::FFIConversion::ffi_to(obj.arc_generic),
                            arc_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.arc_opt_generic,
                            ),
                            opt_arc_complex: match obj.opt_arc_complex {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            crazy_type1: ferment_interfaces::FFIConversion::ffi_to(obj.crazy_type1),
                            crazy_type2: ferment_interfaces::FFIConversion::ffi_to(obj.crazy_type2),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllArcExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllArcExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.arc_simple);
                            ferment_interfaces::unbox_any(ffi_ref.arc_complex);
                            ferment_interfaces::unbox_any(ffi_ref.arc_generic);
                            ferment_interfaces::unbox_any(ffi_ref.arc_opt_generic);
                            if (!(ffi_ref.opt_arc_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_arc_complex);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.crazy_type1);
                            ferment_interfaces::unbox_any(ffi_ref.crazy_type2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_ctor(
                    arc_simple: *mut crate::fermented::generics::std_sync_Arc_u32,
                    arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    arc_generic: *mut crate::fermented::generics::std_sync_Arc_Vec_u8,
                    arc_opt_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    opt_arc_complex: *mut crate::fermented::generics::std_sync_Arc_Option_String,
                    crazy_type1 : * mut crate :: fermented :: generics :: Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                    crazy_type2 : * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                ) -> *mut ferment_example_nested_gen_dict_AllArcExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllArcExamples {
                        arc_simple,
                        arc_complex,
                        arc_generic,
                        arc_opt_generic,
                        opt_arc_complex,
                        crazy_type1,
                        crazy_type2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllArcExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_arc_simple(
                    obj: *const ferment_example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_arc_complex < > (obj : * const ferment_example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_arc_generic(
                    obj: *const ferment_example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_arc_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_opt_arc_complex(
                    obj: *const ferment_example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_crazy_type1 < > (obj : * const ferment_example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type1
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_get_crazy_type2 < > (obj : * const ferment_example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type2
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_arc_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value: *mut crate::fermented::generics::std_sync_Arc_u32,
                ) {
                    (*obj).arc_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_arc_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).arc_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_arc_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value: *mut crate::fermented::generics::std_sync_Arc_Vec_u8,
                ) {
                    (*obj).arc_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_arc_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).arc_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_opt_arc_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value: *mut crate::fermented::generics::std_sync_Arc_Option_String,
                ) {
                    (*obj).opt_arc_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_crazy_type1(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value : * mut crate :: fermented :: generics :: Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    (*obj).crazy_type1 = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArcExamples_set_crazy_type2(
                    obj: *mut ferment_example_nested_gen_dict_AllArcExamples,
                    value : * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    (*obj).crazy_type2 = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllRwLockExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllRwLockExamples { pub rwlock_simple : * mut crate :: fermented :: generics :: std_sync_RwLock_u32 , pub rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot , pub rwlock_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Vec_u8 , pub rwlock_opt_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot , pub opt_rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_String , pub arc_rw_lock_complex : * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllRwLockExamples,
                    > for ferment_example_nested_gen_dict_AllRwLockExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllRwLockExamples,
                    ) -> ferment_example_nested::gen::dict::AllRwLockExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllRwLockExamples {
                            rwlock_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.rwlock_simple,
                            ),
                            rwlock_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.rwlock_complex,
                            ),
                            rwlock_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.rwlock_generic,
                            ),
                            rwlock_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.rwlock_opt_generic,
                            ),
                            opt_rwlock_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_rwlock_complex,
                            ),
                            arc_rw_lock_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_rw_lock_complex,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllRwLockExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllRwLockExamples
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_gen_dict_AllRwLockExamples {
                                rwlock_simple: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.rwlock_simple,
                                ),
                                rwlock_complex: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.rwlock_complex,
                                ),
                                rwlock_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.rwlock_generic,
                                ),
                                rwlock_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.rwlock_opt_generic,
                                ),
                                opt_rwlock_complex: match obj.opt_rwlock_complex {
                                    Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                    None => std::ptr::null_mut(),
                                },
                                arc_rw_lock_complex: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.arc_rw_lock_complex,
                                ),
                            },
                        )
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllRwLockExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllRwLockExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.rwlock_simple);
                            ferment_interfaces::unbox_any(ffi_ref.rwlock_complex);
                            ferment_interfaces::unbox_any(ffi_ref.rwlock_generic);
                            ferment_interfaces::unbox_any(ffi_ref.rwlock_opt_generic);
                            if (!(ffi_ref.opt_rwlock_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_rwlock_complex);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.arc_rw_lock_complex);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_ctor(
                    rwlock_simple: *mut crate::fermented::generics::std_sync_RwLock_u32,
                    rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    rwlock_generic: *mut crate::fermented::generics::std_sync_RwLock_Vec_u8,
                    rwlock_opt_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    opt_rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_String,
                    arc_rw_lock_complex : * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut ferment_example_nested_gen_dict_AllRwLockExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllRwLockExamples {
                        rwlock_simple,
                        rwlock_complex,
                        rwlock_generic,
                        rwlock_opt_generic,
                        opt_rwlock_complex,
                        arc_rw_lock_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_rwlock_simple(
                    obj: *const ferment_example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_u32 {
                    (*obj).rwlock_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_rwlock_complex < > (obj : * const ferment_example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_rwlock_generic(
                    obj: *const ferment_example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Vec_u8 {
                    (*obj).rwlock_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_rwlock_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_opt_rwlock_complex(
                    obj: *const ferment_example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Option_String
                {
                    (*obj).opt_rwlock_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_get_arc_rw_lock_complex < > (obj : * const ferment_example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_rw_lock_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_rwlock_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value: *mut crate::fermented::generics::std_sync_RwLock_u32,
                ) {
                    (*obj).rwlock_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_rwlock_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).rwlock_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_rwlock_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value: *mut crate::fermented::generics::std_sync_RwLock_Vec_u8,
                ) {
                    (*obj).rwlock_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_rwlock_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).rwlock_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_opt_rwlock_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value: *mut crate::fermented::generics::std_sync_RwLock_Option_String,
                ) {
                    (*obj).opt_rwlock_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRwLockExamples_set_arc_rw_lock_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRwLockExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).arc_rw_lock_complex = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllResultExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllResultExamples { pub result_ok_simple_err_simple : * mut crate :: fermented :: generics :: Result_ok_u32_err_u32 , pub result_ok_complex_err_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_String , pub result_ok_complex_2_err_complex : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot , pub result_ok_complex_err_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Vec_u8 , pub result_ok_complex_err_opt_simple : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_u32 , pub result_ok_complex_err_opt_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_String , pub result_ok_complex_err_opt_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_Vec_u8 , pub crazy_type : * mut crate :: fermented :: generics :: Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError , pub crazy_type_2 : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllResultExamples,
                    > for ferment_example_nested_gen_dict_AllResultExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllResultExamples,
                    ) -> ferment_example_nested::gen::dict::AllResultExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllResultExamples {
                            result_ok_simple_err_simple:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_simple_err_simple,
                                ),
                            result_ok_complex_err_complex:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_err_complex,
                                ),
                            result_ok_complex_2_err_complex:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_2_err_complex,
                                ),
                            result_ok_complex_err_generic:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_err_generic,
                                ),
                            result_ok_complex_err_opt_simple:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_err_opt_simple,
                                ),
                            result_ok_complex_err_opt_complex:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_err_opt_complex,
                                ),
                            result_ok_complex_err_opt_generic:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.result_ok_complex_err_opt_generic,
                                ),
                            crazy_type: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.crazy_type,
                            ),
                            crazy_type_2: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.crazy_type_2,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllResultExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllResultExamples
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_gen_dict_AllResultExamples {
                                result_ok_simple_err_simple:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_simple_err_simple,
                                    ),
                                result_ok_complex_err_complex:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_err_complex,
                                    ),
                                result_ok_complex_2_err_complex:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_2_err_complex,
                                    ),
                                result_ok_complex_err_generic:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_err_generic,
                                    ),
                                result_ok_complex_err_opt_simple:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_err_opt_simple,
                                    ),
                                result_ok_complex_err_opt_complex:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_err_opt_complex,
                                    ),
                                result_ok_complex_err_opt_generic:
                                    ferment_interfaces::FFIConversion::ffi_to(
                                        obj.result_ok_complex_err_opt_generic,
                                    ),
                                crazy_type: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.crazy_type,
                                ),
                                crazy_type_2: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.crazy_type_2,
                                ),
                            },
                        )
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllResultExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllResultExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.result_ok_simple_err_simple);
                            ferment_interfaces::unbox_any(ffi_ref.result_ok_complex_err_complex);
                            ferment_interfaces::unbox_any(ffi_ref.result_ok_complex_2_err_complex);
                            ferment_interfaces::unbox_any(ffi_ref.result_ok_complex_err_generic);
                            ferment_interfaces::unbox_any(ffi_ref.result_ok_complex_err_opt_simple);
                            ferment_interfaces::unbox_any(
                                ffi_ref.result_ok_complex_err_opt_complex,
                            );
                            ferment_interfaces::unbox_any(
                                ffi_ref.result_ok_complex_err_opt_generic,
                            );
                            ferment_interfaces::unbox_any(ffi_ref.crazy_type);
                            ferment_interfaces::unbox_any(ffi_ref.crazy_type_2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_ctor(
                    result_ok_simple_err_simple : * mut crate :: fermented :: generics :: Result_ok_u32_err_u32,
                    result_ok_complex_err_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_String,
                    result_ok_complex_2_err_complex : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    result_ok_complex_err_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Vec_u8,
                    result_ok_complex_err_opt_simple : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_u32,
                    result_ok_complex_err_opt_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_String,
                    result_ok_complex_err_opt_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_Vec_u8,
                    crazy_type : * mut crate :: fermented :: generics :: Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                    crazy_type_2 : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError,
                ) -> *mut ferment_example_nested_gen_dict_AllResultExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllResultExamples {
                        result_ok_simple_err_simple,
                        result_ok_complex_err_complex,
                        result_ok_complex_2_err_complex,
                        result_ok_complex_err_generic,
                        result_ok_complex_err_opt_simple,
                        result_ok_complex_err_opt_complex,
                        result_ok_complex_err_opt_generic,
                        crazy_type,
                        crazy_type_2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllResultExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_simple_err_simple(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    (*obj).result_ok_simple_err_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_complex(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_String {
                    (*obj).result_ok_complex_err_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_2_err_complex < > (obj : * const ferment_example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).result_ok_complex_2_err_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_generic(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Vec_u8 {
                    (*obj).result_ok_complex_err_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_simple(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_u32
                {
                    (*obj).result_ok_complex_err_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_complex(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_String
                {
                    (*obj).result_ok_complex_err_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_generic(
                    obj: *const ferment_example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_Vec_u8
                {
                    (*obj).result_ok_complex_err_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_crazy_type < > (obj : * const ferment_example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_get_crazy_type_2 < > (obj : * const ferment_example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_simple_err_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_u32_err_u32,
                ) {
                    (*obj).result_ok_simple_err_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_String_err_String,
                ) {
                    (*obj).result_ok_complex_err_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_2_err_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).result_ok_complex_2_err_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_String_err_Vec_u8,
                ) {
                    (*obj).result_ok_complex_err_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_String_err_Option_u32,
                ) {
                    (*obj).result_ok_complex_err_opt_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_String_err_Option_String,
                ) {
                    (*obj).result_ok_complex_err_opt_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value: *mut crate::fermented::generics::Result_ok_String_err_Option_Vec_u8,
                ) {
                    (*obj).result_ok_complex_err_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_crazy_type(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value : * mut crate :: fermented :: generics :: Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    (*obj).crazy_type = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllResultExamples_set_crazy_type_2(
                    obj: *mut ferment_example_nested_gen_dict_AllResultExamples,
                    value : * mut crate :: fermented :: generics :: Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    (*obj).crazy_type_2 = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllMutexExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllMutexExamples { pub mutex_simple : * mut crate :: fermented :: generics :: std_sync_Mutex_u32 , pub mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot , pub mutex_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Vec_u8 , pub mutex_opt_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot , pub opt_mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_String , pub platform_case : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllMutexExamples,
                    > for ferment_example_nested_gen_dict_AllMutexExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllMutexExamples,
                    ) -> ferment_example_nested::gen::dict::AllMutexExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllMutexExamples {
                            mutex_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.mutex_simple,
                            ),
                            mutex_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.mutex_complex,
                            ),
                            mutex_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.mutex_generic,
                            ),
                            mutex_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.mutex_opt_generic,
                            ),
                            opt_mutex_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_mutex_complex,
                            ),
                            platform_case: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.platform_case,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllMutexExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllMutexExamples
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_gen_dict_AllMutexExamples {
                                mutex_simple: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.mutex_simple,
                                ),
                                mutex_complex: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.mutex_complex,
                                ),
                                mutex_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.mutex_generic,
                                ),
                                mutex_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.mutex_opt_generic,
                                ),
                                opt_mutex_complex: match obj.opt_mutex_complex {
                                    Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                    None => std::ptr::null_mut(),
                                },
                                platform_case: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.platform_case,
                                ),
                            },
                        )
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllMutexExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllMutexExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.mutex_simple);
                            ferment_interfaces::unbox_any(ffi_ref.mutex_complex);
                            ferment_interfaces::unbox_any(ffi_ref.mutex_generic);
                            ferment_interfaces::unbox_any(ffi_ref.mutex_opt_generic);
                            if (!(ffi_ref.opt_mutex_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_mutex_complex);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.platform_case);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_ctor(
                    mutex_simple: *mut crate::fermented::generics::std_sync_Mutex_u32,
                    mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    mutex_generic: *mut crate::fermented::generics::std_sync_Mutex_Vec_u8,
                    mutex_opt_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    opt_mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_String,
                    platform_case : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut ferment_example_nested_gen_dict_AllMutexExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllMutexExamples {
                        mutex_simple,
                        mutex_complex,
                        mutex_generic,
                        mutex_opt_generic,
                        opt_mutex_complex,
                        platform_case,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_mutex_simple(
                    obj: *const ferment_example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_u32 {
                    (*obj).mutex_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_mutex_complex < > (obj : * const ferment_example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_mutex_generic(
                    obj: *const ferment_example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Vec_u8 {
                    (*obj).mutex_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_mutex_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_opt_mutex_complex(
                    obj: *const ferment_example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Option_String {
                    (*obj).opt_mutex_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_get_platform_case < > (obj : * const ferment_example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).platform_case
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_mutex_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value: *mut crate::fermented::generics::std_sync_Mutex_u32,
                ) {
                    (*obj).mutex_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_mutex_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).mutex_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_mutex_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value: *mut crate::fermented::generics::std_sync_Mutex_Vec_u8,
                ) {
                    (*obj).mutex_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_mutex_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).mutex_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_opt_mutex_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value: *mut crate::fermented::generics::std_sync_Mutex_Option_String,
                ) {
                    (*obj).opt_mutex_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMutexExamples_set_platform_case(
                    obj: *mut ferment_example_nested_gen_dict_AllMutexExamples,
                    value : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).platform_case = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllVecExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllVecExamples {
                    pub vec_simple: *mut crate::fermented::generics::Vec_u32,
                    pub vec_complex: *mut crate::fermented::generics::Vec_String,
                    pub vec_generic: *mut crate::fermented::generics::Vec_Vec_u8,
                    pub vec_opt_simple: *mut crate::fermented::generics::Vec_Option_u32,
                    pub vec_opt_complex: *mut crate::fermented::generics::Vec_Option_String,
                    pub vec_opt_generic: *mut crate::fermented::generics::Vec_Option_Vec_u8,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllVecExamples,
                    > for ferment_example_nested_gen_dict_AllVecExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllVecExamples,
                    ) -> ferment_example_nested::gen::dict::AllVecExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllVecExamples {
                            vec_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_simple,
                            ),
                            vec_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_complex,
                            ),
                            vec_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_generic,
                            ),
                            vec_opt_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_opt_simple,
                            ),
                            vec_opt_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_opt_complex,
                            ),
                            vec_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.vec_opt_generic,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllVecExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllVecExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllVecExamples {
                            vec_simple: ferment_interfaces::FFIConversion::ffi_to(obj.vec_simple),
                            vec_complex: ferment_interfaces::FFIConversion::ffi_to(obj.vec_complex),
                            vec_generic: ferment_interfaces::FFIConversion::ffi_to(obj.vec_generic),
                            vec_opt_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.vec_opt_simple,
                            ),
                            vec_opt_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.vec_opt_complex,
                            ),
                            vec_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.vec_opt_generic,
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllVecExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllVecExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.vec_simple);
                            ferment_interfaces::unbox_any(ffi_ref.vec_complex);
                            ferment_interfaces::unbox_any(ffi_ref.vec_generic);
                            ferment_interfaces::unbox_any(ffi_ref.vec_opt_simple);
                            ferment_interfaces::unbox_any(ffi_ref.vec_opt_complex);
                            ferment_interfaces::unbox_any(ffi_ref.vec_opt_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_ctor(
                    vec_simple: *mut crate::fermented::generics::Vec_u32,
                    vec_complex: *mut crate::fermented::generics::Vec_String,
                    vec_generic: *mut crate::fermented::generics::Vec_Vec_u8,
                    vec_opt_simple: *mut crate::fermented::generics::Vec_Option_u32,
                    vec_opt_complex: *mut crate::fermented::generics::Vec_Option_String,
                    vec_opt_generic: *mut crate::fermented::generics::Vec_Option_Vec_u8,
                ) -> *mut ferment_example_nested_gen_dict_AllVecExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllVecExamples {
                        vec_simple,
                        vec_complex,
                        vec_generic,
                        vec_opt_simple,
                        vec_opt_complex,
                        vec_opt_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllVecExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_simple(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_u32 {
                    (*obj).vec_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_complex(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_String {
                    (*obj).vec_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_generic(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Vec_u8 {
                    (*obj).vec_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_opt_simple(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_u32 {
                    (*obj).vec_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_opt_complex(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_String {
                    (*obj).vec_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_get_vec_opt_generic(
                    obj: *const ferment_example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_Vec_u8 {
                    (*obj).vec_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_u32,
                ) {
                    (*obj).vec_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_String,
                ) {
                    (*obj).vec_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_Vec_u8,
                ) {
                    (*obj).vec_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_opt_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_Option_u32,
                ) {
                    (*obj).vec_opt_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_Option_String,
                ) {
                    (*obj).vec_opt_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllVecExamples_set_vec_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllVecExamples,
                    value: *mut crate::fermented::generics::Vec_Option_Vec_u8,
                ) {
                    (*obj).vec_opt_generic = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllRcExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllRcExamples { pub arc_simple : * mut crate :: fermented :: generics :: std_rc_Rc_u32 , pub arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot , pub arc_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Vec_u8 , pub arc_opt_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot , pub opt_arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_Option_String }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllRcExamples,
                    > for ferment_example_nested_gen_dict_AllRcExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllRcExamples,
                    ) -> ferment_example_nested::gen::dict::AllRcExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllRcExamples {
                            arc_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_simple,
                            ),
                            arc_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_complex,
                            ),
                            arc_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_generic,
                            ),
                            arc_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.arc_opt_generic,
                            ),
                            opt_arc_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_arc_complex,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllRcExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllRcExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllRcExamples {
                            arc_simple: ferment_interfaces::FFIConversion::ffi_to(obj.arc_simple),
                            arc_complex: ferment_interfaces::FFIConversion::ffi_to(obj.arc_complex),
                            arc_generic: ferment_interfaces::FFIConversion::ffi_to(obj.arc_generic),
                            arc_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.arc_opt_generic,
                            ),
                            opt_arc_complex: match obj.opt_arc_complex {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllRcExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllRcExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.arc_simple);
                            ferment_interfaces::unbox_any(ffi_ref.arc_complex);
                            ferment_interfaces::unbox_any(ffi_ref.arc_generic);
                            ferment_interfaces::unbox_any(ffi_ref.arc_opt_generic);
                            if (!(ffi_ref.opt_arc_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_arc_complex);
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_ctor(
                    arc_simple: *mut crate::fermented::generics::std_rc_Rc_u32,
                    arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    arc_generic: *mut crate::fermented::generics::std_rc_Rc_Vec_u8,
                    arc_opt_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    opt_arc_complex: *mut crate::fermented::generics::std_rc_Rc_Option_String,
                ) -> *mut ferment_example_nested_gen_dict_AllRcExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllRcExamples {
                        arc_simple,
                        arc_complex,
                        arc_generic,
                        arc_opt_generic,
                        opt_arc_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllRcExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_get_arc_simple(
                    obj: *const ferment_example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_get_arc_complex < > (obj : * const ferment_example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_get_arc_generic(
                    obj: *const ferment_example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_get_arc_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_get_opt_arc_complex(
                    obj: *const ferment_example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_set_arc_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllRcExamples,
                    value: *mut crate::fermented::generics::std_rc_Rc_u32,
                ) {
                    (*obj).arc_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_set_arc_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRcExamples,
                    value : * mut crate :: fermented :: generics :: std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).arc_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_set_arc_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRcExamples,
                    value: *mut crate::fermented::generics::std_rc_Rc_Vec_u8,
                ) {
                    (*obj).arc_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_set_arc_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRcExamples,
                    value : * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).arc_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRcExamples_set_opt_arc_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRcExamples,
                    value: *mut crate::fermented::generics::std_rc_Rc_Option_String,
                ) {
                    (*obj).opt_arc_complex = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllRefCellExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllRefCellExamples { pub refcell_simple : * mut crate :: fermented :: generics :: std_cell_RefCell_u32 , pub refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot , pub refcell_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Vec_u8 , pub refcell_opt_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot , pub opt_refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_String }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllRefCellExamples,
                    > for ferment_example_nested_gen_dict_AllRefCellExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllRefCellExamples,
                    ) -> ferment_example_nested::gen::dict::AllRefCellExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllRefCellExamples {
                            refcell_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.refcell_simple,
                            ),
                            refcell_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.refcell_complex,
                            ),
                            refcell_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.refcell_generic,
                            ),
                            refcell_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.refcell_opt_generic,
                            ),
                            opt_refcell_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_refcell_complex,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllRefCellExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllRefCellExamples
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_gen_dict_AllRefCellExamples {
                                refcell_simple: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.refcell_simple,
                                ),
                                refcell_complex: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.refcell_complex,
                                ),
                                refcell_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.refcell_generic,
                                ),
                                refcell_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.refcell_opt_generic,
                                ),
                                opt_refcell_complex: match obj.opt_refcell_complex {
                                    Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                    None => std::ptr::null_mut(),
                                },
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllRefCellExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.refcell_simple);
                            ferment_interfaces::unbox_any(ffi_ref.refcell_complex);
                            ferment_interfaces::unbox_any(ffi_ref.refcell_generic);
                            ferment_interfaces::unbox_any(ffi_ref.refcell_opt_generic);
                            if (!(ffi_ref.opt_refcell_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_refcell_complex);
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_ctor(
                    refcell_simple: *mut crate::fermented::generics::std_cell_RefCell_u32,
                    refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    refcell_generic: *mut crate::fermented::generics::std_cell_RefCell_Vec_u8,
                    refcell_opt_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                    opt_refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_String,
                ) -> *mut ferment_example_nested_gen_dict_AllRefCellExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllRefCellExamples {
                        refcell_simple,
                        refcell_complex,
                        refcell_generic,
                        refcell_opt_generic,
                        opt_refcell_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_get_refcell_simple(
                    obj: *const ferment_example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_u32 {
                    (*obj).refcell_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_get_refcell_complex < > (obj : * const ferment_example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_get_refcell_generic(
                    obj: *const ferment_example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Vec_u8 {
                    (*obj).refcell_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_get_refcell_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_get_opt_refcell_complex(
                    obj: *const ferment_example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Option_String
                {
                    (*obj).opt_refcell_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_set_refcell_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    value: *mut crate::fermented::generics::std_cell_RefCell_u32,
                ) {
                    (*obj).refcell_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_set_refcell_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    value : * mut crate :: fermented :: generics :: std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).refcell_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_set_refcell_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    value: *mut crate::fermented::generics::std_cell_RefCell_Vec_u8,
                ) {
                    (*obj).refcell_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_set_refcell_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    value : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    (*obj).refcell_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllRefCellExamples_set_opt_refcell_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllRefCellExamples,
                    value: *mut crate::fermented::generics::std_cell_RefCell_Option_String,
                ) {
                    (*obj).opt_refcell_complex = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllMapExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllMapExamples { pub k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32 , pub k_simple_v_opt_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_u32 , pub k_simple_v_opt_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String , pub k_simple_v_opt_generic_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32 , pub k_simple_v_opt_generic_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String , pub opt_map_k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32 , pub opt_map_k_simple_v_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_String , pub opt_map_k_simple_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u8 , pub opt_map_k_generic_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Vec_u8_values_Vec_u8 , pub map_k_opt_generic_v_opt_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllMapExamples,
                    > for ferment_example_nested_gen_dict_AllMapExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllMapExamples,
                    ) -> ferment_example_nested::gen::dict::AllMapExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllMapExamples {
                            k_simple_v_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.k_simple_v_simple,
                            ),
                            k_simple_v_opt_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.k_simple_v_opt_simple,
                            ),
                            k_simple_v_opt_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.k_simple_v_opt_complex,
                            ),
                            k_simple_v_opt_generic_simple:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.k_simple_v_opt_generic_simple,
                                ),
                            k_simple_v_opt_generic_complex:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.k_simple_v_opt_generic_complex,
                                ),
                            opt_map_k_simple_v_simple:
                                ferment_interfaces::FFIConversion::ffi_from_opt(
                                    ffi_ref.opt_map_k_simple_v_simple,
                                ),
                            opt_map_k_simple_v_complex:
                                ferment_interfaces::FFIConversion::ffi_from_opt(
                                    ffi_ref.opt_map_k_simple_v_complex,
                                ),
                            opt_map_k_simple_v_generic:
                                ferment_interfaces::FFIConversion::ffi_from_opt(
                                    ffi_ref.opt_map_k_simple_v_generic,
                                ),
                            opt_map_k_generic_v_generic:
                                ferment_interfaces::FFIConversion::ffi_from_opt(
                                    ffi_ref.opt_map_k_generic_v_generic,
                                ),
                            map_k_opt_generic_v_opt_generic:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.map_k_opt_generic_v_opt_generic,
                                ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllMapExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllMapExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllMapExamples {
                            k_simple_v_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.k_simple_v_simple,
                            ),
                            k_simple_v_opt_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.k_simple_v_opt_simple,
                            ),
                            k_simple_v_opt_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.k_simple_v_opt_complex,
                            ),
                            k_simple_v_opt_generic_simple:
                                ferment_interfaces::FFIConversion::ffi_to(
                                    obj.k_simple_v_opt_generic_simple,
                                ),
                            k_simple_v_opt_generic_complex:
                                ferment_interfaces::FFIConversion::ffi_to(
                                    obj.k_simple_v_opt_generic_complex,
                                ),
                            opt_map_k_simple_v_simple: match obj.opt_map_k_simple_v_simple {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            opt_map_k_simple_v_complex: match obj.opt_map_k_simple_v_complex {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            opt_map_k_simple_v_generic: match obj.opt_map_k_simple_v_generic {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            opt_map_k_generic_v_generic: match obj.opt_map_k_generic_v_generic {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            map_k_opt_generic_v_opt_generic:
                                ferment_interfaces::FFIConversion::ffi_to(
                                    obj.map_k_opt_generic_v_opt_generic,
                                ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllMapExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllMapExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.k_simple_v_simple);
                            ferment_interfaces::unbox_any(ffi_ref.k_simple_v_opt_simple);
                            ferment_interfaces::unbox_any(ffi_ref.k_simple_v_opt_complex);
                            ferment_interfaces::unbox_any(ffi_ref.k_simple_v_opt_generic_simple);
                            ferment_interfaces::unbox_any(ffi_ref.k_simple_v_opt_generic_complex);
                            if (!(ffi_ref.opt_map_k_simple_v_simple).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_map_k_simple_v_simple);
                            };
                            if (!(ffi_ref.opt_map_k_simple_v_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_map_k_simple_v_complex);
                            };
                            if (!(ffi_ref.opt_map_k_simple_v_generic).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_map_k_simple_v_generic);
                            };
                            if (!(ffi_ref.opt_map_k_generic_v_generic).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_map_k_generic_v_generic);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.map_k_opt_generic_v_opt_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_ctor(
                    k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32,
                    k_simple_v_opt_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_u32,
                    k_simple_v_opt_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String,
                    k_simple_v_opt_generic_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32,
                    k_simple_v_opt_generic_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String,
                    opt_map_k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32,
                    opt_map_k_simple_v_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_String,
                    opt_map_k_simple_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u8,
                    opt_map_k_generic_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Vec_u8_values_Vec_u8,
                    map_k_opt_generic_v_opt_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
                ) -> *mut ferment_example_nested_gen_dict_AllMapExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllMapExamples {
                        k_simple_v_simple,
                        k_simple_v_opt_simple,
                        k_simple_v_opt_complex,
                        k_simple_v_opt_generic_simple,
                        k_simple_v_opt_generic_complex,
                        opt_map_k_simple_v_simple,
                        opt_map_k_simple_v_complex,
                        opt_map_k_simple_v_generic,
                        opt_map_k_generic_v_generic,
                        map_k_opt_generic_v_opt_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllMapExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_k_simple_v_simple(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_simple(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Option_u32
                {
                    (*obj).k_simple_v_opt_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_complex < > (obj : * const ferment_example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String{
                    (*obj).k_simple_v_opt_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_generic_simple < > (obj : * const ferment_example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32{
                    (*obj).k_simple_v_opt_generic_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_generic_complex < > (obj : * const ferment_example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String{
                    (*obj).k_simple_v_opt_generic_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_simple(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).opt_map_k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_complex(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_String
                {
                    (*obj).opt_map_k_simple_v_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_generic(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u8
                {
                    (*obj).opt_map_k_simple_v_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_opt_map_k_generic_v_generic(
                    obj: *const ferment_example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_Vec_u8_values_Vec_u8
                {
                    (*obj).opt_map_k_generic_v_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_get_map_k_opt_generic_v_opt_generic < > (obj : * const ferment_example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8{
                    (*obj).map_k_opt_generic_v_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_k_simple_v_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value: *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32,
                ) {
                    (*obj).k_simple_v_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_u32,
                ) {
                    (*obj).k_simple_v_opt_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String,
                ) {
                    (*obj).k_simple_v_opt_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_generic_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32,
                ) {
                    (*obj).k_simple_v_opt_generic_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_generic_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String,
                ) {
                    (*obj).k_simple_v_opt_generic_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value: *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32,
                ) {
                    (*obj).opt_map_k_simple_v_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_String,
                ) {
                    (*obj).opt_map_k_simple_v_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u8,
                ) {
                    (*obj).opt_map_k_simple_v_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_opt_map_k_generic_v_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_Vec_u8_values_Vec_u8,
                ) {
                    (*obj).opt_map_k_generic_v_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllMapExamples_set_map_k_opt_generic_v_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllMapExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
                ) {
                    (*obj).map_k_opt_generic_v_opt_generic = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllExamples { pub name : * mut std :: os :: raw :: c_char , pub all_map_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllMapExamples , pub all_result_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllResultExamples , pub all_set_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllSetExamples , pub all_arr_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllArrExamples , pub all_tuple_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllTupleExamples , pub all_opt_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllOptExamples }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllExamples,
                    > for ferment_example_nested_gen_dict_AllExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllExamples,
                    ) -> ferment_example_nested::gen::dict::AllExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllExamples {
                            name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                            all_map_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_map_examples,
                            ),
                            all_result_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_result_examples,
                            ),
                            all_set_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_set_examples,
                            ),
                            all_arr_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_arr_examples,
                            ),
                            all_tuple_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_tuple_examples,
                            ),
                            all_opt_examples: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_opt_examples,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllExamples {
                            name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                            all_map_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_map_examples,
                            ),
                            all_result_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_result_examples,
                            ),
                            all_set_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_set_examples,
                            ),
                            all_arr_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_arr_examples,
                            ),
                            all_tuple_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_tuple_examples,
                            ),
                            all_opt_examples: ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_opt_examples,
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . name) ;
                            ferment_interfaces::unbox_any(ffi_ref.all_map_examples);
                            ferment_interfaces::unbox_any(ffi_ref.all_result_examples);
                            ferment_interfaces::unbox_any(ffi_ref.all_set_examples);
                            ferment_interfaces::unbox_any(ffi_ref.all_arr_examples);
                            ferment_interfaces::unbox_any(ffi_ref.all_tuple_examples);
                            ferment_interfaces::unbox_any(ffi_ref.all_opt_examples);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_ctor(
                    name: *mut std::os::raw::c_char,
                    all_map_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllMapExamples,
                    all_result_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllResultExamples,
                    all_set_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllSetExamples,
                    all_arr_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllArrExamples,
                    all_tuple_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllTupleExamples,
                    all_opt_examples : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllOptExamples,
                ) -> *mut ferment_example_nested_gen_dict_AllExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllExamples {
                        name,
                        all_map_examples,
                        all_result_examples,
                        all_set_examples,
                        all_arr_examples,
                        all_tuple_examples,
                        all_opt_examples,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_name(
                    obj: *const ferment_example_nested_gen_dict_AllExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).name
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_map_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllMapExamples{
                    (*obj).all_map_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_result_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllResultExamples{
                    (*obj).all_result_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_set_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllSetExamples{
                    (*obj).all_set_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_arr_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllArrExamples{
                    (*obj).all_arr_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_tuple_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllTupleExamples{
                    (*obj).all_tuple_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_get_all_opt_examples < > (obj : * const ferment_example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllOptExamples{
                    (*obj).all_opt_examples
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_name(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value: *mut std::os::raw::c_char,
                ) {
                    (*obj).name = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_map_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllMapExamples,
                ) {
                    (*obj).all_map_examples = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_result_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllResultExamples,
                ) {
                    (*obj).all_result_examples = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_set_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllSetExamples,
                ) {
                    (*obj).all_set_examples = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_arr_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllArrExamples,
                ) {
                    (*obj).all_arr_examples = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_tuple_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllTupleExamples,
                ) {
                    (*obj).all_tuple_examples = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllExamples_set_all_opt_examples(
                    obj: *mut ferment_example_nested_gen_dict_AllExamples,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: gen :: dict :: ferment_example_nested_gen_dict_AllOptExamples,
                ) {
                    (*obj).all_opt_examples = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllArrExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllArrExamples {
                    pub arr: *mut crate::fermented::generics::Arr_u8_32,
                    pub opt_arr: *mut crate::fermented::generics::Arr_u8_32,
                    pub complex_arr: *mut crate::fermented::generics::Arr_String_32,
                    pub complex_arr_2:
                        *mut crate::fermented::generics::Arr_ferment_example_nested_model_Quorum_32,
                    pub generic_arr_2: *mut crate::fermented::generics::Arr_Vec_u8_32,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllArrExamples,
                    > for ferment_example_nested_gen_dict_AllArrExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllArrExamples,
                    ) -> ferment_example_nested::gen::dict::AllArrExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllArrExamples {
                            arr: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.arr),
                            opt_arr: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_arr,
                            ),
                            complex_arr: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.complex_arr,
                            ),
                            complex_arr_2: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.complex_arr_2,
                            ),
                            generic_arr_2: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.generic_arr_2,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllArrExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllArrExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllArrExamples {
                            arr: ferment_interfaces::FFIConversion::ffi_to(obj.arr),
                            opt_arr: match obj.opt_arr {
                                Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                None => std::ptr::null_mut(),
                            },
                            complex_arr: ferment_interfaces::FFIConversion::ffi_to(obj.complex_arr),
                            complex_arr_2: ferment_interfaces::FFIConversion::ffi_to(
                                obj.complex_arr_2,
                            ),
                            generic_arr_2: ferment_interfaces::FFIConversion::ffi_to(
                                obj.generic_arr_2,
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllArrExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllArrExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.arr);
                            if (!(ffi_ref.opt_arr).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_arr);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.complex_arr);
                            ferment_interfaces::unbox_any(ffi_ref.complex_arr_2);
                            ferment_interfaces::unbox_any(ffi_ref.generic_arr_2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_ctor(
                    arr: *mut crate::fermented::generics::Arr_u8_32,
                    opt_arr: *mut crate::fermented::generics::Arr_u8_32,
                    complex_arr: *mut crate::fermented::generics::Arr_String_32,
                    complex_arr_2 : * mut crate :: fermented :: generics :: Arr_ferment_example_nested_model_Quorum_32,
                    generic_arr_2: *mut crate::fermented::generics::Arr_Vec_u8_32,
                ) -> *mut ferment_example_nested_gen_dict_AllArrExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllArrExamples {
                        arr,
                        opt_arr,
                        complex_arr,
                        complex_arr_2,
                        generic_arr_2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllArrExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_get_arr(
                    obj: *const ferment_example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_get_opt_arr(
                    obj: *const ferment_example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).opt_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_get_complex_arr(
                    obj: *const ferment_example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_String_32 {
                    (*obj).complex_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_get_complex_arr_2(
                    obj: *const ferment_example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_ferment_example_nested_model_Quorum_32
                {
                    (*obj).complex_arr_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_get_generic_arr_2(
                    obj: *const ferment_example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_Vec_u8_32 {
                    (*obj).generic_arr_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_set_arr(
                    obj: *mut ferment_example_nested_gen_dict_AllArrExamples,
                    value: *mut crate::fermented::generics::Arr_u8_32,
                ) {
                    (*obj).arr = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_set_opt_arr(
                    obj: *mut ferment_example_nested_gen_dict_AllArrExamples,
                    value: *mut crate::fermented::generics::Arr_u8_32,
                ) {
                    (*obj).opt_arr = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_set_complex_arr(
                    obj: *mut ferment_example_nested_gen_dict_AllArrExamples,
                    value: *mut crate::fermented::generics::Arr_String_32,
                ) {
                    (*obj).complex_arr = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_set_complex_arr_2(
                    obj: *mut ferment_example_nested_gen_dict_AllArrExamples,
                    value : * mut crate :: fermented :: generics :: Arr_ferment_example_nested_model_Quorum_32,
                ) {
                    (*obj).complex_arr_2 = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllArrExamples_set_generic_arr_2(
                    obj: *mut ferment_example_nested_gen_dict_AllArrExamples,
                    value: *mut crate::fermented::generics::Arr_Vec_u8_32,
                ) {
                    (*obj).generic_arr_2 = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllSetExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllSetExamples { pub btreeset_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_u32 , pub btreeset_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String , pub btreeset_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Vec_u8 , pub btreeset_opt_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_u32 , pub btreeset_opt_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_String , pub btreeset_opt_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_Vec_u8 , pub hashset_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_u32 , pub hashset_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_String , pub hashset_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Vec_u8 , pub hashset_opt_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_u32 , pub hashset_opt_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_String , pub hashset_opt_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_Vec_u8 , pub hashset_opt_complex_external : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllSetExamples,
                    > for ferment_example_nested_gen_dict_AllSetExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllSetExamples,
                    ) -> ferment_example_nested::gen::dict::AllSetExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllSetExamples {
                            btreeset_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_simple,
                            ),
                            btreeset_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_complex,
                            ),
                            btreeset_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_generic,
                            ),
                            btreeset_opt_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_opt_simple,
                            ),
                            btreeset_opt_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_opt_complex,
                            ),
                            btreeset_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.btreeset_opt_generic,
                            ),
                            hashset_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_simple,
                            ),
                            hashset_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_complex,
                            ),
                            hashset_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_generic,
                            ),
                            hashset_opt_simple: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_opt_simple,
                            ),
                            hashset_opt_complex: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_opt_complex,
                            ),
                            hashset_opt_generic: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.hashset_opt_generic,
                            ),
                            hashset_opt_complex_external:
                                ferment_interfaces::FFIConversion::ffi_from(
                                    ffi_ref.hashset_opt_complex_external,
                                ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllSetExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllSetExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllSetExamples {
                            btreeset_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_simple,
                            ),
                            btreeset_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_complex,
                            ),
                            btreeset_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_generic,
                            ),
                            btreeset_opt_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_opt_simple,
                            ),
                            btreeset_opt_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_opt_complex,
                            ),
                            btreeset_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.btreeset_opt_generic,
                            ),
                            hashset_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_simple,
                            ),
                            hashset_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_complex,
                            ),
                            hashset_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_generic,
                            ),
                            hashset_opt_simple: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_opt_simple,
                            ),
                            hashset_opt_complex: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_opt_complex,
                            ),
                            hashset_opt_generic: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_opt_generic,
                            ),
                            hashset_opt_complex_external: ferment_interfaces::FFIConversion::ffi_to(
                                obj.hashset_opt_complex_external,
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllSetExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllSetExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_simple);
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_complex);
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_generic);
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_opt_simple);
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_opt_complex);
                            ferment_interfaces::unbox_any(ffi_ref.btreeset_opt_generic);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_simple);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_complex);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_generic);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_opt_simple);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_opt_complex);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_opt_generic);
                            ferment_interfaces::unbox_any(ffi_ref.hashset_opt_complex_external);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_ctor(
                    btreeset_simple: *mut crate::fermented::generics::std_collections_BTreeSet_u32,
                    btreeset_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String,
                    btreeset_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Vec_u8,
                    btreeset_opt_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_u32,
                    btreeset_opt_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_String,
                    btreeset_opt_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_Vec_u8,
                    hashset_simple: *mut crate::fermented::generics::std_collections_HashSet_u32,
                    hashset_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_String,
                    hashset_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Vec_u8,
                    hashset_opt_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_u32,
                    hashset_opt_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_String,
                    hashset_opt_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_Vec_u8,
                    hashset_opt_complex_external : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError,
                ) -> *mut ferment_example_nested_gen_dict_AllSetExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllSetExamples {
                        btreeset_simple,
                        btreeset_complex,
                        btreeset_generic,
                        btreeset_opt_simple,
                        btreeset_opt_complex,
                        btreeset_opt_generic,
                        hashset_simple,
                        hashset_complex,
                        hashset_generic,
                        hashset_opt_simple,
                        hashset_opt_complex,
                        hashset_opt_generic,
                        hashset_opt_complex_external,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllSetExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_simple(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_u32 {
                    (*obj).btreeset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_complex(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                {
                    (*obj).btreeset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_generic(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Vec_u8
                {
                    (*obj).btreeset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_opt_simple(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_u32
                {
                    (*obj).btreeset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_opt_complex(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_String
                {
                    (*obj).btreeset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_btreeset_opt_generic(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_Vec_u8
                {
                    (*obj).btreeset_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_simple(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_u32 {
                    (*obj).hashset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_complex(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_String
                {
                    (*obj).hashset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_generic(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Vec_u8
                {
                    (*obj).hashset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_opt_simple(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_u32
                {
                    (*obj).hashset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_opt_complex(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_String
                {
                    (*obj).hashset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_opt_generic(
                    obj: *const ferment_example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_Vec_u8
                {
                    (*obj).hashset_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_get_hashset_opt_complex_external < > (obj : * const ferment_example_nested_gen_dict_AllSetExamples) -> * mut crate :: fermented :: generics :: std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError{
                    (*obj).hashset_opt_complex_external
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_u32,
                ) {
                    (*obj).btreeset_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_String,
                ) {
                    (*obj).btreeset_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_Vec_u8,
                ) {
                    (*obj).btreeset_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_opt_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_Option_u32,
                ) {
                    (*obj).btreeset_opt_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_Option_String,
                ) {
                    (*obj).btreeset_opt_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_btreeset_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_BTreeSet_Option_Vec_u8,
                ) {
                    (*obj).btreeset_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_u32,
                ) {
                    (*obj).hashset_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_String,
                ) {
                    (*obj).hashset_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_Vec_u8,
                ) {
                    (*obj).hashset_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_opt_simple(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_Option_u32,
                ) {
                    (*obj).hashset_opt_simple = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_Option_String,
                ) {
                    (*obj).hashset_opt_complex = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_opt_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value: *mut crate::fermented::generics::std_collections_HashSet_Option_Vec_u8,
                ) {
                    (*obj).hashset_opt_generic = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllSetExamples_set_hashset_opt_complex_external(
                    obj: *mut ferment_example_nested_gen_dict_AllSetExamples,
                    value : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    (*obj).hashset_opt_complex_external = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllOptExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllOptExamples {
                    pub opt_complex: *mut std::os::raw::c_char,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllOptExamples,
                    > for ferment_example_nested_gen_dict_AllOptExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllOptExamples,
                    ) -> ferment_example_nested::gen::dict::AllOptExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllOptExamples {
                            opt_complex: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.opt_complex,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllOptExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllOptExamples {
                        ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllOptExamples {
                            opt_complex: ferment_interfaces::FFIConversion::ffi_to_opt(
                                obj.opt_complex,
                            ),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllOptExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllOptExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            if (!(ffi_ref.opt_complex).is_null()) {
                                ferment_interfaces::unbox_any(ffi_ref.opt_complex);
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllOptExamples_ctor(
                    opt_complex: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_nested_gen_dict_AllOptExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllOptExamples {
                        opt_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllOptExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllOptExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllOptExamples_get_opt_complex(
                    obj: *const ferment_example_nested_gen_dict_AllOptExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllOptExamples_set_opt_complex(
                    obj: *mut ferment_example_nested_gen_dict_AllOptExamples,
                    value: *mut std::os::raw::c_char,
                ) {
                    (*obj).opt_complex = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested :: gen :: dict :: AllTupleExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_gen_dict_AllTupleExamples {
                    pub tuple_string: *mut crate::fermented::generics::Tuple_String_String,
                    pub tuple_with_generic:
                        *mut crate::fermented::generics::Tuple_String_Vec_String,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::gen::dict::AllTupleExamples,
                    > for ferment_example_nested_gen_dict_AllTupleExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_gen_dict_AllTupleExamples,
                    ) -> ferment_example_nested::gen::dict::AllTupleExamples {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::gen::dict::AllTupleExamples {
                            tuple_string: {
                                let ffi_ref = &*ffi_ref.tuple_string;
                                (
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
                                )
                            },
                            tuple_with_generic: {
                                let ffi_ref = &*ffi_ref.tuple_with_generic;
                                (
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
                                )
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::gen::dict::AllTupleExamples,
                    ) -> *const ferment_example_nested_gen_dict_AllTupleExamples
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_gen_dict_AllTupleExamples {
                                tuple_string: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.tuple_string,
                                ),
                                tuple_with_generic: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.tuple_with_generic,
                                ),
                            },
                        )
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_nested_gen_dict_AllTupleExamples) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_gen_dict_AllTupleExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.tuple_string);
                            ferment_interfaces::unbox_any(ffi_ref.tuple_with_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_ctor(
                    tuple_string: *mut crate::fermented::generics::Tuple_String_String,
                    tuple_with_generic: *mut crate::fermented::generics::Tuple_String_Vec_String,
                ) -> *mut ferment_example_nested_gen_dict_AllTupleExamples {
                    ferment_interfaces::boxed(ferment_example_nested_gen_dict_AllTupleExamples {
                        tuple_string,
                        tuple_with_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_destroy(
                    ffi: *mut ferment_example_nested_gen_dict_AllTupleExamples,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_get_tuple_string(
                    obj: *const ferment_example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_String {
                    (*obj).tuple_string
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_get_tuple_with_generic(
                    obj: *const ferment_example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_Vec_String {
                    (*obj).tuple_with_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_set_tuple_string(
                    obj: *mut ferment_example_nested_gen_dict_AllTupleExamples,
                    value: *mut crate::fermented::generics::Tuple_String_String,
                ) {
                    (*obj).tuple_string = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_gen_dict_AllTupleExamples_set_tuple_with_generic(
                    obj: *mut ferment_example_nested_gen_dict_AllTupleExamples,
                    value: *mut crate::fermented::generics::Tuple_String_Vec_String,
                ) {
                    (*obj).tuple_with_generic = value;
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
    pub struct Arr_Vec_u8_32 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<[Vec<u8>; 32]> for Arr_Vec_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_Vec_u8_32) -> [Vec<u8>; 32] {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
                .try_into()
                .unwrap()
        }
        unsafe fn ffi_to_const(obj: [Vec<u8>; 32]) -> *const Arr_Vec_u8_32 {
            ferment_interfaces::FFIVecConversion::encode(obj.to_vec())
        }
        unsafe fn destroy(ffi: *mut Arr_Vec_u8_32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Arr_Vec_u8_32 {
        type Value = Vec<Vec<u8>>;
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
    impl Drop for Arr_Vec_u8_32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_Vec_u8_32_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Arr_Vec_u8_32 {
        ferment_interfaces::boxed(Arr_Vec_u8_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_Vec_u8_32_destroy(ffi: *mut Arr_Vec_u8_32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
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
            ferment_interfaces::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_group(obj.into_iter()),
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
    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Vec_String {
        ferment_interfaces::boxed(Vec_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_destroy(ffi: *mut Vec_String) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::rc::Rc<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::rc::Rc<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::rc::Rc<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to((*obj).clone()),
            })
        }
        unsafe fn destroy(ffi: *mut std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_rc_Rc_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_String {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<Result<String, String>> for Result_ok_String_err_String {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_String,
        ) -> Result<String, String> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(obj: Result<String, String>) -> *const Result_ok_String_err_String {
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
        unsafe fn destroy(ffi: *mut Result_ok_String_err_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_opt(self.ok);
                ferment_interfaces::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_String_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut std::os::raw::c_char,
    ) -> *mut Result_ok_String_err_String {
        ferment_interfaces::boxed(Result_ok_String_err_String { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_String_destroy(
        ffi: *mut Result_ok_String_err_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<u32>>
        for std_collections_HashSet_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_u32,
        ) -> std::collections::HashSet<u32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<u32>,
        ) -> *const std_collections_HashSet_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_u32 {
        type Value = std::collections::HashSet<u32>;
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
    impl Drop for std_collections_HashSet_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_u32_ctor(
        count: usize,
        values: *mut u32,
    ) -> *mut std_collections_HashSet_u32 {
        ferment_interfaces::boxed(std_collections_HashSet_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_u32_destroy(
        ffi: *mut std_collections_HashSet_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: Mutex :: new (ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . obj)) } unsafe fn ffi_to_const (obj : std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed (Self { obj : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . into_inner () . expect ("Err")) }) } unsafe fn destroy (ffi : * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any (self . obj) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::sync::Mutex<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Mutex<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Mutex_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Mutex<Option<String>>>
        for std_sync_Mutex_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Mutex_Option_String,
        ) -> std::sync::Mutex<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<Option<String>>,
        ) -> *const std_sync_Mutex_Option_String {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_Mutex_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_Mutex_Option_String {
        ferment_interfaces::boxed(std_sync_Mutex_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_String_destroy(
        ffi: *mut std_sync_Mutex_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode }
    impl
        ferment_interfaces::FFIConversion<
            Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>,
        > for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
        ) -> Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>,
        ) -> *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(
            ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion
        for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        type Value = Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>;
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
    impl Drop for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) -> *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        ferment_interfaces::boxed(
            Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode { count, values },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
        ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::rc::Rc<Option<String>>> for std_rc_Rc_Option_String {
        unsafe fn ffi_from_const(
            ffi: *const std_rc_Rc_Option_String,
        ) -> std::rc::Rc<Option<String>> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::rc::Rc<Option<String>>) -> *const std_rc_Rc_Option_String {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt((*obj).clone()),
            })
        }
        unsafe fn destroy(ffi: *mut std_rc_Rc_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_rc_Rc_Option_String {
        ferment_interfaces::boxed(std_rc_Rc_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_String_destroy(ffi: *mut std_rc_Rc_Option_String) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_Vec_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Option<Vec<u32>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_Vec_u32,
        ) -> std::collections::BTreeMap<u32, Option<Vec<u32>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<Vec<u32>>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_Vec_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_opt_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_u32,
    ) -> *mut std_collections_Map_keys_u32_values_Option_Vec_u32 {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_Option_Vec_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: cell :: RefCell :: new (ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . obj)) } unsafe fn ffi_to_const (obj : std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed (Self { obj : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . into_inner ()) }) } unsafe fn destroy (ffi : * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any (self . obj) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: rc :: Rc :: new (ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . obj)) } unsafe fn ffi_to_const (obj : std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed (Self { obj : ferment_interfaces :: FFIConversion :: ffi_to_opt ((* obj) . clone ()) }) } unsafe fn destroy (ffi : * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any (self . obj) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::cell::RefCell<Option<String>>>
        for std_cell_RefCell_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_Option_String,
        ) -> std::cell::RefCell<Option<String>> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::cell::RefCell<Option<String>>,
        ) -> *const std_cell_RefCell_Option_String {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt(obj.into_inner()),
            })
        }
        unsafe fn destroy(ffi: *mut std_cell_RefCell_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_cell_RefCell_Option_String {
        ferment_interfaces::boxed(std_cell_RefCell_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_String_destroy(
        ffi: *mut std_cell_RefCell_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub context: *const std::os::raw::c_void,
        caller: fn(u32, *mut crate::fermented::generics::Arr_u8_32) -> *mut std::os::raw::c_char,
        destructor: fn(result: *mut std::os::raw::c_char),
    }
    impl Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub unsafe fn call(&self, o_0: u32, o_1: [u8; 32]) -> Option<String> {
            let ffi_result = (self.caller)(o_0, ferment_interfaces::FFIConversion::ffi_to(o_1));
            let result =
                <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::ffi_from_opt(
                    ffi_result,
                );
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_u32_u32 {
        pub o_0: u32,
        pub o_1: u32,
    }
    impl ferment_interfaces::FFIConversion<(u32, u32)> for Tuple_u32_u32 {
        unsafe fn ffi_from_const(ffi: *const Tuple_u32_u32) -> (u32, u32) {
            let ffi_ref = &*ffi;
            (ffi_ref.o_0, ffi_ref.o_1)
        }
        unsafe fn ffi_to_const(obj: (u32, u32)) -> *const Tuple_u32_u32 {
            ferment_interfaces::boxed(Self {
                o_0: obj.0,
                o_1: obj.1,
            })
        }
        unsafe fn destroy(ffi: *mut Tuple_u32_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_u32_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_u32_ctor(o_0: u32, o_1: u32) -> *mut Tuple_u32_u32 {
        ferment_interfaces::boxed(Tuple_u32_u32 { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_u32_destroy(ffi: *mut Tuple_u32_u32) {
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
    pub struct std_collections_Map_keys_u32_values_Vec_u8 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Vec<u8>>>
        for std_collections_Map_keys_u32_values_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_u8,
        ) -> std::collections::BTreeMap<u32, Vec<u8>> {
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
            obj: std::collections::BTreeMap<u32, Vec<u8>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_u8 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Vec_u8_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_u32_values_Vec_u8 {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_Vec_u8 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<String>>
        for std_collections_HashSet_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_String,
        ) -> std::collections::HashSet<String> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<String>,
        ) -> *const std_collections_HashSet_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_String {
        type Value = std::collections::HashSet<String>;
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
    impl Drop for std_collections_HashSet_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_HashSet_String {
        ferment_interfaces::boxed(std_collections_HashSet_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_String_destroy(
        ffi: *mut std_collections_HashSet_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < Option < std :: sync :: Arc < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) -> Result < Option < std :: sync :: Arc < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from_opt (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < Option < std :: sync :: Arc < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to_opt (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_opt (self . ok) ; ferment_interfaces :: unbox_any_opt (self . error) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: generics :: std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
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
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<Option<Vec<u8>>>> for Vec_Option_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_Vec_u8) -> Vec<Option<Vec<u8>>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<Option<Vec<u8>>>) -> *const Vec_Option_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Option_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Option_Vec_u8 {
        type Value = Vec<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Vec_Option_Vec_u8 {
        ferment_interfaces::boxed(Vec_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_Vec_u8_destroy(ffi: *mut Vec_Option_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_u32 {
        pub obj: u32,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Mutex<u32>> for std_sync_Mutex_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_u32) -> std::sync::Mutex<u32> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ffi_ref.obj)
        }
        unsafe fn ffi_to_const(obj: std::sync::Mutex<u32>) -> *const std_sync_Mutex_u32 {
            ferment_interfaces::boxed(Self {
                obj: obj.into_inner().expect("Err"),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_Mutex_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_u32_ctor(obj: u32) -> *mut std_sync_Mutex_u32 {
        ferment_interfaces::boxed(std_sync_Mutex_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_u32_destroy(ffi: *mut std_sync_Mutex_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_u32 {
        pub obj: u32,
    }
    impl ferment_interfaces::FFIConversion<std::sync::RwLock<u32>> for std_sync_RwLock_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_u32) -> std::sync::RwLock<u32> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ffi_ref.obj)
        }
        unsafe fn ffi_to_const(obj: std::sync::RwLock<u32>) -> *const std_sync_RwLock_u32 {
            ferment_interfaces::boxed(Self {
                obj: obj.into_inner().expect("Err"),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_RwLock_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_u32_ctor(obj: u32) -> *mut std_sync_RwLock_u32 {
        ferment_interfaces::boxed(std_sync_RwLock_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_u32_destroy(ffi: *mut std_sync_RwLock_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<u32>>
        for std_collections_BTreeSet_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_u32,
        ) -> std::collections::BTreeSet<u32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<u32>,
        ) -> *const std_collections_BTreeSet_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_u32 {
        type Value = std::collections::BTreeSet<u32>;
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
    impl Drop for std_collections_BTreeSet_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_u32_ctor(
        count: usize,
        values: *mut u32,
    ) -> *mut std_collections_BTreeSet_u32 {
        ferment_interfaces::boxed(std_collections_BTreeSet_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_u32_destroy(
        ffi: *mut std_collections_BTreeSet_u32,
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
    pub struct Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < Option < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > , ferment_example :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) -> Result < Option < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > , ferment_example :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from_opt (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < Option < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > , ferment_example :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to_opt (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_opt (self . ok) ; ferment_interfaces :: unbox_any_opt (self . error) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<Option<Vec<u8>>>>
        for std_collections_HashSet_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_Vec_u8,
        ) -> std::collections::HashSet<Option<Vec<u8>>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<Vec<u8>>>,
        ) -> *const std_collections_HashSet_Option_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_Option_Vec_u8 {
        type Value = std::collections::HashSet<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_HashSet_Option_Vec_u8 {
        ferment_interfaces::boxed(std_collections_HashSet_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_Vec_u8_destroy(
        ffi: *mut std_collections_HashSet_Option_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, String>>
        for std_collections_Map_keys_u32_values_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_String,
        ) -> std::collections::BTreeMap<u32, String> {
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
            obj: std::collections::BTreeMap<u32, String>,
        ) -> *const std_collections_Map_keys_u32_values_String {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_u32_values_String {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Option_String {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<Result<String, Option<String>>>
        for Result_ok_String_err_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_String,
        ) -> Result<String, Option<String>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<String, Option<String>>,
        ) -> *const Result_ok_String_err_Option_String {
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (
                        ferment_interfaces::FFIConversion::ffi_to(o),
                        std::ptr::null_mut(),
                    ),
                    Err(o) => (
                        std::ptr::null_mut(),
                        ferment_interfaces::FFIConversion::ffi_to_opt(o),
                    ),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_opt(self.ok);
                ferment_interfaces::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_String_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut std::os::raw::c_char,
    ) -> *mut Result_ok_String_err_Option_String {
        ferment_interfaces::boxed(Result_ok_String_err_Option_String { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_String_destroy(
        ffi: *mut Result_ok_String_err_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Vec<Option<u32>>> for Vec_Option_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_u32) -> Vec<Option<u32>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<Option<u32>>) -> *const Vec_Option_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Option_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Option_u32 {
        type Value = Vec<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut Vec_Option_u32 {
        ferment_interfaces::boxed(Vec_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_u32_destroy(ffi: *mut Vec_Option_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_ferment_example_nested_model_Quorum_32 { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum }
    impl ferment_interfaces::FFIConversion<[ferment_example_nested::model::Quorum; 32]>
        for Arr_ferment_example_nested_model_Quorum_32
    {
        unsafe fn ffi_from_const(
            ffi: *const Arr_ferment_example_nested_model_Quorum_32,
        ) -> [ferment_example_nested::model::Quorum; 32] {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
                .try_into()
                .unwrap()
        }
        unsafe fn ffi_to_const(
            obj: [ferment_example_nested::model::Quorum; 32],
        ) -> *const Arr_ferment_example_nested_model_Quorum_32 {
            ferment_interfaces::FFIVecConversion::encode(obj.to_vec())
        }
        unsafe fn destroy(ffi: *mut Arr_ferment_example_nested_model_Quorum_32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Arr_ferment_example_nested_model_Quorum_32 {
        type Value = Vec<ferment_example_nested::model::Quorum>;
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
    impl Drop for Arr_ferment_example_nested_model_Quorum_32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_ferment_example_nested_model_Quorum_32_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum,
    ) -> *mut Arr_ferment_example_nested_model_Quorum_32 {
        ferment_interfaces::boxed(Arr_ferment_example_nested_model_Quorum_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_ferment_example_nested_model_Quorum_32_destroy(
        ffi: *mut Arr_ferment_example_nested_model_Quorum_32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<Vec<u8>>>
        for std_collections_HashSet_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Vec_u8,
        ) -> std::collections::HashSet<Vec<u8>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Vec<u8>>,
        ) -> *const std_collections_HashSet_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_Vec_u8 {
        type Value = std::collections::HashSet<Vec<u8>>;
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
    impl Drop for std_collections_HashSet_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_HashSet_Vec_u8 {
        ferment_interfaces::boxed(std_collections_HashSet_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Vec_u8_destroy(
        ffi: *mut std_collections_HashSet_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::sync::Arc<
                std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot>,
            >,
        > for std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi : * const std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Arc<std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot>>
        {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<
                std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot>,
            >,
        ) -> *const std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot
        {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(std::sync::RwLock::new(
                    obj.read().expect("Poisoned").clone(),
                )),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: generics :: std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Arc_std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::sync::Mutex<Option<Box<ferment_example_nested::model::snapshot::LLMQSnapshot>>>,
        > for std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi : * const std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Mutex<Option<Box<ferment_example_nested::model::snapshot::LLMQSnapshot>>>
        {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<
                Option<Box<ferment_example_nested::model::snapshot::LLMQSnapshot>>,
            >,
        ) -> *const std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot
        {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Mutex_Option_Box_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
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
    pub struct std_cell_RefCell_u32 {
        pub obj: u32,
    }
    impl ferment_interfaces::FFIConversion<std::cell::RefCell<u32>> for std_cell_RefCell_u32 {
        unsafe fn ffi_from_const(ffi: *const std_cell_RefCell_u32) -> std::cell::RefCell<u32> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ffi_ref.obj)
        }
        unsafe fn ffi_to_const(obj: std::cell::RefCell<u32>) -> *const std_cell_RefCell_u32 {
            ferment_interfaces::boxed(Self {
                obj: obj.into_inner(),
            })
        }
        unsafe fn destroy(ffi: *mut std_cell_RefCell_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_u32_ctor(obj: u32) -> *mut std_cell_RefCell_u32 {
        ferment_interfaces::boxed(std_cell_RefCell_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_u32_destroy(ffi: *mut std_cell_RefCell_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_Vec_u8) -> std::sync::Mutex<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::sync::Mutex<Vec<u8>>) -> *const std_sync_Mutex_Vec_u8 {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_Mutex_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_Mutex_Vec_u8 {
        ferment_interfaces::boxed(std_sync_Mutex_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_destroy(ffi: *mut std_sync_Mutex_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
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
    pub struct std_collections_HashSet_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<Option<u32>>>
        for std_collections_HashSet_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_u32,
        ) -> std::collections::HashSet<Option<u32>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<u32>>,
        ) -> *const std_collections_HashSet_Option_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_Option_u32 {
        type Value = std::collections::HashSet<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut std_collections_HashSet_Option_u32 {
        ferment_interfaces::boxed(std_collections_HashSet_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_u32_destroy(
        ffi: *mut std_collections_HashSet_Option_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<Option<Vec<u8>>>>
        for std_collections_BTreeSet_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_Vec_u8,
        ) -> std::collections::BTreeSet<Option<Vec<u8>>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<Vec<u8>>>,
        ) -> *const std_collections_BTreeSet_Option_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_Option_Vec_u8 {
        type Value = std::collections::BTreeSet<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_BTreeSet_Option_Vec_u8 {
        ferment_interfaces::boxed(std_collections_BTreeSet_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_Vec_u8_destroy(
        ffi: *mut std_collections_BTreeSet_Option_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub context: *const std::os::raw::c_void,
        caller: fn(u32, *mut crate::fermented::generics::Arr_u8_32) -> *mut std::os::raw::c_char,
        destructor: fn(result: *mut std::os::raw::c_char),
    }
    impl FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub unsafe fn call(&self, o_0: u32, o_1: [u8; 32]) -> Option<String> {
            let ffi_result = (self.caller)(o_0, ferment_interfaces::FFIConversion::ffi_to(o_1));
            let result =
                <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::ffi_from_opt(
                    ffi_result,
                );
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::sync::RwLock<Vec<u8>>> for std_sync_RwLock_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_Vec_u8) -> std::sync::RwLock<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::sync::RwLock<Vec<u8>>) -> *const std_sync_RwLock_Vec_u8 {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_RwLock_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_RwLock_Vec_u8 {
        ferment_interfaces::boxed(std_sync_RwLock_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Vec_u8_destroy(ffi: *mut std_sync_RwLock_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Option<u32>>>
        for std_collections_Map_keys_u32_values_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_u32,
        ) -> std::collections::BTreeMap<u32, Option<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::from_opt_primitive(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<u32>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_u32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_opt_primitive_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut u32,
    ) -> *mut std_collections_Map_keys_u32_values_Option_u32 {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_Option_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_String_String {
        pub o_0: *mut std::os::raw::c_char,
        pub o_1: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<(String, String)> for Tuple_String_String {
        unsafe fn ffi_from_const(ffi: *const Tuple_String_String) -> (String, String) {
            let ffi_ref = &*ffi;
            (
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
            )
        }
        unsafe fn ffi_to_const(obj: (String, String)) -> *const Tuple_String_String {
            ferment_interfaces::boxed(Self {
                o_0: ferment_interfaces::FFIConversion::ffi_to(obj.0),
                o_1: ferment_interfaces::FFIConversion::ffi_to(obj.1),
            })
        }
        unsafe fn destroy(ffi: *mut Tuple_String_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_String_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.o_0);
                ferment_interfaces::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_String_ctor(
        o_0: *mut std::os::raw::c_char,
        o_1: *mut std::os::raw::c_char,
    ) -> *mut Tuple_String_String {
        ferment_interfaces::boxed(Tuple_String_String { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_String_destroy(ffi: *mut Tuple_String_String) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::RwLock<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_RwLock_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::generics::Vec_u8,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>,
        > for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
        ) -> std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>,
        ) -> *const std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_opt_complex_group(obj.keys().cloned()),
                values: ferment_interfaces::to_opt_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8_ctor(
        count: usize,
        keys: *mut *mut crate::fermented::generics::Vec_u8,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        ferment_interfaces::boxed(
            std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
                count,
                keys,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_Vec_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_String,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Option<Vec<String>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_Vec_String,
        ) -> std::collections::BTreeMap<u32, Option<Vec<String>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<Vec<String>>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_Vec_String {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_opt_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_String,
    ) -> *mut std_collections_Map_keys_u32_values_Option_Vec_String {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_Option_Vec_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
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
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_primitive_group(obj.values().cloned()),
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
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut u32,
    ) -> *mut std_collections_Map_keys_u32_values_u32 {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<Vec<u8>>>
        for std_collections_BTreeSet_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Vec_u8,
        ) -> std::collections::BTreeSet<Vec<u8>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Vec<u8>>,
        ) -> *const std_collections_BTreeSet_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_Vec_u8 {
        type Value = std::collections::BTreeSet<Vec<u8>>;
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
    impl Drop for std_collections_BTreeSet_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_BTreeSet_Vec_u8 {
        ferment_interfaces::boxed(std_collections_BTreeSet_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Vec_u8_destroy(
        ffi: *mut std_collections_BTreeSet_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<Vec<Option<String>>> for Vec_Option_String {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_String) -> Vec<Option<String>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<Option<String>>) -> *const Vec_Option_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Option_String {
        type Value = Vec<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Vec_Option_String {
        ferment_interfaces::boxed(Vec_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_String_destroy(ffi: *mut Vec_Option_String) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_String_32 {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<[String; 32]> for Arr_String_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_String_32) -> [String; 32] {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
                .try_into()
                .unwrap()
        }
        unsafe fn ffi_to_const(obj: [String; 32]) -> *const Arr_String_32 {
            ferment_interfaces::FFIVecConversion::encode(obj.to_vec())
        }
        unsafe fn destroy(ffi: *mut Arr_String_32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Arr_String_32 {
        type Value = Vec<String>;
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
    impl Drop for Arr_String_32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_String_32_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Arr_String_32 {
        ferment_interfaces::boxed(Arr_String_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_String_32_destroy(ffi: *mut Arr_String_32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::cell::RefCell<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::cell::RefCell<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::cell::RefCell<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_cell_RefCell_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_String_Vec_String {
        pub o_0: *mut std::os::raw::c_char,
        pub o_1: *mut crate::fermented::generics::Vec_String,
    }
    impl ferment_interfaces::FFIConversion<(String, Vec<String>)> for Tuple_String_Vec_String {
        unsafe fn ffi_from_const(ffi: *const Tuple_String_Vec_String) -> (String, Vec<String>) {
            let ffi_ref = &*ffi;
            (
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_0),
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
            )
        }
        unsafe fn ffi_to_const(obj: (String, Vec<String>)) -> *const Tuple_String_Vec_String {
            ferment_interfaces::boxed(Self {
                o_0: ferment_interfaces::FFIConversion::ffi_to(obj.0),
                o_1: ferment_interfaces::FFIConversion::ffi_to(obj.1),
            })
        }
        unsafe fn destroy(ffi: *mut Tuple_String_Vec_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_String_Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.o_0);
                ferment_interfaces::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_Vec_String_ctor(
        o_0: *mut std::os::raw::c_char,
        o_1: *mut crate::fermented::generics::Vec_String,
    ) -> *mut Tuple_String_Vec_String {
        ferment_interfaces::boxed(Tuple_String_Vec_String { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_Vec_String_destroy(ffi: *mut Tuple_String_Vec_String) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Arc<Option<String>>>
        for std_sync_Arc_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_Option_String,
        ) -> std::sync::Arc<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<Option<String>>,
        ) -> *const std_sync_Arc_Option_String {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt((*obj).clone()),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_Arc_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_Arc_Option_String {
        ferment_interfaces::boxed(std_sync_Arc_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_String_destroy(
        ffi: *mut std_sync_Arc_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::sync::Arc<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Arc<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to((*obj).clone()),
            })
        }
        unsafe fn destroy(
            ffi: *mut std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(
            std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Arc_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::HashSet<
                Option<ferment_example::errors::protocol_error::ProtocolError>,
            >,
        > for std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError,
        ) -> std::collections::HashSet<Option<ferment_example::errors::protocol_error::ProtocolError>>
        {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<
                Option<ferment_example::errors::protocol_error::ProtocolError>,
            >,
        ) -> *const std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError
        {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(
            ffi : * mut std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion
        for std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError
    {
        type Value = std::collections::HashSet<
            Option<ferment_example::errors::protocol_error::ProtocolError>,
        >;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError,
    ) -> *mut std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError
    {
        ferment_interfaces::boxed(
            std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError {
                count,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut std_collections_HashSet_Option_ferment_example_errors_protocol_error_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Arc<Vec<u8>>> for std_sync_Arc_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Arc_Vec_u8) -> std::sync::Arc<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::sync::Arc<Vec<u8>>) -> *const std_sync_Arc_Vec_u8 {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to((*obj).clone()),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_Arc_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_Arc_Vec_u8 {
        ferment_interfaces::boxed(std_sync_Arc_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Vec_u8_destroy(ffi: *mut std_sync_Arc_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<Option<u32>>>
        for std_collections_BTreeSet_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_u32,
        ) -> std::collections::BTreeSet<Option<u32>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<u32>>,
        ) -> *const std_collections_BTreeSet_Option_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_Option_u32 {
        type Value = std::collections::BTreeSet<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut std_collections_BTreeSet_Option_u32 {
        ferment_interfaces::boxed(std_collections_BTreeSet_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_u32_destroy(
        ffi: *mut std_collections_BTreeSet_Option_u32,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::cell::RefCell<Vec<u8>>> for std_cell_RefCell_Vec_u8 {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_Vec_u8,
        ) -> std::cell::RefCell<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::cell::RefCell<Vec<u8>>) -> *const std_cell_RefCell_Vec_u8 {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to(obj.into_inner()),
            })
        }
        unsafe fn destroy(ffi: *mut std_cell_RefCell_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_cell_RefCell_Vec_u8 {
        ferment_interfaces::boxed(std_cell_RefCell_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Vec_u8_destroy(ffi: *mut std_cell_RefCell_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError { pub ok : * mut u32 , pub error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl
        ferment_interfaces::FFIConversion<
            Result<u32, ferment_example::errors::protocol_error::ProtocolError>,
        > for Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
        ) -> Result<u32, ferment_example::errors::protocol_error::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| *o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<u32, ferment_example::errors::protocol_error::ProtocolError>,
        ) -> *const Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError {
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (ferment_interfaces::boxed(o), std::ptr::null_mut()),
                    Err(o) => (
                        std::ptr::null_mut(),
                        ferment_interfaces::FFIConversion::ffi_to(o),
                    ),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(
            ffi: *mut Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::destroy_opt_primitive(self.ok);
                ferment_interfaces::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError_ctor(
        ok: *mut u32,
        error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError,
    ) -> *mut Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError { ok, error },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi: *mut Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
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
    pub struct Result_ok_String_err_Option_u32 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Result<String, Option<u32>>>
        for Result_ok_String_err_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_u32,
        ) -> Result<String, Option<u32>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::from_opt_primitive(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<String, Option<u32>>,
        ) -> *const Result_ok_String_err_Option_u32 {
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (
                        ferment_interfaces::FFIConversion::ffi_to(o),
                        std::ptr::null_mut(),
                    ),
                    Err(o) => (
                        std::ptr::null_mut(),
                        ferment_interfaces::to_opt_primitive(o),
                    ),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_opt(self.ok);
                ferment_interfaces::destroy_opt_primitive(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_u32_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut u32,
    ) -> *mut Result_ok_String_err_Option_u32 {
        ferment_interfaces::boxed(Result_ok_String_err_Option_u32 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_u32_destroy(
        ffi: *mut Result_ok_String_err_Option_u32,
    ) {
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
    pub struct Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {
        pub context: *const std::os::raw::c_void,
        caller: fn(
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: fn(result: *mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {
        pub unsafe fn call(&self, o_0: [u8; 32]) -> Option<[u8; 32]> {
            let ffi_result = (self.caller)(ferment_interfaces::FFIConversion::ffi_to(o_0));
            let result =
                <crate::fermented::generics::Arr_u8_32 as ferment_interfaces::FFIConversion<
                    [u8; 32],
                >>::ffi_from_opt(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot { pub ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > for Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot) -> Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , ferment_example_nested :: model :: snapshot :: LLMQSnapshot >) -> * const Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_opt (self . ok) ; ferment_interfaces :: unbox_any_opt (self . error) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_u32 {
        pub obj: u32,
    }
    impl ferment_interfaces::FFIConversion<std::rc::Rc<u32>> for std_rc_Rc_u32 {
        unsafe fn ffi_from_const(ffi: *const std_rc_Rc_u32) -> std::rc::Rc<u32> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ffi_ref.obj)
        }
        unsafe fn ffi_to_const(obj: std::rc::Rc<u32>) -> *const std_rc_Rc_u32 {
            ferment_interfaces::boxed(Self { obj: *obj })
        }
        unsafe fn destroy(ffi: *mut std_rc_Rc_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_u32_ctor(obj: u32) -> *mut std_rc_Rc_u32 {
        ferment_interfaces::boxed(std_rc_Rc_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_u32_destroy(ffi: *mut std_rc_Rc_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<Vec<u8>>> for Vec_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_Vec_u8) -> Vec<Vec<u8>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<Vec<u8>>) -> *const Vec_Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_Vec_u8 {
        type Value = Vec<Vec<u8>>;
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
    impl Drop for Vec_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Vec_Vec_u8 {
        ferment_interfaces::boxed(Vec_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Vec_u8_destroy(ffi: *mut Vec_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_i32 {
        pub count: usize,
        pub values: *mut i32,
    }
    impl ferment_interfaces::FFIConversion<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_i32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_i32 {
        type Value = Vec<i32>;
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
    impl Drop for Vec_i32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(count: usize, values: *mut i32) -> *mut Vec_i32 {
        ferment_interfaces::boxed(Vec_i32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::generics::Vec_u8,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<Vec<u8>, Vec<u8>>>
        for std_collections_Map_keys_Vec_u8_values_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_Vec_u8_values_Vec_u8,
        ) -> std::collections::BTreeMap<Vec<u8>, Vec<u8>> {
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
            obj: std::collections::BTreeMap<Vec<u8>, Vec<u8>>,
        ) -> *const std_collections_Map_keys_Vec_u8_values_Vec_u8 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_Vec_u8_values_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Vec_u8_values_Vec_u8_ctor(
        count: usize,
        keys: *mut *mut crate::fermented::generics::Vec_u8,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        ferment_interfaces::boxed(std_collections_Map_keys_Vec_u8_values_Vec_u8 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Vec_u8_values_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_Vec_u8_values_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < Option < Vec < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) -> Result < Option < Vec < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from_opt (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < Option < Vec < ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > , ferment_example :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to_opt (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_opt (self . ok) ; ferment_interfaces :: unbox_any_opt (self . error) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_err_ferment_example_errors_protocol_error_ProtocolError,
    ) {
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
    pub struct Vec_ferment_example_nested_model_snapshot_LLMQSnapshot { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            Vec<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        > for Vec_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> Vec<ferment_example_nested::model::snapshot::LLMQSnapshot> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<ferment_example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshot {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshot) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion
        for Vec_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        type Value = Vec<ferment_example_nested::model::snapshot::LLMQSnapshot>;
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
    impl Drop for Vec_ferment_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshot {
        ferment_interfaces::boxed(Vec_ferment_example_nested_model_snapshot_LLMQSnapshot {
            count,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Vec_u8 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<Result<String, Vec<u8>>> for Result_ok_String_err_Vec_u8 {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Vec_u8,
        ) -> Result<String, Vec<u8>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(obj: Result<String, Vec<u8>>) -> *const Result_ok_String_err_Vec_u8 {
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
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_opt(self.ok);
                ferment_interfaces::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Vec_u8_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Result_ok_String_err_Vec_u8 {
        ferment_interfaces::boxed(Result_ok_String_err_Vec_u8 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Vec_u8_destroy(
        ffi: *mut Result_ok_String_err_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<Option<String>>>
        for std_collections_BTreeSet_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_String,
        ) -> std::collections::BTreeSet<Option<String>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<String>>,
        ) -> *const std_collections_BTreeSet_Option_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_Option_String {
        type Value = std::collections::BTreeSet<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_BTreeSet_Option_String {
        ferment_interfaces::boxed(std_collections_BTreeSet_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_String_destroy(
        ffi: *mut std_collections_BTreeSet_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub count : usize , pub keys : * mut u32 , pub values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<u32, ferment_example_nested::model::snapshot::LLMQSnapshot>,
        >
        for std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::collections::BTreeMap<u32, ferment_example_nested::model::snapshot::LLMQSnapshot>
        {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }        unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot >) -> * const std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop
        for std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
        count: usize,
        keys: *mut u32,
        values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot
    {
        ferment_interfaces :: boxed (std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { count , keys , values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
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
    pub struct std_collections_HashSet_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::HashSet<Option<String>>>
        for std_collections_HashSet_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_String,
        ) -> std::collections::HashSet<Option<String>> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<String>>,
        ) -> *const std_collections_HashSet_Option_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_HashSet_Option_String {
        type Value = std::collections::HashSet<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_HashSet_Option_String {
        ferment_interfaces::boxed(std_collections_HashSet_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_String_destroy(
        ffi: *mut std_collections_HashSet_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError { pub context : * const std :: os :: raw :: c_void , caller : fn (u32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError , destructor : fn (result : * mut crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError) , }
    impl Fn_ARGS_u32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError {
        pub unsafe fn call(
            &self,
            o_0: u32,
        ) -> Result<u32, ferment_example::errors::protocol_error::ProtocolError> {
            let ffi_result = (self.caller)(o_0);
            let result = < crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError as ferment_interfaces :: FFIConversion < Result < u32 , ferment_example :: errors :: protocol_error :: ProtocolError > >> :: ffi_from (ffi_result) ;
            (self.destructor)(ffi_result);
            result
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_u32_ferment_example_nested_HashID {
        pub o_0: u32,
        pub o_1:
            *mut crate::fermented::types::ferment_example::nested::ferment_example_nested_HashID,
    }
    impl ferment_interfaces::FFIConversion<(u32, ferment_example::nested::HashID)>
        for Tuple_u32_ferment_example_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const Tuple_u32_ferment_example_nested_HashID,
        ) -> (u32, ferment_example::nested::HashID) {
            let ffi_ref = &*ffi;
            (
                ffi_ref.o_0,
                ferment_interfaces::FFIConversion::ffi_from(ffi_ref.o_1),
            )
        }
        unsafe fn ffi_to_const(
            obj: (u32, ferment_example::nested::HashID),
        ) -> *const Tuple_u32_ferment_example_nested_HashID {
            ferment_interfaces::boxed(Self {
                o_0: obj.0,
                o_1: ferment_interfaces::FFIConversion::ffi_to(obj.1),
            })
        }
        unsafe fn destroy(ffi: *mut Tuple_u32_ferment_example_nested_HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_u32_ferment_example_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_ferment_example_nested_HashID_ctor(
        o_0: u32,
        o_1: *mut crate::fermented::types::ferment_example::nested::ferment_example_nested_HashID,
    ) -> *mut Tuple_u32_ferment_example_nested_HashID {
        ferment_interfaces::boxed(Tuple_u32_ferment_example_nested_HashID { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_ferment_example_nested_HashID_destroy(
        ffi: *mut Tuple_u32_ferment_example_nested_HashID,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeMap<u32, Option<String>>>
        for std_collections_Map_keys_u32_values_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_String,
        ) -> std::collections::BTreeMap<u32, Option<String>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<String>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_String {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_primitive_group(obj.keys().cloned()),
                values: ferment_interfaces::to_opt_complex_group(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_u32_values_Option_String {
        ferment_interfaces::boxed(std_collections_Map_keys_u32_values_Option_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::sync::RwLock<Option<String>>>
        for std_sync_RwLock_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_RwLock_Option_String,
        ) -> std::sync::RwLock<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(
            obj: std::sync::RwLock<Option<String>>,
        ) -> *const std_sync_RwLock_Option_String {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
        unsafe fn destroy(ffi: *mut std_sync_RwLock_Option_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_RwLock_Option_String {
        ferment_interfaces::boxed(std_sync_RwLock_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_String_destroy(
        ffi: *mut std_sync_RwLock_Option_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<std::rc::Rc<Vec<u8>>> for std_rc_Rc_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_rc_Rc_Vec_u8) -> std::rc::Rc<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.obj))
        }
        unsafe fn ffi_to_const(obj: std::rc::Rc<Vec<u8>>) -> *const std_rc_Rc_Vec_u8 {
            ferment_interfaces::boxed(Self {
                obj: ferment_interfaces::FFIConversion::ffi_to((*obj).clone()),
            })
        }
        unsafe fn destroy(ffi: *mut std_rc_Rc_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_rc_Rc_Vec_u8 {
        ferment_interfaces::boxed(std_rc_Rc_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Vec_u8_destroy(ffi: *mut std_rc_Rc_Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_u32 {
        pub obj: u32,
    }
    impl ferment_interfaces::FFIConversion<std::sync::Arc<u32>> for std_sync_Arc_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Arc_u32) -> std::sync::Arc<u32> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ffi_ref.obj)
        }
        unsafe fn ffi_to_const(obj: std::sync::Arc<u32>) -> *const std_sync_Arc_u32 {
            ferment_interfaces::boxed(Self { obj: *obj })
        }
        unsafe fn destroy(ffi: *mut std_sync_Arc_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_u32_ctor(obj: u32) -> *mut std_sync_Arc_u32 {
        ferment_interfaces::boxed(std_sync_Arc_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_u32_destroy(ffi: *mut std_sync_Arc_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: Arc :: new (ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . obj)) } unsafe fn ffi_to_const (obj : std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed (Self { obj : ferment_interfaces :: FFIConversion :: ffi_to_opt ((* obj) . clone ()) }) } unsafe fn destroy (ffi : * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any (self . obj) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
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
                    Ok(o) => (ferment_interfaces::boxed(o), std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), ferment_interfaces::boxed(o)),
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
    pub struct std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment_interfaces :: FFIConversion < std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: RwLock :: new (ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . obj)) } unsafe fn ffi_to_const (obj : std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , ferment_example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { ferment_interfaces :: boxed (Self { obj : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . into_inner () . expect ("Err")) }) } unsafe fn destroy (ffi : * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any (self . obj) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot{
        ferment_interfaces :: boxed (std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_ferment_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError { pub context : * const std :: os :: raw :: c_void , caller : fn (* mut crate :: fermented :: generics :: Arr_u8_32 , * mut crate :: fermented :: generics :: Arr_u8_32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError , destructor : fn (result : * mut crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError) , }
    impl Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError { pub unsafe fn call (& self , o_0 : [u8 ; 32] , o_1 : [u8 ; 32]) -> Result < u32 , ferment_example :: errors :: protocol_error :: ProtocolError > { let ffi_result = (self . caller) (ferment_interfaces :: FFIConversion :: ffi_to (o_0) , ferment_interfaces :: FFIConversion :: ffi_to (o_1)) ; let result = < crate :: fermented :: generics :: Result_ok_u32_err_ferment_example_errors_protocol_error_ProtocolError as ferment_interfaces :: FFIConversion < Result < u32 , ferment_example :: errors :: protocol_error :: ProtocolError > >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , Option < ferment_example :: errors :: protocol_error :: ProtocolError > > > for Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError) -> Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , Option < ferment_example :: errors :: protocol_error :: ProtocolError > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from_opt (o)) } unsafe fn ffi_to_const (obj : Result < ferment_example_nested :: model :: snapshot :: LLMQSnapshot , Option < ferment_example :: errors :: protocol_error :: ProtocolError > >) -> * const Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to_opt (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_opt (self . ok) ; ferment_interfaces :: unbox_any_opt (self . error) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: ferment_example :: errors :: protocol_error :: ferment_example_errors_protocol_error_ProtocolError) -> * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_ferment_example_nested_model_snapshot_LLMQSnapshot_err_Option_ferment_example_errors_protocol_error_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Option_Vec_u8 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment_interfaces::FFIConversion<Result<String, Option<Vec<u8>>>>
        for Result_ok_String_err_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_Vec_u8,
        ) -> Result<String, Option<Vec<u8>>> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from_opt(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<String, Option<Vec<u8>>>,
        ) -> *const Result_ok_String_err_Option_Vec_u8 {
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (
                        ferment_interfaces::FFIConversion::ffi_to(o),
                        std::ptr::null_mut(),
                    ),
                    Err(o) => (
                        std::ptr::null_mut(),
                        ferment_interfaces::FFIConversion::ffi_to_opt(o),
                    ),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_opt(self.ok);
                ferment_interfaces::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_Vec_u8_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Result_ok_String_err_Option_Vec_u8 {
        ferment_interfaces::boxed(Result_ok_String_err_Option_Vec_u8 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_Vec_u8_destroy(
        ffi: *mut Result_ok_String_err_Option_Vec_u8,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Option_u8 {
        pub context: *const std::os::raw::c_void,
        caller: fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: fn(result: *mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_u32_RTRN_Option_u8 {
        pub unsafe fn call(&self, o_0: u32) -> Option<[u8; 32]> {
            let ffi_result = (self.caller)(o_0);
            let result =
                <crate::fermented::generics::Arr_u8_32 as ferment_interfaces::FFIConversion<
                    [u8; 32],
                >>::ffi_from_opt(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
}
