mod fermented;
// mod model;
// mod gen;
mod entry;


extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
}
