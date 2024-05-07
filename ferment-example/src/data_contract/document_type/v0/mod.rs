
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Clone)]
#[ferment_macro::export]
pub struct DocumentTypeV0 {
    pub name: String,
    pub binary_paths: BTreeSet<String>,
    // pub binary_map: BTreeMap<String, String>,
    // pub s_paths: BTreeSet<u32>,
}
