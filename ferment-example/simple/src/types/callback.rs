#[ferment_macro::export]
pub type QuorumPublicKeyCallback = extern "C" fn(quorum_type: u32, quorum_hash: [u8; 48], core_chain_locked_height: u32) -> [u8; 32];

#[ferment_macro::export]
pub fn get_quorum<T: Fn(u32, [u8; 48], u32) -> [u8; 32]>(callback: T) -> [u8; 32] {
    callback(0, [0; 48], 0)
}