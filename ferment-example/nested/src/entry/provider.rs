use crate::entry::{BlockHashByHeight, ModelByHeight, SomeModel};

#[ferment_macro::opaque]
pub trait CoreProvider {
    #[allow(unused)]
    fn get_block_hash_by_height(&self, height: u32) -> [u8; 32];
    #[allow(unused)]
    fn model_by_height(&self, height: u32) -> SomeModel;
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

    fn model_by_height(&self, height: u32) -> SomeModel {
        unsafe { (self.model_by_height)(height) }
    }
}
// #[ferment_macro::opaque]
pub struct FFITraitCoreProvider {
    pub block_hash_by_height: Box<dyn Fn(u32) -> [u8; 32]>,
    pub model_by_height: Box<dyn Fn(u32) -> SomeModel>,
}

impl CoreProvider for FFITraitCoreProvider {
    fn get_block_hash_by_height(&self, height: u32) -> [u8; 32] {
        (self.block_hash_by_height)(height)
    }

    fn model_by_height(&self, height: u32) -> SomeModel {
        (self.model_by_height)(height)
    }
}
