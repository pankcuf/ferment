use indexmap::{IndexMap, IndexSet};

#[ferment_macro::export]
pub struct IndexMapExample {
    pub set: IndexSet<String>,
    pub map: IndexMap<String, String>,
}

