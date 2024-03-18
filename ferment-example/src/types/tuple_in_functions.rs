use crate::nested::HashID;

#[ferment_macro::export]
pub struct StructWithTuple {
    pub tuple: (u32, HashID)
}

#[ferment_macro::export]
pub fn get_tuple_simple() -> (u32, u32) {
    (0, 0)
}

#[ferment_macro::export]
pub fn get_tuple_simple_complex() -> (u32, HashID) {
    (0, [0u8; 32])
}

#[ferment_macro::export]
pub fn get_tuple_complex_complex(tuple: (u32, HashID)) -> u32 {
    tuple.0
}

#[ferment_macro::export]
pub fn get_vec_of_tuples() -> Vec<(HashID, HashID)> {
    vec![]
}

#[ferment_macro::export]
pub fn set_vec_of_tuples(vec: Vec<(HashID, HashID)>) {
    println!("set_vec: {:?}", vec)
}
