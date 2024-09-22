

use example_simple::errors::protocol_error::ProtocolError;

#[ferment_macro::export]
pub fn find_current_block_desc<T: Fn(u32, [u8; 32]) -> Option<String>>(_callback: T) {
    println!("find_current_block_desc: ");
}
#[ferment_macro::opaque]
pub type ClassicCallback = unsafe extern "C" fn(u32);
#[ferment_macro::export]
pub fn find_current_block_classic(_callback: ClassicCallback) {
    println!("find_current_block_desc: ");
}
#[ferment_macro::export]
pub fn find_current_block_desc_mut<T: FnMut(u32, [u8; 32]) -> Option<String>>(_callback: T) {
    println!("find_current_block_desc_mut: ");
}

#[ferment_macro::export]
pub fn lookup_block_hash_by_height<T: Fn(u32) -> Option<[u8; 32]>>(_callback: T) {
    println!("lookup_block_hash_by_height:");
}
#[ferment_macro::export]
pub fn lookup_merkle_root_by_hash<T: Fn([u8; 32]) -> Option<[u8; 32]>>(_callback: T) {
    println!("lookup_merkle_root_by_hash:");
}
#[ferment_macro::export]
pub fn should_process_diff_in_range2<T: Fn([u8; 32], [u8; 32]) -> Result<u32, ProtocolError>>(_callback: T) {
    println!("should_process_diff_in_range:");
}

#[ferment_macro::export]
pub fn setup_two_callbacks<
    T: Fn([u8; 32], [u8; 32]) -> Result<u32, ProtocolError>,
    U: Fn(u32) -> Result<u32, ProtocolError>>(_callback1: T, _callback2: U) {
    println!("should_process_diff_in_range:");
}
