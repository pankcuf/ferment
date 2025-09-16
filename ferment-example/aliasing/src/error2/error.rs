#[derive(Debug)]
#[ferment_macro::export]
pub struct Error {
    pub value: String,
}