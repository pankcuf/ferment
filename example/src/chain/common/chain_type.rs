use rs_ffi_macro_derive::ferment;

#[ferment]
#[derive(Clone, PartialOrd, Ord, Hash, Eq, PartialEq)]
pub enum ChainType {
    MainNet,
    TestNet,
}

impl ChainType {
    pub fn get_string(&self) -> String {
        match self {
            Self::MainNet => "mainnet".to_string(),
            Self::TestNet => "testnet".to_string(),
        }
    }
}
