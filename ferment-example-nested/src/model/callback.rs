

// Primitive Return (no destructor)
// #[ferment_macro::export]
// pub type GetBlockHeight = fn(block_hash: [u8; 32]) -> u32;
//
//
// Complex Return (has destructor)
// #[ferment_macro::export]
// pub type GetQuorumHash = fn(block_hash: [u8; 32]) -> String;
//
//
// Generic Return (Optional) (has destructor)
// #[ferment_macro::export]
// pub type GetQuorumHashOpt = fn(block_hash: [u8; 32]) -> Option<String>;
// #[ferment_macro::export]
// pub type GetQuorumHashOptFn<F: Fn([u8; 32]) -> Option<String>> = F;

// #[ferment_macro::export]
// pub type CallbackExample1 = fn(chain_id: u32, block_hash: [u8; 32]) -> Option<String>;
// #[ferment_macro::export]
// pub type CallbackExample11 = dyn Fn(u32, [u8; 32]) -> Option<String>;
// #[ferment_macro::export]
// pub type CallbackExample1<T: Fn(u32, [u8; 32]) -> Option<String>> = T;
//
///////////
// #[ferment_macro::export]
// pub type CallbackExample2 = fn(block_hash: [u8; 32]);
//
// ///////////
// #[ferment_macro::export]
// pub type CallbackExample3 = fn(block_hash: [u8; 32]) -> Option<u32>;
// ///////////
// #[ferment_macro::export]
// pub type CallbackExample4 = fn(chain_id: u32, chain_name: String) -> Option<String>;

// use std::ffi::c_void;
use ferment_example::errors::protocol_error::ProtocolError;

///////////
// #[ferment_macro::export]
// pub fn add_block_height_callback(index: u32, _callback: GetBlockHeight) {
//     println!("add_block_height_callback: {}", index);
// }
// ///////////
//
// #[ferment_macro::export]
// pub fn add_quorum_hash_callback(index: u32, _callback: GetQuorumHash) {
//     println!("add_quorum_hash_callback: {}", index);
// }
//////////
// #[ferment_macro::export]
// pub fn add_quorum_hash_opt_callback(index: u32, _callback: GetQuorumHashOpt) {
//     println!("add_quorum_hash_opt_callback: {}", index);
// }
// /////////
// #[ferment_macro::export]
// pub fn add_quorum_hash_opt_callback_with_primitive_result(index: u32, _callback: GetQuorumHashOpt) -> u32 {
//     println!("add_quorum_hash_opt_callback_with_primitive_result: {}", index);
//     0u32
// }
// /////////
// #[ferment_macro::export]
// pub fn add_quorum_hash_opt_callback_with_complex_result(index: u32, _callback: GetQuorumHashOpt) -> String {
//     println!("add_quorum_hash_opt_callback_with_complex_result: {}", index);
//     "".to_string()
// }
// #[ferment_macro::export]
// pub fn fn_add_quorum_hash_opt_callback_with_complex_result<F: Fn([u8; 32]) -> Option<String>>(index: u32, _callback: F) -> String {
//     println!("add_quorum_hash_opt_callback_with_complex_result: {}", index);
//     _callback([0u8; 32]);
//     "".to_string()
// }
////////
// #[ferment_macro::export]
// pub fn find_current_block_desc(_callback: CallbackExample1) {
//     println!("find_current_block_desc");
// }
#[ferment_macro::export]
pub fn find_current_block_desc<T: Fn(u32, [u8; 32]) -> Option<String>>(_callback: T) {
    println!("find_current_block_desc: ");
}
#[ferment_macro::opaque]
pub type ClassicCallback = unsafe extern "C" fn(u32);
#[ferment_macro::export]
pub fn find_current_block_classic(_callback: ClassicCallback) {
    println!("find_current_block_desc: ");
}
#[ferment_macro::export]
pub fn find_current_block_desc_mut<T: FnMut(u32, [u8; 32]) -> Option<String>>(_callback: T) {
    println!("find_current_block_desc_mut: ");
}

#[ferment_macro::export]
pub fn lookup_block_hash_by_height<T: Fn(u32) -> Option<[u8; 32]>>(_callback: T) {
    println!("lookup_block_hash_by_height:");
}
#[ferment_macro::export]
pub fn lookup_merkle_root_by_hash<T: Fn([u8; 32]) -> Option<[u8; 32]>>(_callback: T) {
    println!("lookup_merkle_root_by_hash:");
}
// pub type ShouldProcessDiffWithRange = unsafe extern "C" fn(
//     base_block_hash: *mut [u8; 32],
//     block_hash: *mut [u8; 32],
//     context: *const c_void,
// ) -> ProcessingError;

// #[ferment_macro::export]
// pub fn should_process_diff_in_range<T: Fn([u8; 32], [u8; 32]) -> Result<bool, ProtocolError>>(_callback: T) {
//     println!("should_process_diff_in_range:");
// }
#[ferment_macro::export]
pub fn should_process_diff_in_range2<T: Fn([u8; 32], [u8; 32]) -> Result<u32, ProtocolError>>(_callback: T) {
    println!("should_process_diff_in_range:");
}

#[ferment_macro::export]
pub fn setup_two_callbacks<
    T: Fn([u8; 32], [u8; 32]) -> Result<u32, ProtocolError>,
    U: Fn(u32) -> Result<u32, ProtocolError>>(_callback1: T, _callback2: U) {
    println!("should_process_diff_in_range:");
}

// #[ferment_macro::export]
// pub fn find_current_block_desc_2<T>(_callback: CallbackExample1<T>) {
//     println!("find_current_block_desc");
// }

///// Sample for callbacks
// pub unsafe extern "C" fn ferment_example_nested_model_callback_find_current_block_desc(_callback: *mut Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String) {
//     let obj = find_current_block_desc(|o_0, o_1| unsafe { (&*_callback).call(o_0, o_1) });
//     obj
// }
// #[repr(C)]
// #[derive(Clone)]
// pub struct Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
//     pub context: *const std::os::raw::c_void,
//     caller: fn(u32, *mut crate::fermented::generics::Arr_u8_32) -> *mut std::os::raw::c_char,
//     destructor: fn(result: *mut std::os::raw::c_char),
// }
//
// impl Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
//     pub unsafe fn call(&self, o_0: u32, o_1: [u8;32]) -> Option<String> {
//         let ffi_result = (self.caller)(o_0, ferment_interfaces::FFIConversionTo::ffi_to(o_1));
//         if ffi_result.is_null() {
//             let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from_opt(ffi_result);
//             (self.destructor)(ffi_result);
//             result
//         } else {
//             None
//         }
//     }
// }


// fn find_current_block_desc2(callback: Box<dyn Fn((u32, [u8; 32])) -> Option<String>>) {
//     let _result = callback((1, [0u8; 32]));
// }
// fn find_current_block_desc3<F: Fn((u32, [u8; 32])) -> Option<String>>(callback: F) {
//     let _result = callback((1, [0u8; 32]));
// }

// #[ferment_macro::export]
// pub fn find_current_block_desc4(_callback: CallbackExample4) {
//     println!("find_current_block_desc4");
// }
// ////////
// #[ferment_macro::export]
// pub fn just_simple_func_to_compare(index: u32, string: String) -> String {
//     println!("just_simple_func_to_compare: {}: {}", index, string);
//     string
// }
//

////////
// #[repr(C)]
// pub struct GetBlockHeight_FFI {
//     pub caller: unsafe extern "C" fn(self_: *const std::os::raw::c_void, block_hash: *mut [u8; 32]) -> u32,
//     // pub caller_orig: fn(block_hash:)
//     /// This is the context which external parties could link to the class
//     /// in order to obtain and bridge external class context (see details in DashSync's DSMasternodeManager + Mndiff)
//     pub context: *const std::os::raw::c_void
// }
//
// impl GetBlockHeight_FFI {
//     pub fn lookup(&self, block_hash: [u8; 32]) -> u32 {
//         callbacks::lookup_merkle_root_by_hash(
//             block_hash,
//             |h: UInt256| unsafe { (self.get_merkle_root_by_hash)(boxed(h.0), self.opaque_context) },
//             |hash: *mut u8| unsafe { (self.destroy_hash)(hash) },
//         )
//     }
//
// }
//
// impl ferment_interfaces::FFICallbackPtr<([u8; 32],), u32> for GetBlockHeight_FFI {
//     unsafe fn get(&self) -> (fn(*const std::os::raw::c_void, ([u8; 32],)) -> u32, *const std::os::raw::c_void) {
//         fn wrapper(self_: *const std::os::raw::c_void, (block_hash,): ([u8; 32],)) -> u32 {
//             unsafe {
//                 let instance = self_ as *const GetBlockHeight_FFI;
//                 let instance = &*instance;
//                 (instance.caller)(self_, ferment_interfaces::boxed(block_hash))
//             }
//         }
//         (wrapper, self as *const GetBlockHeight_FFI as *const std::os::raw::c_void)
//     }
// }
// pub unsafe fn ffi_add_block_height_callback(index: u32, _callback: GetBlockHeight_FFI) {
//     let (callback, context) = ferment_interfaces::FFICallbackPtr::get(&_callback);
//     let result = add_block_height_callback(index, |block_hash| callback(context, (block_hash,)));
//     result
// }
//

// impl GetBlockHeight_FFI {
//     pub fn lookup<C>(&self, block_hash: [u8; 32], caller: C) -> u32 where C: Fn(*mut [u8; 32]) -> u32 {
//         let ffi_result = caller(ferment_interfaces::boxed(block_hash));
//         ffi_result
//     }
//
// }
// impl ferment_interfaces::FFICallback<([u8; 32],), u32> for GetBlockHeight_FFI {
//     unsafe fn get(&self) -> Box<dyn Fn(([u8; 32],)) -> u32> {
//         Box::new(move |(block_hash,): ([u8; 32],)| (self.caller)(ferment_interfaces::boxed(block_hash)))
//     }
// }
// impl ferment_interfaces::FFICallback2<([u8; 32],), u32> for GetBlockHeight_FFI {
//     unsafe fn apply(&self, args: ([u8; 32],)) -> u32 {
//         let (o_0,) = args;
//         let ffi_result = (self.caller)(ferment_interfaces::boxed(o_0));
//         ffi_result
//     }
// }
// impl ferment_interfaces::FFIFnPointer<([u8; 32],), u32> for GetBlockHeight_FFI {
//     unsafe fn register(&self) -> fn(([u8; 32], )) -> u32 {
//         let caller = self.caller;
//
//         // unsafe fn
//
//     }
// }


// pub unsafe fn ffi_add_block_height_callback(index: u32, _callback: GetBlockHeight_FFI) {
//     let callback = ferment_interfaces::FFICallback::get(&_callback);
//     let result = add_block_height_callback(
//         index,
//         callback);
//     result
// }
// pub unsafe fn ffi_add_block_height_callback2(index: u32, _callback: GetBlockHeight_FFI) {
//     let result = add_block_height_callback(
//         index,
//         |block_hash| {
//             let caller = _callback.caller;
//             _callback.lookup(block_hash, |block_hash| caller(block_hash))
//         });
//     // let result = add_block_height_callback(
//     //     index,
//     //     |block_hash| ferment_interfaces::FFICallback2::apply(&_callback, (block_hash,)));
//     result
// }
////////
// #[repr(C)]
// pub struct GetQuorumHash_FFI {
//     pub caller: fn(block_hash: *mut [u8; 32]) -> *mut std::os::raw::c_char,
//     pub destructor: fn(result: *mut std::os::raw::c_char),
//     pub context: *const std::os::raw::c_void
// }
// pub type

// pub const FIELDS_FROM_PRESENTER: ItemComposerPresenterRef<OwnerIteratorPresentationContext> =
//     |composer| composer.fields_from_composer.compose(&());
//
// pub const fn enum_variant_composer_field_presenter() -> OwnedFieldTypeComposerRef {
//     |field_type|
//         OwnedItemPresentableContext::DefaultField(field_type.clone())

// impl GetQuorumHash_FFI {
//     pub fn appl(&self) -> fn([u8; 32]) -> String {
//         // let s = self;
//         move |block_hash| {
//             let caller = self.caller;
//             // let c_: &dyn Fn([u8; 32]) -> *mut std::os::raw::c_char = |block_hash: [u8; 32]| (caller)(ferment_interfaces::boxed(block_hash));
//             let destructor = self.destructor;
//             let ffi_result = caller(ferment_interfaces::boxed(block_hash));
//             let result = unsafe { <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result) };
//             (destructor)(ffi_result);
//             result
//         }
//     }
// }
//
//
// impl GetQuorumHash_FFI {
//     pub unsafe fn _inner(&self, block_hash: [u8; 32]) -> String {
//         let ffi_result = (self.caller)(ferment_interfaces::boxed(block_hash));
//         let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
//         (self.destructor)(ffi_result);
//         result
//
//     }
// }
// impl ferment_interfaces::FFICallback2<([u8; 32],), String> for GetQuorumHash_FFI {
//     unsafe fn apply(&self, (block_hash,): ([u8; 32],)) -> String {
//         self._inner(block_hash)
//     }
// }
// impl ferment_interfaces::FFICallback<([u8; 32],), String> for GetQuorumHash_FFI {
//     unsafe fn get(&self) -> Box<dyn Fn(([u8; 32], )) -> String> {
//         Box::new(move |(block_hash,): ([u8; 32],)| self._inner(block_hash))
//     }
// }
// pub unsafe fn ffi_add_quorum_hash_callback(index: u32, _callback: GetQuorumHash_FFI) {
//     // let result = add_quorum_hash_callback(index, |block_hash,| ferment_interfaces::FFICallback2::apply(&_callback, (block_hash,)));
//     let result = add_quorum_hash_callback(index, _callback.appl());
//     result
// }
////////

// pub const GetQuorumHashOpt_C: GetQuorumHashOpt = |block_hash| {
//     let ffi_result = (self.caller)(ferment_interfaces::boxed(block_hash));
//     (!ffi_result.is_null()).then(|| {
//         let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
//         (self.destructor)(ffi_result);
//         result
//     })
// };
//
// pub type GetQuorumHashOpt_FFI_Caller = fn(block_hash: *mut [u8; 32]) -> *mut std::os::raw::c_char;
// pub type GetQuorumHashOpt_FFI_Destructor = fn(result: *mut std::os::raw::c_char);
// #[repr(C)]
// pub struct GetQuorumHashOpt_FFI {
//     caller: GetQuorumHashOpt_FFI_Caller,
//     destructor: GetQuorumHashOpt_FFI_Destructor,
//     pub context: *const std::os::raw::c_void
// }
//
// impl ferment_interfaces::FFICallback2<([u8; 32],), Option<String>> for GetQuorumHashOpt_FFI {
//     unsafe fn apply(&self, args: ([u8; 32],)) -> Option<String> {
//         let (o_0,) = args;
//         let ffi_result = (self.caller)(ferment_interfaces::boxed(o_0));
//         (!ffi_result.is_null()).then(|| {
//             let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
//             (self.destructor)(ffi_result);
//             result
//         })
//     }
// }
// pub unsafe fn ffi_add_quorum_hash_opt_callback(index: u32, _callback: GetQuorumHashOpt_FFI) {
//     let result = add_quorum_hash_opt_callback(index, |block_hash| ferment_interfaces::FFICallback2::apply(&_callback, (block_hash,)));
//     result
// }
// pub unsafe fn ffi_add_quorum_hash_opt_callback_with_complex_result(index: u32, _callback: GetQuorumHashOpt_FFI) -> *mut std::os::raw::c_char {
//     let result = add_quorum_hash_opt_callback_with_complex_result(
//         index,
//         |block_hash| ferment_interfaces::FFICallback2::apply(&_callback, (block_hash,)));
//     ferment_interfaces::FFIConversionTo::ffi_to(result)
// }

////////
// #[repr(C)]
// pub struct CallbackExample1FFI {
//     caller: fn(chain_id: u32, block_hash: *mut [u8; 32]) -> *mut std::os::raw::c_char,
//     destructor: fn(result: *mut std::os::raw::c_char),
//     pub context: *const std::os::raw::c_void
// }
// pub unsafe fn ffi1_find_current_block_desc(_callback: CallbackExample1FFI) {
//     let result = find_current_block_desc(|chain_id, block_hash| _callback.call(chain_id, block_hash));
//     result
// }
// pub unsafe fn ffi1_find_current_block_desc11(_callback: CallbackExample1FFI) {
//     let result = find_current_block_desc11(|chain_id, block_hash| _callback.call(chain_id, block_hash));
//     result
// }
// impl CallbackExample1FFI {
//     pub fn call(&self, chain_id: u32, block_hash: [u8; 32]) -> Option<String> {
//         let ffi_result = (self.caller)(chain_id, ferment_interfaces::boxed(block_hash));
//         (!ffi_result.is_null()).then(|| {
//             let result = unsafe { ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result) };
//             (self.destructor)(ffi_result);
//             result
//         })
//     }
// }
//
// impl ferment_interfaces::FFICallback2<(u32, [u8; 32]), Option<String>> for CallbackExample1FFI {
//     unsafe fn apply(&self, (chain_id, block_hash,): (u32, [u8; 32],)) -> Option<String> {
//         // let (chain_id, block_hash,) = args;
//         let ffi_result = (self.caller)(chain_id, ferment_interfaces::boxed(block_hash));
//         (!ffi_result.is_null()).then(|| {
//             let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//             (self.destructor)(ffi_result);
//             result
//         })
//     }
// }
// impl ferment_interfaces::FFICallback<(u32, [u8; 32],), Option<String>> for CallbackExample1FFI {
//     unsafe fn get(&self) -> Box<dyn Fn((u32, [u8; 32],)) -> Option<String>> {
//         // let caller = self.caller;
//         // let destructor = self.destructor;
//         Box::new(move |(chain_id, block_hash): (u32, [u8; 32],)| {
//             let ffi_result = (self.caller)(chain_id, ferment_interfaces::boxed(block_hash));
//             (!ffi_result.is_null()).then(|| {
//                 let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//                 (self.destructor)(ffi_result);
//                 result
//             })
//         })
//     }
// }
//
//
//
// impl ferment_interfaces::FFICallback<(u32, [u8; 32],), Option<String>> for CallbackExample1FFI {
//     unsafe fn apply(&self, args: (u32, [u8; 32],)) -> Option<String> {
//         let (chain_id, block_hash,) = args;
//         let ffi_result = (self.caller)(
//             chain_id,
//             ferment_interfaces::FFIConversionTo::ffi_to(block_hash));
//         (!ffi_result.is_null()).then(|| {
//             let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//             (self.destructor)(ffi_result);
//             result
//         })
//     }
// }


// impl ferment_interfaces::FFICallback<(u32, [u8; 32]), Option<String>> for CallbackExample1FFI {
//     unsafe fn get<T>(&self) -> T where T: Fn((u32, [u8; 32])) -> Option<String> {
//         let caller = self.caller;
//         let destructor = self.destructor;
//         let closure = move |(chain_id, block_hash)| {
//             let ffi_result = caller(chain_id, ferment_interfaces::boxed(block_hash));
//             (!ffi_result.is_null()).then(|| {
//                 let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
//                 destructor(ffi_result);
//                 result
//             })
//         };
//         std::mem::transmute::<_, T>(closure)
//     }
// }
// impl ferment_interfaces::FFICallback<(u32, [u8; 32]), Option<String>> for CallbackExample1FFI {
//     unsafe fn get<T>(&self) -> T
//         where T: Fn((u32, [u8; 32])) -> Option<String> {
//         let caller = self.caller;
//         let destructor = self.destructor;
//         let closure = move |args: (u32, [u8; 32])| {
//             let (chain_id, block_hash) = args;
//             let ffi_result = caller(chain_id, ferment_interfaces::boxed(block_hash));
//             (!ffi_result.is_null()).then(|| {
//                 let result = <std::os::raw::c_char as ferment_interfaces::FFIConversionFrom<String>>::ffi_from(ffi_result);
//                 destructor(ffi_result);
//                 result
//             })
//         };
//         std::mem::transmute::<_, T>(closure)
//     }
// }
// impl ferment_interfaces::FFICallback<fn(u32, [u8; 32]) -> Option<String>> for CallbackExample1FFI {
//     unsafe fn get(&self) -> fn(u32, [u8; 32]) -> Option<String> {
//         let caller = self.caller;
//         let destructor = self.destructor;
//
//         let process = move |chain_id: u32, block_hash: [u8; 32]| -> Option<String> {
//             let ffi_result = caller(chain_id, ferment_interfaces::boxed(block_hash));
//             (!ffi_result.is_null()).then(|| {
//                 let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//                 destructor(ffi_result);
//                 result
//             })
//         };
//         process
//
//         // move |chain_id, block_hash| {
//         //     let ffi_result = caller(chain_id, ferment_interfaces::boxed(block_hash));
//         //     (!ffi_result.is_null()).then(|| {
//         //         let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//         //         destructor(ffi_result);
//         //         result
//         //     })
//         // }
//     }
// }

//
// #[repr(C)]
// pub struct CallbackExample2FFI {
//     caller: fn(block_hash: *mut [u8; 32]),
//     pub context: *const std::os::raw::c_void
// }
// impl ferment_interfaces::FFICallback<([u8; 32],), ()> for CallbackExample2FFI {
//     unsafe fn apply(&self, args: ([u8; 32],)) {
//         let (block_hash,) = args;
//         let ffi_result = (self.caller)(ferment_interfaces::FFIConversionTo::ffi_to(block_hash));
//         ffi_result
//     }
// }
//
// #[repr(C)]
// pub struct CallbackExample3FFI {
//     caller: fn(block_hash: *mut [u8; 32]) -> *mut u32,
//     pub context: *const std::os::raw::c_void
// }
// impl ferment_interfaces::FFICallback<([u8; 32],), Option<u32>> for CallbackExample3FFI {
//     unsafe fn apply(&self, args: ([u8; 32],)) -> Option<u32> {
//         let (block_hash,) = args;
//         let ffi_result = (self.caller)(ferment_interfaces::FFIConversionTo::ffi_to(block_hash));
//         (!ffi_result.is_null()).then(|| *ffi_result)
//     }
// }
//
//
//////// FFI functions with callbacks



// #[repr(C)]
// pub struct CallbackExample4FFI {
//     caller: fn(chain_id: u32, chain_name: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char,
//     destructor: fn(result: *mut std::os::raw::c_char),
//     pub context: *const std::os::raw::c_void
// }
// impl ferment_interfaces::FFICallback2<(u32, String), Option<String>> for CallbackExample4FFI {
//     unsafe fn apply(&self, args: (u32, String,)) -> Option<String> {
//         let (chain_id, chain_name,) = args;
//         let ffi_result = (self.caller)(chain_id, ferment_interfaces::FFIConversionTo::ffi_to(chain_name));
//         (!ffi_result.is_null()).then(|| {
//             let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//             (self.destructor)(ffi_result);
//             result
//         })
//     }
// }
// pub unsafe fn ffi_find_current_block_desc(_callback: CallbackExample1FFI) {
//     let result = find_current_block_desc(ferment_interfaces::FFICallback::get(&_callback));
//     result
// }
//
//
// pub unsafe fn ffi2_find_current_block_desc(_callback: CallbackExample1FFI) {
//     let result = find_current_block_desc(
//         |chain_id, block_hash|
//             ferment_interfaces::FFICallback::apply(&_callback, (chain_id, block_hash)));
//     result
// }
// pub unsafe fn ffi4_find_current_block_desc(_callback: CallbackExample4FFI) {
//
//     // let func: CallbackExample4 = |chain_id, chain_name| {
//     //     let ffi_result = (_callback.caller)(chain_id, ferment_interfaces::FFIConversionTo::ffi_to(chain_name));
//     //     (!ffi_result.is_null()).then(|| {
//     //         let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//     //         (_callback.destructor)(ffi_result);
//     //         result
//     //     })
//     // };
//
//     let caller_fn = |chain_id: u32, chain_name: String| {
//         (_callback.caller)(chain_id, ferment_interfaces::FFIConversionTo::ffi_to(chain_name))
//     };
//     // let destroy =
//
//     unsafe fn call(chain_id: u32, chain_name: String) -> Option<String> {
//         let ffi_result = caller_fn(chain_id, chain_name);
//         (!ffi_result.is_null()).then(|| {
//             let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//             (_callback.destructor)(ffi_result);
//             result
//         })
//     }
//     // let func: fn(u32, String) -> Option<String> = unsafe fn(chain_id, chain_name) -> Option<String> {
//     //     let ffi_result = (_callback.caller)(chain_id, ferment_interfaces::FFIConversionTo::ffi_to(chain_name));
//     //     (!ffi_result.is_null()).then(|| {
//     //         let result = ferment_interfaces::FFIConversionFrom::ffi_from(ffi_result);
//     //         (_callback.destructor)(ffi_result);
//     //         result
//     //     })
//     // };
//
//     // let result = find_current_block_desc(callback_);
//     // result
//     let result = find_current_block_desc4(|chain_id, chain_name| call(chain_id, chain_name));
//     result
// }
//
//
// // pub unsafe fn ffi_just_simple_func_to_compare(index: u32, string: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
// //     let result = just_simple_func_to_compare(
// //         index,
// //         ferment_interfaces::FFIConversionFrom::ffi_from(string));
// //     ferment_interfaces::FFIConversionTo::ffi_to(result)
// // }
// // ////////
// //
//
//
//
