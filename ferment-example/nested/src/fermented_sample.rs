// use std::collections::BTreeMap;
// use std::sync::{Arc, Mutex};
// use crate::model::LLMQSnapshot;
//
// pub struct AllMutexExamples {
//     pub mutex_simple: Mutex<u32>,
//     pub mutex_complex: Mutex<LLMQSnapshot>,
//     pub mutex_generic: Mutex<Vec<u8>>,
//     pub mutex_opt_generic: Mutex<Option<BTreeMap<u32, LLMQSnapshot>>>,
//     pub opt_mutex_complex: Option<Mutex<Option<String>>>,
//     pub platform_case: Mutex<Option<Box<LLMQSnapshot>>>,
//
//     pub arc_mutex_simple: Arc<Mutex<u32>>,
//     pub arc_mutex_complex: Arc<Mutex<LLMQSnapshot>>,
//     pub arc_mutex_generic: Arc<Mutex<Vec<u8>>>,
//     pub arc_mutex_opt_generic: Arc<Mutex<Option<BTreeMap<u32, LLMQSnapshot>>>>,
//     pub arc_opt_mutex_complex: Option<Arc<Mutex<Option<String>>>>,
//     pub arc_platform_case: Arc<Mutex<Option<Box<LLMQSnapshot>>>>,
// }
// pub mod fermented {
//     pub mod types {
//         use crate as example_nested;
//         #[repr(C)]
//         #[derive(Clone)]
//         #[non_exhaustive]
//         pub enum example_nested_model_snapshot_LLMQSnapshotSkipMode {
//             NoSkipping = 0,
//             SkipFirst = 1,
//             SkipExcept = 2,
//             SkipAll = 3
//         }
//         impl ferment::FFIConversionFrom<example_nested::model::snapshot::LLMQSnapshotSkipMode> for example_nested_model_snapshot_LLMQSnapshotSkipMode {
//             unsafe fn ffi_from_const(ffi: *const example_nested_model_snapshot_LLMQSnapshotSkipMode) -> example_nested::model::snapshot::LLMQSnapshotSkipMode {
//                 let ffi_ref = &*ffi;
//                 match ffi_ref {
//                     example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping => example_nested::model::snapshot::LLMQSnapshotSkipMode::NoSkipping,
//                     example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst => example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipFirst,
//                     example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept => example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipExcept,
//                     example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll => example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipAll
//                 }
//             }
//         }
//         impl ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshotSkipMode> for example_nested_model_snapshot_LLMQSnapshotSkipMode {
//             unsafe fn ffi_to_const(obj: example_nested::model::snapshot::LLMQSnapshotSkipMode) -> *const example_nested_model_snapshot_LLMQSnapshotSkipMode {
//                 ferment::boxed(match obj {
//                     example_nested::model::snapshot::LLMQSnapshotSkipMode::NoSkipping => example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping,
//                     example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipFirst => example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst,
//                     example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipExcept => example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept,
//                     example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipAll => example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll,
//                 })
//             }
//         }
//         impl Drop for example_nested_model_snapshot_LLMQSnapshotSkipMode {
//             fn drop(&mut self) {
//                 unsafe {
//                     match self {
//                         example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping => {},
//                         example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst => {},
//                         example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept => {},
//                         example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll => {},
//                         _ => unreachable!("This is unreachable")
//                     };
//                 }
//             }
//         }
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct example_nested_model_snapshot_LLMQSnapshot {
//             pub member_list: *mut super::generics::Vec_u8,
//             pub skip_list: *mut super::generics::Vec_i32,
//             pub skip_list_mode: *mut example_nested_model_snapshot_LLMQSnapshotSkipMode ,
//             pub option_vec : * mut super::generics::Vec_u8
//         }
//         impl ferment::FFIConversionFrom<example_nested::model::snapshot::LLMQSnapshot> for example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const example_nested_model_snapshot_LLMQSnapshot) -> example_nested::model::snapshot::LLMQSnapshot {
//                 let ffi_ref = &*ffi;
//                 example_nested::model::snapshot::LLMQSnapshot {
//                     member_list: <super::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(ffi_ref.member_list),
//                     skip_list: <super::generics::Vec_i32 as ferment::FFIConversionFrom<Vec<i32>>>::ffi_from(ffi_ref.skip_list),
//                     skip_list_mode: <example_nested_model_snapshot_LLMQSnapshotSkipMode as ferment::FFIConversionFrom<example_nested::model::snapshot::LLMQSnapshotSkipMode>>::ffi_from(ffi_ref.skip_list_mode),
//                     option_vec: <super::generics::Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from_opt(ffi_ref.option_vec)
//                 }
//             }
//         }
//         impl ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshot> for example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const(obj: example_nested::model::snapshot::LLMQSnapshot) -> * const example_nested_model_snapshot_LLMQSnapshot {
//                 ferment::boxed(example_nested_model_snapshot_LLMQSnapshot {
//                     member_list: <super::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to(obj.member_list),
//                     skip_list: <super::generics::Vec_i32 as ferment::FFIConversionTo<Vec<i32>>>::ffi_to (obj.skip_list),
//                     skip_list_mode: <example_nested_model_snapshot_LLMQSnapshotSkipMode as ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshotSkipMode>>::ffi_to(obj.skip_list_mode),
//                     option_vec: <super::generics::Vec_u8 as ferment::FFIConversionTo<Vec<u8>>>::ffi_to_opt (obj.option_vec)
//                 })
//             }
//         }
//         impl Drop for example_nested_model_snapshot_LLMQSnapshot {
//             fn drop (& mut self) {
//                 unsafe {
//                     let ffi_ref = self;
//                     ferment::unbox_any(ffi_ref.member_list);
//                     ferment::unbox_any(ffi_ref.skip_list);
//                     ferment::unbox_any(ffi_ref.skip_list_mode);
//                     ferment::unbox_any_opt(ffi_ref.option_vec);
//                 }
//             }
//         }
//     }
//     pub mod generics {
//         use crate as example_nested;
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct Vec_u8 {
//             pub count: usize,
//             pub values: *mut u8
//         }
//         impl ferment::FFIConversionFrom<Vec<u8>> for Vec_u8 {
//             unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
//                 let ffi_ref = &*ffi;
//                 ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o)
//             }
//         }
//         impl ferment::FFIConversionTo<Vec<u8>> for Vec_u8 {
//             unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
//                 let count = obj.len();
//                 let values = ferment::to_group(obj.into_iter(), ferment::bypass);
//                 ferment::boxed(Self { count, values })
//             }
//         }
//         impl Drop for Vec_u8 {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_group(self.values, self.count, ferment::black_hole);
//                 };
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct Vec_i32 {
//             pub count: usize,
//             pub values: *mut i32
//         }
//         impl ferment::FFIConversionFrom<Vec<i32>> for Vec_i32 {
//             unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
//                 let ffi_ref = &*ffi;
//                 ferment::from_group(ffi_ref.count, ffi_ref.values, |o| *o)
//             }
//         }
//         impl ferment::FFIConversionTo<Vec<i32>> for Vec_i32 {
//             unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
//                 let count = obj.len();
//                 let values = ferment::to_group(obj.into_iter(), |o| o);
//                 ferment::boxed(Self { count, values })
//             }
//         }
//         impl Drop for Vec_i32 {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_group(self.values, self.count, ferment::black_hole);
//                 };
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             pub count: usize,
//             pub keys: *mut u32,
//             pub values: *mut *mut super::types::example_nested_model_snapshot_LLMQSnapshot
//         }
//         impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>> for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot> {
//                 let ffi_ref = &*ffi;
//                 ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| o, |o| < super::types::example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionFrom<example_nested::model::snapshot::LLMQSnapshot>>::ffi_from(o))
//             }
//         }
//         impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>> for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const(obj: std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>) -> *const std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//                 let (count, keys, values) = ferment::to_map(obj, |o| o, |o| <super::types::example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshot>>::ffi_to(o));
//                 ferment::boxed(Self { count, keys, values })
//             }
//         }
//         impl Drop for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_group(self.keys, self.count, ferment::black_hole);
//                     ferment::unbox_group(self.values, self.count, ferment::unbox_any);
//                 };
//             }
//         }
//
//
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_u32 {
//             pub inner: *mut std::sync::Mutex<u32>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Mutex<Option<String>>> for std_sync_Mutex_u32 {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_u32) -> std::sync::Mutex<u32> {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Mutex<u32>> for std_sync_Mutex_u32 {
//             unsafe fn ffi_to_const(obj: std::sync::Mutex<u32>) -> *const std_sync_Mutex_u32 {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_u32 {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_Option_String {
//             pub inner: *mut std::sync::Mutex<Option<String>>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Mutex<Option<String>>> for std_sync_Mutex_Option_String {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_Option_String) -> std::sync::Mutex<Option<String>> {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Mutex<Option<String>>> for std_sync_Mutex_Option_String {
//             unsafe fn ffi_to_const(obj: std::sync::Mutex<Option<String>>) -> *const std_sync_Mutex_Option_String {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_Option_String {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_example_nested_model_LLMQSnapshot {
//             pub inner: *mut std::sync::Mutex<example_nested::model::LLMQSnapshot>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Mutex<example_nested::model::LLMQSnapshot>> for std_sync_Mutex_example_nested_model_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_example_nested_model_LLMQSnapshot) -> std::sync::Mutex<example_nested::model::LLMQSnapshot> {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Mutex<example_nested::model::LLMQSnapshot>> for std_sync_Mutex_example_nested_model_LLMQSnapshot {
//             unsafe fn ffi_to_const(obj: std::sync::Mutex<example_nested::model::LLMQSnapshot>) -> *const std_sync_Mutex_example_nested_model_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_example_nested_model_LLMQSnapshot {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_Vec_u8 {
//             pub inner: *mut std::sync::Mutex<Vec<u8>>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_Vec_u8) -> std::sync::Mutex<Vec<u8>> {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
//             unsafe fn ffi_to_const(obj: std::sync::Mutex<Vec<u8>>) -> *const std_sync_Mutex_Vec_u8 {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_Vec_u8 {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             pub inner: *mut std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>
//         }
//         impl ferment::FFIConversionFrom<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>> for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>> {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>> for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const (obj : std::sync::Mutex < Option < std::collections::BTreeMap <u32, example_nested::model::snapshot::LLMQSnapshot>>>) -> * const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any_opt(self.inner);
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             pub inner: *mut std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>
//         }
//         impl ferment::FFIConversionFrom < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > > for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const (ffi : * const std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot) -> std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > {
//                 std::ptr::read((&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > > for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const (obj : std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > >) -> * const std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             fn drop (& mut self) {
//                 unsafe {
//                     ferment::unbox_any_opt(self.inner);
//                 }
//             }
//         }
//
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_u32 {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<u32>>,
//         }
//         impl ferment::FFIConversionFrom < std::sync::Arc < std::sync::Mutex < u32 > > > for std_sync_Arc_std_sync_Mutex_u32 {
//             unsafe fn ffi_from_const(ffi: * const std_sync_Arc_std_sync_Mutex_u32) -> std::sync::Arc<std::sync::Mutex<u32>> {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<u32>>> for std_sync_Arc_std_sync_Mutex_u32 {
//             unsafe fn ffi_to_const(obj: std::sync::Arc<std::sync::Mutex<u32>>) -> *const std_sync_Arc_std_sync_Mutex_u32 {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_u32 {
//             fn drop (& mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_Vec_u8 {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
//         }
//         impl ferment::FFIConversionFrom < std::sync::Arc < std::sync::Mutex < Vec < u8 > > > > for std_sync_Arc_std_sync_Mutex_Vec_u8 {
//             unsafe fn ffi_from_const (ffi : * const std_sync_Arc_std_sync_Mutex_Vec_u8) -> std::sync::Arc < std::sync::Mutex < Vec < u8 > > > {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo < std::sync::Arc < std::sync::Mutex < Vec < u8 > > > > for std_sync_Arc_std_sync_Mutex_Vec_u8 {
//             unsafe fn ffi_to_const (obj : std::sync::Arc < std::sync::Mutex < Vec < u8 > > >) -> * const std_sync_Arc_std_sync_Mutex_Vec_u8 {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_Vec_u8 {
//             fn drop (& mut self) {
//                 unsafe {
//                     ferment::unbox_any (self.inner);
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<Option<example_nested::model::LLMQSnapshot>>>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Option<example_nested::model::LLMQSnapshot>>>> for std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot) -> std::sync::Arc<std::sync::Mutex<Option<example_nested::model::LLMQSnapshot>>> {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<example_nested::model::LLMQSnapshot>>>> for std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot {
//             unsafe fn ffi_to_const(obj: std::sync::Arc<std::sync::Mutex<Option<example_nested::model::LLMQSnapshot>>>) -> *const std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot {
//             fn drop(&mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>,
//         }
//         impl ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>> for std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const(ffi: *const std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>> {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>> for std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const(obj: std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>) -> * const std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
//             fn drop(& mut self) {
//                 unsafe {
//                     ferment::unbox_any(self.inner);
//                 }
//             }
//         }
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<Option<Box<example_nested::model::LLMQSnapshot>>>>
//         }
//         impl ferment::FFIConversionFrom < std::sync::Arc < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > > > for std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_from_const (ffi : * const std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot) -> std::sync::Arc < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > > {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo < std::sync::Arc < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > > > for std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             unsafe fn ffi_to_const (obj : std::sync::Arc < std::sync::Mutex < Option < Box < example_nested::model::snapshot::LLMQSnapshot > > > >) -> * const std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
//             fn drop (& mut self) {
//                 unsafe {
//                     ferment::unbox_any (self.inner) ;
//                 }
//             }
//         }
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct std_sync_Arc_std_sync_Mutex_Option_String {
//             pub inner: *mut std::sync::Arc<std::sync::Mutex<Option<String>>>,
//         }
//         impl ferment::FFIConversionFrom < std::sync::Arc < std::sync::Mutex < Option < String > > > > for std_sync_Arc_std_sync_Mutex_Option_String {
//             unsafe fn ffi_from_const (ffi : * const std_sync_Arc_std_sync_Mutex_Option_String) -> std::sync::Arc < std::sync::Mutex < Option < String > > > {
//                 std::sync::Arc::clone(&*(&*ffi).inner)
//             }
//         }
//         impl ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<String>>>> for std_sync_Arc_std_sync_Mutex_Option_String {
//             unsafe fn ffi_to_const (obj : std::sync::Arc < std::sync::Mutex < Option < String > > >) -> * const std_sync_Arc_std_sync_Mutex_Option_String {
//                 ferment::boxed(Self { inner: ferment::boxed(obj) })
//             }
//         }
//         impl Drop for std_sync_Arc_std_sync_Mutex_Option_String {
//             fn drop (& mut self) {
//                 unsafe {
//                     ferment::unbox_any (self.inner) ;
//                 }
//             }
//         }
//
//
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_ctor(obj: *mut Vec_u8) -> *mut std_sync_Mutex_Vec_u8 {
//             ferment::boxed(std_sync_Mutex_Vec_u8 { inner: ferment::boxed(std::sync::Mutex::new(<Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(obj))) })
//         }
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_read(ffi: *mut std_sync_Mutex_Vec_u8) -> *mut Vec_u8 {
//             let lock = match (&*ffi).inner.lock() {
//                 Ok(g) => g,
//                 Err(poisoned) => poisoned.into_inner(),
//             };
//             <Vec_u8 as::ferment::FFIConversionTo<Vec<u8>>>::ffi_to(lock.clone())
//         }
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_write(ffi: *mut std_sync_Mutex_Vec_u8, value: *mut Vec_u8) {
//             let mut lock = match (&*ffi).inner.lock() {
//                 Ok(g) => g,
//                 Err(poisoned) => poisoned.into_inner(),
//             };
//             *lock = <Vec_u8 as::ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(value);
//         }
//
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_destroy (ffi: * mut std_sync_Mutex_Vec_u8) {
//             ferment::unbox_any(ffi);
//         }
//
//
//
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Arc_std_sync_Mutex_Vec_u8_ctor(obj: *mut Vec_u8) -> *mut std_sync_Arc_std_sync_Mutex_Vec_u8 {
//             ferment::boxed(std_sync_Arc_std_sync_Mutex_Vec_u8 { inner: ferment::boxed(std::sync::Arc::new(std::sync::Mutex::new(<Vec_u8 as ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(obj)))) })
//         }
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Arc_std_sync_Mutex_Vec_u8_read(ffi: *mut std_sync_Arc_std_sync_Mutex_Vec_u8) -> *mut Vec_u8 {
//             let lock = match (&*ffi).inner.lock() {
//                 Ok(g) => g,
//                 Err(poisoned) => poisoned.into_inner(),
//             };
//             <Vec_u8 as::ferment::FFIConversionTo<Vec<u8>>>::ffi_to(lock.clone())
//         }
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Arc_std_sync_Mutex_Vec_u8_write(ffi: *mut std_sync_Arc_std_sync_Mutex_Vec_u8, value: *mut Vec_u8) {
//             let mut lock = match (&*ffi).inner.lock() {
//                 Ok(g) => g,
//                 Err(poisoned) => poisoned.into_inner(),
//             };
//             *lock = <Vec_u8 as::ferment::FFIConversionFrom<Vec<u8>>>::ffi_from(value);
//         }
//         #[no_mangle]
//         pub unsafe extern "C" fn std_sync_Arc_std_sync_Mutex_Vec_u8_destroy(ffi: *mut std_sync_Arc_std_sync_Mutex_Vec_u8) {
//             ferment::unbox_any(ffi);
//         }
//
//
//
//
//
//         #[repr(C)]
//         #[derive(Clone)]
//         pub struct example_nested_gen_dict_AllMutexExamples {
//             pub mutex_simple: *mut std_sync_Mutex_u32,
//             pub mutex_complex: *mut std_sync_Mutex_example_nested_model_LLMQSnapshot,
//             pub mutex_generic: *mut std_sync_Mutex_Vec_u8,
//             pub mutex_opt_generic: *mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
//             pub opt_mutex_complex: *mut std_sync_Mutex_Option_String,
//             pub platform_case: *mut std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
//
//             pub arc_mutex_simple: *mut std_sync_Arc_std_sync_Mutex_u32,
//             pub arc_mutex_complex: *mut std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot,
//             pub arc_mutex_generic: *mut std_sync_Arc_std_sync_Mutex_Vec_u8,
//             pub arc_mutex_opt_generic: *mut std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
//             pub arc_opt_mutex_complex: *mut std_sync_Arc_std_sync_Mutex_Option_String,
//             pub arc_platform_case: *mut std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
//         }
//         impl ferment::FFIConversionFrom<example_nested::fermented_sample::AllMutexExamples> for example_nested_gen_dict_AllMutexExamples {
//             unsafe fn ffi_from_const(ffi: *const example_nested_gen_dict_AllMutexExamples) -> example_nested::fermented_sample::AllMutexExamples {
//                 let ffi_ref = &*ffi;
//                 example_nested::fermented_sample::AllMutexExamples {
//                     mutex_simple: <std_sync_Mutex_u32 as ferment::FFIConversionFrom<std::sync::Mutex<u32>>>::ffi_from_const(ffi_ref.mutex_simple),
//                     mutex_complex: <std_sync_Mutex_example_nested_model_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Mutex<example_nested::model::LLMQSnapshot>>>::ffi_from(ffi_ref.mutex_complex),
//                     mutex_generic: <std_sync_Mutex_Vec_u8 as ferment::FFIConversionFrom<std::sync::Mutex<Vec<u8>>>>::ffi_from(ffi_ref.mutex_generic),
//                     mutex_opt_generic: <std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>>::ffi_from(ffi_ref.mutex_opt_generic),
//                     opt_mutex_complex: <std_sync_Mutex_Option_String as ferment::FFIConversionFrom<std::sync::Mutex<Option<String>>>>::ffi_from_opt(ffi_ref.opt_mutex_complex),
//                     platform_case: <std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>>>::ffi_from(ffi_ref.platform_case),
//                     arc_mutex_simple: <std_sync_Arc_std_sync_Mutex_u32 as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<u32>>>>::ffi_from(ffi_ref.arc_mutex_simple),
//                     arc_mutex_complex: <std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<example_nested::model::LLMQSnapshot>>>>::ffi_from(ffi_ref.arc_mutex_complex),
//                     arc_mutex_generic: <std_sync_Arc_std_sync_Mutex_Vec_u8 as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Vec<u8>>>>>::ffi_from(ffi_ref.arc_mutex_generic),
//                     arc_mutex_opt_generic: <std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>>>::ffi_from(ffi_ref.arc_mutex_opt_generic),
//                     arc_opt_mutex_complex: <std_sync_Arc_std_sync_Mutex_Option_String as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Option<String>>>>>::ffi_from_opt(ffi_ref.arc_opt_mutex_complex),
//                     arc_platform_case: <std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionFrom<std::sync::Arc<std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>>>>::ffi_from(ffi_ref.arc_platform_case),
//                 }
//             }
//         }
//         impl ferment::FFIConversionTo<example_nested::fermented_sample::AllMutexExamples> for example_nested_gen_dict_AllMutexExamples {
//             unsafe fn ffi_to_const(obj: example_nested::fermented_sample::AllMutexExamples) -> *const example_nested_gen_dict_AllMutexExamples {
//                 ferment::boxed(example_nested_gen_dict_AllMutexExamples {
//                     mutex_simple: <std_sync_Mutex_u32 as ferment::FFIConversionTo<std::sync::Mutex<u32>>>::ffi_to(obj.mutex_simple),
//                     mutex_complex: <std_sync_Mutex_example_nested_model_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Mutex<example_nested::model::LLMQSnapshot>>>::ffi_to(obj.mutex_complex),
//                     mutex_generic: <std_sync_Mutex_Vec_u8 as ferment::FFIConversionTo<std::sync::Mutex<Vec<u8>>>>::ffi_to(obj.mutex_generic),
//                     mutex_opt_generic: <std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>>::ffi_to(obj.mutex_opt_generic),
//                     opt_mutex_complex: <std_sync_Mutex_Option_String as ferment::FFIConversionTo<std::sync::Mutex<Option<String>>>>::ffi_to_opt(obj.opt_mutex_complex),
//                     platform_case: <std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>>>::ffi_to(obj.platform_case),
//                     arc_mutex_simple: <std_sync_Arc_std_sync_Mutex_u32 as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<u32>>>>::ffi_to(obj.arc_mutex_simple),
//                     arc_mutex_complex: <std_sync_Arc_std_sync_Mutex_Option_example_nested_model_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<example_nested::model::LLMQSnapshot>>>>::ffi_to(obj.arc_mutex_complex),
//                     arc_mutex_generic: <std_sync_Arc_std_sync_Mutex_Vec_u8 as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Vec<u8>>>>>::ffi_to(obj.arc_mutex_generic),
//                     arc_mutex_opt_generic: <std_sync_Arc_std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>>>>>>::ffi_to(obj.arc_mutex_opt_generic),
//                     arc_opt_mutex_complex: <std_sync_Arc_std_sync_Mutex_Option_String as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<String>>>>>::ffi_to_opt(obj.arc_opt_mutex_complex),
//                     arc_platform_case: <std_sync_Arc_std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot as ferment::FFIConversionTo<std::sync::Arc<std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>>>>::ffi_to(obj.arc_platform_case),
//                 })
//             }
//         }
//         impl Drop for example_nested_gen_dict_AllMutexExamples {
//             fn drop(&mut self) {
//                 unsafe {
//                     let ffi_ref = self;
//                     ferment::unbox_any(ffi_ref.mutex_simple);
//                     ferment::unbox_any(ffi_ref.mutex_complex);
//                     ferment::unbox_any(ffi_ref.mutex_generic);
//                     ferment::unbox_any(ffi_ref.mutex_opt_generic);
//                     ferment::unbox_any_opt(ffi_ref.opt_mutex_complex);
//                     ferment::unbox_any(ffi_ref.platform_case);
//                     ferment::unbox_any(ffi_ref.mutex_simple);
//                     ferment::unbox_any(ffi_ref.mutex_complex);
//                     ferment::unbox_any(ffi_ref.mutex_generic);
//                     ferment::unbox_any(ffi_ref.mutex_opt_generic);
//                     ferment::unbox_any_opt(ffi_ref.opt_mutex_complex);
//                     ferment::unbox_any(ffi_ref.platform_case) ;
//                 }
//             }
//         }
//
//
//     }
// }
