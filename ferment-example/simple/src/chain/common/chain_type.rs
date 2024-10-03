use std::collections::BTreeMap;
use crate::nested::{HashID, ProtocolError};

#[ferment_macro::export]
pub trait IHaveChainSettings {
    fn ty() -> &'static str;
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
    fn find_masternode_list(&self, cached_mn_lists: &BTreeMap<HashID, HashID>, unknown_mn_lists: &mut Vec<HashID>) -> Result<HashID, ProtocolError> {
        if let Some(first) = unknown_mn_lists.first() {
            Ok(*first)
        } else if let Some(first) = cached_mn_lists.first_key_value() {
            Ok(*first.1)
        } else {
            Err(ProtocolError::IdentifierError("ERRERERERERE".to_string()))
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialOrd, Ord, Hash, Eq, PartialEq)]
// #[ferment_macro::export(IHaveChainSettings)]
#[ferment_macro::export]
pub enum ChainType {
    MainNet,
    TestNet,
    DevNet(DevnetType)
}

#[derive(Clone, Default, PartialOrd, Ord, Hash, Eq, PartialEq)]
// #[ferment_macro::export(IHaveChainSettings)]
#[ferment_macro::export]
pub enum DevnetType {
    JackDaniels = 0,
    Devnet333 = 1,
    Chacha = 2,
    #[default]
    Mojito = 3,
    WhiteRussian = 4,
}

#[ferment_macro::export]
impl IHaveChainSettings for ChainType {
    fn ty() -> &'static str {
        "ChainType"
    }

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

#[ferment_macro::export]
impl IHaveChainSettings for DevnetType {
    fn ty() -> &'static str {
        "DevnetType"
    }

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
