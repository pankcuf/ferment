#[derive(Debug, Eq, PartialEq)]
#[ferment_macro::export]
pub struct ChainAssetLockProof {
    pub core_chain_locked_height: u32,
}
