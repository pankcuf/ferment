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
    pub mod ferment_example {
        #[doc = "FFI-representation of the [`ferment_example::RootStruct`]"]
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
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_RootStruct {
            ferment_interfaces::boxed(ferment_example_RootStruct { name })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_destroy(
            ffi: *mut ferment_example_RootStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_get_name(
            obj: *const ferment_example_RootStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_set_name(
            obj: *mut ferment_example_RootStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
        }
        pub mod identity {
            pub mod identity {
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::IdentityPublicKey`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_IdentityPublicKey {
                    V0 (* mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityPublicKeyV0) }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::IdentityPublicKey,
                    > for ferment_example_identity_identity_IdentityPublicKey
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_IdentityPublicKey,
                    ) -> ferment_example::identity::identity::IdentityPublicKey
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_identity_identity_IdentityPublicKey::V0(o_0) => {
                                ferment_example::identity::identity::IdentityPublicKey::V0(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::IdentityPublicKey,
                    ) -> *const ferment_example_identity_identity_IdentityPublicKey
                    {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::identity::identity::IdentityPublicKey::V0(o_0) => {
                                ferment_example_identity_identity_IdentityPublicKey::V0(
                                    ferment_interfaces::FFIConversion::ffi_to(o_0),
                                )
                            }
                        })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_identity_identity_IdentityPublicKey,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_IdentityPublicKey {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_identity_identity_IdentityPublicKey::V0(o_0) => {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKey_V0_ctor(
                    o_0 : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityPublicKeyV0,
                ) -> *mut ferment_example_identity_identity_IdentityPublicKey {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_IdentityPublicKey::V0(o_0),
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKey_destroy(
                    ffi: *mut ferment_example_identity_identity_IdentityPublicKey,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn create_basic_identity (id : * mut [u8 ; 32] , _platform_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_PlatformVersion) -> * mut crate :: fermented :: generics :: Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError{
                    let obj = ferment_example::identity::identity::Identity::create_basic_identity(
                        *id,
                        &ferment_interfaces::FFIConversion::ffi_from(_platform_version),
                    );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn create_basic_identity_v0 (id : * mut [u8 ; 32]) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Identity{
                    let obj =
                        ferment_example::identity::identity::Identity::create_basic_identity_v0(
                            *id,
                        );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn get_balance(
                    obj: *mut ferment_example_identity_identity_Identity,
                ) -> u64 {
                    let obj = ferment_example::identity::identity::Identity::get_balance(
                        &ferment_interfaces::FFIConversion::ffi_from(obj),
                    );
                    obj
                }
                #[doc = "FFI-representation of the [`ferment_example::identity::identity::TimestampMillis`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_identity_identity_TimestampMillis(u64);
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::TimestampMillis,
                    > for ferment_example_identity_identity_TimestampMillis
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_TimestampMillis,
                    ) -> ferment_example::identity::identity::TimestampMillis {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::TimestampMillis,
                    ) -> *const ferment_example_identity_identity_TimestampMillis
                    {
                        ferment_interfaces::boxed(
                            ferment_example_identity_identity_TimestampMillis(obj),
                        )
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_TimestampMillis) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_TimestampMillis {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_TimestampMillis_ctor(
                    o_0: u64,
                ) -> *mut ferment_example_identity_identity_TimestampMillis {
                    ferment_interfaces::boxed(ferment_example_identity_identity_TimestampMillis(
                        o_0,
                    ))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_TimestampMillis_destroy(
                    ffi: *mut ferment_example_identity_identity_TimestampMillis,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_TimestampMillis_get_0(
                    obj: *const ferment_example_identity_identity_TimestampMillis,
                ) -> u64 {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_TimestampMillis_set_0(
                    obj: *mut ferment_example_identity_identity_TimestampMillis,
                    value: u64,
                ) {
                    (*obj).0 = value;
                }
                #[doc = "FFI-representation of the [`ferment_example::identity::identity::IdentityV0`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_identity_identity_IdentityV0 { pub id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier , pub public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey , pub balance : u64 , pub revision : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Revision }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::IdentityV0,
                    > for ferment_example_identity_identity_IdentityV0
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_IdentityV0,
                    ) -> ferment_example::identity::identity::IdentityV0 {
                        let ffi_ref = &*ffi;
                        ferment_example::identity::identity::IdentityV0 {
                            id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.id),
                            public_keys: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.public_keys,
                            ),
                            balance: ffi_ref.balance,
                            revision: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.revision),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::IdentityV0,
                    ) -> *const ferment_example_identity_identity_IdentityV0 {
                        ferment_interfaces::boxed(ferment_example_identity_identity_IdentityV0 {
                            id: ferment_interfaces::FFIConversion::ffi_to(obj.id),
                            public_keys: ferment_interfaces::FFIConversion::ffi_to(obj.public_keys),
                            balance: obj.balance,
                            revision: ferment_interfaces::FFIConversion::ffi_to(obj.revision),
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_IdentityV0) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_IdentityV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.id);
                            ferment_interfaces::unbox_any(ffi_ref.public_keys);
                            ferment_interfaces::unbox_any(ffi_ref.revision);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_ctor(
                    id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
                    public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey,
                    balance: u64,
                    revision : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Revision,
                ) -> *mut ferment_example_identity_identity_IdentityV0 {
                    ferment_interfaces::boxed(ferment_example_identity_identity_IdentityV0 {
                        id,
                        public_keys,
                        balance,
                        revision,
                    })
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_destroy(
                    ffi: *mut ferment_example_identity_identity_IdentityV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_get_id (obj : * const ferment_example_identity_identity_IdentityV0) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier{
                    (*obj).id
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_get_public_keys (obj : * const ferment_example_identity_identity_IdentityV0) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey{
                    (*obj).public_keys
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_get_balance(
                    obj: *const ferment_example_identity_identity_IdentityV0,
                ) -> u64 {
                    (*obj).balance
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_get_revision (obj : * const ferment_example_identity_identity_IdentityV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Revision{
                    (*obj).revision
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_set_id(
                    obj: *mut ferment_example_identity_identity_IdentityV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
                ) {
                    (*obj).id = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_set_public_keys(
                    obj: *mut ferment_example_identity_identity_IdentityV0,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey,
                ) {
                    (*obj).public_keys = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_set_balance(
                    obj: *mut ferment_example_identity_identity_IdentityV0,
                    value: u64,
                ) {
                    (*obj).balance = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityV0_set_revision(
                    obj: *mut ferment_example_identity_identity_IdentityV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Revision,
                ) {
                    (*obj).revision = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::ContractBounds`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_ContractBounds {
                    SingleContract { id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier } , SingleContractDocumentType { id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier , document_type_name : * mut std :: os :: raw :: c_char } }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::ContractBounds,
                    > for ferment_example_identity_identity_ContractBounds
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_ContractBounds,
                    ) -> ferment_example::identity::identity::ContractBounds {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_identity_identity_ContractBounds :: SingleContract { id } => ferment_example :: identity :: identity :: ContractBounds :: SingleContract { id : ferment_interfaces :: FFIConversion :: ffi_from (* id) } , ferment_example_identity_identity_ContractBounds :: SingleContractDocumentType { id , document_type_name } => ferment_example :: identity :: identity :: ContractBounds :: SingleContractDocumentType { id : ferment_interfaces :: FFIConversion :: ffi_from (* id) , document_type_name : ferment_interfaces :: FFIConversion :: ffi_from (* document_type_name) } }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::ContractBounds,
                    ) -> *const ferment_example_identity_identity_ContractBounds
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example :: identity :: identity :: ContractBounds :: SingleContract { id } => ferment_example_identity_identity_ContractBounds :: SingleContract { id : ferment_interfaces :: FFIConversion :: ffi_to (id) } , ferment_example :: identity :: identity :: ContractBounds :: SingleContractDocumentType { id , document_type_name } => ferment_example_identity_identity_ContractBounds :: SingleContractDocumentType { id : ferment_interfaces :: FFIConversion :: ffi_to (id) , document_type_name : ferment_interfaces :: FFIConversion :: ffi_to (document_type_name) } })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_ContractBounds) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_ContractBounds {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_identity_identity_ContractBounds :: SingleContract { id } => { ferment_interfaces :: unbox_any (* id) ; } , ferment_example_identity_identity_ContractBounds :: SingleContractDocumentType { id , document_type_name } => { ferment_interfaces :: unbox_any (* id) ; ; < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* document_type_name) } } ;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_ContractBounds_SingleContract_ctor(
                    id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
                ) -> *mut ferment_example_identity_identity_ContractBounds {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_ContractBounds::SingleContract { id },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_ContractBounds_SingleContractDocumentType_ctor(
                    id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
                    document_type_name: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_identity_identity_ContractBounds {
                    ferment_interfaces :: boxed (ferment_example_identity_identity_ContractBounds :: SingleContractDocumentType { id , document_type_name })
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_ContractBounds_destroy(
                    ffi: *mut ferment_example_identity_identity_ContractBounds,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example::identity::identity::IdentityPublicKeyV0`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_identity_identity_IdentityPublicKeyV0 { pub id : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID , pub purpose : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Purpose , pub security_level : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_SecurityLevel , pub contract_bounds : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_ContractBounds , pub key_type : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyType , pub read_only : bool , pub data : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_BinaryData , pub disabled_at : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_TimestampMillis }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::IdentityPublicKeyV0,
                    > for ferment_example_identity_identity_IdentityPublicKeyV0
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_IdentityPublicKeyV0,
                    ) -> ferment_example::identity::identity::IdentityPublicKeyV0
                    {
                        let ffi_ref = &*ffi;
                        ferment_example::identity::identity::IdentityPublicKeyV0 {
                            id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.id),
                            purpose: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.purpose),
                            security_level: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.security_level,
                            ),
                            contract_bounds: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.contract_bounds,
                            ),
                            key_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.key_type),
                            read_only: ffi_ref.read_only,
                            data: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.data),
                            disabled_at: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.disabled_at,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::IdentityPublicKeyV0,
                    ) -> *const ferment_example_identity_identity_IdentityPublicKeyV0
                    {
                        ferment_interfaces::boxed(
                            ferment_example_identity_identity_IdentityPublicKeyV0 {
                                id: ferment_interfaces::FFIConversion::ffi_to(obj.id),
                                purpose: ferment_interfaces::FFIConversion::ffi_to(obj.purpose),
                                security_level: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.security_level,
                                ),
                                contract_bounds: ferment_interfaces::FFIConversion::ffi_to_opt(
                                    obj.contract_bounds,
                                ),
                                key_type: ferment_interfaces::FFIConversion::ffi_to(obj.key_type),
                                read_only: obj.read_only,
                                data: ferment_interfaces::FFIConversion::ffi_to(obj.data),
                                disabled_at: ferment_interfaces::FFIConversion::ffi_to_opt(
                                    obj.disabled_at,
                                ),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_IdentityPublicKeyV0 {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.id);
                            ferment_interfaces::unbox_any(ffi_ref.purpose);
                            ferment_interfaces::unbox_any(ffi_ref.security_level);
                            if !ffi_ref.contract_bounds.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.contract_bounds);
                            };
                            ferment_interfaces::unbox_any(ffi_ref.key_type);
                            ferment_interfaces::unbox_any(ffi_ref.data);
                            if !ffi_ref.disabled_at.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.disabled_at);
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_ctor(
                    id : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID,
                    purpose : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Purpose,
                    security_level : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_SecurityLevel,
                    contract_bounds : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_ContractBounds,
                    key_type : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyType,
                    read_only: bool,
                    data : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_BinaryData,
                    disabled_at : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_TimestampMillis,
                ) -> *mut ferment_example_identity_identity_IdentityPublicKeyV0 {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_IdentityPublicKeyV0 {
                            id,
                            purpose,
                            security_level,
                            contract_bounds,
                            key_type,
                            read_only,
                            data,
                            disabled_at,
                        },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_destroy(
                    ffi: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_id (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID{
                    (*obj).id
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_purpose (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Purpose{
                    (*obj).purpose
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_security_level (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_SecurityLevel{
                    (*obj).security_level
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_contract_bounds (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_ContractBounds{
                    (*obj).contract_bounds
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_key_type (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyType{
                    (*obj).key_type
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_read_only(
                    obj: *const ferment_example_identity_identity_IdentityPublicKeyV0,
                ) -> bool {
                    (*obj).read_only
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_data (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_BinaryData{
                    (*obj).data
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_get_disabled_at (obj : * const ferment_example_identity_identity_IdentityPublicKeyV0) -> * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_TimestampMillis{
                    (*obj).disabled_at
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_id(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID,
                ) {
                    (*obj).id = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_purpose(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Purpose,
                ) {
                    (*obj).purpose = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_security_level(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_SecurityLevel,
                ) {
                    (*obj).security_level = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_contract_bounds(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_ContractBounds,
                ) {
                    (*obj).contract_bounds = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_key_type(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyType,
                ) {
                    (*obj).key_type = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_read_only(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value: bool,
                ) {
                    (*obj).read_only = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_data(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_BinaryData,
                ) {
                    (*obj).data = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_IdentityPublicKeyV0_set_disabled_at(
                    obj: *mut ferment_example_identity_identity_IdentityPublicKeyV0,
                    value : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_TimestampMillis,
                ) {
                    (*obj).disabled_at = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::KeyType`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_KeyType {
                    ECDSA_SECP256K1 = 0,
                    BLS12_381 = 1,
                    ECDSA_HASH160 = 2,
                    BIP13_SCRIPT_HASH = 3,
                    EDDSA_25519_HASH160 = 4,
                }
                impl ferment_interfaces::FFIConversion<ferment_example::identity::identity::KeyType>
                    for ferment_example_identity_identity_KeyType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_KeyType,
                    ) -> ferment_example::identity::identity::KeyType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_identity_identity_KeyType::ECDSA_SECP256K1 => {
                                ferment_example::identity::identity::KeyType::ECDSA_SECP256K1
                            }
                            ferment_example_identity_identity_KeyType::BLS12_381 => {
                                ferment_example::identity::identity::KeyType::BLS12_381
                            }
                            ferment_example_identity_identity_KeyType::ECDSA_HASH160 => {
                                ferment_example::identity::identity::KeyType::ECDSA_HASH160
                            }
                            ferment_example_identity_identity_KeyType::BIP13_SCRIPT_HASH => {
                                ferment_example::identity::identity::KeyType::BIP13_SCRIPT_HASH
                            }
                            ferment_example_identity_identity_KeyType::EDDSA_25519_HASH160 => {
                                ferment_example::identity::identity::KeyType::EDDSA_25519_HASH160
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::KeyType,
                    ) -> *const ferment_example_identity_identity_KeyType {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::identity::identity::KeyType::ECDSA_SECP256K1 => {
                                ferment_example_identity_identity_KeyType::ECDSA_SECP256K1
                            }
                            ferment_example::identity::identity::KeyType::BLS12_381 => {
                                ferment_example_identity_identity_KeyType::BLS12_381
                            }
                            ferment_example::identity::identity::KeyType::ECDSA_HASH160 => {
                                ferment_example_identity_identity_KeyType::ECDSA_HASH160
                            }
                            ferment_example::identity::identity::KeyType::BIP13_SCRIPT_HASH => {
                                ferment_example_identity_identity_KeyType::BIP13_SCRIPT_HASH
                            }
                            ferment_example::identity::identity::KeyType::EDDSA_25519_HASH160 => {
                                ferment_example_identity_identity_KeyType::EDDSA_25519_HASH160
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_KeyType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_KeyType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_identity_identity_KeyType::ECDSA_SECP256K1 => {}
                                ferment_example_identity_identity_KeyType::BLS12_381 => {}
                                ferment_example_identity_identity_KeyType::ECDSA_HASH160 => {}
                                ferment_example_identity_identity_KeyType::BIP13_SCRIPT_HASH => {}
                                ferment_example_identity_identity_KeyType::EDDSA_25519_HASH160 => {}
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_ECDSA_SECP256K1_ctor(
                ) -> *mut ferment_example_identity_identity_KeyType {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_KeyType::ECDSA_SECP256K1,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_BLS12_381_ctor(
                ) -> *mut ferment_example_identity_identity_KeyType {
                    ferment_interfaces::boxed(ferment_example_identity_identity_KeyType::BLS12_381)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_ECDSA_HASH160_ctor(
                ) -> *mut ferment_example_identity_identity_KeyType {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_KeyType::ECDSA_HASH160,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_BIP13_SCRIPT_HASH_ctor(
                ) -> *mut ferment_example_identity_identity_KeyType {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_KeyType::BIP13_SCRIPT_HASH,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_EDDSA_25519_HASH160_ctor(
                ) -> *mut ferment_example_identity_identity_KeyType {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_KeyType::EDDSA_25519_HASH160,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyType_destroy(
                    ffi: *mut ferment_example_identity_identity_KeyType,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example::identity::identity::Revision`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_identity_identity_Revision(u64);
                impl
                    ferment_interfaces::FFIConversion<ferment_example::identity::identity::Revision>
                    for ferment_example_identity_identity_Revision
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_Revision,
                    ) -> ferment_example::identity::identity::Revision {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::Revision,
                    ) -> *const ferment_example_identity_identity_Revision {
                        ferment_interfaces::boxed(ferment_example_identity_identity_Revision(obj))
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_Revision) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_Revision {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Revision_ctor(
                    o_0: u64,
                ) -> *mut ferment_example_identity_identity_Revision {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Revision(o_0))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Revision_destroy(
                    ffi: *mut ferment_example_identity_identity_Revision,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Revision_get_0(
                    obj: *const ferment_example_identity_identity_Revision,
                ) -> u64 {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Revision_set_0(
                    obj: *mut ferment_example_identity_identity_Revision,
                    value: u64,
                ) {
                    (*obj).0 = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::Purpose`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_Purpose {
                    AUTHENTICATION = 0,
                    ENCRYPTION = 1,
                    DECRYPTION = 2,
                    WITHDRAW = 3,
                    SYSTEM = 4,
                    VOTING = 5,
                }
                impl ferment_interfaces::FFIConversion<ferment_example::identity::identity::Purpose>
                    for ferment_example_identity_identity_Purpose
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_Purpose,
                    ) -> ferment_example::identity::identity::Purpose {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_identity_identity_Purpose::AUTHENTICATION => {
                                ferment_example::identity::identity::Purpose::AUTHENTICATION
                            }
                            ferment_example_identity_identity_Purpose::ENCRYPTION => {
                                ferment_example::identity::identity::Purpose::ENCRYPTION
                            }
                            ferment_example_identity_identity_Purpose::DECRYPTION => {
                                ferment_example::identity::identity::Purpose::DECRYPTION
                            }
                            ferment_example_identity_identity_Purpose::WITHDRAW => {
                                ferment_example::identity::identity::Purpose::WITHDRAW
                            }
                            ferment_example_identity_identity_Purpose::SYSTEM => {
                                ferment_example::identity::identity::Purpose::SYSTEM
                            }
                            ferment_example_identity_identity_Purpose::VOTING => {
                                ferment_example::identity::identity::Purpose::VOTING
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::Purpose,
                    ) -> *const ferment_example_identity_identity_Purpose {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::identity::identity::Purpose::AUTHENTICATION => {
                                ferment_example_identity_identity_Purpose::AUTHENTICATION
                            }
                            ferment_example::identity::identity::Purpose::ENCRYPTION => {
                                ferment_example_identity_identity_Purpose::ENCRYPTION
                            }
                            ferment_example::identity::identity::Purpose::DECRYPTION => {
                                ferment_example_identity_identity_Purpose::DECRYPTION
                            }
                            ferment_example::identity::identity::Purpose::WITHDRAW => {
                                ferment_example_identity_identity_Purpose::WITHDRAW
                            }
                            ferment_example::identity::identity::Purpose::SYSTEM => {
                                ferment_example_identity_identity_Purpose::SYSTEM
                            }
                            ferment_example::identity::identity::Purpose::VOTING => {
                                ferment_example_identity_identity_Purpose::VOTING
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_Purpose) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_Purpose {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_identity_identity_Purpose::AUTHENTICATION => {}
                                ferment_example_identity_identity_Purpose::ENCRYPTION => {}
                                ferment_example_identity_identity_Purpose::DECRYPTION => {}
                                ferment_example_identity_identity_Purpose::WITHDRAW => {}
                                ferment_example_identity_identity_Purpose::SYSTEM => {}
                                ferment_example_identity_identity_Purpose::VOTING => {}
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_AUTHENTICATION_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_Purpose::AUTHENTICATION,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_ENCRYPTION_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Purpose::ENCRYPTION)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_DECRYPTION_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Purpose::DECRYPTION)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_WITHDRAW_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Purpose::WITHDRAW)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_SYSTEM_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Purpose::SYSTEM)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_VOTING_ctor(
                ) -> *mut ferment_example_identity_identity_Purpose {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Purpose::VOTING)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Purpose_destroy(
                    ffi: *mut ferment_example_identity_identity_Purpose,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::Identity`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_Identity {
                    V0 (* mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityV0) }
                impl
                    ferment_interfaces::FFIConversion<ferment_example::identity::identity::Identity>
                    for ferment_example_identity_identity_Identity
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_Identity,
                    ) -> ferment_example::identity::identity::Identity {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_identity_identity_Identity::V0(o_0) => {
                                ferment_example::identity::identity::Identity::V0(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::Identity,
                    ) -> *const ferment_example_identity_identity_Identity {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::identity::identity::Identity::V0(o_0) => {
                                ferment_example_identity_identity_Identity::V0(
                                    ferment_interfaces::FFIConversion::ffi_to(o_0),
                                )
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_Identity) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_Identity {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_identity_identity_Identity::V0(o_0) => {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Identity_V0_ctor(
                    o_0 : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityV0,
                ) -> *mut ferment_example_identity_identity_Identity {
                    ferment_interfaces::boxed(ferment_example_identity_identity_Identity::V0(o_0))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_Identity_destroy(
                    ffi: *mut ferment_example_identity_identity_Identity,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example::identity::identity::KeyID`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_identity_identity_KeyID(u32);
                impl ferment_interfaces::FFIConversion<ferment_example::identity::identity::KeyID>
                    for ferment_example_identity_identity_KeyID
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_KeyID,
                    ) -> ferment_example::identity::identity::KeyID {
                        let ffi_ref = &*ffi;
                        ffi_ref.0
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::KeyID,
                    ) -> *const ferment_example_identity_identity_KeyID {
                        ferment_interfaces::boxed(ferment_example_identity_identity_KeyID(obj))
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_KeyID) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_KeyID {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyID_ctor(
                    o_0: u32,
                ) -> *mut ferment_example_identity_identity_KeyID {
                    ferment_interfaces::boxed(ferment_example_identity_identity_KeyID(o_0))
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyID_destroy(
                    ffi: *mut ferment_example_identity_identity_KeyID,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyID_get_0(
                    obj: *const ferment_example_identity_identity_KeyID,
                ) -> u32 {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_KeyID_set_0(
                    obj: *mut ferment_example_identity_identity_KeyID,
                    value: u32,
                ) {
                    (*obj).0 = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::identity::identity::SecurityLevel`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_identity_identity_SecurityLevel {
                    MASTER = 0,
                    CRITICAL = 1,
                    HIGH = 2,
                    MEDIUM = 3,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::identity::identity::SecurityLevel,
                    > for ferment_example_identity_identity_SecurityLevel
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_identity_identity_SecurityLevel,
                    ) -> ferment_example::identity::identity::SecurityLevel {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_identity_identity_SecurityLevel::MASTER => {
                                ferment_example::identity::identity::SecurityLevel::MASTER
                            }
                            ferment_example_identity_identity_SecurityLevel::CRITICAL => {
                                ferment_example::identity::identity::SecurityLevel::CRITICAL
                            }
                            ferment_example_identity_identity_SecurityLevel::HIGH => {
                                ferment_example::identity::identity::SecurityLevel::HIGH
                            }
                            ferment_example_identity_identity_SecurityLevel::MEDIUM => {
                                ferment_example::identity::identity::SecurityLevel::MEDIUM
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::identity::identity::SecurityLevel,
                    ) -> *const ferment_example_identity_identity_SecurityLevel
                    {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::identity::identity::SecurityLevel::MASTER => {
                                ferment_example_identity_identity_SecurityLevel::MASTER
                            }
                            ferment_example::identity::identity::SecurityLevel::CRITICAL => {
                                ferment_example_identity_identity_SecurityLevel::CRITICAL
                            }
                            ferment_example::identity::identity::SecurityLevel::HIGH => {
                                ferment_example_identity_identity_SecurityLevel::HIGH
                            }
                            ferment_example::identity::identity::SecurityLevel::MEDIUM => {
                                ferment_example_identity_identity_SecurityLevel::MEDIUM
                            }
                        })
                    }
                    unsafe fn destroy(ffi: *mut ferment_example_identity_identity_SecurityLevel) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_identity_identity_SecurityLevel {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_identity_identity_SecurityLevel::MASTER => {}
                                ferment_example_identity_identity_SecurityLevel::CRITICAL => {}
                                ferment_example_identity_identity_SecurityLevel::HIGH => {}
                                ferment_example_identity_identity_SecurityLevel::MEDIUM => {}
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_SecurityLevel_MASTER_ctor(
                ) -> *mut ferment_example_identity_identity_SecurityLevel {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_SecurityLevel::MASTER,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_SecurityLevel_CRITICAL_ctor(
                ) -> *mut ferment_example_identity_identity_SecurityLevel {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_SecurityLevel::CRITICAL,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_SecurityLevel_HIGH_ctor(
                ) -> *mut ferment_example_identity_identity_SecurityLevel {
                    ferment_interfaces::boxed(ferment_example_identity_identity_SecurityLevel::HIGH)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_SecurityLevel_MEDIUM_ctor(
                ) -> *mut ferment_example_identity_identity_SecurityLevel {
                    ferment_interfaces::boxed(
                        ferment_example_identity_identity_SecurityLevel::MEDIUM,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_identity_identity_SecurityLevel_destroy(
                    ffi: *mut ferment_example_identity_identity_SecurityLevel,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`create_platform_v0`]"]
                #[doc = r" # Safety"]
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn create_platform_v0 (identity : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds , proofs : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_PlatformVersion{
                    let obj = ferment_example::identity::identity::create_platform_v0(
                        ferment_interfaces::FFIConversion::ffi_from(identity),
                        ferment_interfaces::FFIConversion::ffi_from(proofs),
                    );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
        }
        pub mod nested {
            #[doc = "FFI-representation of the [`ferment_example::nested::IdentifierBytes32`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_IdentifierBytes32(*mut [u8; 32]);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::IdentifierBytes32>
                for ferment_example_nested_IdentifierBytes32
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_IdentifierBytes32,
                ) -> ferment_example::nested::IdentifierBytes32 {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::IdentifierBytes32(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::IdentifierBytes32,
                ) -> *const ferment_example_nested_IdentifierBytes32 {
                    ferment_interfaces::boxed(ferment_example_nested_IdentifierBytes32(
                        ferment_interfaces::boxed(obj.0),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_IdentifierBytes32) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_IdentifierBytes32 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_IdentifierBytes32_ctor(
                o_0: *mut [u8; 32],
            ) -> *mut ferment_example_nested_IdentifierBytes32 {
                ferment_interfaces::boxed(ferment_example_nested_IdentifierBytes32(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_IdentifierBytes32_destroy(
                ffi: *mut ferment_example_nested_IdentifierBytes32,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_IdentifierBytes32_get_0(
                obj: *const ferment_example_nested_IdentifierBytes32,
            ) -> *mut [u8; 32] {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_IdentifierBytes32_set_0(
                obj: *mut ferment_example_nested_IdentifierBytes32,
                value: *mut [u8; 32],
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::FeatureVersionBounds`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_FeatureVersionBounds { pub min_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion , pub max_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion , pub default_current_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion }
            impl ferment_interfaces::FFIConversion<ferment_example::nested::FeatureVersionBounds>
                for ferment_example_nested_FeatureVersionBounds
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_FeatureVersionBounds,
                ) -> ferment_example::nested::FeatureVersionBounds {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::FeatureVersionBounds {
                        min_version: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.min_version,
                        ),
                        max_version: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.max_version,
                        ),
                        default_current_version: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.default_current_version,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::FeatureVersionBounds,
                ) -> *const ferment_example_nested_FeatureVersionBounds {
                    ferment_interfaces::boxed(ferment_example_nested_FeatureVersionBounds {
                        min_version: ferment_interfaces::FFIConversion::ffi_to(obj.min_version),
                        max_version: ferment_interfaces::FFIConversion::ffi_to(obj.max_version),
                        default_current_version: ferment_interfaces::FFIConversion::ffi_to(
                            obj.default_current_version,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_FeatureVersionBounds) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_FeatureVersionBounds {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.min_version);
                        ferment_interfaces::unbox_any(ffi_ref.max_version);
                        ferment_interfaces::unbox_any(ffi_ref.default_current_version);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_ctor(
                min_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
                max_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
                default_current_version : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
            ) -> *mut ferment_example_nested_FeatureVersionBounds {
                ferment_interfaces::boxed(ferment_example_nested_FeatureVersionBounds {
                    min_version,
                    max_version,
                    default_current_version,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_destroy(
                ffi: *mut ferment_example_nested_FeatureVersionBounds,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_get_min_version (obj : * const ferment_example_nested_FeatureVersionBounds) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion{
                (*obj).min_version
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_get_max_version (obj : * const ferment_example_nested_FeatureVersionBounds) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion{
                (*obj).max_version
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_get_default_current_version (obj : * const ferment_example_nested_FeatureVersionBounds) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion{
                (*obj).default_current_version
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_set_min_version(
                obj: *mut ferment_example_nested_FeatureVersionBounds,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
            ) {
                (*obj).min_version = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_set_max_version(
                obj: *mut ferment_example_nested_FeatureVersionBounds,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
            ) {
                (*obj).max_version = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersionBounds_set_default_current_version(
                obj: *mut ferment_example_nested_FeatureVersionBounds,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
            ) {
                (*obj).default_current_version = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::BinaryData`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_BinaryData(*mut crate::fermented::generics::Vec_u8);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::BinaryData>
                for ferment_example_nested_BinaryData
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_BinaryData,
                ) -> ferment_example::nested::BinaryData {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::BinaryData(
                        ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0),
                    )
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::BinaryData,
                ) -> *const ferment_example_nested_BinaryData {
                    ferment_interfaces::boxed(ferment_example_nested_BinaryData(
                        ferment_interfaces::FFIConversion::ffi_to(obj.0),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_BinaryData) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_BinaryData {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_BinaryData_ctor(
                o_0: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut ferment_example_nested_BinaryData {
                ferment_interfaces::boxed(ferment_example_nested_BinaryData(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_BinaryData_destroy(
                ffi: *mut ferment_example_nested_BinaryData,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_BinaryData_get_0(
                obj: *const ferment_example_nested_BinaryData,
            ) -> *mut crate::fermented::generics::Vec_u8 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_BinaryData_set_0(
                obj: *mut ferment_example_nested_BinaryData,
                value: *mut crate::fermented::generics::Vec_u8,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::DataContractNotPresentError`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_DataContractNotPresentError { pub data_contract_id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example::nested::DataContractNotPresentError,
                > for ferment_example_nested_DataContractNotPresentError
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_DataContractNotPresentError,
                ) -> ferment_example::nested::DataContractNotPresentError {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::DataContractNotPresentError {
                        data_contract_id: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.data_contract_id,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::DataContractNotPresentError,
                ) -> *const ferment_example_nested_DataContractNotPresentError {
                    ferment_interfaces::boxed(ferment_example_nested_DataContractNotPresentError {
                        data_contract_id: ferment_interfaces::FFIConversion::ffi_to(
                            obj.data_contract_id,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_DataContractNotPresentError) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_DataContractNotPresentError {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.data_contract_id);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_DataContractNotPresentError_ctor(
                data_contract_id : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
            ) -> *mut ferment_example_nested_DataContractNotPresentError {
                ferment_interfaces::boxed(ferment_example_nested_DataContractNotPresentError {
                    data_contract_id,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_DataContractNotPresentError_destroy(
                ffi: *mut ferment_example_nested_DataContractNotPresentError,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_DataContractNotPresentError_get_data_contract_id (obj : * const ferment_example_nested_DataContractNotPresentError) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier{
                (*obj).data_contract_id
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_DataContractNotPresentError_set_data_contract_id(
                obj: *mut ferment_example_nested_DataContractNotPresentError,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_Identifier,
            ) {
                (*obj).data_contract_id = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::OptionalFeatureVersion`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_OptionalFeatureVersion(u16);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::OptionalFeatureVersion>
                for ferment_example_nested_OptionalFeatureVersion
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_OptionalFeatureVersion,
                ) -> ferment_example::nested::OptionalFeatureVersion {
                    let ffi_ref = &*ffi;
                    (ffi_ref.0 > 0).then(|| ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::OptionalFeatureVersion,
                ) -> *const ferment_example_nested_OptionalFeatureVersion {
                    ferment_interfaces::boxed(ferment_example_nested_OptionalFeatureVersion(
                        obj.unwrap_or(0),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_OptionalFeatureVersion) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_OptionalFeatureVersion {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_OptionalFeatureVersion_ctor(
                o_0: u16,
            ) -> *mut ferment_example_nested_OptionalFeatureVersion {
                ferment_interfaces::boxed(ferment_example_nested_OptionalFeatureVersion(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_OptionalFeatureVersion_destroy(
                ffi: *mut ferment_example_nested_OptionalFeatureVersion,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_OptionalFeatureVersion_get_0(
                obj: *const ferment_example_nested_OptionalFeatureVersion,
            ) -> u16 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_OptionalFeatureVersion_set_0(
                obj: *mut ferment_example_nested_OptionalFeatureVersion,
                value: u16,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::Identifier`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_Identifier (* mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_IdentifierBytes32) ;
            impl ferment_interfaces::FFIConversion<ferment_example::nested::Identifier>
                for ferment_example_nested_Identifier
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_Identifier,
                ) -> ferment_example::nested::Identifier {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::Identifier(
                        ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0),
                    )
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::Identifier,
                ) -> *const ferment_example_nested_Identifier {
                    ferment_interfaces::boxed(ferment_example_nested_Identifier(
                        ferment_interfaces::FFIConversion::ffi_to(obj.0),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_Identifier) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_Identifier {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_Identifier_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_IdentifierBytes32,
            ) -> *mut ferment_example_nested_Identifier {
                ferment_interfaces::boxed(ferment_example_nested_Identifier(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_Identifier_destroy(
                ffi: *mut ferment_example_nested_Identifier,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_Identifier_get_0 (obj : * const ferment_example_nested_Identifier) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_IdentifierBytes32{
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_Identifier_set_0(
                obj: *mut ferment_example_nested_Identifier,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_IdentifierBytes32,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::PlatformVersion`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_PlatformVersion { pub protocol_version : u32 , pub identity : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds , pub proofs : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds }
            impl ferment_interfaces::FFIConversion<ferment_example::nested::PlatformVersion>
                for ferment_example_nested_PlatformVersion
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_PlatformVersion,
                ) -> ferment_example::nested::PlatformVersion {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::PlatformVersion {
                        protocol_version: ffi_ref.protocol_version,
                        identity: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.identity),
                        proofs: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.proofs),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::PlatformVersion,
                ) -> *const ferment_example_nested_PlatformVersion {
                    ferment_interfaces::boxed(ferment_example_nested_PlatformVersion {
                        protocol_version: obj.protocol_version,
                        identity: ferment_interfaces::FFIConversion::ffi_to(obj.identity),
                        proofs: ferment_interfaces::FFIConversion::ffi_to(obj.proofs),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_PlatformVersion) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_PlatformVersion {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.identity);
                        ferment_interfaces::unbox_any(ffi_ref.proofs);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_ctor(
                protocol_version: u32,
                identity : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds,
                proofs : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds,
            ) -> *mut ferment_example_nested_PlatformVersion {
                ferment_interfaces::boxed(ferment_example_nested_PlatformVersion {
                    protocol_version,
                    identity,
                    proofs,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_destroy(
                ffi: *mut ferment_example_nested_PlatformVersion,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_get_protocol_version(
                obj: *const ferment_example_nested_PlatformVersion,
            ) -> u32 {
                (*obj).protocol_version
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_get_identity (obj : * const ferment_example_nested_PlatformVersion) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds{
                (*obj).identity
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_get_proofs (obj : * const ferment_example_nested_PlatformVersion) -> * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds{
                (*obj).proofs
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_set_protocol_version(
                obj: *mut ferment_example_nested_PlatformVersion,
                value: u32,
            ) {
                (*obj).protocol_version = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_set_identity(
                obj: *mut ferment_example_nested_PlatformVersion,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds,
            ) {
                (*obj).identity = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_PlatformVersion_set_proofs(
                obj: *mut ferment_example_nested_PlatformVersion,
                value : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersionBounds,
            ) {
                (*obj).proofs = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::FeatureVersion`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_FeatureVersion(u16);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::FeatureVersion>
                for ferment_example_nested_FeatureVersion
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_FeatureVersion,
                ) -> ferment_example::nested::FeatureVersion {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::FeatureVersion,
                ) -> *const ferment_example_nested_FeatureVersion {
                    ferment_interfaces::boxed(ferment_example_nested_FeatureVersion(obj))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_FeatureVersion) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_FeatureVersion {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersion_ctor(
                o_0: u16,
            ) -> *mut ferment_example_nested_FeatureVersion {
                ferment_interfaces::boxed(ferment_example_nested_FeatureVersion(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersion_destroy(
                ffi: *mut ferment_example_nested_FeatureVersion,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersion_get_0(
                obj: *const ferment_example_nested_FeatureVersion,
            ) -> u16 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_FeatureVersion_set_0(
                obj: *mut ferment_example_nested_FeatureVersion,
                value: u16,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example::nested::ProtocolError`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_nested_ProtocolError {
                IdentifierError (* mut std :: os :: raw :: c_char) , StringDecodeError (* mut std :: os :: raw :: c_char) , StringDecodeError2 (* mut std :: os :: raw :: c_char , u32) , EmptyPublicKeyDataError , MaxEncodedBytesReachedError { max_size_kbytes : usize , size_hit : usize } , EncodingError (* mut std :: os :: raw :: c_char) , EncodingError2 (* mut std :: os :: raw :: c_char) , DataContractNotPresentError (* mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_DataContractNotPresentError) , UnknownVersionMismatch { method : * mut std :: os :: raw :: c_char , known_versions : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_FeatureVersion , received : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion } }
            impl ferment_interfaces::FFIConversion<ferment_example::nested::ProtocolError>
                for ferment_example_nested_ProtocolError
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_ProtocolError,
                ) -> ferment_example::nested::ProtocolError {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_ProtocolError::IdentifierError(o_0) => {
                            ferment_example::nested::ProtocolError::IdentifierError(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_nested_ProtocolError::StringDecodeError(o_0) => {
                            ferment_example::nested::ProtocolError::StringDecodeError(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_nested_ProtocolError::StringDecodeError2(o_0, o_1) => {
                            ferment_example::nested::ProtocolError::StringDecodeError2(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                *o_1,
                            )
                        }
                        ferment_example_nested_ProtocolError::EmptyPublicKeyDataError => {
                            ferment_example::nested::ProtocolError::EmptyPublicKeyDataError
                        }
                        ferment_example_nested_ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => ferment_example::nested::ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes: *max_size_kbytes,
                            size_hit: *size_hit,
                        },
                        ferment_example_nested_ProtocolError::EncodingError(o_0) => {
                            ferment_example::nested::ProtocolError::EncodingError(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_nested_ProtocolError::EncodingError2(o_0) => {
                            ferment_example::nested::ProtocolError::EncodingError2(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_nested_ProtocolError::DataContractNotPresentError(o_0) => {
                            ferment_example::nested::ProtocolError::DataContractNotPresentError(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_nested_ProtocolError::UnknownVersionMismatch {
                            method,
                            known_versions,
                            received,
                        } => ferment_example::nested::ProtocolError::UnknownVersionMismatch {
                            method: ferment_interfaces::FFIConversion::ffi_from(*method),
                            known_versions: ferment_interfaces::FFIConversion::ffi_from(
                                *known_versions,
                            ),
                            received: ferment_interfaces::FFIConversion::ffi_from(*received),
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::ProtocolError,
                ) -> *const ferment_example_nested_ProtocolError {
                    ferment_interfaces::boxed(match obj {
                        ferment_example::nested::ProtocolError::IdentifierError(o_0) => {
                            ferment_example_nested_ProtocolError::IdentifierError(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::nested::ProtocolError::StringDecodeError(o_0) => {
                            ferment_example_nested_ProtocolError::StringDecodeError(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::nested::ProtocolError::StringDecodeError2(o_0, o_1) => {
                            ferment_example_nested_ProtocolError::StringDecodeError2(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                                o_1,
                            )
                        }
                        ferment_example::nested::ProtocolError::EmptyPublicKeyDataError => {
                            ferment_example_nested_ProtocolError::EmptyPublicKeyDataError
                        }
                        ferment_example::nested::ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => ferment_example_nested_ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes: max_size_kbytes,
                            size_hit: size_hit,
                        },
                        ferment_example::nested::ProtocolError::EncodingError(o_0) => {
                            ferment_example_nested_ProtocolError::EncodingError(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::nested::ProtocolError::EncodingError2(o_0) => {
                            ferment_example_nested_ProtocolError::EncodingError2(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::nested::ProtocolError::DataContractNotPresentError(
                            o_0,
                        ) => ferment_example_nested_ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                        ),
                        ferment_example::nested::ProtocolError::UnknownVersionMismatch {
                            method,
                            known_versions,
                            received,
                        } => ferment_example_nested_ProtocolError::UnknownVersionMismatch {
                            method: ferment_interfaces::FFIConversion::ffi_to(method),
                            known_versions: ferment_interfaces::FFIConversion::ffi_to(
                                known_versions,
                            ),
                            received: ferment_interfaces::FFIConversion::ffi_to(received),
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_ProtocolError) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_ProtocolError {
                fn drop(&mut self) {
                    unsafe {
                        match self { ferment_example_nested_ProtocolError :: IdentifierError (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_nested_ProtocolError :: StringDecodeError (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_nested_ProtocolError :: StringDecodeError2 (o_0 , o_1) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ; } , ferment_example_nested_ProtocolError :: EmptyPublicKeyDataError => { } , ferment_example_nested_ProtocolError :: MaxEncodedBytesReachedError { max_size_kbytes , size_hit } => { ; } , ferment_example_nested_ProtocolError :: EncodingError (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) } , ferment_example_nested_ProtocolError :: EncodingError2 (o_0) => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (* o_0) } , ferment_example_nested_ProtocolError :: DataContractNotPresentError (o_0) => { ferment_interfaces :: unbox_any (* o_0) ; } , ferment_example_nested_ProtocolError :: UnknownVersionMismatch { method , known_versions , received } => { < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* method) ; ferment_interfaces :: unbox_any (* known_versions) ; ; ferment_interfaces :: unbox_any (* received) ; } } ;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_IdentifierError_ctor(
                o_0: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_nested_ProtocolError::IdentifierError(
                    o_0,
                ))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_StringDecodeError_ctor(
                o_0: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_nested_ProtocolError::StringDecodeError(
                    o_0,
                ))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_StringDecodeError2_ctor(
                o_0: *mut std::os::raw::c_char,
                o_1: u32,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_nested_ProtocolError::StringDecodeError2(
                    o_0, o_1,
                ))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_EmptyPublicKeyDataError_ctor(
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(
                    ferment_example_nested_ProtocolError::EmptyPublicKeyDataError,
                )
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_MaxEncodedBytesReachedError_ctor(
                max_size_kbytes: usize,
                size_hit: usize,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(
                    ferment_example_nested_ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    },
                )
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_EncodingError_ctor(
                o_0: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_nested_ProtocolError::EncodingError(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_EncodingError2_ctor(
                o_0: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_nested_ProtocolError::EncodingError2(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_DataContractNotPresentError_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_DataContractNotPresentError,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(
                    ferment_example_nested_ProtocolError::DataContractNotPresentError(o_0),
                )
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_UnknownVersionMismatch_ctor(
                method: *mut std::os::raw::c_char,
                known_versions : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_FeatureVersion,
                received : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
            ) -> *mut ferment_example_nested_ProtocolError {
                ferment_interfaces::boxed(
                    ferment_example_nested_ProtocolError::UnknownVersionMismatch {
                        method,
                        known_versions,
                        received,
                    },
                )
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_ProtocolError_destroy(
                ffi: *mut ferment_example_nested_ProtocolError,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::HashID`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_HashID(*mut [u8; 32]);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::HashID>
                for ferment_example_nested_HashID
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_HashID,
                ) -> ferment_example::nested::HashID {
                    let ffi_ref = &*ffi;
                    *ffi_ref.0
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::HashID,
                ) -> *const ferment_example_nested_HashID {
                    ferment_interfaces::boxed(ferment_example_nested_HashID(
                        ferment_interfaces::boxed(obj),
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
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_ctor(
                o_0: *mut [u8; 32],
            ) -> *mut ferment_example_nested_HashID {
                ferment_interfaces::boxed(ferment_example_nested_HashID(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_destroy(
                ffi: *mut ferment_example_nested_HashID,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_get_0(
                obj: *const ferment_example_nested_HashID,
            ) -> *mut [u8; 32] {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_set_0(
                obj: *mut ferment_example_nested_HashID,
                value: *mut [u8; 32],
            ) {
                (*obj).0 = value;
            }
        }
    }
    pub mod ferment_example_nested {
        use crate as ferment_example_nested;
        pub mod some_package {
            use crate as ferment_example_nested;
            #[doc = "FFI-representation of the [`get_hash_id_form_snapshot`]"]
            #[doc = r" # Safety"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn get_hash_id_form_snapshot(
                _snapshot : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
            ) -> *mut crate::fermented::types::ferment_example::nested::ferment_example_nested_HashID
            {
                let obj = ferment_example_nested::some_package::get_hash_id_form_snapshot(
                    ferment_interfaces::FFIConversion::ffi_from(_snapshot),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
        #[doc = "FFI-representation of the [`ferment_example_nested::SomeStruct`]"]
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
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_nested_SomeStruct {
            ferment_interfaces::boxed(ferment_example_nested_SomeStruct { name })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_destroy(
            ffi: *mut ferment_example_nested_SomeStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_get_name(
            obj: *const ferment_example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_set_name(
            obj: *mut ferment_example_nested_SomeStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
        }
        pub mod model {
            use crate as ferment_example_nested;
            pub mod snapshot {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested::model::snapshot::LLMQSnapshot`]"]
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
                            if !ffi_ref.option_vec.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.option_vec);
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
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
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_member_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list_mode (obj : * const ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_option_vec(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_member_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).member_list = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_i32,
                ) {
                    (*obj).skip_list = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list_mode(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    (*obj).skip_list_mode = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_option_vec(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).option_vec = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
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
                        ferment_interfaces :: boxed (match obj { ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll })
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
                            match self { ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => { } } ;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_NoSkipping_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipFirst_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipExcept_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipAll_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
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
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
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
    pub unsafe extern "C" fn Vec_u8_ctor(values: *mut u8, count: usize) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError { pub ok : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Identity , pub error : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < ferment_example :: identity :: identity :: Identity , ferment_example :: nested :: ProtocolError > > for Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError) -> Result < ferment_example :: identity :: identity :: Identity , ferment_example :: nested :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < ferment_example :: identity :: identity :: Identity , ferment_example :: nested :: ProtocolError >) -> * const Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_Identity , error : * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_ProtocolError) -> * mut Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError { ok , error })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError_destroy(
        ffi : * mut Result_ok_ferment_example_identity_identity_Identity_err_ferment_example_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_ferment_example_nested_FeatureVersion { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion }
    impl ferment_interfaces::FFIConversion<Vec<ferment_example::nested::FeatureVersion>>
        for Vec_ferment_example_nested_FeatureVersion
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_ferment_example_nested_FeatureVersion,
        ) -> Vec<ferment_example::nested::FeatureVersion> {
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
        }
        unsafe fn ffi_to_const(
            obj: Vec<ferment_example::nested::FeatureVersion>,
        ) -> *const Vec_ferment_example_nested_FeatureVersion {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_ferment_example_nested_FeatureVersion) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_ferment_example_nested_FeatureVersion {
        type Value = Vec<ferment_example::nested::FeatureVersion>;
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
    impl Drop for Vec_ferment_example_nested_FeatureVersion {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_FeatureVersion_ctor(
        values : * mut * mut crate :: fermented :: types :: ferment_example :: nested :: ferment_example_nested_FeatureVersion,
        count: usize,
    ) -> *mut Vec_ferment_example_nested_FeatureVersion {
        ferment_interfaces::boxed(Vec_ferment_example_nested_FeatureVersion { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_FeatureVersion_destroy(
        ffi: *mut Vec_ferment_example_nested_FeatureVersion,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID , pub values : * mut * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityPublicKey }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < ferment_example :: identity :: identity :: KeyID , ferment_example :: identity :: identity :: IdentityPublicKey > > for std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey) -> std :: collections :: BTreeMap < ferment_example :: identity :: identity :: KeyID , ferment_example :: identity :: identity :: IdentityPublicKey > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < ferment_example :: identity :: identity :: KeyID , ferment_example :: identity :: identity :: IdentityPublicKey >) -> * const std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey_ctor (keys : * mut * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_KeyID , values : * mut * mut crate :: fermented :: types :: ferment_example :: identity :: identity :: ferment_example_identity_identity_IdentityPublicKey , count : usize) -> * mut std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey{
        ferment_interfaces :: boxed (std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey { count , keys , values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey_destroy(
        ffi : * mut std_collections_Map_keys_ferment_example_identity_identity_KeyID_values_ferment_example_identity_identity_IdentityPublicKey,
    ) {
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
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
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
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
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
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(values: *mut i32, count: usize) -> *mut Vec_i32 {
        ferment_interfaces::boxed(Vec_i32 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment_interfaces::unbox_any(ffi);
    }
}
