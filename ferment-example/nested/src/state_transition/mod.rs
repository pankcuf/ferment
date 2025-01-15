#[cfg(feature = "state-transitions")]
use dpp::data_contract::DataContract;
#[cfg(feature = "state-transitions")]
use dpp::errors::ProtocolError;
#[cfg(feature = "state-transitions")]
use dpp::state_transition::state_transitions::contract::data_contract_create_transition::DataContractCreateTransition;
#[cfg(feature = "state-transitions")]
use dpp::version::PlatformVersion;
#[cfg(feature = "state-transitions")]
use platform_version::TryFromPlatformVersioned;
pub mod state_transitions;

// pub use state_transitions::*;



#[derive(Clone, Debug)]
#[ferment_macro::opaque]
pub struct ContractsManager {

}
#[ferment_macro::export]
impl ContractsManager {
    #[cfg(feature = "state-transitions")]
    pub fn contract_registration_transition(&self, contract: DataContract) -> Result<DataContractCreateTransition, ProtocolError> {
        let v = PlatformVersion::first();
        DataContractCreateTransition::try_from_platform_versioned(contract, v)
    }

}
