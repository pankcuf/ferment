#[derive(Default, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[ferment_macro::export]
pub struct ScriptBuf(pub Vec<u8>);


pub mod inner {
    use super::*;

    #[ferment_macro::export]
    pub fn inner_func(value: ScriptBuf) {
        println!("{value:?}");
    }
}