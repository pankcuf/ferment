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
    pub mod example_aliasing {
        use crate as example_aliasing;
        pub mod aa {
            use crate as example_aliasing;
            pub mod at_aa {
                use crate as example_aliasing;
                #[doc = "FFI-representation of the [`AtAa`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_aliasing_aa_at_aa_AtAa {
                    pub version: u32,
                }
                impl ferment::FFIConversionFrom<example_aliasing::aa::at_aa::AtAa>
                    for example_aliasing_aa_at_aa_AtAa
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_aliasing_aa_at_aa_AtAa,
                    ) -> example_aliasing::aa::at_aa::AtAa {
                        let ffi_ref = &*ffi;
                        example_aliasing::aa::at_aa::AtAa {
                            version: ffi_ref.version,
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_aliasing::aa::at_aa::AtAa>
                    for example_aliasing_aa_at_aa_AtAa
                {
                    unsafe fn ffi_to_const(
                        obj: example_aliasing::aa::at_aa::AtAa,
                    ) -> *const example_aliasing_aa_at_aa_AtAa {
                        ferment::boxed(example_aliasing_aa_at_aa_AtAa {
                            version: obj.version,
                        })
                    }
                }
                impl Drop for example_aliasing_aa_at_aa_AtAa {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_aa_at_aa_AtAa_ctor(
                    version: u32,
                ) -> *mut example_aliasing_aa_at_aa_AtAa {
                    ferment::boxed(example_aliasing_aa_at_aa_AtAa { version })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_aa_at_aa_AtAa_destroy(
                    ffi: *mut example_aliasing_aa_at_aa_AtAa,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_aa_at_aa_AtAa_get_version(
                    obj: *const example_aliasing_aa_at_aa_AtAa,
                ) -> u32 {
                    (*obj).version
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_aa_at_aa_AtAa_set_version(
                    obj: *mut example_aliasing_aa_at_aa_AtAa,
                    value: u32,
                ) {
                    (*obj).version = value;
                }
            }
            pub mod bb {
                use crate as example_aliasing;
                pub mod at_bb {
                    use crate as example_aliasing;
                    #[doc = "FFI-representation of the [`AtBb`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_aliasing_aa_bb_at_bb_AtBb {
                        pub version: u32,
                    }
                    impl ferment::FFIConversionFrom<example_aliasing::aa::bb::at_bb::AtBb>
                        for example_aliasing_aa_bb_at_bb_AtBb
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const example_aliasing_aa_bb_at_bb_AtBb,
                        ) -> example_aliasing::aa::bb::at_bb::AtBb {
                            let ffi_ref = &*ffi;
                            example_aliasing::aa::bb::at_bb::AtBb {
                                version: ffi_ref.version,
                            }
                        }
                    }
                    impl ferment::FFIConversionTo<example_aliasing::aa::bb::at_bb::AtBb>
                        for example_aliasing_aa_bb_at_bb_AtBb
                    {
                        unsafe fn ffi_to_const(
                            obj: example_aliasing::aa::bb::at_bb::AtBb,
                        ) -> *const example_aliasing_aa_bb_at_bb_AtBb {
                            ferment::boxed(example_aliasing_aa_bb_at_bb_AtBb {
                                version: obj.version,
                            })
                        }
                    }
                    impl Drop for example_aliasing_aa_bb_at_bb_AtBb {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_aa_bb_at_bb_AtBb_ctor(
                        version: u32,
                    ) -> *mut example_aliasing_aa_bb_at_bb_AtBb {
                        ferment::boxed(example_aliasing_aa_bb_at_bb_AtBb { version })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_aa_bb_at_bb_AtBb_destroy(
                        ffi: *mut example_aliasing_aa_bb_at_bb_AtBb,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_aa_bb_at_bb_AtBb_get_version(
                        obj: *const example_aliasing_aa_bb_at_bb_AtBb,
                    ) -> u32 {
                        (*obj).version
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_aa_bb_at_bb_AtBb_set_version(
                        obj: *mut example_aliasing_aa_bb_at_bb_AtBb,
                        value: u32,
                    ) {
                        (*obj).version = value;
                    }
                }
                pub mod cc {
                    use crate as example_aliasing;
                    pub mod at_cc {
                        use crate as example_aliasing;
                        #[doc = "FFI-representation of the [`AtCc`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct example_aliasing_aa_bb_cc_at_cc_AtCc { pub aa : * mut crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa , pub bb : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb , pub dd : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd , pub ww : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw , pub xx : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx , pub yy : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy , pub zz : * mut crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz }
                        impl ferment::FFIConversionFrom<example_aliasing::aa::bb::cc::at_cc::AtCc>
                            for example_aliasing_aa_bb_cc_at_cc_AtCc
                        {
                            unsafe fn ffi_from_const(
                                ffi: *const example_aliasing_aa_bb_cc_at_cc_AtCc,
                            ) -> example_aliasing::aa::bb::cc::at_cc::AtCc
                            {
                                let ffi_ref = &*ffi;
                                example_aliasing :: aa :: bb :: cc :: at_cc :: AtCc { aa : < crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa as ferment :: FFIConversionFrom < example_aliasing :: aa :: at_aa :: AtAa >> :: ffi_from (ffi_ref . aa) , bb : < crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb as ferment :: FFIConversionFrom < example_aliasing :: aa :: bb :: at_bb :: AtBb >> :: ffi_from (ffi_ref . bb) , dd : < crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd as ferment :: FFIConversionFrom < example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: AtDd >> :: ffi_from (ffi_ref . dd) , ww : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw as ferment :: FFIConversionFrom < example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: AtWw >> :: ffi_from (ffi_ref . ww) , xx : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx as ferment :: FFIConversionFrom < example_aliasing :: zz :: yy :: xx :: at_xx :: AtXx >> :: ffi_from (ffi_ref . xx) , yy : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy as ferment :: FFIConversionFrom < example_aliasing :: zz :: yy :: at_yy :: AtYy >> :: ffi_from (ffi_ref . yy) , zz : < crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz as ferment :: FFIConversionFrom < example_aliasing :: zz :: at_zz :: AtZz >> :: ffi_from (ffi_ref . zz) }
                            }
                        }
                        impl ferment::FFIConversionTo<example_aliasing::aa::bb::cc::at_cc::AtCc>
                            for example_aliasing_aa_bb_cc_at_cc_AtCc
                        {
                            unsafe fn ffi_to_const(
                                obj: example_aliasing::aa::bb::cc::at_cc::AtCc,
                            ) -> *const example_aliasing_aa_bb_cc_at_cc_AtCc
                            {
                                ferment :: boxed (example_aliasing_aa_bb_cc_at_cc_AtCc { aa : < crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa as ferment :: FFIConversionTo < example_aliasing :: aa :: at_aa :: AtAa >> :: ffi_to (obj . aa) , bb : < crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb as ferment :: FFIConversionTo < example_aliasing :: aa :: bb :: at_bb :: AtBb >> :: ffi_to (obj . bb) , dd : < crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd as ferment :: FFIConversionTo < example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: AtDd >> :: ffi_to (obj . dd) , ww : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw as ferment :: FFIConversionTo < example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: AtWw >> :: ffi_to (obj . ww) , xx : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx as ferment :: FFIConversionTo < example_aliasing :: zz :: yy :: xx :: at_xx :: AtXx >> :: ffi_to (obj . xx) , yy : < crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy as ferment :: FFIConversionTo < example_aliasing :: zz :: yy :: at_yy :: AtYy >> :: ffi_to (obj . yy) , zz : < crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz as ferment :: FFIConversionTo < example_aliasing :: zz :: at_zz :: AtZz >> :: ffi_to (obj . zz) })
                            }
                        }
                        impl Drop for example_aliasing_aa_bb_cc_at_cc_AtCc {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.aa);
                                    ferment::unbox_any(ffi_ref.bb);
                                    ferment::unbox_any(ffi_ref.dd);
                                    ferment::unbox_any(ffi_ref.ww);
                                    ferment::unbox_any(ffi_ref.xx);
                                    ferment::unbox_any(ffi_ref.yy);
                                    ferment::unbox_any(ffi_ref.zz);
                                }
                            }
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_ctor(
                            aa : * mut crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa,
                            bb : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb,
                            dd : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                            ww : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                            xx : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx,
                            yy : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy,
                            zz : * mut crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz,
                        ) -> *mut example_aliasing_aa_bb_cc_at_cc_AtCc {
                            ferment::boxed(example_aliasing_aa_bb_cc_at_cc_AtCc {
                                aa,
                                bb,
                                dd,
                                ww,
                                xx,
                                yy,
                                zz,
                            })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_destroy(
                            ffi: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_aa (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa{
                            (*obj).aa
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_bb (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb{
                            (*obj).bb
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_dd (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd{
                            (*obj).dd
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_ww (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw{
                            (*obj).ww
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_xx (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx{
                            (*obj).xx
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_yy (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy{
                            (*obj).yy
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_get_zz (obj : * const example_aliasing_aa_bb_cc_at_cc_AtCc) -> * mut crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz{
                            (*obj).zz
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_aa(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa,
                        ) {
                            (*obj).aa = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_bb(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: at_bb :: example_aliasing_aa_bb_at_bb_AtBb,
                        ) {
                            (*obj).bb = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_dd(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: aa :: bb :: cc :: dd :: at_dd :: example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                        ) {
                            (*obj).dd = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_ww(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: ww :: at_ww :: example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                        ) {
                            (*obj).ww = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_xx(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: xx :: at_xx :: example_aliasing_zz_yy_xx_at_xx_AtXx,
                        ) {
                            (*obj).xx = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_yy(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: zz :: yy :: at_yy :: example_aliasing_zz_yy_at_yy_AtYy,
                        ) {
                            (*obj).yy = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_aa_bb_cc_at_cc_AtCc_set_zz(
                            obj: *mut example_aliasing_aa_bb_cc_at_cc_AtCc,
                            value : * mut crate :: fermented :: types :: example_aliasing :: zz :: at_zz :: example_aliasing_zz_at_zz_AtZz,
                        ) {
                            (*obj).zz = value;
                        }
                    }
                    pub mod dd {
                        use crate as example_aliasing;
                        pub mod at_dd {
                            use crate as example_aliasing;
                            #[doc = "FFI-representation of the [`AtDd`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct example_aliasing_aa_bb_cc_dd_at_dd_AtDd {
                                pub version: u32,
                            }
                            impl
                                ferment::FFIConversionFrom<
                                    example_aliasing::aa::bb::cc::dd::at_dd::AtDd,
                                > for example_aliasing_aa_bb_cc_dd_at_dd_AtDd
                            {
                                unsafe fn ffi_from_const(
                                    ffi: *const example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                                ) -> example_aliasing::aa::bb::cc::dd::at_dd::AtDd
                                {
                                    let ffi_ref = &*ffi;
                                    example_aliasing::aa::bb::cc::dd::at_dd::AtDd {
                                        version: ffi_ref.version,
                                    }
                                }
                            }
                            impl
                                ferment::FFIConversionTo<
                                    example_aliasing::aa::bb::cc::dd::at_dd::AtDd,
                                > for example_aliasing_aa_bb_cc_dd_at_dd_AtDd
                            {
                                unsafe fn ffi_to_const(
                                    obj: example_aliasing::aa::bb::cc::dd::at_dd::AtDd,
                                ) -> *const example_aliasing_aa_bb_cc_dd_at_dd_AtDd
                                {
                                    ferment::boxed(example_aliasing_aa_bb_cc_dd_at_dd_AtDd {
                                        version: obj.version,
                                    })
                                }
                            }
                            impl Drop for example_aliasing_aa_bb_cc_dd_at_dd_AtDd {
                                fn drop(&mut self) {
                                    unsafe {
                                        let ffi_ref = self;
                                    }
                                }
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_aa_bb_cc_dd_at_dd_AtDd_ctor(
                                version: u32,
                            ) -> *mut example_aliasing_aa_bb_cc_dd_at_dd_AtDd
                            {
                                ferment::boxed(example_aliasing_aa_bb_cc_dd_at_dd_AtDd { version })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_aa_bb_cc_dd_at_dd_AtDd_destroy(
                                ffi: *mut example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_aa_bb_cc_dd_at_dd_AtDd_get_version(
                                obj: *const example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                            ) -> u32 {
                                (*obj).version
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_aa_bb_cc_dd_at_dd_AtDd_set_version(
                                obj: *mut example_aliasing_aa_bb_cc_dd_at_dd_AtDd,
                                value: u32,
                            ) {
                                (*obj).version = value;
                            }
                        }
                    }
                }
            }
        }
        pub mod zz {
            use crate as example_aliasing;
            pub mod at_zz {
                use crate as example_aliasing;
                #[doc = "FFI-representation of the [`AtZz`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_aliasing_zz_at_zz_AtZz {
                    pub version: u32,
                }
                impl ferment::FFIConversionFrom<example_aliasing::zz::at_zz::AtZz>
                    for example_aliasing_zz_at_zz_AtZz
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_aliasing_zz_at_zz_AtZz,
                    ) -> example_aliasing::zz::at_zz::AtZz {
                        let ffi_ref = &*ffi;
                        example_aliasing::zz::at_zz::AtZz {
                            version: ffi_ref.version,
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_aliasing::zz::at_zz::AtZz>
                    for example_aliasing_zz_at_zz_AtZz
                {
                    unsafe fn ffi_to_const(
                        obj: example_aliasing::zz::at_zz::AtZz,
                    ) -> *const example_aliasing_zz_at_zz_AtZz {
                        ferment::boxed(example_aliasing_zz_at_zz_AtZz {
                            version: obj.version,
                        })
                    }
                }
                impl Drop for example_aliasing_zz_at_zz_AtZz {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_zz_at_zz_AtZz_ctor(
                    version: u32,
                ) -> *mut example_aliasing_zz_at_zz_AtZz {
                    ferment::boxed(example_aliasing_zz_at_zz_AtZz { version })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_zz_at_zz_AtZz_destroy(
                    ffi: *mut example_aliasing_zz_at_zz_AtZz,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_zz_at_zz_AtZz_get_version(
                    obj: *const example_aliasing_zz_at_zz_AtZz,
                ) -> u32 {
                    (*obj).version
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_aliasing_zz_at_zz_AtZz_set_version(
                    obj: *mut example_aliasing_zz_at_zz_AtZz,
                    value: u32,
                ) {
                    (*obj).version = value;
                }
            }
            pub mod yy {
                use crate as example_aliasing;
                pub mod at_yy {
                    use crate as example_aliasing;
                    #[doc = "FFI-representation of the [`AtYy`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_aliasing_zz_yy_at_yy_AtYy {
                        pub version: u32,
                    }
                    impl ferment::FFIConversionFrom<example_aliasing::zz::yy::at_yy::AtYy>
                        for example_aliasing_zz_yy_at_yy_AtYy
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const example_aliasing_zz_yy_at_yy_AtYy,
                        ) -> example_aliasing::zz::yy::at_yy::AtYy {
                            let ffi_ref = &*ffi;
                            example_aliasing::zz::yy::at_yy::AtYy {
                                version: ffi_ref.version,
                            }
                        }
                    }
                    impl ferment::FFIConversionTo<example_aliasing::zz::yy::at_yy::AtYy>
                        for example_aliasing_zz_yy_at_yy_AtYy
                    {
                        unsafe fn ffi_to_const(
                            obj: example_aliasing::zz::yy::at_yy::AtYy,
                        ) -> *const example_aliasing_zz_yy_at_yy_AtYy {
                            ferment::boxed(example_aliasing_zz_yy_at_yy_AtYy {
                                version: obj.version,
                            })
                        }
                    }
                    impl Drop for example_aliasing_zz_yy_at_yy_AtYy {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_zz_yy_at_yy_AtYy_ctor(
                        version: u32,
                    ) -> *mut example_aliasing_zz_yy_at_yy_AtYy {
                        ferment::boxed(example_aliasing_zz_yy_at_yy_AtYy { version })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_zz_yy_at_yy_AtYy_destroy(
                        ffi: *mut example_aliasing_zz_yy_at_yy_AtYy,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_zz_yy_at_yy_AtYy_get_version(
                        obj: *const example_aliasing_zz_yy_at_yy_AtYy,
                    ) -> u32 {
                        (*obj).version
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_aliasing_zz_yy_at_yy_AtYy_set_version(
                        obj: *mut example_aliasing_zz_yy_at_yy_AtYy,
                        value: u32,
                    ) {
                        (*obj).version = value;
                    }
                }
                pub mod xx {
                    use crate as example_aliasing;
                    pub mod at_xx {
                        use crate as example_aliasing;
                        #[doc = "FFI-representation of the [`AtXx`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct example_aliasing_zz_yy_xx_at_xx_AtXx {
                            pub version: u32,
                        }
                        impl ferment::FFIConversionFrom<example_aliasing::zz::yy::xx::at_xx::AtXx>
                            for example_aliasing_zz_yy_xx_at_xx_AtXx
                        {
                            unsafe fn ffi_from_const(
                                ffi: *const example_aliasing_zz_yy_xx_at_xx_AtXx,
                            ) -> example_aliasing::zz::yy::xx::at_xx::AtXx
                            {
                                let ffi_ref = &*ffi;
                                example_aliasing::zz::yy::xx::at_xx::AtXx {
                                    version: ffi_ref.version,
                                }
                            }
                        }
                        impl ferment::FFIConversionTo<example_aliasing::zz::yy::xx::at_xx::AtXx>
                            for example_aliasing_zz_yy_xx_at_xx_AtXx
                        {
                            unsafe fn ffi_to_const(
                                obj: example_aliasing::zz::yy::xx::at_xx::AtXx,
                            ) -> *const example_aliasing_zz_yy_xx_at_xx_AtXx
                            {
                                ferment::boxed(example_aliasing_zz_yy_xx_at_xx_AtXx {
                                    version: obj.version,
                                })
                            }
                        }
                        impl Drop for example_aliasing_zz_yy_xx_at_xx_AtXx {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                }
                            }
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_zz_yy_xx_at_xx_AtXx_ctor(
                            version: u32,
                        ) -> *mut example_aliasing_zz_yy_xx_at_xx_AtXx {
                            ferment::boxed(example_aliasing_zz_yy_xx_at_xx_AtXx { version })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_zz_yy_xx_at_xx_AtXx_destroy(
                            ffi: *mut example_aliasing_zz_yy_xx_at_xx_AtXx,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_zz_yy_xx_at_xx_AtXx_get_version(
                            obj: *const example_aliasing_zz_yy_xx_at_xx_AtXx,
                        ) -> u32 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_aliasing_zz_yy_xx_at_xx_AtXx_set_version(
                            obj: *mut example_aliasing_zz_yy_xx_at_xx_AtXx,
                            value: u32,
                        ) {
                            (*obj).version = value;
                        }
                    }
                    pub mod ww {
                        use crate as example_aliasing;
                        pub mod at_ww {
                            use crate as example_aliasing;
                            #[doc = "FFI-representation of the [`AtWw`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct example_aliasing_zz_yy_xx_ww_at_ww_AtWw {
                                pub version: u32,
                            }
                            impl
                                ferment::FFIConversionFrom<
                                    example_aliasing::zz::yy::xx::ww::at_ww::AtWw,
                                > for example_aliasing_zz_yy_xx_ww_at_ww_AtWw
                            {
                                unsafe fn ffi_from_const(
                                    ffi: *const example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                                ) -> example_aliasing::zz::yy::xx::ww::at_ww::AtWw
                                {
                                    let ffi_ref = &*ffi;
                                    example_aliasing::zz::yy::xx::ww::at_ww::AtWw {
                                        version: ffi_ref.version,
                                    }
                                }
                            }
                            impl
                                ferment::FFIConversionTo<
                                    example_aliasing::zz::yy::xx::ww::at_ww::AtWw,
                                > for example_aliasing_zz_yy_xx_ww_at_ww_AtWw
                            {
                                unsafe fn ffi_to_const(
                                    obj: example_aliasing::zz::yy::xx::ww::at_ww::AtWw,
                                ) -> *const example_aliasing_zz_yy_xx_ww_at_ww_AtWw
                                {
                                    ferment::boxed(example_aliasing_zz_yy_xx_ww_at_ww_AtWw {
                                        version: obj.version,
                                    })
                                }
                            }
                            impl Drop for example_aliasing_zz_yy_xx_ww_at_ww_AtWw {
                                fn drop(&mut self) {
                                    unsafe {
                                        let ffi_ref = self;
                                    }
                                }
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_zz_yy_xx_ww_at_ww_AtWw_ctor(
                                version: u32,
                            ) -> *mut example_aliasing_zz_yy_xx_ww_at_ww_AtWw
                            {
                                ferment::boxed(example_aliasing_zz_yy_xx_ww_at_ww_AtWw { version })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_zz_yy_xx_ww_at_ww_AtWw_destroy(
                                ffi: *mut example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_zz_yy_xx_ww_at_ww_AtWw_get_version(
                                obj: *const example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                            ) -> u32 {
                                (*obj).version
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_aliasing_zz_yy_xx_ww_at_ww_AtWw_set_version(
                                obj: *mut example_aliasing_zz_yy_xx_ww_at_ww_AtWw,
                                value: u32,
                            ) {
                                (*obj).version = value;
                            }
                        }
                    }
                }
            }
        }
        #[doc = "FFI-representation of the [`SPV`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct example_aliasing_SPV {
            pub version: u32,
        }
        impl ferment::FFIConversionFrom<example_aliasing::SPV> for example_aliasing_SPV {
            unsafe fn ffi_from_const(ffi: *const example_aliasing_SPV) -> example_aliasing::SPV {
                let ffi_ref = &*ffi;
                example_aliasing::SPV {
                    version: ffi_ref.version,
                }
            }
        }
        impl ferment::FFIConversionTo<example_aliasing::SPV> for example_aliasing_SPV {
            unsafe fn ffi_to_const(obj: example_aliasing::SPV) -> *const example_aliasing_SPV {
                ferment::boxed(example_aliasing_SPV {
                    version: obj.version,
                })
            }
        }
        impl Drop for example_aliasing_SPV {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_aliasing_SPV_ctor(
            version: u32,
        ) -> *mut example_aliasing_SPV {
            ferment::boxed(example_aliasing_SPV { version })
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_aliasing_SPV_destroy(ffi: *mut example_aliasing_SPV) {
            ferment::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_aliasing_SPV_get_version(
            obj: *const example_aliasing_SPV,
        ) -> u32 {
            (*obj).version
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_aliasing_SPV_set_version(
            obj: *mut example_aliasing_SPV,
            value: u32,
        ) {
            (*obj).version = value;
        }
        pub mod dash {
            use crate as example_aliasing;
            #[doc = "FFI-representation of the [`example_aliasing::dash::setup_aa`]"]
            #[no_mangle]
            pub unsafe extern "C" fn example_aliasing_dash_setup_aa(
                transaction : * mut crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa,
            ) {
                let obj = example_aliasing :: dash :: setup_aa (< crate :: fermented :: types :: example_aliasing :: aa :: at_aa :: example_aliasing_aa_at_aa_AtAa as ferment :: FFIConversionFrom < example_aliasing :: aa :: at_aa :: AtAa >> :: ffi_from (transaction)) ;
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
    use crate as example_aliasing;
}
