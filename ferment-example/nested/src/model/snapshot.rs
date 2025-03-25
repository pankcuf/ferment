use dashcore::bls_sig_utils::BLSSignature;

#[derive(Clone)]
#[ferment_macro::export]
pub enum LLMQSnapshotSkipMode {
    NoSkipping = 0,
    SkipFirst = 1,
    SkipExcept = 2,
    SkipAll = 3,
}

#[derive(Clone)]
#[ferment_macro::export]
pub struct LLMQSnapshot {
    pub member_list: Vec<u8>,
    pub skip_list: Vec<i32>,
    pub skip_list_mode: LLMQSnapshotSkipMode,
    pub option_vec: Option<Vec<u8>>,
}

impl Default for LLMQSnapshot {
    fn default() -> Self {
        Self { member_list: vec![], skip_list: vec![], skip_list_mode: LLMQSnapshotSkipMode::NoSkipping, option_vec: None }
    }
}


#[derive(Clone)]
#[ferment_macro::export]
pub enum VerifyingChainLockSignaturesType {
    Rotating([BLSSignature; 4]),
    NonRotating(BLSSignature),
}
