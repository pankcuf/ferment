use crate::ScriptBuf;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[ferment_macro::export]
pub struct ProviderUpdateRegistrarPayload {
    pub version: u16,
    pub script_payout: ScriptBuf,
}
