use crate::chain::common::chain_type::{ChainType, IHaveChainSettings};

#[ferment_macro::export]
pub async fn get_chain_type_string_async(chain_type: ChainType) -> String {
    chain_type.name()
}
