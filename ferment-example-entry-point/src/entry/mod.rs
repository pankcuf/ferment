pub mod core;
pub mod processor;
pub mod provider;
pub mod rnt;


#[ferment_macro::export]
pub struct SomeModel {
    pub hash: [u8; 32],
    pub desc: String,
}

#[ferment_macro::opaque]
pub type BlockHashByHeight = unsafe extern "C" fn(u32) -> [u8; 32];
#[ferment_macro::opaque]
pub type ModelByHeight = unsafe extern "C" fn(u32) -> SomeModel;
