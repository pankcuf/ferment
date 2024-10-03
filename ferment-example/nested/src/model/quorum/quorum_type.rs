
#[derive(Clone, Debug)]
#[ferment_macro::export]
pub enum QuorumType {
    Normal,
    Rotated
}