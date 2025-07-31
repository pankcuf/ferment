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
    pub mod example_thread_safe {
        use crate as example_thread_safe;
        pub mod primitives {
            use crate as example_thread_safe;
            pub mod pin {
                use crate as example_thread_safe;
                #[doc = "FFI-representation of the [`PinExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_thread_safe_primitives_pin_PinExamples {
                    pub opaque: *mut std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>,
                }
                impl ferment::FFIConversionFrom<example_thread_safe::primitives::pin::PinExamples>
                    for example_thread_safe_primitives_pin_PinExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_thread_safe_primitives_pin_PinExamples,
                    ) -> example_thread_safe::primitives::pin::PinExamples {
                        let ffi_ref = &*ffi;
                        example_thread_safe::primitives::pin::PinExamples {
                            opaque: std::ptr::read(ffi_ref.opaque),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_thread_safe::primitives::pin::PinExamples>
                    for example_thread_safe_primitives_pin_PinExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_thread_safe::primitives::pin::PinExamples,
                    ) -> *const example_thread_safe_primitives_pin_PinExamples {
                        ferment::boxed(example_thread_safe_primitives_pin_PinExamples {
                            opaque: ferment::boxed(obj.opaque),
                        })
                    }
                }
                impl Drop for example_thread_safe_primitives_pin_PinExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.opaque);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_thread_safe_primitives_pin_PinExamples_ctor(
                    opaque: *mut std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>,
                ) -> *mut example_thread_safe_primitives_pin_PinExamples {
                    ferment::boxed(example_thread_safe_primitives_pin_PinExamples { opaque })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_thread_safe_primitives_pin_PinExamples_destroy(
                    ffi: *mut example_thread_safe_primitives_pin_PinExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_thread_safe_primitives_pin_PinExamples_get_opaque(
                    obj: *const example_thread_safe_primitives_pin_PinExamples,
                ) -> *mut std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>
                {
                    (*obj).opaque
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_thread_safe_primitives_pin_PinExamples_set_opaque(
                    obj: *mut example_thread_safe_primitives_pin_PinExamples,
                    value: *mut std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>,
                ) {
                    (*obj).opaque = value;
                }
            }
        }
        pub mod entry {
            use crate as example_thread_safe;
            #[doc = "FFI-representation of the [`example_thread_safe::entry::PlatformProvider::new`]"]
            #[no_mangle]
            pub unsafe extern "C" fn example_thread_safe_entry_PlatformProvider_new(
                context : * mut crate :: fermented :: generics :: std_sync_Arc_example_thread_safe_entry_FFIContext,
            ) -> *mut example_thread_safe::entry::PlatformProvider {
                let obj = example_thread_safe :: entry :: PlatformProvider :: new (< crate :: fermented :: generics :: std_sync_Arc_example_thread_safe_entry_FFIContext as ferment :: FFIConversionFrom < std :: sync :: Arc < example_thread_safe :: entry :: FFIContext > >> :: ffi_from (context)) ;
                ferment::boxed(obj)
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
    use crate as example_thread_safe;
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        caller: unsafe extern "C" fn(
            *const example_thread_safe::entry::FFIContext,
            u32,
            *mut std::os::raw::c_char,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    }
    impl Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        pub unsafe fn call(
            &self,
            o_0: *const example_thread_safe::entry::FFIContext,
            o_1: u32,
            o_2: &String,
        ) -> String {
            let ffi_result = (self.caller)(
                o_0,
                o_1,
                <std::os::raw::c_char as ferment::FFIConversionTo<String>>::ffi_to(o_2.clone()),
            );
            let result =
                <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send for Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {}
    unsafe impl Sync for Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String_ctor(
        caller: unsafe extern "C" fn(
            *const example_thread_safe::entry::FFIContext,
            u32,
            *mut std::os::raw::c_char,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    ) -> *mut Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        ferment::boxed(
            Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
                caller,
                destructor,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String_destroy(
        ffi: *mut Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_pin_Pin_Box_example_thread_safe_entry_FFIContext {
        pub obj: *mut std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>,
    }
    impl ferment::FFIConversionFrom<std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>>
        for std_pin_Pin_Box_example_thread_safe_entry_FFIContext
    {
        unsafe fn ffi_from_const(
            ffi: *const std_pin_Pin_Box_example_thread_safe_entry_FFIContext,
        ) -> std::pin::Pin<Box<example_thread_safe::entry::FFIContext>> {
            let ffi_ref = &*ffi;
            std::ptr::read(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>>
        for std_pin_Pin_Box_example_thread_safe_entry_FFIContext
    {
        unsafe fn ffi_to_const(obj: std::pin::Pin<Box<example_thread_safe::entry::FFIContext>>) -> *const std_pin_Pin_Box_example_thread_safe_entry_FFIContext {
            ferment::boxed(Self {
                obj: ferment::boxed(obj),
            })
        }
    }
    impl Drop for std_pin_Pin_Box_example_thread_safe_entry_FFIContext {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_pin_Pin_Box_example_thread_safe_entry_FFIContext_ctor(
        obj: *mut example_thread_safe::entry::FFIContext,
    ) -> *mut std_pin_Pin_Box_example_thread_safe_entry_FFIContext {
        ferment::boxed(std_pin_Pin_Box_example_thread_safe_entry_FFIContext {
            obj: ferment::boxed(Box::into_pin(Box::new(std::ptr::read(obj))))
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_pin_Pin_Box_example_thread_safe_entry_FFIContext_destroy(
        ffi: *mut std_pin_Pin_Box_example_thread_safe_entry_FFIContext,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_example_thread_safe_entry_FFIContext {
        pub obj: *mut example_thread_safe::entry::FFIContext,
    }
    impl ferment::FFIConversionFrom<std::sync::Arc<example_thread_safe::entry::FFIContext>>
        for std_sync_Arc_example_thread_safe_entry_FFIContext
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_example_thread_safe_entry_FFIContext,
        ) -> std::sync::Arc<example_thread_safe::entry::FFIContext> {
            let ffi_ref = &*ffi;
            std::sync::Arc::from_raw(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::sync::Arc<example_thread_safe::entry::FFIContext>>
        for std_sync_Arc_example_thread_safe_entry_FFIContext
    {
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<example_thread_safe::entry::FFIContext>,
        ) -> *const std_sync_Arc_example_thread_safe_entry_FFIContext {
            ferment::boxed(Self {
                obj: std::sync::Arc::into_raw(obj).cast_mut(),
            })
        }
    }
    impl Drop for std_sync_Arc_example_thread_safe_entry_FFIContext {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_example_thread_safe_entry_FFIContext_ctor(
        obj: *mut example_thread_safe::entry::FFIContext,
    ) -> *mut std_sync_Arc_example_thread_safe_entry_FFIContext {
        ferment::boxed(std_sync_Arc_example_thread_safe_entry_FFIContext { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_example_thread_safe_entry_FFIContext_destroy(
        ffi: *mut std_sync_Arc_example_thread_safe_entry_FFIContext,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        pub obj : * mut crate :: fermented :: generics :: Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String
    }
    impl ferment :: FFIConversionFrom < std :: sync :: Arc < dyn Fn (* const example_thread_safe :: entry :: FFIContext , u32 , & String) -> String > > for std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        unsafe fn ffi_from_const (ffi : * const std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String) -> std :: sync :: Arc < dyn Fn (* const example_thread_safe :: entry :: FFIContext , u32 , & String) -> String > {
            let ffi_ref = & * ffi ;
            std :: sync :: Arc :: new (move | o_0 , o_1 , o_2 | unsafe { (& * ffi_ref . obj) . call (o_0 , o_1 , o_2) })
        }
    }
    impl Drop for std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String {
        fn drop (& mut self) { unsafe { ; } }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String_ctor (obj : * mut crate :: fermented :: generics :: Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String) -> * mut std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String{
        ferment :: boxed (std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String_destroy(
        ffi : * mut std_sync_Arc_dyn_trait_Fn_ARGS_example_thread_safe_entry_FFIContext_u32_String_RTRN_String,
    ) {
        ferment::unbox_any(ffi);
    }
}
