# [allow (clippy :: let_and_return , clippy :: suspicious_else_formatting , clippy :: redundant_field_names , dead_code , non_camel_case_types , non_snake_case , non_upper_case_globals , redundant_semicolons , unreachable_patterns , unused_braces , unused_imports , unused_parens , unused_qualifications , unused_unsafe , unused_variables)] pub mod types { pub mod example_entry_point { use crate as example_entry_point ; pub mod entry { use crate as example_entry_point ; pub mod provider { use crate as example_entry_point ; } pub mod rnt { use crate as example_entry_point ; # [doc = "FFI-representation of the [`example_entry_point::entry::rnt::DashSharedCoreWithRuntime::with_pointers`]"] # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_rnt_DashSharedCoreWithRuntime_with_pointers (block_hash_by_height : example_entry_point :: entry :: BlockHashByHeight , model_by_height : example_entry_point :: entry :: ModelByHeight , runtime : * mut tokio :: runtime :: Runtime , context : * const std :: os :: raw :: c_void) -> * mut example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime { let obj = example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime :: with_pointers (block_hash_by_height , model_by_height , runtime , context) ; ferment :: boxed (obj) } # [doc = "FFI-representation of the [`example_entry_point::entry::rnt::DashSharedCoreWithRuntime::with_lambdas`]"] # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_rnt_DashSharedCoreWithRuntime_with_lambdas (block_hash_by_height : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Arr_u8_32 , model_by_height : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel , runtime : * mut tokio :: runtime :: Runtime , context : * const std :: os :: raw :: c_void) -> * mut example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime { let obj = example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime :: with_lambdas (move | o_0 | unsafe { block_hash_by_height . call (o_0) } , move | o_0 | unsafe { model_by_height . call (o_0) } , runtime , context) ; ferment :: boxed (obj) } } pub mod processor { use crate as example_entry_point ; } # [doc = "FFI-representation of the [`example_entry_point::entry::PlatformProvider::new`]"] # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_PlatformProvider_new (get_quorum_public_key : crate :: fermented :: generics :: Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String , get_data_contract : crate :: fermented :: generics :: Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String , context : * mut crate :: fermented :: generics :: std_sync_Arc_example_entry_point_entry_FFIContext) -> * mut example_entry_point :: entry :: PlatformProvider { let obj = example_entry_point :: entry :: PlatformProvider :: new (move | o_0 , o_1 , o_2 , o_3 | unsafe { get_quorum_public_key . call (o_0 , o_1 , o_2 , o_3) } , move | o_0 , o_1 | unsafe { get_data_contract . call (o_0 , o_1) } , < crate :: fermented :: generics :: std_sync_Arc_example_entry_point_entry_FFIContext as ferment :: FFIConversionFrom < std :: sync :: Arc < example_entry_point :: entry :: FFIContext > >> :: ffi_from (context)) ; ferment :: boxed (obj) } pub mod core { use crate as example_entry_point ; # [doc = "FFI-representation of the [`example_entry_point::entry::core::DashSharedCore::with_pointers`]"] # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_core_DashSharedCore_with_pointers (block_hash_by_height : example_entry_point :: entry :: BlockHashByHeight , model_by_height : example_entry_point :: entry :: ModelByHeight , context : * const std :: os :: raw :: c_void) -> * mut example_entry_point :: entry :: core :: DashSharedCore { let obj = example_entry_point :: entry :: core :: DashSharedCore :: with_pointers (block_hash_by_height , model_by_height , context) ; ferment :: boxed (obj) } # [doc = "FFI-representation of the [`example_entry_point::entry::core::DashSharedCore::with_lambdas`]"] # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_core_DashSharedCore_with_lambdas (block_hash_by_height : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Arr_u8_32 , model_by_height : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel , context : * const std :: os :: raw :: c_void) -> * mut example_entry_point :: entry :: core :: DashSharedCore { let obj = example_entry_point :: entry :: core :: DashSharedCore :: with_lambdas (move | o_0 | unsafe { block_hash_by_height . call (o_0) } , move | o_0 | unsafe { model_by_height . call (o_0) } , context) ; ferment :: boxed (obj) } } # [doc = "FFI-representation of the [`SomeModel`]"] # [repr (C)] # [derive (Clone)] pub struct example_entry_point_entry_SomeModel { pub hash : * mut crate :: fermented :: generics :: Arr_u8_32 , pub desc : * mut std :: os :: raw :: c_char } impl ferment :: FFIConversionFrom < example_entry_point :: entry :: SomeModel > for example_entry_point_entry_SomeModel { unsafe fn ffi_from_const (ffi : * const example_entry_point_entry_SomeModel) -> example_entry_point :: entry :: SomeModel { let ffi_ref = & * ffi ; example_entry_point :: entry :: SomeModel { hash : < crate :: fermented :: generics :: Arr_u8_32 as ferment :: FFIConversionFrom < [u8 ; 32] >> :: ffi_from (ffi_ref . hash) , desc : < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (ffi_ref . desc) } } } impl ferment :: FFIConversionTo < example_entry_point :: entry :: SomeModel > for example_entry_point_entry_SomeModel { unsafe fn ffi_to_const (obj : example_entry_point :: entry :: SomeModel) -> * const example_entry_point_entry_SomeModel { ferment :: boxed (example_entry_point_entry_SomeModel { hash : < crate :: fermented :: generics :: Arr_u8_32 as ferment :: FFIConversionTo < [u8 ; 32] >> :: ffi_to (obj . hash) , desc : < std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (obj . desc) }) } } impl Drop for example_entry_point_entry_SomeModel { fn drop (& mut self) { unsafe { let ffi_ref = self ; ferment :: unbox_any (ffi_ref . hash) ; ferment :: unbox_string (ffi_ref . desc) ; } } } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_ctor < > (hash : * mut crate :: fermented :: generics :: Arr_u8_32 , desc : * mut std :: os :: raw :: c_char) -> * mut example_entry_point_entry_SomeModel { ferment :: boxed (example_entry_point_entry_SomeModel { hash , desc }) } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_destroy < > (ffi : * mut example_entry_point_entry_SomeModel) { ferment :: unbox_any (ffi) ; } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_get_hash < > (obj : * const example_entry_point_entry_SomeModel) -> * mut crate :: fermented :: generics :: Arr_u8_32 { (* obj) . hash } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_get_desc < > (obj : * const example_entry_point_entry_SomeModel) -> * mut std :: os :: raw :: c_char { (* obj) . desc } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_set_hash < > (obj : * const example_entry_point_entry_SomeModel) -> * mut crate :: fermented :: generics :: Arr_u8_32 { (* obj) . hash } # [no_mangle] pub unsafe extern "C" fn example_entry_point_entry_SomeModel_set_desc < > (obj : * const example_entry_point_entry_SomeModel) -> * mut std :: os :: raw :: c_char { (* obj) . desc } } } } # [allow (clippy :: let_and_return , clippy :: suspicious_else_formatting , clippy :: redundant_field_names , dead_code , non_camel_case_types , non_snake_case , non_upper_case_globals , redundant_semicolons , unreachable_patterns , unused_braces , unused_imports , unused_parens , unused_qualifications , unused_unsafe , unused_variables)] pub mod generics { use crate as example_entry_point ; # [repr (C)] # [derive (Clone)] pub struct std_sync_Arc_example_entry_point_entry_SomeModel { pub obj : * mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel } impl ferment :: FFIConversionFrom < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > for std_sync_Arc_example_entry_point_entry_SomeModel { unsafe fn ffi_from_const (ffi : * const std_sync_Arc_example_entry_point_entry_SomeModel) -> std :: sync :: Arc < example_entry_point :: entry :: SomeModel > { let ffi_ref = & * ffi ; std :: sync :: Arc :: new (ferment :: FFIConversionFrom :: ffi_from (ffi_ref . obj)) } } impl ferment :: FFIConversionTo < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > for std_sync_Arc_example_entry_point_entry_SomeModel { unsafe fn ffi_to_const (obj : std :: sync :: Arc < example_entry_point :: entry :: SomeModel >) -> * const std_sync_Arc_example_entry_point_entry_SomeModel { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to ((* obj) . clone ()) }) } } impl Drop for std_sync_Arc_example_entry_point_entry_SomeModel { fn drop (& mut self) { unsafe { ferment :: unbox_any (self . obj) ; } } } # [no_mangle] pub unsafe extern "C" fn std_sync_Arc_example_entry_point_entry_SomeModel_ctor (obj : * mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel) -> * mut std_sync_Arc_example_entry_point_entry_SomeModel { ferment :: boxed (std_sync_Arc_example_entry_point_entry_SomeModel { obj }) } # [no_mangle] pub unsafe extern "C" fn std_sync_Arc_example_entry_point_entry_SomeModel_destroy (ffi : * mut std_sync_Arc_example_entry_point_entry_SomeModel) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct std_sync_Arc_example_entry_point_entry_FFIContext { pub obj : * mut example_entry_point :: entry :: FFIContext } impl ferment :: FFIConversionFrom < std :: sync :: Arc < example_entry_point :: entry :: FFIContext > > for std_sync_Arc_example_entry_point_entry_FFIContext { unsafe fn ffi_from_const (ffi : * const std_sync_Arc_example_entry_point_entry_FFIContext) -> std :: sync :: Arc < example_entry_point :: entry :: FFIContext > { let ffi_ref = & * ffi ; std :: sync :: Arc :: from_raw (ffi_ref . obj) } } impl ferment :: FFIConversionTo < std :: sync :: Arc < example_entry_point :: entry :: FFIContext > > for std_sync_Arc_example_entry_point_entry_FFIContext { unsafe fn ffi_to_const (obj : std :: sync :: Arc < example_entry_point :: entry :: FFIContext >) -> * const std_sync_Arc_example_entry_point_entry_FFIContext { ferment :: boxed (Self { obj : std :: sync :: Arc :: into_raw (obj) . cast_mut () }) } } impl Drop for std_sync_Arc_example_entry_point_entry_FFIContext { fn drop (& mut self) { unsafe { ferment :: unbox_any (self . obj) ; } } } # [no_mangle] pub unsafe extern "C" fn std_sync_Arc_example_entry_point_entry_FFIContext_ctor (obj : * mut example_entry_point :: entry :: FFIContext) -> * mut std_sync_Arc_example_entry_point_entry_FFIContext { ferment :: boxed (std_sync_Arc_example_entry_point_entry_FFIContext { obj }) } # [no_mangle] pub unsafe extern "C" fn std_sync_Arc_example_entry_point_entry_FFIContext_destroy (ffi : * mut std_sync_Arc_example_entry_point_entry_FFIContext) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Arr_u8_48 { pub count : usize , pub values : * mut u8 } impl ferment :: FFIConversionFrom < [u8 ; 48] > for Arr_u8_48 { unsafe fn ffi_from_const (ffi : * const Arr_u8_48) -> [u8 ; 48] { ferment :: FFIVecConversion :: decode (& * ffi) . try_into () . unwrap () } } impl ferment :: FFIConversionTo < [u8 ; 48] > for Arr_u8_48 { unsafe fn ffi_to_const (obj : [u8 ; 48]) -> * const Arr_u8_48 { ferment :: FFIVecConversion :: encode (obj . to_vec ()) } } impl ferment :: FFIVecConversion for Arr_u8_48 { type Value = Vec < u8 > ; unsafe fn decode (& self) -> Self :: Value { ferment :: from_primitive_group (self . values , self . count) } unsafe fn encode (obj : Self :: Value) -> * mut Self { ferment :: boxed (Self { count : obj . len () , values : ferment :: to_primitive_group (obj . into_iter ()) }) } } impl Drop for Arr_u8_48 { fn drop (& mut self) { unsafe { ferment :: unbox_vec_ptr (self . values , self . count) ; } } } # [no_mangle] pub unsafe extern "C" fn Arr_u8_48_ctor (count : usize , values : * mut u8) -> * mut Arr_u8_48 { ferment :: boxed (Arr_u8_48 { count , values }) } # [no_mangle] pub unsafe extern "C" fn Arr_u8_48_destroy (ffi : * mut Arr_u8_48) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Arr_u8_32 { pub count : usize , pub values : * mut u8 } impl ferment :: FFIConversionFrom < [u8 ; 32] > for Arr_u8_32 { unsafe fn ffi_from_const (ffi : * const Arr_u8_32) -> [u8 ; 32] { ferment :: FFIVecConversion :: decode (& * ffi) . try_into () . unwrap () } } impl ferment :: FFIConversionTo < [u8 ; 32] > for Arr_u8_32 { unsafe fn ffi_to_const (obj : [u8 ; 32]) -> * const Arr_u8_32 { ferment :: FFIVecConversion :: encode (obj . to_vec ()) } } impl ferment :: FFIVecConversion for Arr_u8_32 { type Value = Vec < u8 > ; unsafe fn decode (& self) -> Self :: Value { ferment :: from_primitive_group (self . values , self . count) } unsafe fn encode (obj : Self :: Value) -> * mut Self { ferment :: boxed (Self { count : obj . len () , values : ferment :: to_primitive_group (obj . into_iter ()) }) } } impl Drop for Arr_u8_32 { fn drop (& mut self) { unsafe { ferment :: unbox_vec_ptr (self . values , self . count) ; } } } # [no_mangle] pub unsafe extern "C" fn Arr_u8_32_ctor (count : usize , values : * mut u8) -> * mut Arr_u8_32 { ferment :: boxed (Arr_u8_32 { count , values }) } # [no_mangle] pub unsafe extern "C" fn Arr_u8_32_destroy (ffi : * mut Arr_u8_32) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct std_collections_Map_keys_String_values_String { pub count : usize , pub keys : * mut * mut std :: os :: raw :: c_char , pub values : * mut * mut std :: os :: raw :: c_char } impl ferment :: FFIConversionFrom < std :: collections :: BTreeMap < String , String > > for std_collections_Map_keys_String_values_String { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_String_values_String) -> std :: collections :: BTreeMap < String , String > { let ffi_ref = & * ffi ; ferment :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (o) , | o | < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (o)) } } impl ferment :: FFIConversionTo < std :: collections :: BTreeMap < String , String > > for std_collections_Map_keys_String_values_String { unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < String , String >) -> * const std_collections_Map_keys_String_values_String { ferment :: boxed (Self { count : obj . len () , keys : ferment :: to_complex_group (obj . keys () . cloned ()) , values : ferment :: to_complex_group (obj . values () . cloned ()) }) } } impl Drop for std_collections_Map_keys_String_values_String { fn drop (& mut self) { unsafe { ferment :: unbox_any_vec_ptr_composer (self . keys , self . count , ferment :: unbox_string) ; ferment :: unbox_any_vec_ptr_composer (self . values , self . count , ferment :: unbox_string) ; } } } # [no_mangle] pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_ctor (count : usize , keys : * mut * mut std :: os :: raw :: c_char , values : * mut * mut std :: os :: raw :: c_char) -> * mut std_collections_Map_keys_String_values_String { ferment :: boxed (std_collections_Map_keys_String_values_String { count , keys , values }) } # [no_mangle] pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_destroy (ffi : * mut std_collections_Map_keys_String_values_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { caller : unsafe extern "C" fn (* const example_entry_point :: entry :: FFIContext , * mut std :: os :: raw :: c_char) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String) } impl Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { pub unsafe fn call (& self , o_0 : * const example_entry_point :: entry :: FFIContext , o_1 : String) -> Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String > { let ffi_result = (self . caller) (o_0 , < std :: os :: raw :: c_char as ferment :: FFIConversionTo < String >> :: ffi_to (o_1)) ; let result = < crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String as ferment :: FFIConversionFrom < Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String > >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } } unsafe impl Send for Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { } unsafe impl Sync for Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String_ctor (caller : unsafe extern "C" fn (* const example_entry_point :: entry :: FFIContext , * mut std :: os :: raw :: c_char) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String)) -> * mut Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { ferment :: boxed (Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String_destroy (ffi : * mut Fn_ARGS_example_entry_point_entry_FFIContext_String_RTRN_Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { caller : unsafe extern "C" fn (* const example_entry_point :: entry :: FFIContext , u32 , * mut crate :: fermented :: generics :: Arr_u8_32 , u32) -> * mut crate :: fermented :: generics :: Result_ok_u8_arr_48_err_String , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u8_arr_48_err_String) } impl Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { pub unsafe fn call (& self , o_0 : * const example_entry_point :: entry :: FFIContext , o_1 : u32 , o_2 : [u8 ; 32] , o_3 : u32) -> Result < [u8 ; 48] , String > { let ffi_result = (self . caller) (o_0 , o_1 , < crate :: fermented :: generics :: Arr_u8_32 as ferment :: FFIConversionTo < [u8 ; 32] >> :: ffi_to (o_2) , o_3) ; let result = < crate :: fermented :: generics :: Result_ok_u8_arr_48_err_String as ferment :: FFIConversionFrom < Result < [u8 ; 48] , String > >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } } unsafe impl Send for Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { } unsafe impl Sync for Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String_ctor (caller : unsafe extern "C" fn (* const example_entry_point :: entry :: FFIContext , u32 , * mut crate :: fermented :: generics :: Arr_u8_32 , u32) -> * mut crate :: fermented :: generics :: Result_ok_u8_arr_48_err_String , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u8_arr_48_err_String)) -> * mut Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { ferment :: boxed (Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String_destroy (ffi : * mut Fn_ARGS_example_entry_point_entry_FFIContext_u32_Arr_u8_32_u32_RTRN_Result_ok_u8_arr_48_err_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_Arr_u8_32_RTRN_u32 { caller : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32) -> u32 , destructor : unsafe extern "C" fn (u32) } impl Fn_ARGS_Arr_u8_32_RTRN_u32 { pub unsafe fn call (& self , o_0 : [u8 ; 32]) -> u32 { let ffi_result = (self . caller) (< crate :: fermented :: generics :: Arr_u8_32 as ferment :: FFIConversionTo < [u8 ; 32] >> :: ffi_to (o_0)) ; ffi_result } } unsafe impl Send for Fn_ARGS_Arr_u8_32_RTRN_u32 { } unsafe impl Sync for Fn_ARGS_Arr_u8_32_RTRN_u32 { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_RTRN_u32_ctor (caller : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32) -> u32 , destructor : unsafe extern "C" fn (u32)) -> * mut Fn_ARGS_Arr_u8_32_RTRN_u32 { ferment :: boxed (Fn_ARGS_Arr_u8_32_RTRN_u32 { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_RTRN_u32_destroy (ffi : * mut Fn_ARGS_Arr_u8_32_RTRN_u32) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { pub ok : * mut crate :: fermented :: generics :: std_sync_Arc_example_entry_point_entry_SomeModel , pub error : * mut std :: os :: raw :: c_char } impl ferment :: FFIConversionFrom < Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String > > for Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String) -> Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | < crate :: fermented :: generics :: std_sync_Arc_example_entry_point_entry_SomeModel as ferment :: FFIConversionFrom < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > >> :: ffi_from_opt (o) , ffi_ref . error , | o | < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (o)) } } impl ferment :: FFIConversionTo < Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String > > for Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { unsafe fn ffi_to_const (obj : Result < Option < std :: sync :: Arc < example_entry_point :: entry :: SomeModel > > , String >) -> * const Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to_opt (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } } impl Drop for Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } } # [no_mangle] pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String_ctor (ok : * mut crate :: fermented :: generics :: std_sync_Arc_example_entry_point_entry_SomeModel , error : * mut std :: os :: raw :: c_char) -> * mut Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { ferment :: boxed (Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String { ok , error }) } # [no_mangle] pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String_destroy (ffi : * mut Result_ok_Option_std_sync_Arc_example_entry_point_entry_SomeModel_err_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_u32_RTRN_Arr_u8_32 { caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: generics :: Arr_u8_32 , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32) } impl Fn_ARGS_u32_RTRN_Arr_u8_32 { pub unsafe fn call (& self , o_0 : u32) -> [u8 ; 32] { let ffi_result = (self . caller) (o_0) ; let result = < crate :: fermented :: generics :: Arr_u8_32 as ferment :: FFIConversionFrom < [u8 ; 32] >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } } unsafe impl Send for Fn_ARGS_u32_RTRN_Arr_u8_32 { } unsafe impl Sync for Fn_ARGS_u32_RTRN_Arr_u8_32 { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Arr_u8_32_ctor (caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: generics :: Arr_u8_32 , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32)) -> * mut Fn_ARGS_u32_RTRN_Arr_u8_32 { ferment :: boxed (Fn_ARGS_u32_RTRN_Arr_u8_32 { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Arr_u8_32_destroy (ffi : * mut Fn_ARGS_u32_RTRN_Arr_u8_32) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_u32_RTRN_Option_String { caller : unsafe extern "C" fn (u32) -> * mut std :: os :: raw :: c_char , destructor : unsafe extern "C" fn (* mut std :: os :: raw :: c_char) } impl Fn_ARGS_u32_RTRN_Option_String { pub unsafe fn call (& self , o_0 : u32) -> Option < String > { let ffi_result = (self . caller) (o_0) ; if ffi_result . is_null () { None } else { let result = < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from_opt (ffi_result) ; (self . destructor) (ffi_result) ; result } } } unsafe impl Send for Fn_ARGS_u32_RTRN_Option_String { } unsafe impl Sync for Fn_ARGS_u32_RTRN_Option_String { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Option_String_ctor (caller : unsafe extern "C" fn (u32) -> * mut std :: os :: raw :: c_char , destructor : unsafe extern "C" fn (* mut std :: os :: raw :: c_char)) -> * mut Fn_ARGS_u32_RTRN_Option_String { ferment :: boxed (Fn_ARGS_u32_RTRN_Option_String { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Option_String_destroy (ffi : * mut Fn_ARGS_u32_RTRN_Option_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Result_ok_u8_arr_48_err_String { pub ok : * mut crate :: fermented :: generics :: Arr_u8_48 , pub error : * mut std :: os :: raw :: c_char } impl ferment :: FFIConversionFrom < Result < [u8 ; 48] , String > > for Result_ok_u8_arr_48_err_String { unsafe fn ffi_from_const (ffi : * const Result_ok_u8_arr_48_err_String) -> Result < [u8 ; 48] , String > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | < crate :: fermented :: generics :: Arr_u8_48 as ferment :: FFIConversionFrom < [u8 ; 48] >> :: ffi_from (o) , ffi_ref . error , | o | < std :: os :: raw :: c_char as ferment :: FFIConversionFrom < String >> :: ffi_from (o)) } } impl ferment :: FFIConversionTo < Result < [u8 ; 48] , String > > for Result_ok_u8_arr_48_err_String { unsafe fn ffi_to_const (obj : Result < [u8 ; 48] , String >) -> * const Result_ok_u8_arr_48_err_String { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } } impl Drop for Result_ok_u8_arr_48_err_String { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } } # [no_mangle] pub unsafe extern "C" fn Result_ok_u8_arr_48_err_String_ctor (ok : * mut crate :: fermented :: generics :: Arr_u8_48 , error : * mut std :: os :: raw :: c_char) -> * mut Result_ok_u8_arr_48_err_String { ferment :: boxed (Result_ok_u8_arr_48_err_String { ok , error }) } # [no_mangle] pub unsafe extern "C" fn Result_ok_u8_arr_48_err_String_destroy (ffi : * mut Result_ok_u8_arr_48_err_String) { ferment :: unbox_any (ffi) ; } # [repr (C)] # [derive (Clone)] pub struct Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel , destructor : unsafe extern "C" fn (* mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel) } impl Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { pub unsafe fn call (& self , o_0 : u32) -> example_entry_point :: entry :: SomeModel { let ffi_result = (self . caller) (o_0) ; let result = < crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel as ferment :: FFIConversionFrom < example_entry_point :: entry :: SomeModel >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } } unsafe impl Send for Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { } unsafe impl Sync for Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel_ctor (caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel , destructor : unsafe extern "C" fn (* mut crate :: fermented :: types :: example_entry_point :: entry :: example_entry_point_entry_SomeModel)) -> * mut Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { ferment :: boxed (Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel { caller , destructor }) } # [no_mangle] pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel_destroy (ffi : * mut Fn_ARGS_u32_RTRN_example_entry_point_entry_SomeModel) { ferment :: unbox_any (ffi) ; } }