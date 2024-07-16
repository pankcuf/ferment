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
    pub mod ferment_example_entry_point {
        use crate as ferment_example_entry_point;
        pub mod entry {
            use crate as ferment_example_entry_point;
            pub mod core {
                use crate as ferment_example_entry_point;
                #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: core :: DashSharedCore :: with_pointers`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_core_DashSharedCore_with_pointers(
                    block_hash_by_height: ferment_example_entry_point::entry::BlockHashByHeight,
                    model_by_height: ferment_example_entry_point::entry::ModelByHeight,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_entry_point::entry::core::DashSharedCore {
                    let obj =
                        ferment_example_entry_point::entry::core::DashSharedCore::with_pointers(
                            block_hash_by_height,
                            model_by_height,
                            context,
                        );
                    ferment_interfaces::boxed(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: core :: DashSharedCore :: with_lambdas`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_core_DashSharedCore_with_lambdas(
                    block_hash_by_height : * mut crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Arr_u8_32,
                    model_by_height : * mut crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_ferment_example_entry_point_entry_SomeModel,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_entry_point::entry::core::DashSharedCore {
                    let obj =
                        ferment_example_entry_point::entry::core::DashSharedCore::with_lambdas(
                            move |o_0| unsafe { (&*block_hash_by_height).call(o_0) },
                            move |o_0| unsafe { (&*model_by_height).call(o_0) },
                            context,
                        );
                    ferment_interfaces::boxed(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: core :: DashSharedCore`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_core_DashSharedCore_ctor(
                    processor: *mut ferment_example_entry_point::entry::processor::Processor,
                    cache : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_String,
                    context: *mut std::os::raw::c_void,
                ) -> *mut ferment_example_entry_point::entry::core::DashSharedCore {
                    ferment_interfaces::boxed(
                        ferment_example_entry_point::entry::core::DashSharedCore {
                            processor: processor,
                            cache: ferment_interfaces::FFIConversion::ffi_from(cache),
                            context: context,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_core_DashSharedCore_destroy(
                    ffi: *mut ferment_example_entry_point::entry::core::DashSharedCore,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: SomeModel`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_entry_point_entry_SomeModel {
                pub hash: *mut crate::fermented::generics::Arr_u8_32,
                pub desc: *mut std::os::raw::c_char,
            }
            impl ferment_interfaces::FFIConversion<ferment_example_entry_point::entry::SomeModel>
                for ferment_example_entry_point_entry_SomeModel
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_entry_point_entry_SomeModel,
                ) -> ferment_example_entry_point::entry::SomeModel {
                    let ffi_ref = &*ffi;
                    ferment_example_entry_point::entry::SomeModel {
                        hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.hash),
                        desc: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.desc),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_entry_point::entry::SomeModel,
                ) -> *const ferment_example_entry_point_entry_SomeModel {
                    ferment_interfaces::boxed(ferment_example_entry_point_entry_SomeModel {
                        hash: ferment_interfaces::FFIConversion::ffi_to(obj.hash),
                        desc: ferment_interfaces::FFIConversion::ffi_to(obj.desc),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_entry_point_entry_SomeModel) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_entry_point_entry_SomeModel {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.hash);
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . desc) ;
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_ctor(
                hash: *mut crate::fermented::generics::Arr_u8_32,
                desc: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_entry_point_entry_SomeModel {
                ferment_interfaces::boxed(ferment_example_entry_point_entry_SomeModel {
                    hash,
                    desc,
                })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_destroy(
                ffi: *mut ferment_example_entry_point_entry_SomeModel,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_get_hash(
                obj: *const ferment_example_entry_point_entry_SomeModel,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).hash
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_get_desc(
                obj: *const ferment_example_entry_point_entry_SomeModel,
            ) -> *mut std::os::raw::c_char {
                (*obj).desc
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_set_hash(
                obj: *mut ferment_example_entry_point_entry_SomeModel,
                value: *mut crate::fermented::generics::Arr_u8_32,
            ) {
                (*obj).hash = value;
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_entry_point_entry_SomeModel_set_desc(
                obj: *mut ferment_example_entry_point_entry_SomeModel,
                value: *mut std::os::raw::c_char,
            ) {
                (*obj).desc = value;
            }
            pub mod processor {
                use crate as ferment_example_entry_point;
                #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: processor :: Processor`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_processor_Processor_ctor(
                    chain_id: *mut dyn ferment_example_entry_point::entry::provider::CoreProvider,
                ) -> *mut ferment_example_entry_point::entry::processor::Processor {
                    ferment_interfaces::boxed(
                        ferment_example_entry_point::entry::processor::Processor {
                            chain_id: Box::from_raw(chain_id),
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_processor_Processor_destroy(
                    ffi: *mut ferment_example_entry_point::entry::processor::Processor,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod provider {
                use crate as ferment_example_entry_point;
                #[doc = "FFI-representation of the [`ferment_example_entry_point :: entry :: provider :: FFIPtrCoreProvider`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_provider_FFIPtrCoreProvider_ctor(
                    block_hash_by_height: ferment_example_entry_point::entry::BlockHashByHeight,
                    model_by_height: ferment_example_entry_point::entry::ModelByHeight,
                ) -> *mut ferment_example_entry_point::entry::provider::FFIPtrCoreProvider
                {
                    ferment_interfaces::boxed(
                        ferment_example_entry_point::entry::provider::FFIPtrCoreProvider {
                            block_hash_by_height: block_hash_by_height,
                            model_by_height: model_by_height,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_entry_point_entry_provider_FFIPtrCoreProvider_destroy(
                    ffi: *mut ferment_example_entry_point::entry::provider::FFIPtrCoreProvider,
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
    use crate as ferment_example_entry_point;
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_ferment_example_entry_point_entry_SomeModel { pub context : * const std :: os :: raw :: c_void , caller : fn (u32) -> * mut crate :: fermented :: types :: ferment_example_entry_point :: entry :: ferment_example_entry_point_entry_SomeModel , destructor : fn (result : * mut crate :: fermented :: types :: ferment_example_entry_point :: entry :: ferment_example_entry_point_entry_SomeModel) , }
    impl Fn_ARGS_u32_RTRN_ferment_example_entry_point_entry_SomeModel {
        pub unsafe fn call(&self, o_0: u32) -> ferment_example_entry_point::entry::SomeModel {
            let ffi_result = (self.caller)(o_0);
            let result = < crate :: fermented :: types :: ferment_example_entry_point :: entry :: ferment_example_entry_point_entry_SomeModel as ferment_interfaces :: FFIConversion < ferment_example_entry_point :: entry :: SomeModel >> :: ffi_from (ffi_result) ;
            (self.destructor)(ffi_result);
            result
        }
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
}
