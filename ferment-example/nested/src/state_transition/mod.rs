#[cfg(feature = "state-transitions")]
use dpp::data_contract::DataContract;
use dpp::data_contract::errors::DataContractError;
use dpp::errors::consensus::basic::BasicError;
#[cfg(feature = "state-transitions")]
use dpp::errors::ProtocolError;
#[cfg(feature = "state-transitions")]
use dpp::state_transition::state_transitions::contract::data_contract_update_transition::DataContractUpdateTransition;
#[cfg(feature = "state-transitions")]
use dpp::version::PlatformVersion;
#[cfg(feature = "state-transitions")]
use platform_version::TryFromPlatformVersioned;
pub mod state_transitions;
#[derive(Clone, Debug)]
#[ferment_macro::opaque]
pub struct ContractsManager {}

#[ferment_macro::export]
impl ContractsManager {
    #[cfg(feature = "state-transitions")]
    pub fn contract_registration_transition(&self, contract: DataContract) -> Result<DataContractUpdateTransition, ProtocolError> {
        let v = PlatformVersion::first();
        DataContractUpdateTransition::try_from_platform_versioned((contract, 0), v)
    }

    pub fn basic_errror(&self) -> BasicError {
        BasicError::ContractError(DataContractError::FieldRequirementUnmet("".to_string()))
    }

}
