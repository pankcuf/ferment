#[ferment_macro::export]
pub trait Trait111 {
    fn method1(&self) -> String;
}

#[ferment_macro::opaque]
pub struct Struct111 {
    pub _field1: i32,
}

#[ferment_macro::export]
impl Struct111 {
    pub fn new() -> Self {
        Self { _field1: 0 }
    }
}

#[ferment_macro::export]
impl Trait111 for Struct111 {
    fn method1(&self) -> String {
        todo!()
    }
}