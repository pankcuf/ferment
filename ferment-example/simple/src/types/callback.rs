#[ferment_macro::export]
pub type QuorumPublicKeyCallback = extern "C" fn(quorum_type: u32, quorum_hash: [u8; 48], core_chain_locked_height: u32) -> [u8; 32];

