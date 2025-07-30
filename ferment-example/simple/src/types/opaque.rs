use std::collections::{BTreeMap, HashSet};

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[ferment_macro::opaque]
pub struct OpaqueCloneableType {
    pub field1: u32,
    pub field2: String
}

#[ferment_macro::export]
pub fn opaque_cloneable_function(input: OpaqueCloneableType) -> OpaqueCloneableType {
    input
}
#[ferment_macro::export]
pub fn opaque_cloneable_with_ref(input: &OpaqueCloneableType) -> u32 {
    input.field1
}
#[ferment_macro::export]
pub fn opaque_cloneable_modifier(input: &mut OpaqueCloneableType, value: u32) {
    input.field1 = value
}

#[ferment_macro::export]
pub fn maybe_opaque_cloneable(input: OpaqueCloneableType) -> Result<OpaqueCloneableType, u32> {
    Ok(input)
}
#[ferment_macro::export]
pub fn opaque_cloneable_vec(input: OpaqueCloneableType) -> Vec<OpaqueCloneableType> {
    vec![input]
}
#[ferment_macro::export]
pub fn opaque_cloneable_map1(input: OpaqueCloneableType) -> BTreeMap<u32, OpaqueCloneableType> {
    BTreeMap::from_iter([(0, input)])
}
#[ferment_macro::export]
pub fn opaque_cloneable_map2(input: OpaqueCloneableType) -> BTreeMap<OpaqueCloneableType, u32> {
    BTreeMap::from_iter([(input, 0)])
}
#[ferment_macro::export]
pub fn opaque_cloneable_map3(input: OpaqueCloneableType) -> BTreeMap<u32, Option<OpaqueCloneableType>> {
    BTreeMap::from_iter([(0, Some(input))])
}
#[ferment_macro::export]
pub fn opaque_cloneable_set(input: OpaqueCloneableType) -> HashSet<OpaqueCloneableType> {
    HashSet::from_iter([input])
}

// #[ferment_macro::export]
// pub fn opaque_cloneable_arr(input: OpaqueCloneableType) -> [OpaqueCloneableType; 1] {
//     [input]
// }
// #[ferment_macro::export]
// pub fn opaque_cloneable_slice(_input: &[OpaqueCloneableType]) -> u32 {
//     0u32
// }
#[ferment_macro::export]
pub fn opaque_cloneable_tuple(input: (OpaqueCloneableType, u32)) -> (OpaqueCloneableType, u32) {
    input
}

#[ferment_macro::export]
pub fn opaque_cloneable_callback<T: Fn(OpaqueCloneableType)>(input: OpaqueCloneableType, callback: T) {
    callback(input)
}

#[ferment_macro::export]
impl OpaqueCloneableType {
    pub fn new(field1: u32, field2: String) -> OpaqueCloneableType {
        OpaqueCloneableType { field1, field2 }
    }
    pub fn field1(&self) -> u32 {
        self.field1
    }
    pub fn field2(&self) -> String {
        self.field2.clone()
    }
    pub fn update_field1(&mut self, value: u32) {
        self.field1 = value;
    }
    pub fn update_field2(&mut self, value: String) {
        self.field2 = value;
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash)]
#[ferment_macro::opaque]
pub struct OpaqueNonCloneableType {
    pub field1: u32,
    pub field2: String
}

#[ferment_macro::export]
pub fn opaque_non_cloneable_function(input: OpaqueNonCloneableType) -> OpaqueNonCloneableType {
    input
}
#[ferment_macro::export]
pub fn opaque_non_cloneable_with_ref(input: &OpaqueNonCloneableType) -> u32 {
    input.field1
}
#[ferment_macro::export]
pub fn opaque_non_cloneable_modifier(input: &mut OpaqueNonCloneableType, value: u32) {
    input.field1 = value
}
#[ferment_macro::export]
pub fn maybe_opaque_non_cloneable(input: OpaqueNonCloneableType) -> Result<OpaqueNonCloneableType, u32> {
    Ok(input)
}
#[ferment_macro::export]
pub fn opaque_non_cloneable_vec(input: OpaqueNonCloneableType) -> Vec<OpaqueNonCloneableType> {
    vec![input]
}
#[ferment_macro::export]
pub fn opaque_non_cloneable_map1(input: OpaqueNonCloneableType) -> BTreeMap<u32, OpaqueNonCloneableType> {
    BTreeMap::from_iter([(0, input)])
}
// #[ferment_macro::export]
// pub fn opaque_non_cloneable_map2(input: OpaqueNonCloneableType) -> BTreeMap<OpaqueNonCloneableType, u32> {
//     BTreeMap::from_iter([(input, 0)])
// }

#[ferment_macro::export]
pub fn opaque_non_cloneable_set(input: OpaqueNonCloneableType) -> HashSet<OpaqueNonCloneableType> {
    HashSet::from_iter([input])
}

// #[ferment_macro::export]
// pub fn opaque_non_cloneable_arr(input: OpaqueNonCloneableType) -> [OpaqueNonCloneableType; 1] {
//     [input]
// }

// #[ferment_macro::export]
// pub fn opaque_non_cloneable_slice(_input: &[OpaqueNonCloneableType]) -> u32 {
//     0u32
// }
#[ferment_macro::export]
pub fn opaque_non_cloneable_tuple(input: (OpaqueNonCloneableType, u32)) -> (OpaqueNonCloneableType, u32) {
    input
}
#[ferment_macro::export]
pub fn opaque_non_cloneable_callback<T: Fn(OpaqueNonCloneableType)>(input: OpaqueNonCloneableType, callback: T) {
    callback(input)
}

#[ferment_macro::export]
impl OpaqueNonCloneableType {
    pub fn new(field1: u32, field2: String) -> OpaqueNonCloneableType {
        OpaqueNonCloneableType { field1, field2 }
    }
    pub fn field1(&self) -> u32 {
        self.field1
    }
    pub fn field2(&self) -> String {
        self.field2.clone()
    }
    pub fn update_field1(&mut self, value: u32) {
        self.field1 = value;
    }
    pub fn update_field2(&mut self, value: String) {
        self.field2 = value;
    }
}
