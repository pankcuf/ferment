
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub enum QuorumType {
    Normal,
    Rotated
}

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub enum OBJCEnumTest {
    Regular,
    UnnamedEx(String),
    NamedEx { qtype: String }
}