pub use instant::*;
use crate::identity::state_transition::asset_lock_proof::chain::ChainAssetLockProof;

pub mod chain;
pub mod instant;
#[derive(Debug, Eq, PartialEq)]
#[ferment_macro::export]
pub enum AssetLockProof {
    Instant(InstantAssetLockProof),
    Chain(ChainAssetLockProof),
}
