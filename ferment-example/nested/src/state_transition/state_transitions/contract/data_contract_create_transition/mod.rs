pub mod v0;

pub use v0::DataContractCreateTransitionV0;

#[ferment_macro::export]
pub enum DataContractCreateTransition {
    V0(DataContractCreateTransitionV0),
}
