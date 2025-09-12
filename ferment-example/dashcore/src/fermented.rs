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
    pub mod dashcore {
        pub mod network {
            #[cfg(feature = "std")]
            pub mod message_qrinfo {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`QuorumSnapshot`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_network_message_qrinfo_QuorumSnapshot {
                    pub skip_list_mode : * mut crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode ,
                    pub active_quorum_members : * mut crate :: fermented :: generics :: Vec_bool ,
                    pub skip_list : * mut crate :: fermented :: generics :: Vec_i32
                }
                impl ferment::FFIConversionFrom<dashcore::network::message_qrinfo::QuorumSnapshot>
                    for dashcore_network_message_qrinfo_QuorumSnapshot
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_network_message_qrinfo_QuorumSnapshot,
                    ) -> dashcore::network::message_qrinfo::QuorumSnapshot {
                        let ffi_ref = &*ffi;
                        dashcore :: network :: message_qrinfo :: QuorumSnapshot {
                            skip_list_mode : < crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode as ferment :: FFIConversionFrom < dashcore :: network :: message_qrinfo :: MNSkipListMode >> :: ffi_from (ffi_ref . skip_list_mode) ,
                            active_quorum_members : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionFrom < Vec < bool > >> :: ffi_from (ffi_ref . active_quorum_members) ,
                            skip_list : < crate :: fermented :: generics :: Vec_i32 as ferment :: FFIConversionFrom < Vec < i32 > >> :: ffi_from (ffi_ref . skip_list)
                        }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::network::message_qrinfo::QuorumSnapshot>
                    for dashcore_network_message_qrinfo_QuorumSnapshot
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::network::message_qrinfo::QuorumSnapshot,
                    ) -> *const dashcore_network_message_qrinfo_QuorumSnapshot {
                        ferment :: boxed (dashcore_network_message_qrinfo_QuorumSnapshot { skip_list_mode : < crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode as ferment :: FFIConversionTo < dashcore :: network :: message_qrinfo :: MNSkipListMode >> :: ffi_to (obj . skip_list_mode) , active_quorum_members : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionTo < Vec < bool > >> :: ffi_to (obj . active_quorum_members) , skip_list : < crate :: fermented :: generics :: Vec_i32 as ferment :: FFIConversionTo < Vec < i32 > >> :: ffi_to (obj . skip_list) })
                    }
                }
                impl Drop for dashcore_network_message_qrinfo_QuorumSnapshot {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.skip_list_mode);
                            ferment::unbox_any(ffi_ref.active_quorum_members);
                            ferment::unbox_any(ffi_ref.skip_list);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_ctor(
                    skip_list_mode : * mut crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode,
                    active_quorum_members: *mut crate::fermented::generics::Vec_bool,
                    skip_list: *mut crate::fermented::generics::Vec_i32,
                ) -> *mut dashcore_network_message_qrinfo_QuorumSnapshot {
                    ferment::boxed(dashcore_network_message_qrinfo_QuorumSnapshot {
                        skip_list_mode,
                        active_quorum_members,
                        skip_list,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_destroy(
                    ffi: *mut dashcore_network_message_qrinfo_QuorumSnapshot,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_get_skip_list_mode (obj : * const dashcore_network_message_qrinfo_QuorumSnapshot) -> * mut crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode{
                    (*obj).skip_list_mode
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_get_active_quorum_members(
                    obj: *const dashcore_network_message_qrinfo_QuorumSnapshot,
                ) -> *mut crate::fermented::generics::Vec_bool {
                    (*obj).active_quorum_members
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_get_skip_list(
                    obj: *const dashcore_network_message_qrinfo_QuorumSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_set_skip_list_mode(
                    obj: *mut dashcore_network_message_qrinfo_QuorumSnapshot,
                    value : * mut crate :: fermented :: types :: dashcore :: network :: message_qrinfo :: dashcore_network_message_qrinfo_MNSkipListMode,
                ) {
                    (*obj).skip_list_mode = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_set_active_quorum_members(
                    obj: *mut dashcore_network_message_qrinfo_QuorumSnapshot,
                    value: *mut crate::fermented::generics::Vec_bool,
                ) {
                    (*obj).active_quorum_members = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_QuorumSnapshot_set_skip_list(
                    obj: *mut dashcore_network_message_qrinfo_QuorumSnapshot,
                    value: *mut crate::fermented::generics::Vec_i32,
                ) {
                    (*obj).skip_list = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`MNSkipListMode`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_network_message_qrinfo_MNSkipListMode {
                    NoSkipping = 0,
                    SkipFirst = 1,
                    SkipExcept = 2,
                    SkipAll = 3,
                }
                impl ferment::FFIConversionFrom<dashcore::network::message_qrinfo::MNSkipListMode>
                    for dashcore_network_message_qrinfo_MNSkipListMode
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_network_message_qrinfo_MNSkipListMode,
                    ) -> dashcore::network::message_qrinfo::MNSkipListMode {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            dashcore_network_message_qrinfo_MNSkipListMode::NoSkipping => {
                                dashcore::network::message_qrinfo::MNSkipListMode::NoSkipping
                            }
                            dashcore_network_message_qrinfo_MNSkipListMode::SkipFirst => {
                                dashcore::network::message_qrinfo::MNSkipListMode::SkipFirst
                            }
                            dashcore_network_message_qrinfo_MNSkipListMode::SkipExcept => {
                                dashcore::network::message_qrinfo::MNSkipListMode::SkipExcept
                            }
                            dashcore_network_message_qrinfo_MNSkipListMode::SkipAll => {
                                dashcore::network::message_qrinfo::MNSkipListMode::SkipAll
                            }
                        }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::network::message_qrinfo::MNSkipListMode>
                    for dashcore_network_message_qrinfo_MNSkipListMode
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::network::message_qrinfo::MNSkipListMode,
                    ) -> *const dashcore_network_message_qrinfo_MNSkipListMode {
                        ferment::boxed(match obj {
                            dashcore::network::message_qrinfo::MNSkipListMode::NoSkipping => {
                                dashcore_network_message_qrinfo_MNSkipListMode::NoSkipping
                            }
                            dashcore::network::message_qrinfo::MNSkipListMode::SkipFirst => {
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipFirst
                            }
                            dashcore::network::message_qrinfo::MNSkipListMode::SkipExcept => {
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipExcept
                            }
                            dashcore::network::message_qrinfo::MNSkipListMode::SkipAll => {
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipAll
                            }
                            _ => unreachable!("This is unreachable"),
                        })
                    }
                }
                impl Drop for dashcore_network_message_qrinfo_MNSkipListMode {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dashcore_network_message_qrinfo_MNSkipListMode::NoSkipping => {}
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipFirst => {}
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipExcept => {}
                                dashcore_network_message_qrinfo_MNSkipListMode::SkipAll => {}
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_MNSkipListMode_NoSkipping_ctor(
                ) -> *mut dashcore_network_message_qrinfo_MNSkipListMode {
                    ferment::boxed(dashcore_network_message_qrinfo_MNSkipListMode::NoSkipping {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_MNSkipListMode_SkipFirst_ctor(
                ) -> *mut dashcore_network_message_qrinfo_MNSkipListMode {
                    ferment::boxed(dashcore_network_message_qrinfo_MNSkipListMode::SkipFirst {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_MNSkipListMode_SkipExcept_ctor(
                ) -> *mut dashcore_network_message_qrinfo_MNSkipListMode {
                    ferment::boxed(dashcore_network_message_qrinfo_MNSkipListMode::SkipExcept {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_MNSkipListMode_SkipAll_ctor(
                ) -> *mut dashcore_network_message_qrinfo_MNSkipListMode {
                    ferment::boxed(dashcore_network_message_qrinfo_MNSkipListMode::SkipAll {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_network_message_qrinfo_MNSkipListMode_destroy(
                    ffi: *mut dashcore_network_message_qrinfo_MNSkipListMode,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
        }
        pub mod blockdata {
            pub mod script {
                pub mod owned {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`ScriptBuf`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_blockdata_script_owned_ScriptBuf(
                        *mut crate::fermented::generics::Vec_u8,
                    );
                    impl ferment::FFIConversionFrom<dashcore::blockdata::script::owned::ScriptBuf>
                        for dashcore_blockdata_script_owned_ScriptBuf
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const dashcore_blockdata_script_owned_ScriptBuf,
                        ) -> dashcore::blockdata::script::owned::ScriptBuf {
                            let ffi_ref = &*ffi;
                            dashcore :: blockdata :: script :: owned :: ScriptBuf (< crate :: fermented :: generics :: Vec_u8 as ferment :: FFIConversionFrom < Vec < u8 > >> :: ffi_from (ffi_ref . 0))
                        }
                    }
                    impl ferment::FFIConversionTo<dashcore::blockdata::script::owned::ScriptBuf>
                        for dashcore_blockdata_script_owned_ScriptBuf
                    {
                        unsafe fn ffi_to_const(
                            obj: dashcore::blockdata::script::owned::ScriptBuf,
                        ) -> *const dashcore_blockdata_script_owned_ScriptBuf
                        {
                            ferment::boxed(dashcore_blockdata_script_owned_ScriptBuf(
                                <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<
                                    Vec<u8>,
                                >>::ffi_to(obj.0),
                            ))
                        }
                    }
                    impl Drop for dashcore_blockdata_script_owned_ScriptBuf {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.0);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_script_owned_ScriptBuf_ctor(
                        o_0: *mut crate::fermented::generics::Vec_u8,
                    ) -> *mut dashcore_blockdata_script_owned_ScriptBuf {
                        ferment::boxed(dashcore_blockdata_script_owned_ScriptBuf(o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_script_owned_ScriptBuf_destroy(
                        ffi: *mut dashcore_blockdata_script_owned_ScriptBuf,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_script_owned_ScriptBuf_get_0(
                        obj: *const dashcore_blockdata_script_owned_ScriptBuf,
                    ) -> *mut crate::fermented::generics::Vec_u8 {
                        (*obj).0
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_script_owned_ScriptBuf_set_0(
                        obj: *mut dashcore_blockdata_script_owned_ScriptBuf,
                        value: *mut crate::fermented::generics::Vec_u8,
                    ) {
                        (*obj).0 = value;
                    }
                }
            }
            pub mod transaction {
                use crate as example_dashcore;
                pub mod outpoint {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`OutPoint`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_blockdata_transaction_outpoint_OutPoint {
                        pub txid: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                        pub vout: u32,
                    }
                    impl
                        ferment::FFIConversionFrom<
                            dashcore::blockdata::transaction::outpoint::OutPoint,
                        > for dashcore_blockdata_transaction_outpoint_OutPoint
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const dashcore_blockdata_transaction_outpoint_OutPoint,
                        ) -> dashcore::blockdata::transaction::outpoint::OutPoint
                        {
                            let ffi_ref = &*ffi;
                            dashcore :: blockdata :: transaction :: outpoint :: OutPoint { txid : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (ffi_ref . txid) , vout : ffi_ref . vout }
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            dashcore::blockdata::transaction::outpoint::OutPoint,
                        > for dashcore_blockdata_transaction_outpoint_OutPoint
                    {
                        unsafe fn ffi_to_const(
                            obj: dashcore::blockdata::transaction::outpoint::OutPoint,
                        ) -> *const dashcore_blockdata_transaction_outpoint_OutPoint
                        {
                            ferment :: boxed (dashcore_blockdata_transaction_outpoint_OutPoint { txid : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj . txid) , vout : obj . vout })
                        }
                    }
                    impl Drop for dashcore_blockdata_transaction_outpoint_OutPoint {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.txid);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_ctor(
                        txid: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                        vout: u32,
                    ) -> *mut dashcore_blockdata_transaction_outpoint_OutPoint {
                        ferment::boxed(dashcore_blockdata_transaction_outpoint_OutPoint {
                            txid,
                            vout,
                        })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_destroy(
                        ffi: *mut dashcore_blockdata_transaction_outpoint_OutPoint,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_get_txid(
                        obj: *const dashcore_blockdata_transaction_outpoint_OutPoint,
                    ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid
                    {
                        (*obj).txid
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_get_vout(
                        obj: *const dashcore_blockdata_transaction_outpoint_OutPoint,
                    ) -> u32 {
                        (*obj).vout
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_set_txid(
                        obj: *mut dashcore_blockdata_transaction_outpoint_OutPoint,
                        value: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                    ) {
                        (*obj).txid = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_set_vout(
                        obj: *mut dashcore_blockdata_transaction_outpoint_OutPoint,
                        value: u32,
                    ) {
                        (*obj).vout = value;
                    }
                    #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::outpoint::OutPoint::new`]"]
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_new (txid : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , vout : u32) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint{
                        let obj = dashcore :: blockdata :: transaction :: outpoint :: OutPoint :: new (< example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (txid) , vout) ;
                        < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: outpoint :: OutPoint >> :: ffi_to (obj)
                    }
                    #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::outpoint::OutPoint::null`]"]
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_null () -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint{
                        let obj = dashcore::blockdata::transaction::outpoint::OutPoint::null();
                        < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: outpoint :: OutPoint >> :: ffi_to (obj)
                    }
                    #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::outpoint::OutPoint::is_null`]"]
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_outpoint_OutPoint_is_null(
                        self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint,
                    ) -> bool {
                        let obj = dashcore :: blockdata :: transaction :: outpoint :: OutPoint :: is_null (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: outpoint :: dashcore_blockdata_transaction_outpoint_OutPoint as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: outpoint :: OutPoint >> :: ffi_from (self_)))) ;
                        obj
                    }
                }
                pub mod special_transaction {
                    use crate as example_dashcore;
                    pub mod asset_lock {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`AssetLockPayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { pub version : u8 , pub credit_outputs : * mut crate :: fermented :: generics :: Vec_dashcore_transaction_txout_TxOut }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload > for dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload { version : ffi_ref . version , credit_outputs : < crate :: fermented :: generics :: Vec_dashcore_transaction_txout_TxOut as ferment :: FFIConversionFrom < Vec < dashcore :: transaction :: txout :: TxOut > >> :: ffi_from (ffi_ref . credit_outputs) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload > for dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload) -> * const dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { version : obj . version , credit_outputs : < crate :: fermented :: generics :: Vec_dashcore_transaction_txout_TxOut as ferment :: FFIConversionTo < Vec < dashcore :: transaction :: txout :: TxOut > >> :: ffi_to (obj . credit_outputs) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.credit_outputs);
                                }
                            }
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_ctor (version : u8 , credit_outputs : * mut crate :: fermented :: generics :: Vec_dashcore_transaction_txout_TxOut) -> * mut dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload { version , credit_outputs })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                        ) -> u8 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_get_credit_outputs(
                            obj : * const dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                        ) -> *mut crate::fermented::generics::Vec_dashcore_transaction_txout_TxOut
                        {
                            (*obj).credit_outputs
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                            value: u8,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_set_credit_outputs(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                            value : * mut crate :: fermented :: generics :: Vec_dashcore_transaction_txout_TxOut,
                        ) {
                            (*obj).credit_outputs = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::asset_lock::AssetLockPayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod asset_unlock {
                        pub mod qualified_asset_unlock {
                            use crate as example_dashcore;
                            #[doc = "FFI-representation of the [`AssetUnlockPayload`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { pub base : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload , pub request_info : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo , pub quorum_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                            impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload > for dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload { base : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload >> :: ffi_from (ffi_ref . base) , request_info : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo >> :: ffi_from (ffi_ref . request_info) , quorum_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . quorum_sig) } } }
                            impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload > for dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload) -> * const dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { base : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload >> :: ffi_to (obj . base) , request_info : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo >> :: ffi_to (obj . request_info) , quorum_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . quorum_sig) }) } }
                            impl Drop for dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ferment :: unbox_any (ffi_ref . base) ; ferment :: unbox_any (ffi_ref . request_info) ; ferment :: unbox_any (ffi_ref . quorum_sig) ; } } }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_ctor (base : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload , request_info : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo , quorum_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) -> * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload{
                                ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload { base , request_info , quorum_sig })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_destroy(
                                ffi : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_get_base (obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload{
                                (*obj).base
                            }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_get_request_info (obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo{
                                (*obj).request_info
                            }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_get_quorum_sig (obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                                (*obj).quorum_sig
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_set_base(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload,
                                value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                            ) {
                                (*obj).base = value;
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_set_request_info(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload,
                                value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                            ) {
                                (*obj).request_info = value;
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload_set_quorum_sig(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload,
                                value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                            ) {
                                (*obj).quorum_sig = value;
                            }
                        }
                        pub mod request_info {
                            use crate as example_dashcore;
                            #[doc = "FFI-representation of the [`AssetUnlockRequestInfo`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo
                            {
                                pub request_height: u32,
                                pub quorum_hash: *mut dashcore::hash_types::QuorumHash,
                            }
                            impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo > for dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo) -> dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo { request_height : ffi_ref . request_height , quorum_hash : std :: ptr :: read (ffi_ref . quorum_hash) } } }
                            impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo > for dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: request_info :: AssetUnlockRequestInfo) -> * const dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { request_height : obj . request_height , quorum_hash : ferment :: boxed (obj . quorum_hash) }) } }
                            impl Drop for dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ferment :: unbox_any (ffi_ref . quorum_hash) ; } } }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_ctor (request_height : u32 , quorum_hash : * mut dashcore :: hash_types :: QuorumHash) -> * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo{
                                ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo { request_height , quorum_hash })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_destroy(
                                ffi : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_get_request_height(
                                obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                            ) -> u32 {
                                (*obj).request_height
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_get_quorum_hash(
                                obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                            ) -> *mut dashcore::hash_types::QuorumHash {
                                (*obj).quorum_hash
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_set_request_height(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                                value: u32,
                            ) {
                                (*obj).request_height = value;
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo_set_quorum_hash(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_request_info_AssetUnlockRequestInfo,
                                value: *mut dashcore::hash_types::QuorumHash,
                            ) {
                                (*obj).quorum_hash = value;
                            }
                        }
                        pub mod unqualified_asset_unlock {
                            use crate as example_dashcore;
                            #[doc = "FFI-representation of the [`AssetUnlockBasePayload`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload
                            {
                                pub version: u8,
                                pub index: u64,
                                pub fee: u32,
                            }
                            impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload > for dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload) -> dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload { version : ffi_ref . version , index : ffi_ref . index , fee : ffi_ref . fee } } }
                            impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload > for dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: unqualified_asset_unlock :: AssetUnlockBasePayload) -> * const dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { version : obj . version , index : obj . index , fee : obj . fee }) } }
                            impl Drop for dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ; ; } } }
                            #[no_mangle]                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_ctor (version : u8 , index : u64 , fee : u32) -> * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload{
                                ferment :: boxed (dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload { version , index , fee })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_destroy(
                                ffi : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_get_version(
                                obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                            ) -> u8 {
                                (*obj).version
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_get_index(
                                obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                            ) -> u64 {
                                (*obj).index
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_get_fee(
                                obj : * const dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                            ) -> u32 {
                                (*obj).fee
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_set_version(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                                value: u8,
                            ) {
                                (*obj).version = value;
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_set_index(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                                value: u64,
                            ) {
                                (*obj).index = value;
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload_set_fee(
                                obj : * mut dashcore_blockdata_transaction_special_transaction_asset_unlock_unqualified_asset_unlock_AssetUnlockBasePayload,
                                value: u32,
                            ) {
                                (*obj).fee = value;
                            }
                        }
                    }
                    pub mod coinbase {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`CoinbasePayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { pub version : u16 , pub height : u32 , pub merkle_root_masternode_list : * mut dashcore :: hash_types :: MerkleRootMasternodeList , pub merkle_root_quorums : * mut dashcore :: hash_types :: MerkleRootQuorums , pub best_cl_height : * mut u32 , pub best_cl_signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature , pub asset_locked_amount : * mut u64 }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload > for dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload) -> dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload { version : ffi_ref . version , height : ffi_ref . height , merkle_root_masternode_list : std :: ptr :: read (ffi_ref . merkle_root_masternode_list) , merkle_root_quorums : std :: ptr :: read (ffi_ref . merkle_root_quorums) , best_cl_height : ferment :: from_opt_primitive (ffi_ref . best_cl_height) , best_cl_signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from_opt (ffi_ref . best_cl_signature) , asset_locked_amount : ferment :: from_opt_primitive (ffi_ref . asset_locked_amount) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload > for dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload) -> * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { version : obj . version , height : obj . height , merkle_root_masternode_list : ferment :: boxed (obj . merkle_root_masternode_list) , merkle_root_quorums : ferment :: boxed (obj . merkle_root_quorums) , best_cl_height : ferment :: to_opt_primitive (obj . best_cl_height) , best_cl_signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to_opt (obj . best_cl_signature) , asset_locked_amount : ferment :: to_opt_primitive (obj . asset_locked_amount) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.merkle_root_masternode_list);
                                    ferment::unbox_any(ffi_ref.merkle_root_quorums);
                                    ferment::unbox_any_opt(ffi_ref.best_cl_height);
                                    ferment::unbox_any_opt(ffi_ref.best_cl_signature);
                                    ferment::unbox_any_opt(ffi_ref.asset_locked_amount);
                                }
                            }
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_ctor (version : u16 , height : u32 , merkle_root_masternode_list : * mut dashcore :: hash_types :: MerkleRootMasternodeList , merkle_root_quorums : * mut dashcore :: hash_types :: MerkleRootQuorums , best_cl_height : * mut u32 , best_cl_signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature , asset_locked_amount : * mut u64) -> * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload { version , height , merkle_root_masternode_list , merkle_root_quorums , best_cl_height , best_cl_signature , asset_locked_amount })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_height(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> u32 {
                            (*obj).height
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_merkle_root_masternode_list(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> *mut dashcore::hash_types::MerkleRootMasternodeList
                        {
                            (*obj).merkle_root_masternode_list
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_merkle_root_quorums(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> *mut dashcore::hash_types::MerkleRootQuorums {
                            (*obj).merkle_root_quorums
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_best_cl_height(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> *mut u32 {
                            (*obj).best_cl_height
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_best_cl_signature (obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                            (*obj).best_cl_signature
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_get_asset_locked_amount(
                            obj : * const dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> *mut u64 {
                            (*obj).asset_locked_amount
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_height(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: u32,
                        ) {
                            (*obj).height = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_merkle_root_masternode_list(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: *mut dashcore::hash_types::MerkleRootMasternodeList,
                        ) {
                            (*obj).merkle_root_masternode_list = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_merkle_root_quorums(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: *mut dashcore::hash_types::MerkleRootQuorums,
                        ) {
                            (*obj).merkle_root_quorums = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_best_cl_height(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: *mut u32,
                        ) {
                            (*obj).best_cl_height = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_best_cl_signature(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                        ) {
                            (*obj).best_cl_signature = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_set_asset_locked_amount(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                            value: *mut u64,
                        ) {
                            (*obj).asset_locked_amount = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::coinbase::CoinbasePayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod provider_registration {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ProviderMasternodeType`]\"`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        #[non_exhaustive]
                        pub enum dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType
                        {
                            Regular = 0,
                            HighPerformance = 1,
                        }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType > for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType) -> dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType { let ffi_ref = & * ffi ; match ffi_ref { dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: Regular => dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType :: Regular , dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: HighPerformance => dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType :: HighPerformance } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType > for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType) -> * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType { ferment :: boxed (match obj { dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType :: Regular => dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: Regular , dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType :: HighPerformance => dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: HighPerformance , _ => unreachable ! ("This is unreachable") }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType { fn drop (& mut self) { unsafe { match self { dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: Regular => { } , dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: HighPerformance => { } , _ => unreachable ! ("This is unreachable") } ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType_Regular_ctor () -> * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: Regular { })
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType_HighPerformance_ctor () -> * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType :: HighPerformance { })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[doc = "FFI-representation of the [`ProviderRegistrationPayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { pub version : u16 , pub masternode_type : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType , pub masternode_mode : u16 , pub collateral_outpoint : * mut dashcore :: transaction :: outpoint :: OutPoint , pub service_address : * mut std :: net :: SocketAddr , pub owner_key_hash : * mut dashcore :: hash_types :: PubkeyHash , pub operator_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , pub voting_key_hash : * mut dashcore :: hash_types :: PubkeyHash , pub operator_reward : u16 , pub script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , pub inputs_hash : * mut dashcore :: hash_types :: InputsHash , pub signature : * mut crate :: fermented :: generics :: Vec_u8 , pub platform_node_id : * mut dashcore :: hash_types :: PubkeyHash , pub platform_p2p_port : * mut u16 , pub platform_http_port : * mut u16 }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload > for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload { version : ffi_ref . version , masternode_type : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType >> :: ffi_from (ffi_ref . masternode_type) , masternode_mode : ffi_ref . masternode_mode , collateral_outpoint : std :: ptr :: read (ffi_ref . collateral_outpoint) , service_address : std :: ptr :: read (ffi_ref . service_address) , owner_key_hash : std :: ptr :: read (ffi_ref . owner_key_hash) , operator_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_from (ffi_ref . operator_public_key) , voting_key_hash : std :: ptr :: read (ffi_ref . voting_key_hash) , operator_reward : ffi_ref . operator_reward , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionFrom < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_from (ffi_ref . script_payout) , inputs_hash : std :: ptr :: read (ffi_ref . inputs_hash) , signature : < crate :: fermented :: generics :: Vec_u8 as ferment :: FFIConversionFrom < Vec < u8 > >> :: ffi_from (ffi_ref . signature) , platform_node_id : ferment :: from_opt_opaque (ffi_ref . platform_node_id) , platform_p2p_port : ferment :: from_opt_primitive (ffi_ref . platform_p2p_port) , platform_http_port : ferment :: from_opt_primitive (ffi_ref . platform_http_port) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload > for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload) -> * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { version : obj . version , masternode_type : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderMasternodeType >> :: ffi_to (obj . masternode_type) , masternode_mode : obj . masternode_mode , collateral_outpoint : ferment :: boxed (obj . collateral_outpoint) , service_address : ferment :: boxed (obj . service_address) , owner_key_hash : ferment :: boxed (obj . owner_key_hash) , operator_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_to (obj . operator_public_key) , voting_key_hash : ferment :: boxed (obj . voting_key_hash) , operator_reward : obj . operator_reward , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionTo < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_to (obj . script_payout) , inputs_hash : ferment :: boxed (obj . inputs_hash) , signature : < crate :: fermented :: generics :: Vec_u8 as ferment :: FFIConversionTo < Vec < u8 > >> :: ffi_to (obj . signature) , platform_node_id : ferment :: to_opt_primitive (obj . platform_node_id) , platform_p2p_port : ferment :: to_opt_primitive (obj . platform_p2p_port) , platform_http_port : ferment :: to_opt_primitive (obj . platform_http_port) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ferment :: unbox_any (ffi_ref . masternode_type) ; ; ferment :: unbox_any (ffi_ref . collateral_outpoint) ; ferment :: unbox_any (ffi_ref . service_address) ; ferment :: unbox_any (ffi_ref . owner_key_hash) ; ferment :: unbox_any (ffi_ref . operator_public_key) ; ferment :: unbox_any (ffi_ref . voting_key_hash) ; ; ferment :: unbox_any (ffi_ref . script_payout) ; ferment :: unbox_any (ffi_ref . inputs_hash) ; ferment :: unbox_any (ffi_ref . signature) ; ferment :: unbox_any_opt (ffi_ref . platform_node_id) ; ferment :: unbox_any_opt (ffi_ref . platform_p2p_port) ; ferment :: unbox_any_opt (ffi_ref . platform_http_port) ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_ctor (version : u16 , masternode_type : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType , masternode_mode : u16 , collateral_outpoint : * mut dashcore :: transaction :: outpoint :: OutPoint , service_address : * mut std :: net :: SocketAddr , owner_key_hash : * mut dashcore :: hash_types :: PubkeyHash , operator_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , voting_key_hash : * mut dashcore :: hash_types :: PubkeyHash , operator_reward : u16 , script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , inputs_hash : * mut dashcore :: hash_types :: InputsHash , signature : * mut crate :: fermented :: generics :: Vec_u8 , platform_node_id : * mut dashcore :: hash_types :: PubkeyHash , platform_p2p_port : * mut u16 , platform_http_port : * mut u16) -> * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload { version , masternode_type , masternode_mode , collateral_outpoint , service_address , owner_key_hash , operator_public_key , voting_key_hash , operator_reward , script_payout , inputs_hash , signature , platform_node_id , platform_p2p_port , platform_http_port })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_masternode_type (obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType{
                            (*obj).masternode_type
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_masternode_mode(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> u16 {
                            (*obj).masternode_mode
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_collateral_outpoint(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut dashcore::transaction::outpoint::OutPoint
                        {
                            (*obj).collateral_outpoint
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_service_address(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut std::net::SocketAddr {
                            (*obj).service_address
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_owner_key_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut dashcore::hash_types::PubkeyHash {
                            (*obj).owner_key_hash
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_operator_public_key (obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey{
                            (*obj).operator_public_key
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_voting_key_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut dashcore::hash_types::PubkeyHash {
                            (*obj).voting_key_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_operator_reward(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> u16 {
                            (*obj).operator_reward
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_script_payout (obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf{
                            (*obj).script_payout
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_inputs_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut dashcore::hash_types::InputsHash {
                            (*obj).inputs_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_signature(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut crate::fermented::generics::Vec_u8 {
                            (*obj).signature
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_platform_node_id(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut dashcore::hash_types::PubkeyHash {
                            (*obj).platform_node_id
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_platform_p2p_port(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut u16 {
                            (*obj).platform_p2p_port
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_get_platform_http_port(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> *mut u16 {
                            (*obj).platform_http_port
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_masternode_type(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderMasternodeType,
                        ) {
                            (*obj).masternode_type = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_masternode_mode(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: u16,
                        ) {
                            (*obj).masternode_mode = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_collateral_outpoint(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut dashcore::transaction::outpoint::OutPoint,
                        ) {
                            (*obj).collateral_outpoint = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_service_address(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut std::net::SocketAddr,
                        ) {
                            (*obj).service_address = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_owner_key_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut dashcore::hash_types::PubkeyHash,
                        ) {
                            (*obj).owner_key_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_operator_public_key(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey,
                        ) {
                            (*obj).operator_public_key = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_voting_key_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut dashcore::hash_types::PubkeyHash,
                        ) {
                            (*obj).voting_key_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_operator_reward(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: u16,
                        ) {
                            (*obj).operator_reward = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_script_payout(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                        ) {
                            (*obj).script_payout = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_inputs_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut dashcore::hash_types::InputsHash,
                        ) {
                            (*obj).inputs_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_signature(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut crate::fermented::generics::Vec_u8,
                        ) {
                            (*obj).signature = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_platform_node_id(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut dashcore::hash_types::PubkeyHash,
                        ) {
                            (*obj).platform_node_id = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_platform_p2p_port(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut u16,
                        ) {
                            (*obj).platform_p2p_port = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_set_platform_http_port(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                            value: *mut u16,
                        ) {
                            (*obj).platform_http_port = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::provider_registration::ProviderRegistrationPayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod provider_update_registrar {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`ProviderUpdateRegistrarPayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { pub version : u16 , pub pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , pub provider_mode : u16 , pub operator_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , pub voting_key_hash : * mut dashcore :: hash_types :: PubkeyHash , pub script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , pub inputs_hash : * mut dashcore :: hash_types :: InputsHash , pub payload_sig : * mut crate :: fermented :: generics :: Vec_u8 }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload > for dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload { version : ffi_ref . version , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (ffi_ref . pro_tx_hash) , provider_mode : ffi_ref . provider_mode , operator_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_from (ffi_ref . operator_public_key) , voting_key_hash : std :: ptr :: read (ffi_ref . voting_key_hash) , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionFrom < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_from (ffi_ref . script_payout) , inputs_hash : std :: ptr :: read (ffi_ref . inputs_hash) , payload_sig : < crate :: fermented :: generics :: Vec_u8 as ferment :: FFIConversionFrom < Vec < u8 > >> :: ffi_from (ffi_ref . payload_sig) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload > for dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload) -> * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { version : obj . version , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj . pro_tx_hash) , provider_mode : obj . provider_mode , operator_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_to (obj . operator_public_key) , voting_key_hash : ferment :: boxed (obj . voting_key_hash) , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionTo < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_to (obj . script_payout) , inputs_hash : ferment :: boxed (obj . inputs_hash) , payload_sig : < crate :: fermented :: generics :: Vec_u8 as ferment :: FFIConversionTo < Vec < u8 > >> :: ffi_to (obj . payload_sig) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ferment :: unbox_any (ffi_ref . pro_tx_hash) ; ; ferment :: unbox_any (ffi_ref . operator_public_key) ; ferment :: unbox_any (ffi_ref . voting_key_hash) ; ferment :: unbox_any (ffi_ref . script_payout) ; ferment :: unbox_any (ffi_ref . inputs_hash) ; ferment :: unbox_any (ffi_ref . payload_sig) ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_ctor (version : u16 , pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , provider_mode : u16 , operator_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , voting_key_hash : * mut dashcore :: hash_types :: PubkeyHash , script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , inputs_hash : * mut dashcore :: hash_types :: InputsHash , payload_sig : * mut crate :: fermented :: generics :: Vec_u8) -> * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload { version , pro_tx_hash , provider_mode , operator_public_key , voting_key_hash , script_payout , inputs_hash , payload_sig })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_pro_tx_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid
                        {
                            (*obj).pro_tx_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_provider_mode(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> u16 {
                            (*obj).provider_mode
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_operator_public_key (obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey{
                            (*obj).operator_public_key
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_voting_key_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> *mut dashcore::hash_types::PubkeyHash {
                            (*obj).voting_key_hash
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_script_payout (obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf{
                            (*obj).script_payout
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_inputs_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> *mut dashcore::hash_types::InputsHash {
                            (*obj).inputs_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_get_payload_sig(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> *mut crate::fermented::generics::Vec_u8 {
                            (*obj).payload_sig
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_pro_tx_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                        ) {
                            (*obj).pro_tx_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_provider_mode(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: u16,
                        ) {
                            (*obj).provider_mode = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_operator_public_key(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey,
                        ) {
                            (*obj).operator_public_key = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_voting_key_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: *mut dashcore::hash_types::PubkeyHash,
                        ) {
                            (*obj).voting_key_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_script_payout(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                        ) {
                            (*obj).script_payout = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_inputs_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: *mut dashcore::hash_types::InputsHash,
                        ) {
                            (*obj).inputs_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_set_payload_sig(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                            value: *mut crate::fermented::generics::Vec_u8,
                        ) {
                            (*obj).payload_sig = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::provider_update_registrar::ProviderUpdateRegistrarPayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod provider_update_revocation {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`ProviderUpdateRevocationPayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { pub version : u16 , pub pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , pub reason : u16 , pub inputs_hash : * mut dashcore :: hash_types :: InputsHash , pub payload_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload > for dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload { version : ffi_ref . version , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (ffi_ref . pro_tx_hash) , reason : ffi_ref . reason , inputs_hash : std :: ptr :: read (ffi_ref . inputs_hash) , payload_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . payload_sig) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload > for dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload) -> * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { version : obj . version , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj . pro_tx_hash) , reason : obj . reason , inputs_hash : ferment :: boxed (obj . inputs_hash) , payload_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . payload_sig) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ferment :: unbox_any (ffi_ref . pro_tx_hash) ; ; ferment :: unbox_any (ffi_ref . inputs_hash) ; ferment :: unbox_any (ffi_ref . payload_sig) ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_ctor (version : u16 , pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , reason : u16 , inputs_hash : * mut dashcore :: hash_types :: InputsHash , payload_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) -> * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload { version , pro_tx_hash , reason , inputs_hash , payload_sig })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_get_pro_tx_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid
                        {
                            (*obj).pro_tx_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_get_reason(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) -> u16 {
                            (*obj).reason
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_get_inputs_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) -> *mut dashcore::hash_types::InputsHash {
                            (*obj).inputs_hash
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_get_payload_sig (obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                            (*obj).payload_sig
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_set_pro_tx_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                            value: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                        ) {
                            (*obj).pro_tx_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_set_reason(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                            value: u16,
                        ) {
                            (*obj).reason = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_set_inputs_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                            value: *mut dashcore::hash_types::InputsHash,
                        ) {
                            (*obj).inputs_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_set_payload_sig(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                        ) {
                            (*obj).payload_sig = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::provider_update_revocation::ProviderUpdateRevocationPayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod provider_update_service {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`ProviderUpdateServicePayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { pub version : u16 , pub mn_type : * mut u16 , pub pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , pub ip_address : * mut [u8 ; 16] , pub port : u16 , pub script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , pub inputs_hash : * mut dashcore :: hash_types :: InputsHash , pub platform_node_id : * mut crate :: fermented :: generics :: Arr_u8_20 , pub platform_p2p_port : * mut u16 , pub platform_http_port : * mut u16 , pub payload_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload > for dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload) -> dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload { version : ffi_ref . version , mn_type : ferment :: from_opt_primitive (ffi_ref . mn_type) , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (ffi_ref . pro_tx_hash) , ip_address : < [u8 ; 16] as ferment :: FFIConversionFrom < u128 >> :: ffi_from (ffi_ref . ip_address) , port : ffi_ref . port , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionFrom < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_from (ffi_ref . script_payout) , inputs_hash : std :: ptr :: read (ffi_ref . inputs_hash) , platform_node_id : < crate :: fermented :: generics :: Arr_u8_20 as ferment :: FFIConversionFrom < [u8 ; 20] >> :: ffi_from_opt (ffi_ref . platform_node_id) , platform_p2p_port : ferment :: from_opt_primitive (ffi_ref . platform_p2p_port) , platform_http_port : ferment :: from_opt_primitive (ffi_ref . platform_http_port) , payload_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . payload_sig) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload > for dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload) -> * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { version : obj . version , mn_type : ferment :: to_opt_primitive (obj . mn_type) , pro_tx_hash : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj . pro_tx_hash) , ip_address : < [u8 ; 16] as ferment :: FFIConversionTo < u128 >> :: ffi_to (obj . ip_address) , port : obj . port , script_payout : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionTo < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_to (obj . script_payout) , inputs_hash : ferment :: boxed (obj . inputs_hash) , platform_node_id : < crate :: fermented :: generics :: Arr_u8_20 as ferment :: FFIConversionTo < [u8 ; 20] >> :: ffi_to_opt (obj . platform_node_id) , platform_p2p_port : ferment :: to_opt_primitive (obj . platform_p2p_port) , platform_http_port : ferment :: to_opt_primitive (obj . platform_http_port) , payload_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . payload_sig) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ferment :: unbox_any_opt (ffi_ref . mn_type) ; ferment :: unbox_any (ffi_ref . pro_tx_hash) ; ferment :: unbox_any_opt (ffi_ref . ip_address) ; ; ferment :: unbox_any (ffi_ref . script_payout) ; ferment :: unbox_any (ffi_ref . inputs_hash) ; ferment :: unbox_any_opt (ffi_ref . platform_node_id) ; ferment :: unbox_any_opt (ffi_ref . platform_p2p_port) ; ferment :: unbox_any_opt (ffi_ref . platform_http_port) ; ferment :: unbox_any (ffi_ref . payload_sig) ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_ctor (version : u16 , mn_type : * mut u16 , pro_tx_hash : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , ip_address : * mut [u8 ; 16] , port : u16 , script_payout : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , inputs_hash : * mut dashcore :: hash_types :: InputsHash , platform_node_id : * mut crate :: fermented :: generics :: Arr_u8_20 , platform_p2p_port : * mut u16 , platform_http_port : * mut u16 , payload_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) -> * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload { version , mn_type , pro_tx_hash , ip_address , port , script_payout , inputs_hash , platform_node_id , platform_p2p_port , platform_http_port , payload_sig })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_mn_type(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut u16 {
                            (*obj).mn_type
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_pro_tx_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid
                        {
                            (*obj).pro_tx_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_ip_address(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut [u8; 16] {
                            (*obj).ip_address
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_port(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> u16 {
                            (*obj).port
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_script_payout (obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf{
                            (*obj).script_payout
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_inputs_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut dashcore::hash_types::InputsHash {
                            (*obj).inputs_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_platform_node_id(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut crate::fermented::generics::Arr_u8_20 {
                            (*obj).platform_node_id
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_platform_p2p_port(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut u16 {
                            (*obj).platform_p2p_port
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_platform_http_port(
                            obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> *mut u16 {
                            (*obj).platform_http_port
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_get_payload_sig (obj : * const dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                            (*obj).payload_sig
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_mn_type(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut u16,
                        ) {
                            (*obj).mn_type = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_pro_tx_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                        ) {
                            (*obj).pro_tx_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_ip_address(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut [u8; 16],
                        ) {
                            (*obj).ip_address = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_port(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: u16,
                        ) {
                            (*obj).port = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_script_payout(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                        ) {
                            (*obj).script_payout = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_inputs_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut dashcore::hash_types::InputsHash,
                        ) {
                            (*obj).inputs_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_platform_node_id(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut crate::fermented::generics::Arr_u8_20,
                        ) {
                            (*obj).platform_node_id = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_platform_p2p_port(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut u16,
                        ) {
                            (*obj).platform_p2p_port = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_platform_http_port(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value: *mut u16,
                        ) {
                            (*obj).platform_http_port = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_set_payload_sig(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                        ) {
                            (*obj).payload_sig = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::provider_update_service::ProviderUpdateServicePayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    pub mod quorum_commitment {
                        use crate as example_dashcore;
                        #[doc = "FFI-representation of the [`QuorumEntry`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { pub version : u16 , pub llmq_type : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType , pub quorum_hash : * mut dashcore :: hash_types :: QuorumHash , pub quorum_index : * mut i16 , pub signers : * mut crate :: fermented :: generics :: Vec_bool , pub valid_members : * mut crate :: fermented :: generics :: Vec_bool , pub quorum_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , pub quorum_vvec_hash : * mut dashcore :: hash_types :: QuorumVVecHash , pub threshold_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature , pub all_commitment_aggregated_signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry > for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry { version : ffi_ref . version , llmq_type : < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (ffi_ref . llmq_type) , quorum_hash : std :: ptr :: read (ffi_ref . quorum_hash) , quorum_index : ferment :: from_opt_primitive (ffi_ref . quorum_index) , signers : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionFrom < Vec < bool > >> :: ffi_from (ffi_ref . signers) , valid_members : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionFrom < Vec < bool > >> :: ffi_from (ffi_ref . valid_members) , quorum_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_from (ffi_ref . quorum_public_key) , quorum_vvec_hash : std :: ptr :: read (ffi_ref . quorum_vvec_hash) , threshold_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . threshold_sig) , all_commitment_aggregated_signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . all_commitment_aggregated_signature) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry > for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry) -> * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { version : obj . version , llmq_type : < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionTo < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_to (obj . llmq_type) , quorum_hash : ferment :: boxed (obj . quorum_hash) , quorum_index : ferment :: to_opt_primitive (obj . quorum_index) , signers : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionTo < Vec < bool > >> :: ffi_to (obj . signers) , valid_members : < crate :: fermented :: generics :: Vec_bool as ferment :: FFIConversionTo < Vec < bool > >> :: ffi_to (obj . valid_members) , quorum_public_key : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_to (obj . quorum_public_key) , quorum_vvec_hash : ferment :: boxed (obj . quorum_vvec_hash) , threshold_sig : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . threshold_sig) , all_commitment_aggregated_signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . all_commitment_aggregated_signature) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry {
                            fn drop(&mut self) {
                                unsafe {
                                    let ffi_ref = self;
                                    ferment::unbox_any(ffi_ref.llmq_type);
                                    ferment::unbox_any(ffi_ref.quorum_hash);
                                    ferment::unbox_any_opt(ffi_ref.quorum_index);
                                    ferment::unbox_any(ffi_ref.signers);
                                    ferment::unbox_any(ffi_ref.valid_members);
                                    ferment::unbox_any(ffi_ref.quorum_public_key);
                                    ferment::unbox_any(ffi_ref.quorum_vvec_hash);
                                    ferment::unbox_any(ffi_ref.threshold_sig);
                                    ferment::unbox_any(ffi_ref.all_commitment_aggregated_signature);
                                }
                            }
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_ctor (version : u16 , llmq_type : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType , quorum_hash : * mut dashcore :: hash_types :: QuorumHash , quorum_index : * mut i16 , signers : * mut crate :: fermented :: generics :: Vec_bool , valid_members : * mut crate :: fermented :: generics :: Vec_bool , quorum_public_key : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , quorum_vvec_hash : * mut dashcore :: hash_types :: QuorumVVecHash , threshold_sig : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature , all_commitment_aggregated_signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) -> * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry { version , llmq_type , quorum_hash , quorum_index , signers , valid_members , quorum_public_key , quorum_vvec_hash , threshold_sig , all_commitment_aggregated_signature })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_llmq_type (obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType{
                            (*obj).llmq_type
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_quorum_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> *mut dashcore::hash_types::QuorumHash {
                            (*obj).quorum_hash
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_quorum_index(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> *mut i16 {
                            (*obj).quorum_index
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_signers(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> *mut crate::fermented::generics::Vec_bool {
                            (*obj).signers
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_valid_members(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> *mut crate::fermented::generics::Vec_bool {
                            (*obj).valid_members
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_quorum_public_key (obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey{
                            (*obj).quorum_public_key
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_quorum_vvec_hash(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> *mut dashcore::hash_types::QuorumVVecHash {
                            (*obj).quorum_vvec_hash
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_threshold_sig (obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                            (*obj).threshold_sig
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_get_all_commitment_aggregated_signature (obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                            (*obj).all_commitment_aggregated_signature
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_llmq_type(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                        ) {
                            (*obj).llmq_type = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_quorum_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: *mut dashcore::hash_types::QuorumHash,
                        ) {
                            (*obj).quorum_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_quorum_index(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: *mut i16,
                        ) {
                            (*obj).quorum_index = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_signers(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: *mut crate::fermented::generics::Vec_bool,
                        ) {
                            (*obj).signers = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_valid_members(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: *mut crate::fermented::generics::Vec_bool,
                        ) {
                            (*obj).valid_members = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_quorum_public_key(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey,
                        ) {
                            (*obj).quorum_public_key = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_quorum_vvec_hash(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value: *mut dashcore::hash_types::QuorumVVecHash,
                        ) {
                            (*obj).quorum_vvec_hash = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_threshold_sig(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                        ) {
                            (*obj).threshold_sig = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_set_all_commitment_aggregated_signature(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                            value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                        ) {
                            (*obj).all_commitment_aggregated_signature = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::quorum_commitment::QuorumEntry::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry >> :: ffi_from (self_)))) ;
                            obj
                        }
                        #[doc = "FFI-representation of the [`QuorumCommitmentPayload`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        pub struct dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { pub version : u16 , pub height : u32 , pub finalization_commitment : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry }
                        impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload > for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload { let ffi_ref = & * ffi ; dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload { version : ffi_ref . version , height : ffi_ref . height , finalization_commitment : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry >> :: ffi_from (ffi_ref . finalization_commitment) } } }
                        impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload > for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload) -> * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { ferment :: boxed (dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { version : obj . version , height : obj . height , finalization_commitment : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry >> :: ffi_to (obj . finalization_commitment) }) } }
                        impl Drop for dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { fn drop (& mut self) { unsafe { let ffi_ref = self ; ; ; ferment :: unbox_any (ffi_ref . finalization_commitment) ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_ctor (version : u16 , height : u32 , finalization_commitment : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry) -> * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload{
                            ferment :: boxed (dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload { version , height , finalization_commitment })
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_destroy(
                            ffi : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_get_version(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                        ) -> u16 {
                            (*obj).version
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_get_height(
                            obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                        ) -> u32 {
                            (*obj).height
                        }
                        #[no_mangle]                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_get_finalization_commitment (obj : * const dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry{
                            (*obj).finalization_commitment
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_set_version(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                            value: u16,
                        ) {
                            (*obj).version = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_set_height(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                            value: u32,
                        ) {
                            (*obj).height = value;
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_set_finalization_commitment(
                            obj : * mut dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                            value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumEntry,
                        ) {
                            (*obj).finalization_commitment = value;
                        }
                        #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::quorum_commitment::QuorumCommitmentPayload::size`]"]
                        #[no_mangle]
                        pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload_size(
                            self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                        ) -> usize {
                            let obj = dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload :: size (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload >> :: ffi_from (self_)))) ;
                            obj
                        }
                    }
                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`TransactionPayload`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dashcore_blockdata_transaction_special_transaction_TransactionPayload {
                        ProviderRegistrationPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload) , ProviderUpdateServicePayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload) , ProviderUpdateRegistrarPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload) , ProviderUpdateRevocationPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload) , CoinbasePayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload) , QuorumCommitmentPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload) , MnhfSignalPayloadType (* mut dashcore :: blockdata :: transaction :: special_transaction :: mnhf_signal :: MnhfSignalPayload) , AssetLockPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload) , AssetUnlockPayloadType (* mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload) }
                    impl ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload > for dashcore_blockdata_transaction_special_transaction_TransactionPayload { unsafe fn ffi_from_const (ffi : * const dashcore_blockdata_transaction_special_transaction_TransactionPayload) -> dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload { let ffi_ref = & * ffi ; match ffi_ref { dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderRegistrationPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderRegistrationPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateServicePayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateServicePayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRegistrarPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateRegistrarPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRevocationPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateRevocationPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: CoinbasePayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: CoinbasePayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: QuorumCommitmentPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: QuorumCommitmentPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: MnhfSignalPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: MnhfSignalPayloadType (std :: ptr :: read (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetLockPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: AssetLockPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload >> :: ffi_from (* o_0)) , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetUnlockPayloadType (o_0) => dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: AssetUnlockPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload >> :: ffi_from (* o_0)) } } }
                    impl ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload > for dashcore_blockdata_transaction_special_transaction_TransactionPayload { unsafe fn ffi_to_const (obj : dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload) -> * const dashcore_blockdata_transaction_special_transaction_TransactionPayload { ferment :: boxed (match obj { dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderRegistrationPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderRegistrationPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: ProviderRegistrationPayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateServicePayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateServicePayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: ProviderUpdateServicePayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateRegistrarPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRegistrarPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: ProviderUpdateRegistrarPayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: ProviderUpdateRevocationPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRevocationPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: ProviderUpdateRevocationPayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: CoinbasePayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: CoinbasePayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: CoinbasePayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: QuorumCommitmentPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: QuorumCommitmentPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: QuorumCommitmentPayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: MnhfSignalPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: MnhfSignalPayloadType (ferment :: boxed (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: AssetLockPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetLockPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: AssetLockPayload >> :: ffi_to (o_0)) , dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: AssetUnlockPayloadType (o_0) => dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetUnlockPayloadType (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: AssetUnlockPayload >> :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") }) } }
                    impl Drop for dashcore_blockdata_transaction_special_transaction_TransactionPayload {
                        fn drop(&mut self) {
                            unsafe {
                                match self { dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderRegistrationPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateServicePayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRegistrarPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRevocationPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: CoinbasePayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: QuorumCommitmentPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: MnhfSignalPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetLockPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetUnlockPayloadType (o_0) => { ferment :: unbox_any (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_ProviderRegistrationPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_registration :: dashcore_blockdata_transaction_special_transaction_provider_registration_ProviderRegistrationPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderRegistrationPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_ProviderUpdateServicePayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_service :: dashcore_blockdata_transaction_special_transaction_provider_update_service_ProviderUpdateServicePayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateServicePayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_ProviderUpdateRegistrarPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_registrar :: dashcore_blockdata_transaction_special_transaction_provider_update_registrar_ProviderUpdateRegistrarPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRegistrarPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_ProviderUpdateRevocationPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: provider_update_revocation :: dashcore_blockdata_transaction_special_transaction_provider_update_revocation_ProviderUpdateRevocationPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: ProviderUpdateRevocationPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_CoinbasePayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: coinbase :: dashcore_blockdata_transaction_special_transaction_coinbase_CoinbasePayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: CoinbasePayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_QuorumCommitmentPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: quorum_commitment :: dashcore_blockdata_transaction_special_transaction_quorum_commitment_QuorumCommitmentPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: QuorumCommitmentPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_MnhfSignalPayloadType_ctor(
                        o_o_0 : * mut dashcore :: blockdata :: transaction :: special_transaction :: mnhf_signal :: MnhfSignalPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: MnhfSignalPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_AssetLockPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_lock :: dashcore_blockdata_transaction_special_transaction_asset_lock_AssetLockPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetLockPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_AssetUnlockPayloadType_ctor(
                        o_o_0 : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: asset_unlock :: qualified_asset_unlock :: dashcore_blockdata_transaction_special_transaction_asset_unlock_qualified_asset_unlock_AssetUnlockPayload,
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionPayload
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionPayload :: AssetUnlockPayloadType (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_destroy(
                        ffi : * mut dashcore_blockdata_transaction_special_transaction_TransactionPayload,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::TransactionPayload::get_type`]"]
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_get_type (self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionType{
                        let obj = dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: get_type (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload >> :: ffi_from (self_)))) ;
                        < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionType as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: TransactionType >> :: ffi_to (obj)
                    }
                    #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::special_transaction::TransactionPayload::len`]"]
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionPayload_len(
                        self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload,
                    ) -> usize {
                        let obj = dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload :: len (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload >> :: ffi_from (self_)))) ;
                        obj
                    }
                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`TransactionType`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dashcore_blockdata_transaction_special_transaction_TransactionType {
                        Classic = 0,
                        ProviderRegistration = 1,
                        ProviderUpdateService = 2,
                        ProviderUpdateRegistrar = 3,
                        ProviderUpdateRevocation = 4,
                        Coinbase = 5,
                        QuorumCommitment = 6,
                        MnhfSignal = 7,
                        AssetLock = 8,
                        AssetUnlock = 9,
                    }
                    impl
                        ferment::FFIConversionFrom<
                            dashcore::blockdata::transaction::special_transaction::TransactionType,
                        > for dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const dashcore_blockdata_transaction_special_transaction_TransactionType,
                        ) -> dashcore::blockdata::transaction::special_transaction::TransactionType
                        {
                            let ffi_ref = &*ffi;
                            match ffi_ref { dashcore_blockdata_transaction_special_transaction_TransactionType :: Classic => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: Classic , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderRegistration => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderRegistration , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateService => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateService , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRegistrar => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateRegistrar , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRevocation => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateRevocation , dashcore_blockdata_transaction_special_transaction_TransactionType :: Coinbase => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: Coinbase , dashcore_blockdata_transaction_special_transaction_TransactionType :: QuorumCommitment => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: QuorumCommitment , dashcore_blockdata_transaction_special_transaction_TransactionType :: MnhfSignal => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: MnhfSignal , dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetLock => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: AssetLock , dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetUnlock => dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: AssetUnlock }
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            dashcore::blockdata::transaction::special_transaction::TransactionType,
                        > for dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        unsafe fn ffi_to_const(
                            obj : dashcore :: blockdata :: transaction :: special_transaction :: TransactionType,
                        ) -> *const dashcore_blockdata_transaction_special_transaction_TransactionType
                        {
                            ferment :: boxed (match obj { dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: Classic => dashcore_blockdata_transaction_special_transaction_TransactionType :: Classic , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderRegistration => dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderRegistration , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateService => dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateService , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateRegistrar => dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRegistrar , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: ProviderUpdateRevocation => dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRevocation , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: Coinbase => dashcore_blockdata_transaction_special_transaction_TransactionType :: Coinbase , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: QuorumCommitment => dashcore_blockdata_transaction_special_transaction_TransactionType :: QuorumCommitment , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: MnhfSignal => dashcore_blockdata_transaction_special_transaction_TransactionType :: MnhfSignal , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: AssetLock => dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetLock , dashcore :: blockdata :: transaction :: special_transaction :: TransactionType :: AssetUnlock => dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetUnlock , _ => unreachable ! ("This is unreachable") })
                        }
                    }
                    impl Drop for dashcore_blockdata_transaction_special_transaction_TransactionType {
                        fn drop(&mut self) {
                            unsafe {
                                match self { dashcore_blockdata_transaction_special_transaction_TransactionType :: Classic => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderRegistration => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateService => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRegistrar => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRevocation => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: Coinbase => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: QuorumCommitment => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: MnhfSignal => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetLock => { } , dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetUnlock => { } , _ => unreachable ! ("This is unreachable") } ;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_Classic_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: Classic { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_ProviderRegistration_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderRegistration { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_ProviderUpdateService_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateService { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_ProviderUpdateRegistrar_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRegistrar { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_ProviderUpdateRevocation_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: ProviderUpdateRevocation { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_Coinbase_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: Coinbase { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_QuorumCommitment_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: QuorumCommitment { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_MnhfSignal_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: MnhfSignal { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_AssetLock_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetLock { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_AssetUnlock_ctor(
                    ) -> *mut dashcore_blockdata_transaction_special_transaction_TransactionType
                    {
                        ferment :: boxed (dashcore_blockdata_transaction_special_transaction_TransactionType :: AssetUnlock { })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_special_transaction_TransactionType_destroy(
                        ffi : * mut dashcore_blockdata_transaction_special_transaction_TransactionType,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                pub mod txin {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`TxIn`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_blockdata_transaction_txin_TxIn { pub previous_output : * mut dashcore :: transaction :: outpoint :: OutPoint , pub script_sig : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf , pub sequence : u32 , pub witness : * mut crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness }
                    impl ferment::FFIConversionFrom<dashcore::blockdata::transaction::txin::TxIn>
                        for dashcore_blockdata_transaction_txin_TxIn
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const dashcore_blockdata_transaction_txin_TxIn,
                        ) -> dashcore::blockdata::transaction::txin::TxIn {
                            let ffi_ref = &*ffi;
                            dashcore :: blockdata :: transaction :: txin :: TxIn { previous_output : std :: ptr :: read (ffi_ref . previous_output) , script_sig : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionFrom < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_from (ffi_ref . script_sig) , sequence : ffi_ref . sequence , witness : < crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness as ferment :: FFIConversionFrom < dashcore :: blockdata :: witness :: Witness >> :: ffi_from (ffi_ref . witness) }
                        }
                    }
                    impl ferment::FFIConversionTo<dashcore::blockdata::transaction::txin::TxIn>
                        for dashcore_blockdata_transaction_txin_TxIn
                    {
                        unsafe fn ffi_to_const(
                            obj: dashcore::blockdata::transaction::txin::TxIn,
                        ) -> *const dashcore_blockdata_transaction_txin_TxIn
                        {
                            ferment :: boxed (dashcore_blockdata_transaction_txin_TxIn { previous_output : ferment :: boxed (obj . previous_output) , script_sig : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionTo < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_to (obj . script_sig) , sequence : obj . sequence , witness : < crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness as ferment :: FFIConversionTo < dashcore :: blockdata :: witness :: Witness >> :: ffi_to (obj . witness) })
                        }
                    }
                    impl Drop for dashcore_blockdata_transaction_txin_TxIn {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.previous_output);
                                ferment::unbox_any(ffi_ref.script_sig);
                                ferment::unbox_any(ffi_ref.witness);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_ctor(
                        previous_output: *mut dashcore::transaction::outpoint::OutPoint,
                        script_sig : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                        sequence: u32,
                        witness : * mut crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness,
                    ) -> *mut dashcore_blockdata_transaction_txin_TxIn {
                        ferment::boxed(dashcore_blockdata_transaction_txin_TxIn {
                            previous_output,
                            script_sig,
                            sequence,
                            witness,
                        })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_destroy(
                        ffi: *mut dashcore_blockdata_transaction_txin_TxIn,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_get_previous_output(
                        obj: *const dashcore_blockdata_transaction_txin_TxIn,
                    ) -> *mut dashcore::transaction::outpoint::OutPoint {
                        (*obj).previous_output
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_get_script_sig (obj : * const dashcore_blockdata_transaction_txin_TxIn) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf{
                        (*obj).script_sig
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_get_sequence(
                        obj: *const dashcore_blockdata_transaction_txin_TxIn,
                    ) -> u32 {
                        (*obj).sequence
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_get_witness (obj : * const dashcore_blockdata_transaction_txin_TxIn) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness{
                        (*obj).witness
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_set_previous_output(
                        obj: *mut dashcore_blockdata_transaction_txin_TxIn,
                        value: *mut dashcore::transaction::outpoint::OutPoint,
                    ) {
                        (*obj).previous_output = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_set_script_sig(
                        obj: *mut dashcore_blockdata_transaction_txin_TxIn,
                        value : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                    ) {
                        (*obj).script_sig = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_set_sequence(
                        obj: *mut dashcore_blockdata_transaction_txin_TxIn,
                        value: u32,
                    ) {
                        (*obj).sequence = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txin_TxIn_set_witness(
                        obj: *mut dashcore_blockdata_transaction_txin_TxIn,
                        value : * mut crate :: fermented :: types :: dashcore :: blockdata :: witness :: dashcore_blockdata_witness_Witness,
                    ) {
                        (*obj).witness = value;
                    }
                }
                pub mod txout {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`TxOut`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_blockdata_transaction_txout_TxOut { pub value : u64 , pub script_pubkey : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf }
                    impl ferment::FFIConversionFrom<dashcore::blockdata::transaction::txout::TxOut>
                        for dashcore_blockdata_transaction_txout_TxOut
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const dashcore_blockdata_transaction_txout_TxOut,
                        ) -> dashcore::blockdata::transaction::txout::TxOut
                        {
                            let ffi_ref = &*ffi;
                            dashcore :: blockdata :: transaction :: txout :: TxOut { value : ffi_ref . value , script_pubkey : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionFrom < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_from (ffi_ref . script_pubkey) }
                        }
                    }
                    impl ferment::FFIConversionTo<dashcore::blockdata::transaction::txout::TxOut>
                        for dashcore_blockdata_transaction_txout_TxOut
                    {
                        unsafe fn ffi_to_const(
                            obj: dashcore::blockdata::transaction::txout::TxOut,
                        ) -> *const dashcore_blockdata_transaction_txout_TxOut
                        {
                            ferment :: boxed (dashcore_blockdata_transaction_txout_TxOut { value : obj . value , script_pubkey : < crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf as ferment :: FFIConversionTo < dashcore :: blockdata :: script :: owned :: ScriptBuf >> :: ffi_to (obj . script_pubkey) })
                        }
                    }
                    impl Drop for dashcore_blockdata_transaction_txout_TxOut {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.script_pubkey);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_ctor(
                        value: u64,
                        script_pubkey : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                    ) -> *mut dashcore_blockdata_transaction_txout_TxOut {
                        ferment::boxed(dashcore_blockdata_transaction_txout_TxOut {
                            value,
                            script_pubkey,
                        })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_destroy(
                        ffi: *mut dashcore_blockdata_transaction_txout_TxOut,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_get_value(
                        obj: *const dashcore_blockdata_transaction_txout_TxOut,
                    ) -> u64 {
                        (*obj).value
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_get_script_pubkey (obj : * const dashcore_blockdata_transaction_txout_TxOut) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf{
                        (*obj).script_pubkey
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_set_value(
                        obj: *mut dashcore_blockdata_transaction_txout_TxOut,
                        value: u64,
                    ) {
                        (*obj).value = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_blockdata_transaction_txout_TxOut_set_script_pubkey(
                        obj: *mut dashcore_blockdata_transaction_txout_TxOut,
                        value : * mut crate :: fermented :: types :: dashcore :: blockdata :: script :: owned :: dashcore_blockdata_script_owned_ScriptBuf,
                    ) {
                        (*obj).script_pubkey = value;
                    }
                }
                #[doc = "FFI-representation of the [`Transaction`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_blockdata_transaction_Transaction { pub version : u16 , pub lock_time : u32 , pub input : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txin_TxIn , pub output : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txout_TxOut , pub special_transaction_payload : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload }
                impl ferment::FFIConversionFrom<dashcore::blockdata::transaction::Transaction>
                    for dashcore_blockdata_transaction_Transaction
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_blockdata_transaction_Transaction,
                    ) -> dashcore::blockdata::transaction::Transaction {
                        let ffi_ref = &*ffi;
                        dashcore :: blockdata :: transaction :: Transaction { version : ffi_ref . version , lock_time : ffi_ref . lock_time , input : < crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txin_TxIn as ferment :: FFIConversionFrom < Vec < dashcore :: blockdata :: transaction :: txin :: TxIn > >> :: ffi_from (ffi_ref . input) , output : < crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txout_TxOut as ferment :: FFIConversionFrom < Vec < dashcore :: blockdata :: transaction :: txout :: TxOut > >> :: ffi_from (ffi_ref . output) , special_transaction_payload : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload >> :: ffi_from_opt (ffi_ref . special_transaction_payload) }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::blockdata::transaction::Transaction>
                    for dashcore_blockdata_transaction_Transaction
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::blockdata::transaction::Transaction,
                    ) -> *const dashcore_blockdata_transaction_Transaction {
                        ferment :: boxed (dashcore_blockdata_transaction_Transaction { version : obj . version , lock_time : obj . lock_time , input : < crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txin_TxIn as ferment :: FFIConversionTo < Vec < dashcore :: blockdata :: transaction :: txin :: TxIn > >> :: ffi_to (obj . input) , output : < crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txout_TxOut as ferment :: FFIConversionTo < Vec < dashcore :: blockdata :: transaction :: txout :: TxOut > >> :: ffi_to (obj . output) , special_transaction_payload : < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: TransactionPayload >> :: ffi_to_opt (obj . special_transaction_payload) })
                    }
                }
                impl Drop for dashcore_blockdata_transaction_Transaction {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.input);
                            ferment::unbox_any(ffi_ref.output);
                            ferment::unbox_any_opt(ffi_ref.special_transaction_payload);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_ctor(
                    version: u16,
                    lock_time: u32,
                    input : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txin_TxIn,
                    output : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txout_TxOut,
                    special_transaction_payload : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload,
                ) -> *mut dashcore_blockdata_transaction_Transaction {
                    ferment::boxed(dashcore_blockdata_transaction_Transaction {
                        version,
                        lock_time,
                        input,
                        output,
                        special_transaction_payload,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_destroy(
                    ffi: *mut dashcore_blockdata_transaction_Transaction,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_get_version(
                    obj: *const dashcore_blockdata_transaction_Transaction,
                ) -> u16 {
                    (*obj).version
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_get_lock_time(
                    obj: *const dashcore_blockdata_transaction_Transaction,
                ) -> u32 {
                    (*obj).lock_time
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_get_input(
                    obj: *const dashcore_blockdata_transaction_Transaction,
                ) -> *mut crate::fermented::generics::Vec_dashcore_blockdata_transaction_txin_TxIn
                {
                    (*obj).input
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_get_output(
                    obj: *const dashcore_blockdata_transaction_Transaction,
                ) -> *mut crate::fermented::generics::Vec_dashcore_blockdata_transaction_txout_TxOut
                {
                    (*obj).output
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_get_special_transaction_payload (obj : * const dashcore_blockdata_transaction_Transaction) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload{
                    (*obj).special_transaction_payload
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_set_version(
                    obj: *mut dashcore_blockdata_transaction_Transaction,
                    value: u16,
                ) {
                    (*obj).version = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_set_lock_time(
                    obj: *mut dashcore_blockdata_transaction_Transaction,
                    value: u32,
                ) {
                    (*obj).lock_time = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_set_input(
                    obj: *mut dashcore_blockdata_transaction_Transaction,
                    value : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txin_TxIn,
                ) {
                    (*obj).input = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_set_output(
                    obj: *mut dashcore_blockdata_transaction_Transaction,
                    value : * mut crate :: fermented :: generics :: Vec_dashcore_blockdata_transaction_txout_TxOut,
                ) {
                    (*obj).output = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_set_special_transaction_payload(
                    obj: *mut dashcore_blockdata_transaction_Transaction,
                    value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionPayload,
                ) {
                    (*obj).special_transaction_payload = value;
                }
                #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::Transaction::txid`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_txid(
                    self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction,
                ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid {
                    let obj = dashcore :: blockdata :: transaction :: Transaction :: txid (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: Transaction >> :: ffi_from (self_)))) ;
                    < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj)
                }
                #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::Transaction::tx_type`]"]
                #[no_mangle]                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_tx_type (self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionType{
                    let obj = dashcore :: blockdata :: transaction :: Transaction :: tx_type (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: Transaction >> :: ffi_from (self_)))) ;
                    < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: special_transaction :: dashcore_blockdata_transaction_special_transaction_TransactionType as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: special_transaction :: TransactionType >> :: ffi_to (obj)
                }
                #[doc = "FFI-representation of the [`dashcore::blockdata::transaction::Transaction::is_coin_base`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_transaction_Transaction_is_coin_base(
                    self_ : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction,
                ) -> bool {
                    let obj = dashcore :: blockdata :: transaction :: Transaction :: is_coin_base (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: Transaction >> :: ffi_from (self_)))) ;
                    obj
                }
            }
            pub mod witness {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`Witness`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_blockdata_witness_Witness {
                    pub content: *mut crate::fermented::generics::Vec_u8,
                    pub witness_elements: usize,
                    pub indices_start: usize,
                }
                impl ferment::FFIConversionFrom<dashcore::blockdata::witness::Witness>
                    for dashcore_blockdata_witness_Witness
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_blockdata_witness_Witness,
                    ) -> dashcore::blockdata::witness::Witness {
                        let ffi_ref = &*ffi;
                        dashcore::blockdata::witness::Witness {
                            content:
                                <crate::fermented::generics::Vec_u8 as ferment::FFIConversionFrom<
                                    Vec<u8>,
                                >>::ffi_from(ffi_ref.content),
                            witness_elements: ffi_ref.witness_elements,
                            indices_start: ffi_ref.indices_start,
                        }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::blockdata::witness::Witness>
                    for dashcore_blockdata_witness_Witness
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::blockdata::witness::Witness,
                    ) -> *const dashcore_blockdata_witness_Witness {
                        ferment::boxed(dashcore_blockdata_witness_Witness {
                            content:
                                <crate::fermented::generics::Vec_u8 as ferment::FFIConversionTo<
                                    Vec<u8>,
                                >>::ffi_to(obj.content),
                            witness_elements: obj.witness_elements,
                            indices_start: obj.indices_start,
                        })
                    }
                }
                impl Drop for dashcore_blockdata_witness_Witness {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.content);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_ctor(
                    content: *mut crate::fermented::generics::Vec_u8,
                    witness_elements: usize,
                    indices_start: usize,
                ) -> *mut dashcore_blockdata_witness_Witness {
                    ferment::boxed(dashcore_blockdata_witness_Witness {
                        content,
                        witness_elements,
                        indices_start,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_destroy(
                    ffi: *mut dashcore_blockdata_witness_Witness,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_get_content(
                    obj: *const dashcore_blockdata_witness_Witness,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).content
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_get_witness_elements(
                    obj: *const dashcore_blockdata_witness_Witness,
                ) -> usize {
                    (*obj).witness_elements
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_get_indices_start(
                    obj: *const dashcore_blockdata_witness_Witness,
                ) -> usize {
                    (*obj).indices_start
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_set_content(
                    obj: *mut dashcore_blockdata_witness_Witness,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).content = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_set_witness_elements(
                    obj: *mut dashcore_blockdata_witness_Witness,
                    value: usize,
                ) {
                    (*obj).witness_elements = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_blockdata_witness_Witness_set_indices_start(
                    obj: *mut dashcore_blockdata_witness_Witness,
                    value: usize,
                ) {
                    (*obj).indices_start = value;
                }
            }
        }
        pub mod bls_sig_utils {
            use crate as example_dashcore;
            #[doc = "FFI-representation of the [`BLSPublicKey`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dashcore_bls_sig_utils_BLSPublicKey(
                *mut crate::fermented::generics::Arr_u8_48,
            );
            impl ferment::FFIConversionFrom<dashcore::bls_sig_utils::BLSPublicKey>
                for dashcore_bls_sig_utils_BLSPublicKey
            {
                unsafe fn ffi_from_const(
                    ffi: *const dashcore_bls_sig_utils_BLSPublicKey,
                ) -> dashcore::bls_sig_utils::BLSPublicKey {
                    let ffi_ref = &*ffi;
                    dashcore::bls_sig_utils::BLSPublicKey(
                        <crate::fermented::generics::Arr_u8_48 as ferment::FFIConversionFrom<
                            [u8; 48],
                        >>::ffi_from(ffi_ref.0),
                    )
                }
            }
            impl ferment::FFIConversionTo<dashcore::bls_sig_utils::BLSPublicKey>
                for dashcore_bls_sig_utils_BLSPublicKey
            {
                unsafe fn ffi_to_const(
                    obj: dashcore::bls_sig_utils::BLSPublicKey,
                ) -> *const dashcore_bls_sig_utils_BLSPublicKey {
                    ferment::boxed(dashcore_bls_sig_utils_BLSPublicKey(
                        <crate::fermented::generics::Arr_u8_48 as ferment::FFIConversionTo<
                            [u8; 48],
                        >>::ffi_to(obj.0),
                    ))
                }
            }
            impl Drop for dashcore_bls_sig_utils_BLSPublicKey {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSPublicKey_ctor(
                o_0: *mut crate::fermented::generics::Arr_u8_48,
            ) -> *mut dashcore_bls_sig_utils_BLSPublicKey {
                ferment::boxed(dashcore_bls_sig_utils_BLSPublicKey(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSPublicKey_destroy(
                ffi: *mut dashcore_bls_sig_utils_BLSPublicKey,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSPublicKey_get_0(
                obj: *const dashcore_bls_sig_utils_BLSPublicKey,
            ) -> *mut crate::fermented::generics::Arr_u8_48 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSPublicKey_set_0(
                obj: *mut dashcore_bls_sig_utils_BLSPublicKey,
                value: *mut crate::fermented::generics::Arr_u8_48,
            ) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`BLSSignature`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dashcore_bls_sig_utils_BLSSignature(
                *mut crate::fermented::generics::Arr_u8_96,
            );
            impl ferment::FFIConversionFrom<dashcore::bls_sig_utils::BLSSignature>
                for dashcore_bls_sig_utils_BLSSignature
            {
                unsafe fn ffi_from_const(
                    ffi: *const dashcore_bls_sig_utils_BLSSignature,
                ) -> dashcore::bls_sig_utils::BLSSignature {
                    let ffi_ref = &*ffi;
                    dashcore::bls_sig_utils::BLSSignature(
                        <crate::fermented::generics::Arr_u8_96 as ferment::FFIConversionFrom<
                            [u8; 96],
                        >>::ffi_from(ffi_ref.0),
                    )
                }
            }
            impl ferment::FFIConversionTo<dashcore::bls_sig_utils::BLSSignature>
                for dashcore_bls_sig_utils_BLSSignature
            {
                unsafe fn ffi_to_const(
                    obj: dashcore::bls_sig_utils::BLSSignature,
                ) -> *const dashcore_bls_sig_utils_BLSSignature {
                    ferment::boxed(dashcore_bls_sig_utils_BLSSignature(
                        <crate::fermented::generics::Arr_u8_96 as ferment::FFIConversionTo<
                            [u8; 96],
                        >>::ffi_to(obj.0),
                    ))
                }
            }
            impl Drop for dashcore_bls_sig_utils_BLSSignature {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSSignature_ctor(
                o_0: *mut crate::fermented::generics::Arr_u8_96,
            ) -> *mut dashcore_bls_sig_utils_BLSSignature {
                ferment::boxed(dashcore_bls_sig_utils_BLSSignature(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSSignature_destroy(
                ffi: *mut dashcore_bls_sig_utils_BLSSignature,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSSignature_get_0(
                obj: *const dashcore_bls_sig_utils_BLSSignature,
            ) -> *mut crate::fermented::generics::Arr_u8_96 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_bls_sig_utils_BLSSignature_set_0(
                obj: *mut dashcore_bls_sig_utils_BLSSignature,
                value: *mut crate::fermented::generics::Arr_u8_96,
            ) {
                (*obj).0 = value;
            }
        }
        pub mod ephemerealdata {
            pub mod chain_lock {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`ChainLock`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_ephemerealdata_chain_lock_ChainLock { pub block_height : u32 , pub block_hash : * mut dashcore :: hash_types :: BlockHash , pub signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                impl ferment::FFIConversionFrom<dashcore::ephemerealdata::chain_lock::ChainLock>
                    for dashcore_ephemerealdata_chain_lock_ChainLock
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_ephemerealdata_chain_lock_ChainLock,
                    ) -> dashcore::ephemerealdata::chain_lock::ChainLock {
                        let ffi_ref = &*ffi;
                        dashcore :: ephemerealdata :: chain_lock :: ChainLock { block_height : ffi_ref . block_height , block_hash : std :: ptr :: read (ffi_ref . block_hash) , signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . signature) }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::ephemerealdata::chain_lock::ChainLock>
                    for dashcore_ephemerealdata_chain_lock_ChainLock
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::ephemerealdata::chain_lock::ChainLock,
                    ) -> *const dashcore_ephemerealdata_chain_lock_ChainLock {
                        ferment :: boxed (dashcore_ephemerealdata_chain_lock_ChainLock { block_height : obj . block_height , block_hash : ferment :: boxed (obj . block_hash) , signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . signature) })
                    }
                }
                impl Drop for dashcore_ephemerealdata_chain_lock_ChainLock {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.block_hash);
                            ferment::unbox_any(ffi_ref.signature);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_ctor(
                    block_height: u32,
                    block_hash: *mut dashcore::hash_types::BlockHash,
                    signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                ) -> *mut dashcore_ephemerealdata_chain_lock_ChainLock {
                    ferment::boxed(dashcore_ephemerealdata_chain_lock_ChainLock {
                        block_height,
                        block_hash,
                        signature,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_destroy(
                    ffi: *mut dashcore_ephemerealdata_chain_lock_ChainLock,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_get_block_height(
                    obj: *const dashcore_ephemerealdata_chain_lock_ChainLock,
                ) -> u32 {
                    (*obj).block_height
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_get_block_hash(
                    obj: *const dashcore_ephemerealdata_chain_lock_ChainLock,
                ) -> *mut dashcore::hash_types::BlockHash {
                    (*obj).block_hash
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_get_signature (obj : * const dashcore_ephemerealdata_chain_lock_ChainLock) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                    (*obj).signature
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_set_block_height(
                    obj: *mut dashcore_ephemerealdata_chain_lock_ChainLock,
                    value: u32,
                ) {
                    (*obj).block_height = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_set_block_hash(
                    obj: *mut dashcore_ephemerealdata_chain_lock_ChainLock,
                    value: *mut dashcore::hash_types::BlockHash,
                ) {
                    (*obj).block_hash = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_chain_lock_ChainLock_set_signature(
                    obj: *mut dashcore_ephemerealdata_chain_lock_ChainLock,
                    value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                ) {
                    (*obj).signature = value;
                }
            }
            pub mod instant_lock {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`InstantLock`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_ephemerealdata_instant_lock_InstantLock { pub version : u8 , pub inputs : * mut crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint , pub txid : * mut example_dashcore :: custom :: dashcore :: dashcore_Txid , pub cyclehash : * mut dashcore :: hash_types :: CycleHash , pub signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
                impl ferment::FFIConversionFrom<dashcore::ephemerealdata::instant_lock::InstantLock>
                    for dashcore_ephemerealdata_instant_lock_InstantLock
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_ephemerealdata_instant_lock_InstantLock,
                    ) -> dashcore::ephemerealdata::instant_lock::InstantLock {
                        let ffi_ref = &*ffi;
                        dashcore :: ephemerealdata :: instant_lock :: InstantLock { version : ffi_ref . version , inputs : < crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint as ferment :: FFIConversionFrom < std :: vec :: Vec < dashcore :: transaction :: outpoint :: OutPoint > >> :: ffi_from (ffi_ref . inputs) , txid : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionFrom < dashcore :: hash_types :: Txid >> :: ffi_from (ffi_ref . txid) , cyclehash : std :: ptr :: read (ffi_ref . cyclehash) , signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (ffi_ref . signature) }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::ephemerealdata::instant_lock::InstantLock>
                    for dashcore_ephemerealdata_instant_lock_InstantLock
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::ephemerealdata::instant_lock::InstantLock,
                    ) -> *const dashcore_ephemerealdata_instant_lock_InstantLock
                    {
                        ferment :: boxed (dashcore_ephemerealdata_instant_lock_InstantLock { version : obj . version , inputs : < crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint as ferment :: FFIConversionTo < std :: vec :: Vec < dashcore :: transaction :: outpoint :: OutPoint > >> :: ffi_to (obj . inputs) , txid : < example_dashcore :: custom :: dashcore :: dashcore_Txid as ferment :: FFIConversionTo < dashcore :: hash_types :: Txid >> :: ffi_to (obj . txid) , cyclehash : ferment :: boxed (obj . cyclehash) , signature : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (obj . signature) })
                    }
                }
                impl Drop for dashcore_ephemerealdata_instant_lock_InstantLock {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.inputs);
                            ferment::unbox_any(ffi_ref.txid);
                            ferment::unbox_any(ffi_ref.cyclehash);
                            ferment::unbox_any(ffi_ref.signature);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_ctor(
                    version: u8,
                    inputs : * mut crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
                    txid: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                    cyclehash: *mut dashcore::hash_types::CycleHash,
                    signature : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                ) -> *mut dashcore_ephemerealdata_instant_lock_InstantLock {
                    ferment::boxed(dashcore_ephemerealdata_instant_lock_InstantLock {
                        version,
                        inputs,
                        txid,
                        cyclehash,
                        signature,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_destroy(
                    ffi: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_get_version(
                    obj: *const dashcore_ephemerealdata_instant_lock_InstantLock,
                ) -> u8 {
                    (*obj).version
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_get_inputs (obj : * const dashcore_ephemerealdata_instant_lock_InstantLock) -> * mut crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint{
                    (*obj).inputs
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_get_txid(
                    obj: *const dashcore_ephemerealdata_instant_lock_InstantLock,
                ) -> *mut example_dashcore::custom::dashcore::dashcore_Txid {
                    (*obj).txid
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_get_cyclehash(
                    obj: *const dashcore_ephemerealdata_instant_lock_InstantLock,
                ) -> *mut dashcore::hash_types::CycleHash {
                    (*obj).cyclehash
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_get_signature (obj : * const dashcore_ephemerealdata_instant_lock_InstantLock) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature{
                    (*obj).signature
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_set_version(
                    obj: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                    value: u8,
                ) {
                    (*obj).version = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_set_inputs(
                    obj: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                    value : * mut crate :: fermented :: generics :: std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
                ) {
                    (*obj).inputs = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_set_txid(
                    obj: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                    value: *mut example_dashcore::custom::dashcore::dashcore_Txid,
                ) {
                    (*obj).txid = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_set_cyclehash(
                    obj: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                    value: *mut dashcore::hash_types::CycleHash,
                ) {
                    (*obj).cyclehash = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_ephemerealdata_instant_lock_InstantLock_set_signature(
                    obj: *mut dashcore_ephemerealdata_instant_lock_InstantLock,
                    value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
                ) {
                    (*obj).signature = value;
                }
            }
        }
        pub mod sml {
            pub mod error {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`SmlError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_error_SmlError {
                    BaseBlockNotGenesis(*mut dashcore::hash_types::BlockHash),
                    BlockHashLookupFailed(*mut dashcore::hash_types::BlockHash),
                    IncompleteMnListDiff,
                    MissingStartMasternodeList(*mut dashcore::hash_types::BlockHash),
                    BaseBlockHashMismatch {
                        expected: *mut dashcore::hash_types::BlockHash,
                        found: *mut dashcore::hash_types::BlockHash,
                    },
                    UnknownError,
                    CorruptedCodeExecution(*mut std::os::raw::c_char),
                    FeatureNotTurnedOn(*mut std::os::raw::c_char),
                    InvalidIndexInSignatureSet(u16),
                    IncompleteSignatureSet,
                }
                impl ferment::FFIConversionFrom<dashcore::sml::error::SmlError> for dashcore_sml_error_SmlError {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_error_SmlError,
                    ) -> dashcore::sml::error::SmlError {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dashcore_sml_error_SmlError :: BaseBlockNotGenesis (o_0) => dashcore :: sml :: error :: SmlError :: BaseBlockNotGenesis (std :: ptr :: read (* o_0)) , dashcore_sml_error_SmlError :: BlockHashLookupFailed (o_0) => dashcore :: sml :: error :: SmlError :: BlockHashLookupFailed (std :: ptr :: read (* o_0)) , dashcore_sml_error_SmlError :: IncompleteMnListDiff => dashcore :: sml :: error :: SmlError :: IncompleteMnListDiff , dashcore_sml_error_SmlError :: MissingStartMasternodeList (o_0) => dashcore :: sml :: error :: SmlError :: MissingStartMasternodeList (std :: ptr :: read (* o_0)) , dashcore_sml_error_SmlError :: BaseBlockHashMismatch { expected , found } => dashcore :: sml :: error :: SmlError :: BaseBlockHashMismatch { expected : std :: ptr :: read (* expected) , found : std :: ptr :: read (* found) } , dashcore_sml_error_SmlError :: UnknownError => dashcore :: sml :: error :: SmlError :: UnknownError , dashcore_sml_error_SmlError :: CorruptedCodeExecution (o_0) => dashcore :: sml :: error :: SmlError :: CorruptedCodeExecution (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_error_SmlError :: FeatureNotTurnedOn (o_0) => dashcore :: sml :: error :: SmlError :: FeatureNotTurnedOn (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_error_SmlError :: InvalidIndexInSignatureSet (o_0) => dashcore :: sml :: error :: SmlError :: InvalidIndexInSignatureSet (* o_0) , dashcore_sml_error_SmlError :: IncompleteSignatureSet => dashcore :: sml :: error :: SmlError :: IncompleteSignatureSet }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::sml::error::SmlError> for dashcore_sml_error_SmlError {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::error::SmlError,
                    ) -> *const dashcore_sml_error_SmlError {
                        ferment::boxed(
                            match obj {
                                dashcore::sml::error::SmlError::BaseBlockNotGenesis(o_0) => {
                                    dashcore_sml_error_SmlError::BaseBlockNotGenesis(
                                        ferment::boxed(o_0),
                                    )
                                }
                                dashcore::sml::error::SmlError::BlockHashLookupFailed(o_0) => {
                                    dashcore_sml_error_SmlError::BlockHashLookupFailed(
                                        ferment::boxed(o_0),
                                    )
                                }
                                dashcore::sml::error::SmlError::IncompleteMnListDiff => {
                                    dashcore_sml_error_SmlError::IncompleteMnListDiff
                                }
                                dashcore::sml::error::SmlError::MissingStartMasternodeList(o_0) => {
                                    dashcore_sml_error_SmlError::MissingStartMasternodeList(
                                        ferment::boxed(o_0),
                                    )
                                }
                                dashcore::sml::error::SmlError::BaseBlockHashMismatch {
                                    expected,
                                    found,
                                } => dashcore_sml_error_SmlError::BaseBlockHashMismatch {
                                    expected: ferment::boxed(expected),
                                    found: ferment::boxed(found),
                                },
                                dashcore::sml::error::SmlError::UnknownError => {
                                    dashcore_sml_error_SmlError::UnknownError
                                }
                                dashcore::sml::error::SmlError::CorruptedCodeExecution(o_0) => {
                                    dashcore_sml_error_SmlError::CorruptedCodeExecution(
                                        <std::os::raw::c_char as ferment::FFIConversionTo<
                                            String,
                                        >>::ffi_to(o_0),
                                    )
                                }
                                dashcore::sml::error::SmlError::FeatureNotTurnedOn(o_0) => {
                                    dashcore_sml_error_SmlError::FeatureNotTurnedOn(
                                        <std::os::raw::c_char as ferment::FFIConversionTo<
                                            String,
                                        >>::ffi_to(o_0),
                                    )
                                }
                                dashcore::sml::error::SmlError::InvalidIndexInSignatureSet(o_0) => {
                                    dashcore_sml_error_SmlError::InvalidIndexInSignatureSet(o_0)
                                }
                                dashcore::sml::error::SmlError::IncompleteSignatureSet => {
                                    dashcore_sml_error_SmlError::IncompleteSignatureSet
                                }
                                _ => unreachable!("This is unreachable"),
                            },
                        )
                    }
                }
                impl Drop for dashcore_sml_error_SmlError {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dashcore_sml_error_SmlError::BaseBlockNotGenesis(o_0) => {
                                    ferment::unbox_any(*o_0);
                                }
                                dashcore_sml_error_SmlError::BlockHashLookupFailed(o_0) => {
                                    ferment::unbox_any(*o_0);
                                }
                                dashcore_sml_error_SmlError::IncompleteMnListDiff => {}
                                dashcore_sml_error_SmlError::MissingStartMasternodeList(o_0) => {
                                    ferment::unbox_any(*o_0);
                                }
                                dashcore_sml_error_SmlError::BaseBlockHashMismatch {
                                    expected,
                                    found,
                                } => {
                                    ferment::unbox_any(*expected);
                                    ferment::unbox_any(*found);
                                }
                                dashcore_sml_error_SmlError::UnknownError => {}
                                dashcore_sml_error_SmlError::CorruptedCodeExecution(o_0) => {
                                    ferment::unbox_string(*o_0);
                                }
                                dashcore_sml_error_SmlError::FeatureNotTurnedOn(o_0) => {
                                    ferment::unbox_string(*o_0);
                                }
                                dashcore_sml_error_SmlError::InvalidIndexInSignatureSet(o_0) => {}
                                dashcore_sml_error_SmlError::IncompleteSignatureSet => {}
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_BaseBlockNotGenesis_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::BaseBlockNotGenesis(o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_BlockHashLookupFailed_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::BlockHashLookupFailed(o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_IncompleteMnListDiff_ctor(
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::IncompleteMnListDiff {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_MissingStartMasternodeList_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::MissingStartMasternodeList(
                        o_o_0,
                    ))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_BaseBlockHashMismatch_ctor(
                    expected: *mut dashcore::hash_types::BlockHash,
                    found: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::BaseBlockHashMismatch {
                        expected,
                        found,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_UnknownError_ctor(
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::UnknownError {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_CorruptedCodeExecution_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::CorruptedCodeExecution(o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_FeatureNotTurnedOn_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::FeatureNotTurnedOn(o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_InvalidIndexInSignatureSet_ctor(
                    o_o_0: u16,
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::InvalidIndexInSignatureSet(
                        o_o_0,
                    ))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_IncompleteSignatureSet_ctor(
                ) -> *mut dashcore_sml_error_SmlError {
                    ferment::boxed(dashcore_sml_error_SmlError::IncompleteSignatureSet {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_error_SmlError_destroy(
                    ffi: *mut dashcore_sml_error_SmlError,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
            pub mod llmq_entry_verification {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`LLMQEntryVerificationStatus`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus {
                    Unknown , Verified , Skipped (* mut dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationSkipStatus) , Invalid (* mut crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_QuorumValidationError) }
                impl
                    ferment::FFIConversionFrom<
                        dashcore::sml::llmq_entry_verification::LLMQEntryVerificationStatus,
                    > for dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    unsafe fn ffi_from_const(
                        ffi : * const dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus,
                    ) -> dashcore::sml::llmq_entry_verification::LLMQEntryVerificationStatus
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Unknown => dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Unknown , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Verified => dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Verified , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Skipped (o_0) => dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Skipped (std :: ptr :: read (* o_0)) , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Invalid (o_0) => dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Invalid (< crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_QuorumValidationError as ferment :: FFIConversionFrom < dashcore :: sml :: quorum_validation_error :: QuorumValidationError >> :: ffi_from (* o_0)) }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        dashcore::sml::llmq_entry_verification::LLMQEntryVerificationStatus,
                    > for dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::llmq_entry_verification::LLMQEntryVerificationStatus,
                    ) -> *const dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                    {
                        ferment :: boxed (match obj { dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Unknown => dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Unknown , dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Verified => dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Verified , dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Skipped (o_0) => dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Skipped (ferment :: boxed (o_0)) , dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus :: Invalid (o_0) => dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Invalid (< crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_QuorumValidationError as ferment :: FFIConversionTo < dashcore :: sml :: quorum_validation_error :: QuorumValidationError >> :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl Drop for dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus {
                    fn drop(&mut self) {
                        unsafe {
                            match self { dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Unknown => { } , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Verified => { } , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Skipped (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Invalid (o_0) => { ferment :: unbox_any (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus_Unknown_ctor(
                ) -> *mut dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    ferment::boxed(
                        dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus::Unknown {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus_Verified_ctor(
                ) -> *mut dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    ferment :: boxed (dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus :: Verified { })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus_Skipped_ctor(
                    o_o_0 : * mut dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationSkipStatus,
                ) -> *mut dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    ferment::boxed(
                        dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus::Skipped(
                            o_o_0,
                        ),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus_Invalid_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_QuorumValidationError,
                ) -> *mut dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus
                {
                    ferment::boxed(
                        dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus::Invalid(
                            o_o_0,
                        ),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus_destroy(
                    ffi: *mut dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
            pub mod llmq_type {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`LLMQType`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_llmq_type_LLMQType {
                    LlmqtypeUnknown = 0,
                    Llmqtype50_60 = 1,
                    Llmqtype400_60 = 2,
                    Llmqtype400_85 = 3,
                    Llmqtype100_67 = 4,
                    Llmqtype60_75 = 5,
                    Llmqtype25_67 = 6,
                    LlmqtypeTest = 100,
                    LlmqtypeDevnet = 101,
                    LlmqtypeTestV17 = 102,
                    LlmqtypeTestDIP0024 = 103,
                    LlmqtypeTestInstantSend = 104,
                    LlmqtypeDevnetDIP0024 = 105,
                    LlmqtypeTestnetPlatform = 106,
                    LlmqtypeDevnetPlatform = 107,
                }
                impl ferment::FFIConversionFrom<dashcore::sml::llmq_type::LLMQType>
                    for dashcore_sml_llmq_type_LLMQType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_llmq_type_LLMQType,
                    ) -> dashcore::sml::llmq_type::LLMQType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeUnknown => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeUnknown
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype50_60 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype50_60
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype400_60 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype400_60
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype400_85 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype400_85
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype100_67 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype100_67
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype60_75 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype60_75
                            }
                            dashcore_sml_llmq_type_LLMQType::Llmqtype25_67 => {
                                dashcore::sml::llmq_type::LLMQType::Llmqtype25_67
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeTest => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeTest
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnet => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnet
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeTestV17 => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeTestV17
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeTestDIP0024 => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeTestDIP0024
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeTestInstantSend => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeTestInstantSend
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetDIP0024 => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnetDIP0024
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeTestnetPlatform => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeTestnetPlatform
                            }
                            dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetPlatform => {
                                dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnetPlatform
                            }
                        }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::sml::llmq_type::LLMQType>
                    for dashcore_sml_llmq_type_LLMQType
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::llmq_type::LLMQType,
                    ) -> *const dashcore_sml_llmq_type_LLMQType {
                        ferment::boxed(match obj {
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeUnknown => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeUnknown
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype50_60 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype50_60
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype400_60 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype400_60
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype400_85 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype400_85
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype100_67 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype100_67
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype60_75 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype60_75
                            }
                            dashcore::sml::llmq_type::LLMQType::Llmqtype25_67 => {
                                dashcore_sml_llmq_type_LLMQType::Llmqtype25_67
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeTest => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTest
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnet => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnet
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeTestV17 => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestV17
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeTestDIP0024 => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestDIP0024
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeTestInstantSend => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestInstantSend
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnetDIP0024 => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetDIP0024
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeTestnetPlatform => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestnetPlatform
                            }
                            dashcore::sml::llmq_type::LLMQType::LlmqtypeDevnetPlatform => {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetPlatform
                            }
                            _ => unreachable!("This is unreachable"),
                        })
                    }
                }
                impl Drop for dashcore_sml_llmq_type_LLMQType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeUnknown => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype50_60 => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype400_60 => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype400_85 => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype100_67 => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype60_75 => {}
                                dashcore_sml_llmq_type_LLMQType::Llmqtype25_67 => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTest => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnet => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestV17 => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestDIP0024 => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestInstantSend => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetDIP0024 => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeTestnetPlatform => {}
                                dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetPlatform => {}
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeUnknown_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeUnknown {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype50_60_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype50_60 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype400_60_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype400_60 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype400_85_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype400_85 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype100_67_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype100_67 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype60_75_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype60_75 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_Llmqtype25_67_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::Llmqtype25_67 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeTest_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeTest {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeDevnet_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnet {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeTestV17_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeTestV17 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeTestDIP0024_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeTestDIP0024 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeTestInstantSend_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeTestInstantSend {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeDevnetDIP0024_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetDIP0024 {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeTestnetPlatform_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeTestnetPlatform {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_LlmqtypeDevnetPlatform_ctor(
                ) -> *mut dashcore_sml_llmq_type_LLMQType {
                    ferment::boxed(dashcore_sml_llmq_type_LLMQType::LlmqtypeDevnetPlatform {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_destroy(
                    ffi: *mut dashcore_sml_llmq_type_LLMQType,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::index`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_index(
                    self_ : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                ) -> u8 {
                    let obj = dashcore :: sml :: llmq_type :: LLMQType :: index (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (self_)))) ;
                    obj
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::from_u16`]"]
                #[no_mangle]                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_from_u16 (index : u16) -> * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType{
                    let obj = dashcore::sml::llmq_type::LLMQType::from_u16(index);
                    < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionTo < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_to (obj)
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::from_u8`]"]
                #[no_mangle]                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_from_u8 (index : u8) -> * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType{
                    let obj = dashcore::sml::llmq_type::LLMQType::from_u8(index);
                    < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionTo < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_to (obj)
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::is_rotating_quorum_type`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_is_rotating_quorum_type(
                    self_ : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                ) -> bool {
                    let obj = dashcore :: sml :: llmq_type :: LLMQType :: is_rotating_quorum_type (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (self_)))) ;
                    obj
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::get_cycle_base_height`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_get_cycle_base_height(
                    self_ : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                    height: u32,
                ) -> u32 {
                    let obj = dashcore :: sml :: llmq_type :: LLMQType :: get_cycle_base_height (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (self_))) , height) ;
                    obj
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::get_dkg_window_for_height`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_get_dkg_window_for_height(
                    self_ : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                    height: u32,
                ) -> *mut dashcore::sml::llmq_type::DKGWindow {
                    let obj = dashcore :: sml :: llmq_type :: LLMQType :: get_dkg_window_for_height (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (self_))) , height) ;
                    ferment::boxed(obj)
                }
                #[doc = "FFI-representation of the [`dashcore::sml::llmq_type::LLMQType::get_dkg_windows_in_range`]"]
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_llmq_type_LLMQType_get_dkg_windows_in_range(
                    self_ : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                    start: u32,
                    end: u32,
                ) -> *mut crate::fermented::generics::Vec_dashcore_sml_llmq_type_DKGWindow
                {
                    let obj = dashcore :: sml :: llmq_type :: LLMQType :: get_dkg_windows_in_range (Box :: leak (Box :: new (< crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (self_))) , start , end) ;
                    < crate :: fermented :: generics :: Vec_dashcore_sml_llmq_type_DKGWindow as ferment :: FFIConversionTo < Vec < dashcore :: sml :: llmq_type :: DKGWindow > >> :: ffi_to (obj)
                }
            }
            pub mod masternode_list {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`MasternodeList`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_sml_masternode_list_MasternodeList { pub block_hash : * mut dashcore :: hash_types :: BlockHash , pub known_height : u32 , pub masternode_merkle_root : * mut dashcore :: hash_types :: MerkleRootMasternodeList , pub llmq_merkle_root : * mut dashcore :: hash_types :: MerkleRootQuorums , pub masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry , pub quorums : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry }
                impl ferment::FFIConversionFrom<dashcore::sml::masternode_list::MasternodeList>
                    for dashcore_sml_masternode_list_MasternodeList
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_masternode_list_MasternodeList,
                    ) -> dashcore::sml::masternode_list::MasternodeList {
                        let ffi_ref = &*ffi;
                        dashcore :: sml :: masternode_list :: MasternodeList { block_hash : std :: ptr :: read (ffi_ref . block_hash) , known_height : ffi_ref . known_height , masternode_merkle_root : ferment :: from_opt_opaque (ffi_ref . masternode_merkle_root) , llmq_merkle_root : ferment :: from_opt_opaque (ffi_ref . llmq_merkle_root) , masternodes : < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry as ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > >> :: ffi_from (ffi_ref . masternodes) , quorums : < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > >> :: ffi_from (ffi_ref . quorums) }
                    }
                }
                impl ferment::FFIConversionTo<dashcore::sml::masternode_list::MasternodeList>
                    for dashcore_sml_masternode_list_MasternodeList
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::masternode_list::MasternodeList,
                    ) -> *const dashcore_sml_masternode_list_MasternodeList {
                        ferment :: boxed (dashcore_sml_masternode_list_MasternodeList { block_hash : ferment :: boxed (obj . block_hash) , known_height : obj . known_height , masternode_merkle_root : ferment :: to_opt_primitive (obj . masternode_merkle_root) , llmq_merkle_root : ferment :: to_opt_primitive (obj . llmq_merkle_root) , masternodes : < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry as ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > >> :: ffi_to (obj . masternodes) , quorums : < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > >> :: ffi_to (obj . quorums) })
                    }
                }
                impl Drop for dashcore_sml_masternode_list_MasternodeList {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.block_hash);
                            ferment::unbox_any_opt(ffi_ref.masternode_merkle_root);
                            ferment::unbox_any_opt(ffi_ref.llmq_merkle_root);
                            ferment::unbox_any(ffi_ref.masternodes);
                            ferment::unbox_any(ffi_ref.quorums);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_ctor(
                    block_hash: *mut dashcore::hash_types::BlockHash,
                    known_height: u32,
                    masternode_merkle_root: *mut dashcore::hash_types::MerkleRootMasternodeList,
                    llmq_merkle_root: *mut dashcore::hash_types::MerkleRootQuorums,
                    masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                    quorums : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                ) -> *mut dashcore_sml_masternode_list_MasternodeList {
                    ferment::boxed(dashcore_sml_masternode_list_MasternodeList {
                        block_hash,
                        known_height,
                        masternode_merkle_root,
                        llmq_merkle_root,
                        masternodes,
                        quorums,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_destroy(
                    ffi: *mut dashcore_sml_masternode_list_MasternodeList,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_block_hash(
                    obj: *const dashcore_sml_masternode_list_MasternodeList,
                ) -> *mut dashcore::hash_types::BlockHash {
                    (*obj).block_hash
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_known_height(
                    obj: *const dashcore_sml_masternode_list_MasternodeList,
                ) -> u32 {
                    (*obj).known_height
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_masternode_merkle_root(
                    obj: *const dashcore_sml_masternode_list_MasternodeList,
                ) -> *mut dashcore::hash_types::MerkleRootMasternodeList {
                    (*obj).masternode_merkle_root
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_llmq_merkle_root(
                    obj: *const dashcore_sml_masternode_list_MasternodeList,
                ) -> *mut dashcore::hash_types::MerkleRootQuorums {
                    (*obj).llmq_merkle_root
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_masternodes (obj : * const dashcore_sml_masternode_list_MasternodeList) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry{
                    (*obj).masternodes
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_get_quorums (obj : * const dashcore_sml_masternode_list_MasternodeList) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry{
                    (*obj).quorums
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_block_hash(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value: *mut dashcore::hash_types::BlockHash,
                ) {
                    (*obj).block_hash = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_known_height(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value: u32,
                ) {
                    (*obj).known_height = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_masternode_merkle_root(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value: *mut dashcore::hash_types::MerkleRootMasternodeList,
                ) {
                    (*obj).masternode_merkle_root = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_llmq_merkle_root(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value: *mut dashcore::hash_types::MerkleRootQuorums,
                ) {
                    (*obj).llmq_merkle_root = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_masternodes(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                ) {
                    (*obj).masternodes = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_MasternodeList_set_quorums(
                    obj: *mut dashcore_sml_masternode_list_MasternodeList,
                    value : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                ) {
                    (*obj).quorums = value;
                }
            }
            pub mod masternode_list_entry {
                use crate as example_dashcore;
                pub mod qualified_masternode_list_entry {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`QualifiedMasternodeListEntry`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry
                    {
                        pub masternode_list_entry:
                            *mut dashcore::sml::masternode_list_entry::MasternodeListEntry,
                        pub entry_hash: *mut dashcore::hash_types::Sha256dHash,
                        pub confirmed_hash_hashed_with_pro_reg_tx:
                            *mut dashcore::hash_types::ConfirmedHashHashedWithProRegTx,
                    }
                    impl ferment :: FFIConversionFrom < dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > for dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { unsafe fn ffi_from_const (ffi : * const dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry) -> dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry { let ffi_ref = & * ffi ; dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry { masternode_list_entry : std :: ptr :: read (ffi_ref . masternode_list_entry) , entry_hash : std :: ptr :: read (ffi_ref . entry_hash) , confirmed_hash_hashed_with_pro_reg_tx : ferment :: from_opt_opaque (ffi_ref . confirmed_hash_hashed_with_pro_reg_tx) } } }
                    impl ferment :: FFIConversionTo < dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > for dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { unsafe fn ffi_to_const (obj : dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry) -> * const dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { ferment :: boxed (dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { masternode_list_entry : ferment :: boxed (obj . masternode_list_entry) , entry_hash : ferment :: boxed (obj . entry_hash) , confirmed_hash_hashed_with_pro_reg_tx : ferment :: to_opt_primitive (obj . confirmed_hash_hashed_with_pro_reg_tx) }) } }
                    impl Drop for dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { fn drop (& mut self) { unsafe { let ffi_ref = self ; ferment :: unbox_any (ffi_ref . masternode_list_entry) ; ferment :: unbox_any (ffi_ref . entry_hash) ; ferment :: unbox_any_opt (ffi_ref . confirmed_hash_hashed_with_pro_reg_tx) ; } } }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_ctor (masternode_list_entry : * mut dashcore :: sml :: masternode_list_entry :: MasternodeListEntry , entry_hash : * mut dashcore :: hash_types :: Sha256dHash , confirmed_hash_hashed_with_pro_reg_tx : * mut dashcore :: hash_types :: ConfirmedHashHashedWithProRegTx) -> * mut dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry{
                        ferment :: boxed (dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { masternode_list_entry , entry_hash , confirmed_hash_hashed_with_pro_reg_tx })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_destroy(
                        ffi : * mut dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_get_masternode_list_entry(
                        obj : * const dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                    ) -> *mut dashcore::sml::masternode_list_entry::MasternodeListEntry
                    {
                        (*obj).masternode_list_entry
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_get_entry_hash(
                        obj : * const dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                    ) -> *mut dashcore::hash_types::Sha256dHash {
                        (*obj).entry_hash
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_get_confirmed_hash_hashed_with_pro_reg_tx(
                        obj : * const dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                    ) -> *mut dashcore::hash_types::ConfirmedHashHashedWithProRegTx
                    {
                        (*obj).confirmed_hash_hashed_with_pro_reg_tx
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_set_masternode_list_entry(
                        obj : * mut dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                        value: *mut dashcore::sml::masternode_list_entry::MasternodeListEntry,
                    ) {
                        (*obj).masternode_list_entry = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_set_entry_hash(
                        obj : * mut dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                        value: *mut dashcore::hash_types::Sha256dHash,
                    ) {
                        (*obj).entry_hash = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_set_confirmed_hash_hashed_with_pro_reg_tx(
                        obj : * mut dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
                        value: *mut dashcore::hash_types::ConfirmedHashHashedWithProRegTx,
                    ) {
                        (*obj).confirmed_hash_hashed_with_pro_reg_tx = value;
                    }
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`EntryMasternodeType`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_masternode_list_entry_EntryMasternodeType {
                    Regular,
                    HighPerformance {
                        platform_http_port: u16,
                        platform_node_id: *mut dashcore::hash_types::PubkeyHash,
                    },
                }
                impl
                    ferment::FFIConversionFrom<
                        dashcore::sml::masternode_list_entry::EntryMasternodeType,
                    > for dashcore_sml_masternode_list_entry_EntryMasternodeType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_masternode_list_entry_EntryMasternodeType,
                    ) -> dashcore::sml::masternode_list_entry::EntryMasternodeType
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dashcore_sml_masternode_list_entry_EntryMasternodeType :: Regular => dashcore :: sml :: masternode_list_entry :: EntryMasternodeType :: Regular , dashcore_sml_masternode_list_entry_EntryMasternodeType :: HighPerformance { platform_http_port , platform_node_id } => dashcore :: sml :: masternode_list_entry :: EntryMasternodeType :: HighPerformance { platform_http_port : * platform_http_port , platform_node_id : std :: ptr :: read (* platform_node_id) } }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        dashcore::sml::masternode_list_entry::EntryMasternodeType,
                    > for dashcore_sml_masternode_list_entry_EntryMasternodeType
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::masternode_list_entry::EntryMasternodeType,
                    ) -> *const dashcore_sml_masternode_list_entry_EntryMasternodeType
                    {
                        ferment :: boxed (match obj { dashcore :: sml :: masternode_list_entry :: EntryMasternodeType :: Regular => dashcore_sml_masternode_list_entry_EntryMasternodeType :: Regular , dashcore :: sml :: masternode_list_entry :: EntryMasternodeType :: HighPerformance { platform_http_port , platform_node_id } => dashcore_sml_masternode_list_entry_EntryMasternodeType :: HighPerformance { platform_http_port : platform_http_port , platform_node_id : ferment :: boxed (platform_node_id) } , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl Drop for dashcore_sml_masternode_list_entry_EntryMasternodeType {
                    fn drop(&mut self) {
                        unsafe {
                            match self { dashcore_sml_masternode_list_entry_EntryMasternodeType :: Regular => { } , dashcore_sml_masternode_list_entry_EntryMasternodeType :: HighPerformance { platform_http_port , platform_node_id } => { ; ; ferment :: unbox_any (* platform_node_id) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_EntryMasternodeType_Regular_ctor(
                ) -> *mut dashcore_sml_masternode_list_entry_EntryMasternodeType {
                    ferment::boxed(
                        dashcore_sml_masternode_list_entry_EntryMasternodeType::Regular {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_EntryMasternodeType_HighPerformance_ctor(
                    platform_http_port: u16,
                    platform_node_id: *mut dashcore::hash_types::PubkeyHash,
                ) -> *mut dashcore_sml_masternode_list_entry_EntryMasternodeType {
                    ferment::boxed(
                        dashcore_sml_masternode_list_entry_EntryMasternodeType::HighPerformance {
                            platform_http_port,
                            platform_node_id,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_EntryMasternodeType_destroy(
                    ffi: *mut dashcore_sml_masternode_list_entry_EntryMasternodeType,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`OperatorPublicKey`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct dashcore_sml_masternode_list_entry_OperatorPublicKey { pub data : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey , pub version : u16 }
                impl
                    ferment::FFIConversionFrom<
                        dashcore::sml::masternode_list_entry::OperatorPublicKey,
                    > for dashcore_sml_masternode_list_entry_OperatorPublicKey
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_masternode_list_entry_OperatorPublicKey,
                    ) -> dashcore::sml::masternode_list_entry::OperatorPublicKey
                    {
                        let ffi_ref = &*ffi;
                        dashcore :: sml :: masternode_list_entry :: OperatorPublicKey { data : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_from (ffi_ref . data) , version : ffi_ref . version }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        dashcore::sml::masternode_list_entry::OperatorPublicKey,
                    > for dashcore_sml_masternode_list_entry_OperatorPublicKey
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::masternode_list_entry::OperatorPublicKey,
                    ) -> *const dashcore_sml_masternode_list_entry_OperatorPublicKey
                    {
                        ferment :: boxed (dashcore_sml_masternode_list_entry_OperatorPublicKey { data : < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSPublicKey >> :: ffi_to (obj . data) , version : obj . version })
                    }
                }
                impl Drop for dashcore_sml_masternode_list_entry_OperatorPublicKey {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.data);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_ctor(
                    data : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey,
                    version: u16,
                ) -> *mut dashcore_sml_masternode_list_entry_OperatorPublicKey {
                    ferment::boxed(dashcore_sml_masternode_list_entry_OperatorPublicKey {
                        data,
                        version,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_destroy(
                    ffi: *mut dashcore_sml_masternode_list_entry_OperatorPublicKey,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_get_data (obj : * const dashcore_sml_masternode_list_entry_OperatorPublicKey) -> * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey{
                    (*obj).data
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_get_version(
                    obj: *const dashcore_sml_masternode_list_entry_OperatorPublicKey,
                ) -> u16 {
                    (*obj).version
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_set_data(
                    obj: *mut dashcore_sml_masternode_list_entry_OperatorPublicKey,
                    value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSPublicKey,
                ) {
                    (*obj).data = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_masternode_list_entry_OperatorPublicKey_set_version(
                    obj: *mut dashcore_sml_masternode_list_entry_OperatorPublicKey,
                    value: u16,
                ) {
                    (*obj).version = value;
                }
            }
            pub mod quorum_entry {
                pub mod qualified_quorum_entry {
                    use crate as example_dashcore;
                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`VerifyingChainLockSignaturesType`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType {
                        Rotating (* mut crate :: fermented :: generics :: Arr_dashcore_bls_sig_utils_BLSSignature_4) , NonRotating (* mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) }
                    impl ferment :: FFIConversionFrom < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType > for dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType { unsafe fn ffi_from_const (ffi : * const dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType) -> dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType { let ffi_ref = & * ffi ; match ffi_ref { dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: Rotating (o_0) => dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType :: Rotating (< crate :: fermented :: generics :: Arr_dashcore_bls_sig_utils_BLSSignature_4 as ferment :: FFIConversionFrom < [dashcore :: bls_sig_utils :: BLSSignature ; 4] >> :: ffi_from (* o_0)) , dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: NonRotating (o_0) => dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType :: NonRotating (< crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (* o_0)) } } }
                    impl ferment :: FFIConversionTo < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType > for dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType { unsafe fn ffi_to_const (obj : dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType) -> * const dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType { ferment :: boxed (match obj { dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType :: Rotating (o_0) => dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: Rotating (< crate :: fermented :: generics :: Arr_dashcore_bls_sig_utils_BLSSignature_4 as ferment :: FFIConversionTo < [dashcore :: bls_sig_utils :: BLSSignature ; 4] >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType :: NonRotating (o_0) => dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: NonRotating (< crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") }) } }
                    impl Drop for dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType {
                        fn drop(&mut self) {
                            unsafe {
                                match self { dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: Rotating (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: NonRotating (o_0) => { ferment :: unbox_any (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                            }
                        }
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType_Rotating_ctor (o_o_0 : * mut crate :: fermented :: generics :: Arr_dashcore_bls_sig_utils_BLSSignature_4) -> * mut dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType{
                        ferment :: boxed (dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: Rotating (o_o_0))
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType_NonRotating_ctor (o_o_0 : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature) -> * mut dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType{
                        ferment :: boxed (dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType :: NonRotating (o_o_0))
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType_destroy(
                        ffi : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[doc = "FFI-representation of the [`QualifiedQuorumEntry`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { pub quorum_entry : * mut dashcore :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry , pub verified : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus , pub commitment_hash : * mut dashcore :: hash_types :: QuorumCommitmentHash , pub entry_hash : * mut dashcore :: hash_types :: QuorumEntryHash , pub verifying_chain_lock_signature : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType }
                    impl ferment :: FFIConversionFrom < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > for dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_from_const (ffi : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry { let ffi_ref = & * ffi ; dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry { quorum_entry : std :: ptr :: read (ffi_ref . quorum_entry) , verified : < crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus >> :: ffi_from (ffi_ref . verified) , commitment_hash : std :: ptr :: read (ffi_ref . commitment_hash) , entry_hash : std :: ptr :: read (ffi_ref . entry_hash) , verifying_chain_lock_signature : < crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType as ferment :: FFIConversionFrom < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType >> :: ffi_from_opt (ffi_ref . verifying_chain_lock_signature) } } }
                    impl ferment :: FFIConversionTo < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > for dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_to_const (obj : dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry) -> * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { ferment :: boxed (dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { quorum_entry : ferment :: boxed (obj . quorum_entry) , verified : < crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus as ferment :: FFIConversionTo < dashcore :: sml :: llmq_entry_verification :: LLMQEntryVerificationStatus >> :: ffi_to (obj . verified) , commitment_hash : ferment :: boxed (obj . commitment_hash) , entry_hash : ferment :: boxed (obj . entry_hash) , verifying_chain_lock_signature : < crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType as ferment :: FFIConversionTo < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: VerifyingChainLockSignaturesType >> :: ffi_to_opt (obj . verifying_chain_lock_signature) }) } }
                    impl Drop for dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                ferment::unbox_any(ffi_ref.quorum_entry);
                                ferment::unbox_any(ffi_ref.verified);
                                ferment::unbox_any(ffi_ref.commitment_hash);
                                ferment::unbox_any(ffi_ref.entry_hash);
                                ferment::unbox_any_opt(ffi_ref.verifying_chain_lock_signature);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_ctor(
                        quorum_entry : * mut dashcore :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry,
                        verified : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus,
                        commitment_hash: *mut dashcore::hash_types::QuorumCommitmentHash,
                        entry_hash: *mut dashcore::hash_types::QuorumEntryHash,
                        verifying_chain_lock_signature : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType,
                    ) -> *mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry
                    {
                        ferment::boxed(
                            dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry {
                                quorum_entry,
                                verified,
                                commitment_hash,
                                entry_hash,
                                verifying_chain_lock_signature,
                            },
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_destroy(
                        ffi : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_get_quorum_entry (obj : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> * mut dashcore :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry{
                        (*obj).quorum_entry
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_get_verified (obj : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus{
                        (*obj).verified
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_get_commitment_hash(
                        obj : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                    ) -> *mut dashcore::hash_types::QuorumCommitmentHash {
                        (*obj).commitment_hash
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_get_entry_hash(
                        obj : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                    ) -> *mut dashcore::hash_types::QuorumEntryHash {
                        (*obj).entry_hash
                    }
                    #[no_mangle]                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_get_verifying_chain_lock_signature (obj : * const dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType{
                        (*obj).verifying_chain_lock_signature
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_quorum_entry(
                        obj : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                        value : * mut dashcore :: transaction :: special_transaction :: quorum_commitment :: QuorumEntry,
                    ) {
                        (*obj).quorum_entry = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_verified(
                        obj : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                        value : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_entry_verification :: dashcore_sml_llmq_entry_verification_LLMQEntryVerificationStatus,
                    ) {
                        (*obj).verified = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_commitment_hash(
                        obj : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                        value: *mut dashcore::hash_types::QuorumCommitmentHash,
                    ) {
                        (*obj).commitment_hash = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_entry_hash(
                        obj : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                        value: *mut dashcore::hash_types::QuorumEntryHash,
                    ) {
                        (*obj).entry_hash = value;
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_verifying_chain_lock_signature(
                        obj : * mut dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
                        value : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_VerifyingChainLockSignaturesType,
                    ) {
                        (*obj).verifying_chain_lock_signature = value;
                    }
                }
            }
            pub mod quorum_validation_error {
                use crate as example_dashcore;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ClientDataRetrievalError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_quorum_validation_error_ClientDataRetrievalError {
                    RequiredBlockNotPresent(*mut dashcore::hash_types::BlockHash),
                    CoinbaseNotFoundOnBlock(*mut dashcore::hash_types::BlockHash),
                }
                impl
                    ferment::FFIConversionFrom<
                        dashcore::sml::quorum_validation_error::ClientDataRetrievalError,
                    > for dashcore_sml_quorum_validation_error_ClientDataRetrievalError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_quorum_validation_error_ClientDataRetrievalError,
                    ) -> dashcore::sml::quorum_validation_error::ClientDataRetrievalError
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: RequiredBlockNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError :: RequiredBlockNotPresent (std :: ptr :: read (* o_0)) , dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (o_0) => dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (std :: ptr :: read (* o_0)) }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        dashcore::sml::quorum_validation_error::ClientDataRetrievalError,
                    > for dashcore_sml_quorum_validation_error_ClientDataRetrievalError
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::quorum_validation_error::ClientDataRetrievalError,
                    ) -> *const dashcore_sml_quorum_validation_error_ClientDataRetrievalError
                    {
                        ferment :: boxed (match obj { dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError :: RequiredBlockNotPresent (o_0) => dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: RequiredBlockNotPresent (ferment :: boxed (o_0)) , dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (o_0) => dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (ferment :: boxed (o_0)) , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl Drop for dashcore_sml_quorum_validation_error_ClientDataRetrievalError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: RequiredBlockNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (o_0) => { ferment :: unbox_any (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_ClientDataRetrievalError_RequiredBlockNotPresent_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_ClientDataRetrievalError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: RequiredBlockNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_ClientDataRetrievalError_CoinbaseNotFoundOnBlock_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_ClientDataRetrievalError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_ClientDataRetrievalError :: CoinbaseNotFoundOnBlock (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_ClientDataRetrievalError_destroy(
                    ffi: *mut dashcore_sml_quorum_validation_error_ClientDataRetrievalError,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`QuorumValidationError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum dashcore_sml_quorum_validation_error_QuorumValidationError {
                    RequiredBlockNotPresent (* mut dashcore :: hash_types :: BlockHash , * mut std :: os :: raw :: c_char) , RequiredBlockHeightNotPresent (* mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight) , VerifyingMasternodeListNotPresent (* mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight) , RequiredMasternodeListNotPresent (* mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight) , RequiredChainLockNotPresent (* mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight , * mut dashcore :: hash_types :: BlockHash) , RequiredRotatedChainLockSigNotPresent (u8 , * mut dashcore :: hash_types :: BlockHash) , RequiredRotatedChainLockSigsNotPresent (* mut dashcore :: hash_types :: BlockHash) , InsufficientSigners { required : u64 , found : u64 } , InsufficientValidMembers { required : u64 , found : u64 } , MismatchedBitsetLengths { signers_len : usize , valid_members_len : usize } , InvalidQuorumPublicKey , InvalidBLSPublicKey (* mut std :: os :: raw :: c_char) , InvalidBLSSignature (* mut std :: os :: raw :: c_char) , InvalidQuorumSignature , InvalidFinalSignature , AllCommitmentAggregatedSignatureNotValid (* mut std :: os :: raw :: c_char) , ThresholdSignatureNotValid (* mut std :: os :: raw :: c_char) , CommitmentHashNotPresent , RequiredSnapshotNotPresent (* mut dashcore :: hash_types :: BlockHash) , SMLError (* mut crate :: fermented :: types :: dashcore :: sml :: error :: dashcore_sml_error_SmlError) , RequiredQuorumIndexNotPresent (* mut dashcore :: hash_types :: QuorumHash) , CorruptedCodeExecution (* mut std :: os :: raw :: c_char) , ExpectedOnlyRotatedQuorums (* mut dashcore :: hash_types :: QuorumHash , * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType) , ClientDataRetrievalError (* mut crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_ClientDataRetrievalError) , FeatureNotTurnedOn (* mut std :: os :: raw :: c_char) }
                impl
                    ferment::FFIConversionFrom<
                        dashcore::sml::quorum_validation_error::QuorumValidationError,
                    > for dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const dashcore_sml_quorum_validation_error_QuorumValidationError,
                    ) -> dashcore::sml::quorum_validation_error::QuorumValidationError
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockNotPresent (o_0 , o_1) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredBlockNotPresent (std :: ptr :: read (* o_0) , < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_1)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockHeightNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredBlockHeightNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionFrom < dashcore :: prelude :: CoreBlockHeight >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: VerifyingMasternodeListNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: VerifyingMasternodeListNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionFrom < dashcore :: prelude :: CoreBlockHeight >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredMasternodeListNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredMasternodeListNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionFrom < dashcore :: prelude :: CoreBlockHeight >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredChainLockNotPresent (o_0 , o_1) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredChainLockNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionFrom < dashcore :: prelude :: CoreBlockHeight >> :: ffi_from (* o_0) , std :: ptr :: read (* o_1)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (o_0 , o_1) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (* o_0 , std :: ptr :: read (* o_1)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (std :: ptr :: read (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientSigners { required , found } => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InsufficientSigners { required : * required , found : * found } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientValidMembers { required , found } => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InsufficientValidMembers { required : * required , found : * found } , dashcore_sml_quorum_validation_error_QuorumValidationError :: MismatchedBitsetLengths { signers_len , valid_members_len } => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: MismatchedBitsetLengths { signers_len : * signers_len , valid_members_len : * valid_members_len } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumPublicKey => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidQuorumPublicKey , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSPublicKey (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidBLSPublicKey (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSSignature (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidBLSSignature (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumSignature => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidQuorumSignature , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidFinalSignature => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidFinalSignature , dashcore_sml_quorum_validation_error_QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: ThresholdSignatureNotValid (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ThresholdSignatureNotValid (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: CommitmentHashNotPresent => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: CommitmentHashNotPresent , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredSnapshotNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredSnapshotNotPresent (std :: ptr :: read (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: SMLError (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: SMLError (< crate :: fermented :: types :: dashcore :: sml :: error :: dashcore_sml_error_SmlError as ferment :: FFIConversionFrom < dashcore :: sml :: error :: SmlError >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredQuorumIndexNotPresent (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredQuorumIndexNotPresent (std :: ptr :: read (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: CorruptedCodeExecution (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: CorruptedCodeExecution (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: ExpectedOnlyRotatedQuorums (o_0 , o_1) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ExpectedOnlyRotatedQuorums (std :: ptr :: read (* o_0) , < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (* o_1)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: ClientDataRetrievalError (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ClientDataRetrievalError (< crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_ClientDataRetrievalError as ferment :: FFIConversionFrom < dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError >> :: ffi_from (* o_0)) , dashcore_sml_quorum_validation_error_QuorumValidationError :: FeatureNotTurnedOn (o_0) => dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: FeatureNotTurnedOn (< std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (* o_0)) }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        dashcore::sml::quorum_validation_error::QuorumValidationError,
                    > for dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    unsafe fn ffi_to_const(
                        obj: dashcore::sml::quorum_validation_error::QuorumValidationError,
                    ) -> *const dashcore_sml_quorum_validation_error_QuorumValidationError
                    {
                        ferment :: boxed (match obj { dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredBlockNotPresent (o_0 , o_1) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockNotPresent (ferment :: boxed (o_0) , < std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_1)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredBlockHeightNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockHeightNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionTo < dashcore :: prelude :: CoreBlockHeight >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: VerifyingMasternodeListNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: VerifyingMasternodeListNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionTo < dashcore :: prelude :: CoreBlockHeight >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredMasternodeListNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredMasternodeListNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionTo < dashcore :: prelude :: CoreBlockHeight >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredChainLockNotPresent (o_0 , o_1) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredChainLockNotPresent (< crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight as ferment :: FFIConversionTo < dashcore :: prelude :: CoreBlockHeight >> :: ffi_to (o_0) , ferment :: boxed (o_1)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (o_0 , o_1) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (o_0 , ferment :: boxed (o_1)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (ferment :: boxed (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InsufficientSigners { required , found } => dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientSigners { required : required , found : found } , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InsufficientValidMembers { required , found } => dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientValidMembers { required : required , found : found } , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: MismatchedBitsetLengths { signers_len , valid_members_len } => dashcore_sml_quorum_validation_error_QuorumValidationError :: MismatchedBitsetLengths { signers_len : signers_len , valid_members_len : valid_members_len } , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidQuorumPublicKey => dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumPublicKey , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidBLSPublicKey (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSPublicKey (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidBLSSignature (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSSignature (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidQuorumSignature => dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumSignature , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: InvalidFinalSignature => dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidFinalSignature , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ThresholdSignatureNotValid (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: ThresholdSignatureNotValid (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: CommitmentHashNotPresent => dashcore_sml_quorum_validation_error_QuorumValidationError :: CommitmentHashNotPresent , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredSnapshotNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredSnapshotNotPresent (ferment :: boxed (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: SMLError (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: SMLError (< crate :: fermented :: types :: dashcore :: sml :: error :: dashcore_sml_error_SmlError as ferment :: FFIConversionTo < dashcore :: sml :: error :: SmlError >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: RequiredQuorumIndexNotPresent (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredQuorumIndexNotPresent (ferment :: boxed (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: CorruptedCodeExecution (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: CorruptedCodeExecution (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ExpectedOnlyRotatedQuorums (o_0 , o_1) => dashcore_sml_quorum_validation_error_QuorumValidationError :: ExpectedOnlyRotatedQuorums (ferment :: boxed (o_0) , < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionTo < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_to (o_1)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: ClientDataRetrievalError (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: ClientDataRetrievalError (< crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_ClientDataRetrievalError as ferment :: FFIConversionTo < dashcore :: sml :: quorum_validation_error :: ClientDataRetrievalError >> :: ffi_to (o_0)) , dashcore :: sml :: quorum_validation_error :: QuorumValidationError :: FeatureNotTurnedOn (o_0) => dashcore_sml_quorum_validation_error_QuorumValidationError :: FeatureNotTurnedOn (< std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl Drop for dashcore_sml_quorum_validation_error_QuorumValidationError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockNotPresent (o_0 , o_1) => { ferment :: unbox_any (* o_0) ; ; ferment :: unbox_string (* o_1) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockHeightNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: VerifyingMasternodeListNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredMasternodeListNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredChainLockNotPresent (o_0 , o_1) => { ferment :: unbox_any (* o_0) ; ; ferment :: unbox_any (* o_1) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (o_0 , o_1) => { ; ; ferment :: unbox_any (* o_1) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientSigners { required , found } => { ; ; ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientValidMembers { required , found } => { ; ; ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: MismatchedBitsetLengths { signers_len , valid_members_len } => { ; ; ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumPublicKey => { } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSPublicKey (o_0) => { ferment :: unbox_string (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSSignature (o_0) => { ferment :: unbox_string (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumSignature => { } , dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidFinalSignature => { } , dashcore_sml_quorum_validation_error_QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (o_0) => { ferment :: unbox_string (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: ThresholdSignatureNotValid (o_0) => { ferment :: unbox_string (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: CommitmentHashNotPresent => { } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredSnapshotNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: SMLError (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredQuorumIndexNotPresent (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: CorruptedCodeExecution (o_0) => { ferment :: unbox_string (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: ExpectedOnlyRotatedQuorums (o_0 , o_1) => { ferment :: unbox_any (* o_0) ; ; ferment :: unbox_any (* o_1) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: ClientDataRetrievalError (o_0) => { ferment :: unbox_any (* o_0) ; } , dashcore_sml_quorum_validation_error_QuorumValidationError :: FeatureNotTurnedOn (o_0) => { ferment :: unbox_string (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredBlockNotPresent_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                    o_o_1: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockNotPresent (o_o_0 , o_o_1))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredBlockHeightNotPresent_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredBlockHeightNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_VerifyingMasternodeListNotPresent_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: VerifyingMasternodeListNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredMasternodeListNotPresent_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredMasternodeListNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredChainLockNotPresent_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: prelude :: dashcore_prelude_CoreBlockHeight,
                    o_o_1: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredChainLockNotPresent (o_o_0 , o_o_1))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredRotatedChainLockSigNotPresent_ctor(
                    o_o_0: u8,
                    o_o_1: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigNotPresent (o_o_0 , o_o_1))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredRotatedChainLockSigsNotPresent_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredRotatedChainLockSigsNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InsufficientSigners_ctor(
                    required: u64,
                    found: u64,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientSigners { required , found })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InsufficientValidMembers_ctor(
                    required: u64,
                    found: u64,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InsufficientValidMembers { required , found })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_MismatchedBitsetLengths_ctor(
                    signers_len: usize,
                    valid_members_len: usize,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: MismatchedBitsetLengths { signers_len , valid_members_len })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InvalidQuorumPublicKey_ctor(
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumPublicKey { })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InvalidBLSPublicKey_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSPublicKey (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InvalidBLSSignature_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidBLSSignature (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InvalidQuorumSignature_ctor(
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidQuorumSignature { })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_InvalidFinalSignature_ctor(
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: InvalidFinalSignature { })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_AllCommitmentAggregatedSignatureNotValid_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: AllCommitmentAggregatedSignatureNotValid (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_ThresholdSignatureNotValid_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: ThresholdSignatureNotValid (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_CommitmentHashNotPresent_ctor(
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: CommitmentHashNotPresent { })
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredSnapshotNotPresent_ctor(
                    o_o_0: *mut dashcore::hash_types::BlockHash,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredSnapshotNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_SMLError_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: sml :: error :: dashcore_sml_error_SmlError,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment::boxed(
                        dashcore_sml_quorum_validation_error_QuorumValidationError::SMLError(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_RequiredQuorumIndexNotPresent_ctor(
                    o_o_0: *mut dashcore::hash_types::QuorumHash,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: RequiredQuorumIndexNotPresent (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_CorruptedCodeExecution_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: CorruptedCodeExecution (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_ExpectedOnlyRotatedQuorums_ctor(
                    o_o_0: *mut dashcore::hash_types::QuorumHash,
                    o_o_1 : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: ExpectedOnlyRotatedQuorums (o_o_0 , o_o_1))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_ClientDataRetrievalError_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_validation_error :: dashcore_sml_quorum_validation_error_ClientDataRetrievalError,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: ClientDataRetrievalError (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_FeatureNotTurnedOn_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut dashcore_sml_quorum_validation_error_QuorumValidationError
                {
                    ferment :: boxed (dashcore_sml_quorum_validation_error_QuorumValidationError :: FeatureNotTurnedOn (o_o_0))
                }
                #[no_mangle]
                pub unsafe extern "C" fn dashcore_sml_quorum_validation_error_QuorumValidationError_destroy(
                    ffi: *mut dashcore_sml_quorum_validation_error_QuorumValidationError,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
        }
        pub mod prelude {
            use crate as example_dashcore;
            #[doc = "FFI-representation of the [`CoreBlockHeight`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct dashcore_prelude_CoreBlockHeight(u32);
            impl ferment::FFIConversionFrom<dashcore::prelude::CoreBlockHeight>
                for dashcore_prelude_CoreBlockHeight
            {
                unsafe fn ffi_from_const(
                    ffi: *const dashcore_prelude_CoreBlockHeight,
                ) -> dashcore::prelude::CoreBlockHeight {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
            }
            impl ferment::FFIConversionTo<dashcore::prelude::CoreBlockHeight>
                for dashcore_prelude_CoreBlockHeight
            {
                unsafe fn ffi_to_const(
                    obj: dashcore::prelude::CoreBlockHeight,
                ) -> *const dashcore_prelude_CoreBlockHeight {
                    ferment::boxed(dashcore_prelude_CoreBlockHeight(obj))
                }
            }
            impl Drop for dashcore_prelude_CoreBlockHeight {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_prelude_CoreBlockHeight_ctor(
                o_0: u32,
            ) -> *mut dashcore_prelude_CoreBlockHeight {
                ferment::boxed(dashcore_prelude_CoreBlockHeight(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_prelude_CoreBlockHeight_destroy(
                ffi: *mut dashcore_prelude_CoreBlockHeight,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_prelude_CoreBlockHeight_get_0(
                obj: *const dashcore_prelude_CoreBlockHeight,
            ) -> u32 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn dashcore_prelude_CoreBlockHeight_set_0(
                obj: *mut dashcore_prelude_CoreBlockHeight,
                value: u32,
            ) {
                (*obj).0 = value;
            }
        }
    }
    pub mod example_dashcore {
        use crate as example_dashcore;
        #[doc = "FFI-representation of the [`SPV`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct example_dashcore_SPV {
            pub version: u32,
        }
        impl ferment::FFIConversionFrom<example_dashcore::SPV> for example_dashcore_SPV {
            unsafe fn ffi_from_const(ffi: *const example_dashcore_SPV) -> example_dashcore::SPV {
                let ffi_ref = &*ffi;
                example_dashcore::SPV {
                    version: ffi_ref.version,
                }
            }
        }
        impl ferment::FFIConversionTo<example_dashcore::SPV> for example_dashcore_SPV {
            unsafe fn ffi_to_const(obj: example_dashcore::SPV) -> *const example_dashcore_SPV {
                ferment::boxed(example_dashcore_SPV {
                    version: obj.version,
                })
            }
        }
        impl Drop for example_dashcore_SPV {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_dashcore_SPV_ctor(
            version: u32,
        ) -> *mut example_dashcore_SPV {
            ferment::boxed(example_dashcore_SPV { version })
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_dashcore_SPV_destroy(ffi: *mut example_dashcore_SPV) {
            ferment::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_dashcore_SPV_get_version(
            obj: *const example_dashcore_SPV,
        ) -> u32 {
            (*obj).version
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_dashcore_SPV_set_version(
            obj: *mut example_dashcore_SPV,
            value: u32,
        ) {
            (*obj).version = value;
        }
        pub mod dash {
            use crate as example_dashcore;
            #[doc = "FFI-representation of the [`example_dashcore::dash::setup_dashcore`]"]
            #[no_mangle]
            pub unsafe extern "C" fn example_dashcore_dash_setup_dashcore(
                transaction : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction,
            ) {
                let obj = example_dashcore :: dash :: setup_dashcore (< crate :: fermented :: types :: dashcore :: blockdata :: transaction :: dashcore_blockdata_transaction_Transaction as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: Transaction >> :: ffi_from (transaction)) ;
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
    use crate as example_dashcore;
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dashcore_blockdata_transaction_txin_TxIn { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn }
    impl ferment::FFIConversionFrom<Vec<dashcore::blockdata::transaction::txin::TxIn>>
        for Vec_dashcore_blockdata_transaction_txin_TxIn
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_dashcore_blockdata_transaction_txin_TxIn,
        ) -> Vec<dashcore::blockdata::transaction::txin::TxIn> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| {
                < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: txin :: TxIn >> :: ffi_from (* o)
            })
        }
    }
    impl ferment::FFIConversionTo<Vec<dashcore::blockdata::transaction::txin::TxIn>>
        for Vec_dashcore_blockdata_transaction_txin_TxIn
    {
        unsafe fn ffi_to_const(
            obj: Vec<dashcore::blockdata::transaction::txin::TxIn>,
        ) -> *const Vec_dashcore_blockdata_transaction_txin_TxIn {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| {
                    < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: txin :: TxIn >> :: ffi_to (o)
                }),
            })
        }
    }
    impl Drop for Vec_dashcore_blockdata_transaction_txin_TxIn {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txin_TxIn_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn,
    ) -> *mut Vec_dashcore_blockdata_transaction_txin_TxIn {
        ferment::boxed(Vec_dashcore_blockdata_transaction_txin_TxIn { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txin_TxIn_destroy(
        ffi: *mut Vec_dashcore_blockdata_transaction_txin_TxIn,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txin_TxIn_value_at_index (ffi : * const Vec_dashcore_blockdata_transaction_txin_TxIn , index : usize) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn{
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txin_TxIn_set_value_at_index(
        ffi: *mut Vec_dashcore_blockdata_transaction_txin_TxIn,
        index: usize,
        value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txin :: dashcore_blockdata_transaction_txin_TxIn,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dashcore_blockdata_transaction_txout_TxOut { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut }
    impl ferment::FFIConversionFrom<Vec<dashcore::blockdata::transaction::txout::TxOut>>
        for Vec_dashcore_blockdata_transaction_txout_TxOut
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_dashcore_blockdata_transaction_txout_TxOut,
        ) -> Vec<dashcore::blockdata::transaction::txout::TxOut> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| {
                < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut as ferment :: FFIConversionFrom < dashcore :: blockdata :: transaction :: txout :: TxOut >> :: ffi_from (* o)
            })
        }
    }
    impl ferment::FFIConversionTo<Vec<dashcore::blockdata::transaction::txout::TxOut>>
        for Vec_dashcore_blockdata_transaction_txout_TxOut
    {
        unsafe fn ffi_to_const(
            obj: Vec<dashcore::blockdata::transaction::txout::TxOut>,
        ) -> *const Vec_dashcore_blockdata_transaction_txout_TxOut {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| {
                    < crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut as ferment :: FFIConversionTo < dashcore :: blockdata :: transaction :: txout :: TxOut >> :: ffi_to (o)
                }),
            })
        }
    }
    impl Drop for Vec_dashcore_blockdata_transaction_txout_TxOut {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txout_TxOut_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut,
    ) -> *mut Vec_dashcore_blockdata_transaction_txout_TxOut {
        ferment::boxed(Vec_dashcore_blockdata_transaction_txout_TxOut { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txout_TxOut_destroy(
        ffi: *mut Vec_dashcore_blockdata_transaction_txout_TxOut,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txout_TxOut_value_at_index (ffi : * const Vec_dashcore_blockdata_transaction_txout_TxOut , index : usize) -> * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut{
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_blockdata_transaction_txout_TxOut_set_value_at_index(
        ffi: *mut Vec_dashcore_blockdata_transaction_txout_TxOut,
        index: usize,
        value : * mut crate :: fermented :: types :: dashcore :: blockdata :: transaction :: txout :: dashcore_blockdata_transaction_txout_TxOut,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_96 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<[u8; 96]> for Arr_u8_96 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_96) -> [u8; 96] {
            let ffi_ref = &*ffi;
            TryFrom::<Vec<u8>>::try_from(ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o))
                .unwrap()
        }
    }
    impl ferment::FFIConversionTo<[u8; 96]> for Arr_u8_96 {
        unsafe fn ffi_to_const(obj: [u8; 96]) -> *const Arr_u8_96 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    impl Drop for Arr_u8_96 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_96_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_96 {
        ferment::boxed(Arr_u8_96 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_96_destroy(ffi: *mut Arr_u8_96) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_96_value_at_index(ffi: *const Arr_u8_96, index: usize) -> u8 {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_96_set_value_at_index(
        ffi: *mut Arr_u8_96,
        index: usize,
        value: u8,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_20 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<[u8; 20]> for Arr_u8_20 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_20) -> [u8; 20] {
            let ffi_ref = &*ffi;
            TryFrom::<Vec<u8>>::try_from(ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o))
                .unwrap()
        }
    }
    impl ferment::FFIConversionTo<[u8; 20]> for Arr_u8_20 {
        unsafe fn ffi_to_const(obj: [u8; 20]) -> *const Arr_u8_20 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    impl Drop for Arr_u8_20 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_20 {
        ferment::boxed(Arr_u8_20 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_destroy(ffi: *mut Arr_u8_20) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_value_at_index(ffi: *const Arr_u8_20, index: usize) -> u8 {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_20_set_value_at_index(
        ffi: *mut Arr_u8_20,
        index: usize,
        value: u8,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o)
        }
    }
    impl ferment::FFIConversionTo<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    impl Drop for Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment::boxed(Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_value_at_index(ffi: *const Vec_u8, index: usize) -> u8 {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_set_value_at_index(ffi: *mut Vec_u8, index: usize, value: u8) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_bool {
        pub count: usize,
        pub values: *mut bool,
    }
    impl ferment::FFIConversionFrom<Vec<bool>> for Vec_bool {
        unsafe fn ffi_from_const(ffi: *const Vec_bool) -> Vec<bool> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o)
        }
    }
    impl ferment::FFIConversionTo<Vec<bool>> for Vec_bool {
        unsafe fn ffi_to_const(obj: Vec<bool>) -> *const Vec_bool {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    impl Drop for Vec_bool {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_bool_ctor(count: usize, values: *mut bool) -> *mut Vec_bool {
        ferment::boxed(Vec_bool { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_bool_destroy(ffi: *mut Vec_bool) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_bool_value_at_index(ffi: *const Vec_bool, index: usize) -> bool {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_bool_set_value_at_index(
        ffi: *mut Vec_bool,
        index: usize,
        value: bool,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[cfg(any(any(feature = "std")))]
    #[repr(C)]
    #[derive(Clone)]
    #[cfg(any(any(feature = "std")))]
    pub struct Vec_i32 {
        pub count: usize,
        pub values: *mut i32,
    }
    #[cfg(any(any(feature = "std")))]
    #[cfg(any(any(feature = "std")))]
    impl ferment::FFIConversionFrom<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o)
        }
    }
    #[cfg(any(any(feature = "std")))]
    #[cfg(any(any(feature = "std")))]
    impl ferment::FFIConversionTo<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    #[cfg(any(any(feature = "std")))]
    #[cfg(any(any(feature = "std")))]
    impl Drop for Vec_i32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[cfg(any(any(feature = "std")))]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(count: usize, values: *mut i32) -> *mut Vec_i32 {
        ferment::boxed(Vec_i32 { count, values })
    }
    #[cfg(any(any(feature = "std")))]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment::unbox_any(ffi);
    }
    #[cfg(any(any(feature = "std")))]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_value_at_index(ffi: *const Vec_i32, index: usize) -> i32 {
        *(*ffi).values.add(index)
    }
    #[cfg(any(any(feature = "std")))]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_set_value_at_index(
        ffi: *mut Vec_i32,
        index: usize,
        value: i32,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_vec_Vec_dashcore_transaction_outpoint_OutPoint {
        pub count: usize,
        pub values: *mut *mut dashcore::transaction::outpoint::OutPoint,
    }
    impl ferment::FFIConversionFrom<std::vec::Vec<dashcore::transaction::outpoint::OutPoint>>
        for std_vec_Vec_dashcore_transaction_outpoint_OutPoint
    {
        unsafe fn ffi_from_const(
            ffi: *const std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
        ) -> std::vec::Vec<dashcore::transaction::outpoint::OutPoint> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| std::ptr::read(*o))
        }
    }
    impl ferment::FFIConversionTo<std::vec::Vec<dashcore::transaction::outpoint::OutPoint>>
        for std_vec_Vec_dashcore_transaction_outpoint_OutPoint
    {
        unsafe fn ffi_to_const(
            obj: std::vec::Vec<dashcore::transaction::outpoint::OutPoint>,
        ) -> *const std_vec_Vec_dashcore_transaction_outpoint_OutPoint {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| ferment::boxed(o)),
            })
        }
    }
    impl Drop for std_vec_Vec_dashcore_transaction_outpoint_OutPoint {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_vec_Vec_dashcore_transaction_outpoint_OutPoint_ctor(
        count: usize,
        values: *mut *mut dashcore::transaction::outpoint::OutPoint,
    ) -> *mut std_vec_Vec_dashcore_transaction_outpoint_OutPoint {
        ferment::boxed(std_vec_Vec_dashcore_transaction_outpoint_OutPoint { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_vec_Vec_dashcore_transaction_outpoint_OutPoint_destroy(
        ffi: *mut std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_vec_Vec_dashcore_transaction_outpoint_OutPoint_value_at_index(
        ffi: *const std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
        index: usize,
    ) -> *mut dashcore::transaction::outpoint::OutPoint {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_vec_Vec_dashcore_transaction_outpoint_OutPoint_set_value_at_index(
        ffi: *mut std_vec_Vec_dashcore_transaction_outpoint_OutPoint,
        index: usize,
        value: *mut dashcore::transaction::outpoint::OutPoint,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dashcore_sml_llmq_type_DKGWindow {
        pub count: usize,
        pub values: *mut *mut dashcore::sml::llmq_type::DKGWindow,
    }
    impl ferment::FFIConversionFrom<Vec<dashcore::sml::llmq_type::DKGWindow>>
        for Vec_dashcore_sml_llmq_type_DKGWindow
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_dashcore_sml_llmq_type_DKGWindow,
        ) -> Vec<dashcore::sml::llmq_type::DKGWindow> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| std::ptr::read(*o))
        }
    }
    impl ferment::FFIConversionTo<Vec<dashcore::sml::llmq_type::DKGWindow>>
        for Vec_dashcore_sml_llmq_type_DKGWindow
    {
        unsafe fn ffi_to_const(
            obj: Vec<dashcore::sml::llmq_type::DKGWindow>,
        ) -> *const Vec_dashcore_sml_llmq_type_DKGWindow {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| ferment::boxed(o)),
            })
        }
    }
    impl Drop for Vec_dashcore_sml_llmq_type_DKGWindow {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_sml_llmq_type_DKGWindow_ctor(
        count: usize,
        values: *mut *mut dashcore::sml::llmq_type::DKGWindow,
    ) -> *mut Vec_dashcore_sml_llmq_type_DKGWindow {
        ferment::boxed(Vec_dashcore_sml_llmq_type_DKGWindow { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_sml_llmq_type_DKGWindow_destroy(
        ffi: *mut Vec_dashcore_sml_llmq_type_DKGWindow,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_sml_llmq_type_DKGWindow_value_at_index(
        ffi: *const Vec_dashcore_sml_llmq_type_DKGWindow,
        index: usize,
    ) -> *mut dashcore::sml::llmq_type::DKGWindow {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_sml_llmq_type_DKGWindow_set_value_at_index(
        ffi: *mut Vec_dashcore_sml_llmq_type_DKGWindow,
        index: usize,
        value: *mut dashcore::sml::llmq_type::DKGWindow,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_dashcore_bls_sig_utils_BLSSignature_4 { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature }
    impl ferment::FFIConversionFrom<[dashcore::bls_sig_utils::BLSSignature; 4]>
        for Arr_dashcore_bls_sig_utils_BLSSignature_4
    {
        unsafe fn ffi_from_const(
            ffi: *const Arr_dashcore_bls_sig_utils_BLSSignature_4,
        ) -> [dashcore::bls_sig_utils::BLSSignature; 4] {
            let ffi_ref = &*ffi;
            TryFrom :: < Vec < dashcore :: bls_sig_utils :: BLSSignature >> :: try_from (ferment :: from_group (ffi_ref . count , ffi_ref . values , | o | < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionFrom < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_from (* o))) . unwrap ()
        }
    }
    impl ferment::FFIConversionTo<[dashcore::bls_sig_utils::BLSSignature; 4]>
        for Arr_dashcore_bls_sig_utils_BLSSignature_4
    {
        unsafe fn ffi_to_const(
            obj: [dashcore::bls_sig_utils::BLSSignature; 4],
        ) -> *const Arr_dashcore_bls_sig_utils_BLSSignature_4 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| {
                    < crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature as ferment :: FFIConversionTo < dashcore :: bls_sig_utils :: BLSSignature >> :: ffi_to (o)
                }),
            })
        }
    }
    impl Drop for Arr_dashcore_bls_sig_utils_BLSSignature_4 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_dashcore_bls_sig_utils_BLSSignature_4_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
    ) -> *mut Arr_dashcore_bls_sig_utils_BLSSignature_4 {
        ferment::boxed(Arr_dashcore_bls_sig_utils_BLSSignature_4 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_dashcore_bls_sig_utils_BLSSignature_4_destroy(
        ffi: *mut Arr_dashcore_bls_sig_utils_BLSSignature_4,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_dashcore_bls_sig_utils_BLSSignature_4_value_at_index(
        ffi: *const Arr_dashcore_bls_sig_utils_BLSSignature_4,
        index: usize,
    ) -> *mut crate::fermented::types::dashcore::bls_sig_utils::dashcore_bls_sig_utils_BLSSignature
    {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_dashcore_bls_sig_utils_BLSSignature_4_set_value_at_index(
        ffi: *mut Arr_dashcore_bls_sig_utils_BLSSignature_4,
        index: usize,
        value : * mut crate :: fermented :: types :: dashcore :: bls_sig_utils :: dashcore_bls_sig_utils_BLSSignature,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_dashcore_transaction_txout_TxOut {
        pub count: usize,
        pub values: *mut *mut dashcore::transaction::txout::TxOut,
    }
    impl ferment::FFIConversionFrom<Vec<dashcore::transaction::txout::TxOut>>
        for Vec_dashcore_transaction_txout_TxOut
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_dashcore_transaction_txout_TxOut,
        ) -> Vec<dashcore::transaction::txout::TxOut> {
            let ffi_ref = &*ffi;
            ferment::from_group(ffi_ref.count, ffi_ref.values, |o| std::ptr::read(*o))
        }
    }
    impl ferment::FFIConversionTo<Vec<dashcore::transaction::txout::TxOut>>
        for Vec_dashcore_transaction_txout_TxOut
    {
        unsafe fn ffi_to_const(
            obj: Vec<dashcore::transaction::txout::TxOut>,
        ) -> *const Vec_dashcore_transaction_txout_TxOut {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| ferment::boxed(o)),
            })
        }
    }
    impl Drop for Vec_dashcore_transaction_txout_TxOut {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::unbox_any(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_transaction_txout_TxOut_ctor(
        count: usize,
        values: *mut *mut dashcore::transaction::txout::TxOut,
    ) -> *mut Vec_dashcore_transaction_txout_TxOut {
        ferment::boxed(Vec_dashcore_transaction_txout_TxOut { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_transaction_txout_TxOut_destroy(
        ffi: *mut Vec_dashcore_transaction_txout_TxOut,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_transaction_txout_TxOut_value_at_index(
        ffi: *const Vec_dashcore_transaction_txout_TxOut,
        index: usize,
    ) -> *mut dashcore::transaction::txout::TxOut {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_dashcore_transaction_txout_TxOut_set_value_at_index(
        ffi: *mut Vec_dashcore_transaction_txout_TxOut,
        index: usize,
        value: *mut dashcore::transaction::txout::TxOut,
    ) {
        *(*ffi).values.add(index) = value
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { pub count : usize , pub keys : * mut * mut dashcore :: hash_types :: ProTxHash , pub values : * mut * mut crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry }
    impl ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > > for std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry) -> std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > { let ffi_ref = & * ffi ; ferment :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | std :: ptr :: read (o) , | o | < crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry as ferment :: FFIConversionFrom < dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry >> :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry > > for std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < dashcore :: hash_types :: ProTxHash , dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry >) -> * const std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { let (count , keys , values) = ferment :: to_map (obj , | o | ferment :: boxed (o) , | o | < crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry as ferment :: FFIConversionTo < dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: QualifiedMasternodeListEntry >> :: ffi_to (o)) ; ferment :: boxed (Self { count , keys , values }) } }
    impl Drop for std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { fn drop (& mut self) { unsafe { ferment :: unbox_group (self . keys , self . count , | o | ferment :: unbox_any (o)) ; ferment :: unbox_group (self . values , self . count , | o | ferment :: unbox_any (o)) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_ctor (count : usize , keys : * mut * mut dashcore :: hash_types :: ProTxHash , values : * mut * mut crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry) -> * mut std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry{
        ferment :: boxed (std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry { count , keys , values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_destroy(
        ffi : * mut std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_value_by_key (ffi : * const std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry , key : * mut dashcore :: hash_types :: ProTxHash) -> * mut crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry{
        let ffi_ref = &*ffi;
        let key = key;
        for i in 0..ffi_ref.count {
            if key == *ffi_ref.keys.add(i) {
                return *ffi_ref.values.add(i);
            }
        }
        std::ptr::null_mut()
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry_set_value_for_key(
        ffi : * mut std_collections_Map_keys_dashcore_hash_types_ProTxHash_values_dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
        key: *mut dashcore::hash_types::ProTxHash,
        value : * mut crate :: fermented :: types :: dashcore :: sml :: masternode_list_entry :: qualified_masternode_list_entry :: dashcore_sml_masternode_list_entry_qualified_masternode_list_entry_QualifiedMasternodeListEntry,
    ) {
        let ffi_ref = &*ffi;
        let target_key = key;
        for i in 0..ffi_ref.count {
            let candidate_key = *ffi_ref.keys.add(i);
            if candidate_key.eq(&target_key) {
                let new_value = (*ffi).values.add(i);
                let old_value = *new_value;
                if (!(old_value).is_null()) {
                    ferment::unbox_any(old_value);
                }
                *new_value = value;
                break;
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { pub count : usize , pub keys : * mut * mut dashcore :: hash_types :: QuorumHash , pub values : * mut * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry }
    impl ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > for std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > { let ffi_ref = & * ffi ; ferment :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | std :: ptr :: read (o) , | o | < crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionFrom < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry >> :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > for std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry >) -> * const std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { let (count , keys , values) = ferment :: to_map (obj , | o | ferment :: boxed (o) , | o | < crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionTo < dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry >> :: ffi_to (o)) ; ferment :: boxed (Self { count , keys , values }) } }
    impl Drop for std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { fn drop (& mut self) { unsafe { ferment :: unbox_group (self . keys , self . count , | o | ferment :: unbox_any (o)) ; ferment :: unbox_group (self . values , self . count , | o | ferment :: unbox_any (o)) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_ctor (count : usize , keys : * mut * mut dashcore :: hash_types :: QuorumHash , values : * mut * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> * mut std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry{
        ferment :: boxed (std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { count , keys , values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_destroy(
        ffi : * mut std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_value_by_key (ffi : * const std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry , key : * mut dashcore :: hash_types :: QuorumHash) -> * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry{
        let ffi_ref = &*ffi;
        let key = key;
        for i in 0..ffi_ref.count {
            if key == *ffi_ref.keys.add(i) {
                return *ffi_ref.values.add(i);
            }
        }
        std::ptr::null_mut()
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_value_for_key(
        ffi : * mut std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
        key: *mut dashcore::hash_types::QuorumHash,
        value : * mut crate :: fermented :: types :: dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
    ) {
        let ffi_ref = &*ffi;
        let target_key = key;
        for i in 0..ffi_ref.count {
            let candidate_key = *ffi_ref.keys.add(i);
            if candidate_key.eq(&target_key) {
                let new_value = (*ffi).values.add(i);
                let old_value = *new_value;
                if (!(old_value).is_null()) {
                    ferment::unbox_any(old_value);
                }
                *new_value = value;
                break;
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry }
    impl ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > > for std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > { let ffi_ref = & * ffi ; ferment :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionFrom < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_from (o) , | o | < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionFrom < std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > >> :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > > > for std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < dashcore :: sml :: llmq_type :: LLMQType , std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > >) -> * const std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { let (count , keys , values) = ferment :: to_map (obj , | o | < crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType as ferment :: FFIConversionTo < dashcore :: sml :: llmq_type :: LLMQType >> :: ffi_to (o) , | o | < crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry as ferment :: FFIConversionTo < std :: collections :: BTreeMap < dashcore :: hash_types :: QuorumHash , dashcore :: sml :: quorum_entry :: qualified_quorum_entry :: QualifiedQuorumEntry > >> :: ffi_to (o)) ; ferment :: boxed (Self { count , keys , values }) } }
    impl Drop for std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { fn drop (& mut self) { unsafe { ferment :: unbox_group (self . keys , self . count , | o | ferment :: unbox_any (o)) ; ferment :: unbox_group (self . values , self . count , | o | ferment :: unbox_any (o)) ; ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_ctor (count : usize , keys : * mut * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType , values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry) -> * mut std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry{
        ferment :: boxed (std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry { count , keys , values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_destroy(
        ffi : * mut std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
    ) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_value_by_key (ffi : * const std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry , key : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry{
        let ffi_ref = &*ffi;
        let key = key;
        for i in 0..ffi_ref.count {
            if key == *ffi_ref.keys.add(i) {
                return *ffi_ref.values.add(i);
            }
        }
        std::ptr::null_mut()
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry_set_value_for_key(
        ffi : * mut std_collections_Map_keys_dashcore_sml_llmq_type_LLMQType_values_std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
        key : * mut crate :: fermented :: types :: dashcore :: sml :: llmq_type :: dashcore_sml_llmq_type_LLMQType,
        value : * mut crate :: fermented :: generics :: std_collections_Map_keys_dashcore_hash_types_QuorumHash_values_dashcore_sml_quorum_entry_qualified_quorum_entry_QualifiedQuorumEntry,
    ) {
        let ffi_ref = &*ffi;
        let target_key = key;
        for i in 0..ffi_ref.count {
            let candidate_key = *ffi_ref.keys.add(i);
            if candidate_key.eq(&target_key) {
                let new_value = (*ffi).values.add(i);
                let old_value = *new_value;
                if (!(old_value).is_null()) {
                    ferment::unbox_any(old_value);
                }
                *new_value = value;
                break;
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_48 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<[u8; 48]> for Arr_u8_48 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_48) -> [u8; 48] {
            let ffi_ref = &*ffi;
            TryFrom::<Vec<u8>>::try_from(ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o))
                .unwrap()
        }
    }
    impl ferment::FFIConversionTo<[u8; 48]> for Arr_u8_48 {
        unsafe fn ffi_to_const(obj: [u8; 48]) -> *const Arr_u8_48 {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_group(obj.into_iter(), |o| o),
            })
        }
    }
    impl Drop for Arr_u8_48 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_group(self.values, self.count, |o| ferment::black_hole(o));
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_48_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_48 {
        ferment::boxed(Arr_u8_48 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_48_destroy(ffi: *mut Arr_u8_48) {
        ferment::unbox_any(ffi);
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_48_value_at_index(ffi: *const Arr_u8_48, index: usize) -> u8 {
        *(*ffi).values.add(index)
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_48_set_value_at_index(
        ffi: *mut Arr_u8_48,
        index: usize,
        value: u8,
    ) {
        *(*ffi).values.add(index) = value
    }
}
