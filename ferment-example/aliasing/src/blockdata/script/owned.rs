#[derive(Default, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[ferment_macro::export]
pub struct ScriptBuf(pub Vec<u8>);
