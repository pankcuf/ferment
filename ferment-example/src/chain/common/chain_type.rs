use crate::nested::HashID;

#[ferment_macro::export]
pub trait IHaveChainSettings {
    fn name(&self) -> String;
    fn genesis_hash(&self) -> HashID;
    fn genesis_height(&self) -> u32;
    fn has_genesis_hash(&self, hash: HashID) -> bool {
        self.genesis_hash() == hash
    }
    fn get_hash_by_hash(&self, hash: HashID) -> HashID {
        hash
    }
    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool;
}

#[allow(clippy::enum_variant_names)]
#[ferment_macro::export(IHaveChainSettings)]
#[derive(Clone, PartialOrd, Ord, Hash, Eq, PartialEq)]
pub enum ChainType {
    MainNet,
    TestNet,
    DevNet(DevnetType)
}

#[derive(Clone, Default, PartialOrd, Ord, Hash, Eq, PartialEq)]
#[ferment_macro::export(IHaveChainSettings)]
pub enum DevnetType {
    JackDaniels = 0,
    Devnet333 = 1,
    Chacha = 2,
    #[default]
    Mojito = 3,
    WhiteRussian = 4,
}

impl IHaveChainSettings for ChainType {
    fn name(&self) -> String {
        match self {
            Self::MainNet => "mainnet".to_string(),
            Self::TestNet => "testnet".to_string(),
            Self::DevNet(devnet) => devnet.name()
        }
    }

    fn genesis_hash(&self) -> HashID {
        [0u8; 32]
    }

    fn genesis_height(&self) -> u32 {
        0
    }

    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool {
        llmq_type != 0
    }
}

impl IHaveChainSettings for DevnetType {
    fn name(&self) -> String {
        format!("devnet-{}", match self {
            DevnetType::JackDaniels => "jack-daniels",
            DevnetType::Devnet333 => "333",
            DevnetType::Chacha => "chacha",
            DevnetType::Mojito => "mojito",
            DevnetType::WhiteRussian => "white-russian",
        })
    }

    fn genesis_hash(&self) -> HashID {
        [0u8; 32]
    }

    fn genesis_height(&self) -> u32 {
        1
    }

    fn should_process_llmq_of_type(&self, llmq_type: u16) -> bool {
        llmq_type != 0
    }
}

#[allow(dead_code)]
pub enum ExcludedEnum {
    Variant1,
    Variant2,
}
