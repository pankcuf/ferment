#[derive(Debug, Eq, PartialEq)]
#[ferment_macro::export]
pub struct InstantAssetLockProof {
    pub output_index: u32,
}
