pub mod snapshot;
pub mod quorum;

pub use snapshot::LLMQSnapshot;
pub use snapshot::LLMQSnapshotSkipMode;
pub use quorum::QuorumType;

#[ferment_macro::export]
pub enum TestModLevelSnapshot {
    VO(LLMQSnapshot)
}
#[ferment_macro::export]
pub enum TestModLevelVecSnapshot {
    VO(Vec<LLMQSnapshotSkipMode>)
}

#[ferment_macro::export]
pub enum TestModLevelOptSnapshot {
    VO(Option<LLMQSnapshotSkipMode>)
}

#[ferment_macro::export]
pub struct Quorum {
    pub llmq_type: QuorumType
}

pub mod ferment_example {
    use crate::model::{Quorum, QuorumType};

    pub fn get_rotated_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Rotated }
    }

    pub fn get_crazy_case() -> Quorum {
        Quorum { llmq_type: QuorumType::Rotated }
    }
}