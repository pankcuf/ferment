use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use example_simple::errors::protocol_error::ProtocolError;
use crate::model::{LLMQSnapshot, Quorum};

#[ferment_macro::export]
pub struct AllMapExamples {
    pub k_simple_v_simple: BTreeMap<u32, u32>,
    pub k_simple_v_opt_simple: BTreeMap<u32, Option<u32>>,
    pub k_simple_v_opt_complex: BTreeMap<u32, Option<String>>,
    pub k_simple_v_opt_generic_simple: BTreeMap<u32, Option<Vec<u32>>>,
    pub k_simple_v_opt_generic_complex: BTreeMap<u32, Option<Vec<String>>>,
    pub opt_map_k_simple_v_simple: Option<BTreeMap<u32, u32>>,
    pub opt_map_k_simple_v_complex: Option<BTreeMap<u32, String>>,
    pub opt_map_k_simple_v_generic: Option<BTreeMap<u32, Vec<u8>>>,
    pub opt_map_k_generic_v_generic: Option<BTreeMap<Vec<u8>, Vec<u8>>>,
    pub map_k_opt_generic_v_opt_generic: BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>,
}
#[ferment_macro::export]
pub struct AllResultExamples {
    pub result_ok_simple_err_simple: Result<u32, u32>,
    pub result_ok_complex_err_complex: Result<String, String>,
    pub result_ok_complex_2_err_complex: Result<LLMQSnapshot, LLMQSnapshot>,
    pub result_ok_complex_err_generic: Result<String, Vec<u8>>,
    pub result_ok_complex_err_opt_simple: Result<String, Option<u32>>,
    pub result_ok_complex_err_opt_complex: Result<String, Option<String>>,
    pub result_ok_complex_err_opt_generic: Result<String, Option<Vec<u8>>>,
    pub crazy_type: Result<Option<LLMQSnapshot>, ProtocolError>,
    pub crazy_type_2: Result<LLMQSnapshot, Option<ProtocolError>>,

}
#[ferment_macro::export]
pub struct AllSetExamples {
    pub btreeset_simple: BTreeSet<u32>,
    pub btreeset_complex: BTreeSet<String>,
    pub btreeset_generic: BTreeSet<Vec<u8>>,
    pub btreeset_opt_simple: BTreeSet<Option<u32>>,
    pub btreeset_opt_complex: BTreeSet<Option<String>>,
    pub btreeset_opt_generic: BTreeSet<Option<Vec<u8>>>,

    pub hashset_simple: HashSet<u32>,
    pub hashset_complex: HashSet<String>,
    pub hashset_generic: HashSet<Vec<u8>>,
    pub hashset_opt_simple: HashSet<Option<u32>>,
    pub hashset_opt_complex: HashSet<Option<String>>,
    pub hashset_opt_generic: HashSet<Option<Vec<u8>>>,

    pub hashset_opt_complex_external: HashSet<Option<ProtocolError>>,

}
#[ferment_macro::export]
pub struct AllVecExamples {
    pub vec_simple: Vec<u32>,
    pub vec_complex: Vec<String>,
    pub vec_generic: Vec<Vec<u8>>,
    pub vec_opt_simple: Vec<Option<u32>>,
    pub vec_opt_complex: Vec<Option<String>>,
    pub vec_opt_generic: Vec<Option<Vec<u8>>>,

}
#[ferment_macro::export]
pub struct AllArrExamples {
    pub arr: [u8; 32],
    pub opt_arr: Option<[u8; 32]>,
    pub complex_arr: [String; 32],
    pub complex_arr_2: [Quorum; 32],
    pub generic_arr_2: [Vec<u8>; 32],
}
#[ferment_macro::export]
pub struct AllTupleExamples {
    pub tuple_string: (String, String),
    pub tuple_with_generic: (String, Vec<String>),
}
#[ferment_macro::export]
pub struct AllOptExamples {
    pub opt_complex: Option<String>,
}

#[ferment_macro::export]
pub struct AllArcExamples {
    pub arc_simple: Arc<u32>,
    pub arc_complex: Arc<LLMQSnapshot>,
    pub arc_generic: Arc<Vec<u8>>,
    pub arc_opt_generic: Arc<Option<BTreeMap<u32, LLMQSnapshot>>>,
    pub opt_arc_complex: Option<Arc<Option<String>>>,
    pub crazy_type1: Result<Option<Vec<LLMQSnapshot>>, ProtocolError>,
    pub crazy_type2: Result<Option<Arc<LLMQSnapshot>>, ProtocolError>,
}

#[ferment_macro::export]
pub struct AllRcExamples {
    pub arc_simple: Rc<u32>,
    pub arc_complex: Rc<LLMQSnapshot>,
    pub arc_generic: Rc<Vec<u8>>,
    pub arc_opt_generic: Rc<Option<BTreeMap<u32, LLMQSnapshot>>>,
    pub opt_arc_complex: Option<Rc<Option<String>>>,
}
#[ferment_macro::export]
pub struct AllMutexExamples {
    pub mutex_simple: Mutex<u32>,
    pub mutex_complex: Mutex<LLMQSnapshot>,
    pub mutex_generic: Mutex<Vec<u8>>,
    pub mutex_opt_generic: Mutex<Option<BTreeMap<u32, LLMQSnapshot>>>,
    pub opt_mutex_complex: Option<Mutex<Option<String>>>,
    pub platform_case: Mutex<Option<Box<LLMQSnapshot>>>,
}

#[ferment_macro::opaque]
#[derive(Clone)]
pub struct CacheExample {
    pub _map: Arc<RwLock<BTreeMap<u32, LLMQSnapshot>>>,
}

#[ferment_macro::export]
pub struct AllRwLockExamples {
    pub rwlock_simple: RwLock<u32>,
    pub rwlock_complex: RwLock<LLMQSnapshot>,
    pub rwlock_generic: RwLock<Vec<u8>>,
    pub rwlock_opt_generic: RwLock<Option<BTreeMap<u32, LLMQSnapshot>>>,
    pub opt_rwlock_complex: Option<RwLock<Option<String>>>,
    pub arc_rw_lock_complex: Arc<RwLock<LLMQSnapshot>>,
    pub arc_rw_lock_complex_opaque: Arc<CacheExample>,
}
#[ferment_macro::export]
pub struct AllRefCellExamples {
    pub refcell_simple: RefCell<u32>,
    pub refcell_complex: RefCell<LLMQSnapshot>,
    pub refcell_generic: RefCell<Vec<u8>>,
    pub refcell_opt_generic: RefCell<Option<BTreeMap<u32, LLMQSnapshot>>>,
    pub opt_refcell_complex: Option<RefCell<Option<String>>>,
}

// #[ferment_macro::export]
// pub struct AllPinExamples {
//     pub pin_simple: Pin<Box<u32>>,
//     // pub pin_complex: Pin<Box<LLMQSnapshot>>,
//     // pub pin_generic: Pin<Box<Vec<u8>>>,
//     // pub pin_opt_generic: Pin<Box<Option<BTreeMap<u32, LLMQSnapshot>>>>,
//     // pub pin_arc_complex: Option<Pin<Box<Option<String>>>>,
// }

// TODO: make it work (as_slice() doesn't)
// #[ferment_macro::export]
// pub struct AllSliceExamples<'a> {
//     pub simple: &'a [u8],
//     // pub opt_simple: Option<&'a [u8]>,
// }

#[ferment_macro::export]
pub struct AllExamples {
    pub name: String,
    pub all_map_examples: AllMapExamples,
    pub all_result_examples: AllResultExamples,
    pub all_set_examples: AllSetExamples,
    pub all_arr_examples: AllArrExamples,
    pub all_tuple_examples: AllTupleExamples,
    pub all_opt_examples: AllOptExamples,
    // pub opt_arc: Arc<String>,
    // pub indexes: Option<[u8; 32]>,
}
