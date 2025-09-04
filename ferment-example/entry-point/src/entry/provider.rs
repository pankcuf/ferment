use crate::entry::{BlockHashByHeight, ModelByHeight};

#[ferment_macro::opaque]
pub trait CoreProvider {
    fn get_block_hash_by_height(&self, height: u32) -> [u8; 32];
    fn model_by_height(&self, height: u32) -> u64;
}
#[ferment_macro::opaque]
pub struct FFIPtrCoreProvider {
    pub block_hash_by_height: BlockHashByHeight,
    pub model_by_height: ModelByHeight,
}
impl CoreProvider for FFIPtrCoreProvider {
    fn get_block_hash_by_height(&self, height: u32) -> [u8; 32] {
        unsafe { (self.block_hash_by_height)(height) }
    }

    fn model_by_height(&self, height: u32) -> u64 {
        unsafe { (self.model_by_height)(height) }
    }
}
// #[ferment_macro::opaque]
pub struct FFITraitCoreProvider {
    pub block_hash_by_height: Box<dyn Fn(u32) -> [u8; 32]>,
    pub model_by_height: Box<dyn Fn(u32) -> u64>,
}

impl CoreProvider for FFITraitCoreProvider {
    fn get_block_hash_by_height(&self, height: u32) -> [u8; 32] {
        (self.block_hash_by_height)(height)
    }

    fn model_by_height(&self, height: u32) -> u64 {
        (self.model_by_height)(height)
    }
}
