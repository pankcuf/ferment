#[ferment_macro::export]
pub type GetBlockHeight = fn(block_hash: [u8; 32]) -> u32;


// #[ferment_macro::export]
// pub fn register_callback(callback: GetBlockHeight) {
//     println!("register_callback: {:?}", callback.type_id());
// }